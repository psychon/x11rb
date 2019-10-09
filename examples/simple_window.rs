extern crate x11rb;

use x11rb::xcb_ffi::{Connection, Event, GenericError};
use x11rb::generated::xproto::*;
use std::convert::TryFrom;


fn main() {
    let (conn, screen_num) = Connection::connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];

    let win_id = conn.generate_id();
    let gc_id = conn.generate_id();

    let (wm_protocols, wm_delete_window) = {
        let protocols = intern_atom(&conn, 0, "WM_PROTOCOLS".as_bytes()).unwrap();
        let delete = intern_atom(&conn, 0, "WM_DELETE_WINDOW".as_bytes()).unwrap();
        (protocols.reply().unwrap().atom, delete.reply().unwrap().atom)
    };

    let mut win_aux = CreateWindowAux::default();
    // FIXME: Figure out how to make this nicer... somehow
    let expose: u32 = EventMask::Exposure.into();
    let structure: u32 = EventMask::StructureNotify.into();
    win_aux.event_mask = Some(expose | structure);
    win_aux.background_pixel = Some(screen.white_pixel);

    let mut gc_aux = CreateGCAux::default();
    gc_aux.foreground = Some(screen.black_pixel);

    let (mut width, mut height) = (100, 100);

    create_window(&conn, 24, win_id, screen.root, 0, 0, width, height, 0, WindowClass::InputOutput, 0, &win_aux).unwrap();

    let title = "Simple Window";
    change_property(&conn, PropMode::Replace, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), 8, title.as_bytes()).unwrap();
    // FIXME: format != 8 is broken, because the XML uses <op> stuff in a way that we cannot
    // really use. Perhaps special-case change_property and make variants of it for the different
    // valid formats?
    change_property(&conn, PropMode::Replace, win_id, wm_protocols, Atom::WINDOW.into(), 32, &wm_delete_window.to_ne_bytes()).unwrap();

    let reply = get_property(&conn, 0, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), 0, 1024).unwrap();
    let reply = reply.reply().unwrap();
    assert_eq!(reply.value, title.as_bytes());

    create_gc(&conn, gc_id, win_id, &gc_aux).unwrap();

    map_window(&conn, win_id).unwrap();

    conn.flush();

    loop {
        let event = conn.wait_for_event();
        let event = match event {
            Err(e) => {
                eprintln!("Got connection error: {:?}", e);
                return;
            },
            Ok(event) => event
        };
        match event.response_type() {
            EXPOSE_EVENT => {
                let event = ExposeEvent::try_from(event);
                println!("{:?})", event);
                if let Ok(event) = event {
                    if event.count == 0 {
                        // We ought to clear the background before drawing something new, but...
                        // whatever
                        let (width, height): (i16, i16) = (width as _, height as _);
                        let points = [
                            Point { x: width, y: height },
                            Point { x: -10, y: -10 },
                            Point { x: -10, y: height + 10 },
                            Point { x: width + 10, y: -10 },
                        ];
                        poly_line(&conn, CoordMode::Origin, win_id, gc_id, &points).unwrap();
                        conn.flush();
                    }
                }
            },
            CONFIGURE_NOTIFY_EVENT => {
                let event = ConfigureNotifyEvent::try_from(event);
                println!("{:?})", event);
                if let Ok(event) = event {
                    width = event.width;
                    height = event.height;
                }
            }
            0 => { println!("Unknown error {:?}", GenericError::try_from(event)); },
            _ => { println!("Unknown event {:?}", event); }
        }
    }
}
