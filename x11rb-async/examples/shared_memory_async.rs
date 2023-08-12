use std::fs::{remove_file, File, OpenOptions};
use std::io::{Result as IOResult, Write};
use std::ptr::null_mut;

use async_executor::LocalExecutor;
use rustix::mm;

use x11rb_async::connection::Connection;
use x11rb_async::errors::{ConnectionError, ReplyError, ReplyOrIdError};
use x11rb_async::protocol::shm::{self, ConnectionExt as _};
use x11rb_async::protocol::xproto::{self, ConnectionExt as _, ImageFormat};
use x11rb_async::rust_connection::RustConnection;

const TEMP_FILE_CONTENT: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0xff, 0xfe, 0xfd, 0xfc];

/// Get the supported SHM version from the X11 server
async fn check_shm_version(conn: &impl Connection) -> Result<Option<(u16, u16)>, ReplyError> {
    if conn
        .extension_information(shm::X11_EXTENSION_NAME)
        .await?
        .is_none()
    {
        return Ok(None);
    }

    let shm_version = conn.shm_query_version().await?.reply().await?;
    Ok(Some((shm_version.major_version, shm_version.minor_version)))
}

/// Get the bytes describing the first pixel at the given offset of the given shared memory segment
/// (interpreted in the screen's root_visual)
async fn get_shared_memory_content_at_offset(
    conn: &impl Connection,
    screen: &xproto::Screen,
    shmseg: shm::Seg,
    offset: u32,
) -> Result<Vec<u8>, ReplyOrIdError> {
    let width = match screen.root_depth {
        24 => 1,
        16 => 2,
        8 => 4,
        _ => panic!("I do not know how to handle depth {}", screen.root_depth),
    };
    let pixmap = conn.generate_id().await?;
    conn.shm_create_pixmap(
        pixmap,
        screen.root,
        width,
        1,
        screen.root_depth,
        shmseg,
        offset,
    )
    .await?;

    let image = xproto::get_image(conn, ImageFormat::Z_PIXMAP, pixmap, 0, 0, width, 1, !0)
        .await?
        .reply()
        .await?;

    conn.free_pixmap(pixmap).await?;

    Ok(image.data)
}

async fn use_shared_mem(
    conn: &impl Connection,
    screen_num: usize,
    shmseg: shm::Seg,
) -> Result<(), ReplyOrIdError> {
    let screen = &conn.setup().roots[screen_num];

    let content = get_shared_memory_content_at_offset(conn, screen, shmseg, 0).await?;
    assert_eq!(content[..], TEMP_FILE_CONTENT[0..4]);

    let content = get_shared_memory_content_at_offset(conn, screen, shmseg, 4).await?;
    assert_eq!(content[..], TEMP_FILE_CONTENT[4..8]);

    Ok(())
}

/// Make a temporary file
fn make_file() -> IOResult<File> {
    let file_name = "shared_memory.bin";
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(file_name)?;
    file.write_all(&TEMP_FILE_CONTENT)?;
    remove_file(file_name)?;
    Ok(file)
}

async fn send_fd(
    conn: &impl Connection,
    screen_num: usize,
    file: File,
) -> Result<(), ReplyOrIdError> {
    let shmseg = conn.generate_id().await?;
    conn.shm_attach_fd(shmseg, file, false).await?;

    use_shared_mem(conn, screen_num, shmseg).await?;

    conn.shm_detach(shmseg).await?;

    Ok(())
}

async fn receive_fd(conn: &impl Connection, screen_num: usize) -> Result<(), ReplyOrIdError> {
    let shmseg = conn.generate_id().await?;
    let segment_size = TEMP_FILE_CONTENT.len() as _;
    let reply = conn
        .shm_create_segment(shmseg, segment_size, false)
        .await?
        .reply()
        .await?;
    let shm::CreateSegmentReply { shm_fd, .. } = reply;

    let addr = unsafe {
        mm::mmap(
            null_mut(),
            segment_size as usize,
            mm::ProtFlags::READ | mm::ProtFlags::WRITE,
            mm::MapFlags::SHARED,
            &shm_fd,
            0,
        )
    };

    let addr = match addr {
        Ok(addr) => addr,
        Err(_) => {
            conn.shm_detach(shmseg).await?;
            return Err(ConnectionError::InsufficientMemory.into());
        }
    };
    unsafe {
        let slice = std::slice::from_raw_parts_mut(addr as *mut u8, segment_size as _);
        slice.copy_from_slice(&TEMP_FILE_CONTENT);
    }

    use_shared_mem(conn, screen_num, shmseg).await?;

    conn.shm_detach(shmseg).await?;
    // Let's ignore the munmap() that we should do here

    Ok(())
}

async fn main2(file: File) -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num, drive) = RustConnection::connect(None).await?;

    let ex = LocalExecutor::new();
    ex.run({
        let ex = &ex;
        let conn = &conn;

        async move {
            // Spawn a task to read from the connection.
            ex.spawn(async move {
                if let Err(e) = drive.await {
                    tracing::error!("Error while driving the connection: {}", e);
                }
            })
            .detach();

            // Check for SHM 1.2 support (needed for fd passing)
            if let Some((major, minor)) = check_shm_version(conn).await? {
                if major < 1 || (major == 1 && minor < 2) {
                    eprintln!(
                        "X11 server supports version {}.{} of the SHM extension, but version 1.2 \
                     is needed",
                        major, minor,
                    );
                    return Ok(());
                }
            } else {
                eprintln!("X11 server does not support SHM extension");
                return Ok(());
            }

            println!("Trying to send an FD");
            send_fd(conn, screen_num, file).await?;
            println!("Trying to receive an FD");
            receive_fd(conn, screen_num).await?;

            Ok(())
        }
    })
    .await
}

fn main() {
    let file = make_file().expect("Failed to create temporary file for FD passing");
    if let Err(e) = async_io::block_on(main2(file)) {
        tracing::error!("Fatal Error: {}", e);
    }
}
