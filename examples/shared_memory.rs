extern crate x11rb;

use std::fs::{remove_file, File, OpenOptions};
use std::io::{Write, Result as IOResult};

use x11rb::xcb_ffi::XCBConnection;
use x11rb::connection::Connection;
use x11rb::generated::xproto::{self, ImageFormat, ConnectionExt as _};
use x11rb::generated::shm;
use x11rb::errors::ConnectionErrorOrX11Error;

const TEMP_FILE_CONTENT: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0xff, 0xfe, 0xfd, 0xfc];

struct FreePixmap<'c, C: Connection>(&'c C, xproto::PIXMAP);
impl<C: Connection> Drop for FreePixmap<'_, C> {
    fn drop(&mut self) {
        self.0.free_pixmap(self.1).unwrap();
    }
}

/// Get the supported SHM version from the X11 server
fn check_shm_version<C: Connection>(conn: &C) -> Result<Option<(u16, u16)>, ConnectionErrorOrX11Error>
{
    if conn.extension_information(shm::X11_EXTENSION_NAME).is_none() {
        return Ok(None);
    }

    let shm_version = shm::ConnectionExt::query_version(conn)?.reply()?;
    Ok(Some((shm_version.major_version, shm_version.minor_version)))
}

/// Get the bytes describing the first pixel at the given offset of the given shared memory segment
/// (interpreted in the screen's root_visual)
fn get_shared_memory_content_at_offset<C: Connection>(conn: &C, screen: &xproto::Screen, shmseg: shm::SEG, offset: u32)
-> Result<Vec<u8>, ConnectionErrorOrX11Error>
{
    let pixmap = conn.generate_id();
    shm::ConnectionExt::create_pixmap(conn, pixmap, screen.root, 1, 1, screen.root_depth, shmseg, offset)?;
    let pixmap = FreePixmap(conn, pixmap);

    let image = xproto::ConnectionExt::get_image(conn, ImageFormat::ZPixmap, pixmap.1, 0, 0, 1, 1, !0)?.reply()?;
    Ok(image.data)
}

/// Make a temporary file
fn make_file() -> IOResult<File>
{
    let file_name = "shared_memory.bin";
    let mut file = OpenOptions::new().create(true).read(true).write(true).truncate(true).open(file_name)?;
    file.write_all(&TEMP_FILE_CONTENT)?;
    remove_file(file_name)?;
    Ok(file)
}

fn real_main<C: Connection>(conn: &C, screen_num: usize, file: File) -> Result<(), ConnectionErrorOrX11Error> {
    let screen = &conn.setup().roots[screen_num];

    // Check for SHM 1.2 support (needed for fd passing)
    if let Some((major, minor)) = check_shm_version(conn)? {
        if major < 1 || (major == 1 && minor < 2) {
            eprintln!("X11 server supports version {}.{} of the SHM extension, but version 1.2 is needed",
                      major, minor);
            return Ok(());
        }
    } else {
        eprintln!("X11 server does not support SHM extension");
        return Ok(());
    }


    let shmseg = conn.generate_id();
    shm::ConnectionExt::attach_fd(conn, shmseg, file, 0)?;

    let content = get_shared_memory_content_at_offset(conn, screen, shmseg, 0)?;
    assert_eq!(content[..], TEMP_FILE_CONTENT[0..4]);

    let content = get_shared_memory_content_at_offset(conn, screen, shmseg, 4)?;
    assert_eq!(content[..], TEMP_FILE_CONTENT[4..8]);

    Ok(())
}

fn main() {
    let file = make_file().expect("Failed to create temporary file for FD passing");
    match XCBConnection::connect(None) {
        Err(err) => eprintln!("Failed to connect to the X11 server: {}", err),
        Ok((conn, screen_num)) => real_main(&conn, screen_num, file).unwrap()
    }
}
