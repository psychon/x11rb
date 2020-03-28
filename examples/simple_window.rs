extern crate x11rb;

use x11rb::connection::Connection;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::xproto::*;
use x11rb::Event;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();

    // The following is only needed for start_timeout_thread(), which is used for 'tests'
    let conn1 = std::sync::Arc::new(conn);
    let conn = &*conn1;

    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id().unwrap();
    let gc_id = conn.generate_id().unwrap();

    let wm_protocols = conn.intern_atom(false, b"WM_PROTOCOLS").unwrap();
    let wm_delete_window = conn.intern_atom(false, b"WM_DELETE_WINDOW").unwrap();
    let wm_protocols = wm_protocols.reply().unwrap().atom;
    let wm_delete_window = wm_delete_window.reply().unwrap().atom;

    let win_aux = CreateWindowAux::new()
        .event_mask(EventMask::Exposure | EventMask::StructureNotify | EventMask::NoEvent)
        .background_pixel(screen.white_pixel)
        .win_gravity(Gravity::NorthWest);

    let gc_aux = CreateGCAux::new().foreground(screen.black_pixel);

    let (mut width, mut height) = (100, 100);

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
    )
    .unwrap();

    util::start_timeout_thread(conn1.clone(), win_id);

    let title = "Simple Window";
    conn.change_property8(
        PropMode::Replace,
        win_id,
        Atom::WM_NAME.into(),
        Atom::STRING.into(),
        title.as_bytes(),
    )
    .unwrap();
    conn.change_property32(
        PropMode::Replace,
        win_id,
        wm_protocols,
        Atom::ATOM.into(),
        &[wm_delete_window],
    )
    .unwrap();

    let reply = conn
        .get_property(
            false,
            win_id,
            Atom::WM_NAME.into(),
            Atom::STRING.into(),
            0,
            1024,
        )
        .unwrap();
    let reply = reply.reply().unwrap();
    assert_eq!(reply.value, title.as_bytes());

    conn.create_gc(gc_id, win_id, &gc_aux).unwrap();

    conn.map_window(win_id).unwrap();

    conn.flush().unwrap();

    loop {
        let event = conn.wait_for_event().unwrap();
        let event = conn.parse_event(event).unwrap();
        println!("{:?})", event);
        match event {
            Event::XprotoExposeEvent(event) => {
                if event.count == 0 {
                    // We ought to clear the background before drawing something new, but...
                    // whatever
                    let (width, height): (i16, i16) = (width as _, height as _);
                    let points = [
                        Point {
                            x: width,
                            y: height,
                        },
                        Point { x: -10, y: -10 },
                        Point {
                            x: -10,
                            y: height + 10,
                        },
                        Point {
                            x: width + 10,
                            y: -10,
                        },
                    ];
                    conn.poly_line(CoordMode::Origin, win_id, gc_id, &points)
                        .unwrap();
                    conn.flush().unwrap();
                }
            }
            Event::XprotoConfigureNotifyEvent(event) => {
                width = event.width;
                height = event.height;
            }
            Event::XprotoClientMessageEvent(event) => {
                let data = event.data.as_data32();
                if event.format == 32 && event.window == win_id && data[0] == wm_delete_window {
                    println!("Window was asked to close");
                    return;
                }
            }
            Event::Error(_) => println!("Got an unexpected error"),
            _ => println!("Got an unknown event"),
        }
    }
}

include!("integration_test_util/util.rs");
