// This code is dual licensed under MIT OR Apache 2.0.

//! An implementation of the "XClock" example from `x11rb`, but using `x11rb-async`.

use async_executor::LocalExecutor;
use async_io::Timer;
use futures_lite::prelude::*;

use std::cell::RefCell;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use x11rb_async::connection::Connection;
use x11rb_async::errors::{ConnectionError, ReplyOrIdError};
use x11rb_async::protocol::{xproto::*, Event};
use x11rb_async::rust_connection::RustConnection;

struct Atoms {
    utf8_string: Atom,
    wm_protocols: Atom,
    wm_delete_window: Atom,
    wm_name: Atom,
}

impl Atoms {
    async fn load(conn: &impl Connection) -> Result<Self, ReplyOrIdError> {
        let utf8_string = conn.intern_atom(false, b"UTF8_STRING").await?;
        let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS").await?;
        let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW").await?;
        let wm_name = conn.intern_atom(false, b"WM_NAME").await?;

        let utf8_string = utf8_string.reply().await?.atom;
        let wm_protocols = wm_protocols.reply().await?.atom;
        let wm_delete_window = wm_delete_window.reply().await?.atom;
        let wm_name = wm_name.reply().await?.atom;

        Ok(Self {
            utf8_string,
            wm_protocols,
            wm_delete_window,
            wm_name,
        })
    }
}

/// Get the current time as (hour, minute, second)
fn get_time() -> (u8, u8, u8) {
    let total_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (second, total_minutes) = (total_secs % 60, total_secs / 60);
    let (minute, total_hours) = (total_minutes % 60, total_minutes / 60);
    let hour = total_hours % 24;

    // This is in UTC. Getting local time is complicated and not important for us.
    (hour as _, minute as _, second as _)
}

async fn create_window(
    conn: &impl Connection,
    screen: &Screen,
    atoms: &Atoms,
    (width, height): (u16, u16),
) -> Result<Window, ReplyOrIdError> {
    // Create the window.
    let win_id = conn.generate_id().await?;
    let win_aux =
        CreateWindowAux::new().event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY);

    conn.create_window(
        screen.root_depth,
        win_id,
        screen.root,
        0,
        0,
        width,
        height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &win_aux,
    )
    .await?;

    let title = "xclock (async redox)";
    conn.change_property(
        PropMode::REPLACE,
        win_id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        8,
        title.len() as _,
        title.as_bytes(),
    )
    .await?;
    conn.change_property(
        PropMode::REPLACE,
        win_id,
        atoms.wm_name,
        atoms.utf8_string,
        8,
        title.len() as _,
        title.as_bytes(),
    )
    .await?;
    conn.change_property(
        PropMode::REPLACE,
        win_id,
        atoms.wm_protocols,
        AtomEnum::ATOM,
        32,
        1,
        &atoms.wm_delete_window.to_ne_bytes(),
    )
    .await?;

    conn.map_window(win_id).await?;

    Ok(win_id)
}

async fn redraw(
    conn: &impl Connection,
    screen: &Screen,
    win_id: Window,
    gc_id: Gcontext,
    (width, height): (u16, u16),
) -> Result<(), ConnectionError> {
    let (hour, minute, second) = get_time();

    let center = ((width as f32) / 2.0, (height as f32) / 2.0);
    let size = (width.min(height) as f32) / 2.0;

    // Transform a value between 0 and 60 to a position on the clock (relative to the center)
    let minute_to_outer_position = |minute: f32| {
        let angle = (30.0 - minute) * 2.0 * std::f32::consts::PI / 60.0;
        let (sin, cos) = angle.sin_cos();
        (size * sin, size * cos)
    };

    // Create a line segment
    fn create_line(center: (f32, f32), from: (f32, f32), to: (f32, f32)) -> Segment {
        Segment {
            x1: (center.0 + from.0).round() as _,
            y1: (center.1 + from.1).round() as _,
            x2: (center.0 + to.0).round() as _,
            y2: (center.1 + to.1).round() as _,
        }
    }

    // Draw the background
    conn.change_gc(gc_id, &ChangeGCAux::new().foreground(screen.white_pixel))
        .await?;
    conn.poly_fill_rectangle(
        win_id,
        gc_id,
        &[Rectangle {
            x: 0,
            y: 0,
            width,
            height,
        }],
    )
    .await?;
    conn.change_gc(gc_id, &ChangeGCAux::new().foreground(screen.black_pixel))
        .await?;

    // Get a list of lines for the clock's face
    let mut lines = (0..60)
        .map(|minute| {
            let outer = minute_to_outer_position(minute as _);
            let length_factor = if minute % 5 == 0 { 0.8 } else { 0.9 };
            create_line(
                center,
                outer,
                (outer.0 * length_factor, outer.1 * length_factor),
            )
        })
        .collect::<Vec<_>>();
    // ... and also the hand for seconds
    lines.push(create_line(
        center,
        (0.0, 0.0),
        minute_to_outer_position(second as _),
    ));

    // Draw everything
    conn.poly_segment(win_id, gc_id, &lines).await?;

    // Now draw the hands
    let point = |pos: (f32, f32), factor: f32| Point {
        x: (center.0 + pos.0 * factor).round() as _,
        y: (center.1 + pos.1 * factor).round() as _,
    };
    let hour_to_60 = (hour % 12) as f32 * 60.0 / 12.0;
    for &(position, hand_length, hand_width) in
        &[(hour_to_60, 0.6, 0.08), (minute as f32, 0.8, 0.05)]
    {
        let outer = minute_to_outer_position(position);
        let ortho1 = (outer.1, -outer.0);
        let ortho2 = (-outer.1, outer.0);
        let opposite = (-outer.0, -outer.1);
        let polygon = [
            point(ortho1, hand_width),
            point(opposite, hand_width),
            point(ortho2, hand_width),
            point(outer, hand_length),
        ];
        conn.fill_poly(
            win_id,
            gc_id,
            PolyShape::COMPLEX,
            CoordMode::ORIGIN,
            &polygon,
        )
        .await?;
    }

    Ok(())
}

async fn main2() -> Result<(), Box<dyn std::error::Error>> {
    // This is shared between tasks.
    let (width, height) = (100, 100);
    let size = RefCell::new((width, height));

    // Open a new connection.
    let (conn, screen_index, drive) = RustConnection::connect(None).await?;
    let screen = conn.setup().roots.get(screen_index).unwrap();

    // Create an executor for spawning tasks.
    let ex = LocalExecutor::new();

    ex.run({
        let ex = &ex;
        let size = &size;
        let conn = &conn;

        async move {
            // Spawn a task to poll for events.
            let driver = ex.spawn(async move {
                if let Err(e) = drive.await {
                    eprintln!("Error while driving the connection: {}", e);
                }
            });

            // On X11RB_EXAMPLE_TIMEOUT, exit after a set timeout.
            if let Some(timeout) = env::var("X11RB_EXAMPLE_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
            {
                ex.spawn(async move {
                    Timer::after(Duration::from_secs(timeout)).await;
                    eprintln!("Cancelling drive task due to $X11RB_EXAMPLE_TIMEOUT");
                    driver.cancel().await;
                })
                .detach();
            } else {
                driver.detach();
            }

            // Setup atoms for this connection.
            let atoms = Atoms::load(conn).await?;

            // Create a window.
            let window = create_window(conn, screen, &atoms, (width, height)).await?;

            // Create a graphics context.
            let gc_id = conn.generate_id().await?;
            conn.create_gc(gc_id, window, &CreateGCAux::new()).await?;

            // Span a task that redraws the window every second.
            ex.spawn(async move {
                // Create a timer that fires every second.
                let timer = Timer::interval(Duration::from_millis(1_000));

                // Iterate over this timer endlessly.
                timer
                    .then(move |_| async move {
                        // Redraw after one second has passed.
                        let (width, height) = *size.borrow();
                        redraw(conn, screen, window, gc_id, (width, height)).await?;
                        conn.flush().await?;

                        Ok::<_, ConnectionError>(())
                    })
                    .for_each(|res| {
                        if let Err(e) = res {
                            eprintln!("Timer task failed: {}", e);
                        }
                    })
                    .await;
            })
            .detach();

            // Flush the connection so far.
            conn.flush().await?;

            loop {
                // Get the next event.
                let event = conn.wait_for_event().await?;

                println!("{:?}", event);

                match event {
                    Event::ConfigureNotify(cme) => {
                        // The window was resized.
                        *size.borrow_mut() = (cme.width, cme.height);
                    }

                    Event::ClientMessage(cme) => {
                        let data = cme.data.as_data32();

                        if cme.format == 32
                            && cme.window == window
                            && data[0] == atoms.wm_delete_window
                        {
                            println!("Window was asked to close");
                            return Ok::<_, Box<dyn std::error::Error>>(());
                        }
                    }

                    Event::Error(_) => println!("Unexpected error"),

                    _ => println!("Unhandled event"),
                }

                // Redraw and flush.
                let (width, height) = *size.borrow();
                redraw(conn, screen, window, gc_id, (width, height)).await?;
                conn.flush().await?;
            }
        }
    })
    .await?;

    Ok(())
}

fn main() {
    if let Err(e) = async_io::block_on(main2()) {
        eprintln!("Fatal Error: {}", e);
    }
}
