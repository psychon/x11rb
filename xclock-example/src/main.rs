use chrono::prelude::*;
use nix::poll::{poll, PollFd, PollFlags};
use x11rb::atom_manager;
use x11rb::connection::Connection;
use x11rb::errors::{ConnectionError, ReplyOrIdError};
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt as _;

atom_manager! {
    pub Atoms: AtomsCookie {
        UTF8_STRING,
        WM_DELETE_WINDOW,
        WM_PROTOCOLS,
        _NET_WM_NAME,
    }
}

fn create_window(
    conn: &impl Connection,
    screen: &Screen,
    atoms: &Atoms,
    (width, height): (u16, u16),
) -> Result<Window, ReplyOrIdError> {
    let win_id = conn.generate_id()?;
    let win_aux =
        CreateWindowAux::new().event_mask(EventMask::Exposure | EventMask::StructureNotify);

    conn.create_window(
        screen.root_depth,
        win_id,
        screen.root,
        0,
        0,
        width,
        height,
        0,
        WindowClass::InputOutput,
        0,
        &win_aux,
    )?;

    let title = "xclock";
    conn.change_property8(
        PropMode::Replace,
        win_id,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes(),
    )?;
    conn.change_property8(
        PropMode::Replace,
        win_id,
        atoms._NET_WM_NAME,
        atoms.UTF8_STRING,
        title.as_bytes(),
    )?;
    conn.change_property32(
        PropMode::Replace,
        win_id,
        atoms.WM_PROTOCOLS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )?;

    conn.map_window(win_id)?;

    Ok(win_id)
}

fn redraw(
    conn: &impl Connection,
    screen: &Screen,
    win_id: Window,
    gc_id: Gcontext,
    (width, height): (u16, u16),
) -> Result<(), ConnectionError> {
    let time = Local::now();
    let (hour, minute, second) = (time.hour(), time.minute(), time.second());

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
    conn.change_gc(gc_id, &ChangeGCAux::new().foreground(screen.white_pixel))?;
    conn.poly_fill_rectangle(
        win_id,
        gc_id,
        &[Rectangle {
            x: 0,
            y: 0,
            width,
            height,
        }],
    )?;
    conn.change_gc(gc_id, &ChangeGCAux::new().foreground(screen.black_pixel))?;

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
    conn.poly_segment(win_id, gc_id, &lines)?;

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
            PolyShape::Complex,
            CoordMode::Origin,
            &polygon,
        )?;
    }

    Ok(())
}

fn do_poll(conn: &RustConnection) -> nix::Result<()> {
    use std::os::unix::io::AsRawFd;

    let fd = conn.with_read(|r| r.get_ref().as_raw_fd());

    // We do not really care about the result of poll. Either there was a timeout, in which case we
    // try to handle events (there are none) and then redraw. Or there was an event, in which case
    // we handle it and then still redraw.
    poll(&mut [PollFd::new(fd, PollFlags::POLLIN)], 1_000)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = RustConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let atoms = Atoms::new(&conn)?.reply()?;

    let (mut width, mut height) = (100, 100);
    let win_id = create_window(&conn, &screen, &atoms, (width, height))?;

    let gc_id = conn.generate_id().unwrap();
    conn.create_gc(gc_id, win_id, &CreateGCAux::default())?;

    conn.flush()?;

    loop {
        do_poll(&conn)?;
        while let Some(event) = conn.poll_for_event()? {
            println!("{:?})", event);
            match event {
                Event::ConfigureNotify(event) => {
                    width = event.width;
                    height = event.height;
                }
                Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32
                        && event.window == win_id
                        && data[0] == atoms.WM_DELETE_WINDOW
                    {
                        println!("Window was asked to close");
                        return Ok(());
                    }
                }
                Event::Error(_) => println!("Got an unexpected error"),
                _ => println!("Got an unknown event"),
            }
        }

        redraw(&conn, &screen, win_id, gc_id, (width, height))?;
        conn.flush()?;
    }
}
