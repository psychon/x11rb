extern crate x11rb;

use x11rb::xcb_ffi::Connection;
use x11rb::generated::xproto::*;
use std::convert::TryFrom;


fn main() {
    let conn = Connection::new();
    let conn = match conn {
        Ok(conn) => conn,
        Err(_) => panic!("")
    };
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
    create_window(&conn, 24, win_id, 0x14d, 0, 0, 20, 20, 0, WindowClass::InputOutput.into(), 0, &win_aux).unwrap();

    create_gc(&conn, gc_id, win_id, &gc_aux);

    map_window(&conn, win_id).unwrap();

    println!("{:?}", get_input_focus(&conn).unwrap().reply());

    loop {
        let event = conn.wait_for_event().unwrap();
        println!("{:?}", event);
        if (*event)[0] == EXPOSE_EVENT {
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
                    poly_line(&conn, CoordMode::Origin.into(), win_id, gc_id, &points).unwrap();
                    conn.flush();
                }
            }
        }
    }
}
