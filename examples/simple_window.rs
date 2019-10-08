extern crate x11rb;

use x11rb::xcb_ffi::{Connection, Event};
use x11rb::generated::xproto::*;
use std::convert::TryFrom;


fn main() {
    let (conn, _screen) = Connection::connect(None).unwrap();
    println!("{:?}", no_operation(&conn).unwrap());
    println!("{:?}", get_input_focus(&conn).unwrap().reply());

    let win_id = conn.generate_id();
    let gc_id = conn.generate_id();

    let mut win_aux = CreateWindowAux::default();
    win_aux.event_mask = Some(EventMask::Exposure.into());
    win_aux.background_pixel = Some(0xffffff);

    let mut gc_aux = CreateGCAux::default();
    gc_aux.foreground = Some(0);

    // The root window is hardcoded, because so far the generated code cannot tell me its id
    create_window(&conn, 24, win_id, 0x14d, 0, 0, 100, 100, 0, WindowClass::InputOutput, 0, &win_aux).unwrap();

    let title = "Simple Window";
    change_property(&conn, PropMode::Replace, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), 8, title.as_bytes()).unwrap();
    // FIXME: format != 8 is broken (because the XML uses <op> stuff)

    let reply = get_property(&conn, 0, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), 0, 1024).unwrap();
    let reply = reply.reply().unwrap();
    println!("{:?}", reply);
    assert_eq!(reply.value, title.as_bytes());

    create_gc(&conn, gc_id, win_id, &gc_aux).unwrap();

    map_window(&conn, win_id).unwrap();

    println!("{:?}", get_input_focus(&conn).unwrap().reply());

    loop {
        let event = conn.wait_for_event().unwrap();
        println!("{:?}", event);
        if event.response_type() == EXPOSE_EVENT {
            let event = ExposeEvent::try_from(event);
            println!("{:?})", event);
            if let Ok(event) = event {
                if event.count == 0 {
                    let points = [
                        Point { x: 100, y: 100 },
                        Point { x: -10, y: -10 },
                        Point { x: -10, y: 110 },
                        Point { x: 110, y: -10 },
                    ];
                    poly_line(&conn, CoordMode::Origin, win_id, gc_id, &points).unwrap();
                    conn.flush();
                }
            }
        }
    }
}
