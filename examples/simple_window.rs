extern crate x11rb;

use std::convert::TryFrom;

use x11rb::xcb_ffi::XCBConnection;
use x11rb::x11_utils::{Event, GenericError};
use x11rb::generated::xproto::*;
use x11rb::connection::Connection;
use x11rb::wrapper::ConnectionExt as _;

fn main() {
    let (conn, screen_num) = XCBConnection::connect(None).unwrap();

    // The following is only needed for start_timeout_thread(), which is used for 'tests'
    let conn1 = std::sync::Arc::new(conn);
    let conn = &*conn1;

    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id();
    let gc_id = conn.generate_id();

    let (wm_protocols, wm_delete_window) = {
        let protocols = conn.intern_atom(false, "WM_PROTOCOLS".as_bytes()).unwrap();
        let delete = conn.intern_atom(false, "WM_DELETE_WINDOW".as_bytes()).unwrap();
        (protocols.reply().unwrap().atom, delete.reply().unwrap().atom)
    };

    let win_aux = CreateWindowAux::new()
        .event_mask(EventMask::Exposure | EventMask::StructureNotify | EventMask::NoEvent)
        .background_pixel(screen.white_pixel)
        .win_gravity(Gravity::NorthWest);

    let gc_aux = CreateGCAux::new()
        .foreground(screen.black_pixel);

    let (mut width, mut height) = (100, 100);

    conn.create_window(24, win_id, screen.root, 0, 0, width, height, 0, WindowClass::InputOutput, 0, &win_aux).unwrap();

    util::start_timeout_thread(conn1.clone(), win_id);

    let title = "Simple Window";
    conn.change_property8(PropMode::Replace, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), title.as_bytes()).unwrap();
    conn.change_property32(PropMode::Replace, win_id, wm_protocols, Atom::ATOM.into(), &[wm_delete_window]).unwrap();

    let reply = conn.get_property(0, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), 0, 1024).unwrap();
    let reply = reply.reply().unwrap();
    assert_eq!(reply.value, title.as_bytes());

    conn.create_gc(gc_id, win_id, &gc_aux).unwrap();

    conn.map_window(win_id).unwrap();

    conn.flush();

    loop {
        let event = conn.wait_for_event().unwrap();
        match event.response_type() {
            EXPOSE_EVENT => {
                let event = ExposeEvent::from(event);
                println!("{:?})", event);
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
                    conn.poly_line(CoordMode::Origin, win_id, gc_id, &points).unwrap();
                    conn.flush();
                }
            },
            CONFIGURE_NOTIFY_EVENT => {
                let event = ConfigureNotifyEvent::from(event);
                width = event.width;
                height = event.height;
            },
            CLIENT_MESSAGE_EVENT => {
                let event = ClientMessageEvent::from(event);
                println!("{:?})", event);
                let data = event.data.as_data32();
                if event.format == 32 && event.window == win_id && data[0] == wm_delete_window {
                    println!("Window was asked to close");
                    return;
                }
            }
            0 => { println!("Unknown error {:?}", GenericError::try_from(event)); },
            _ => { println!("Unknown event {:?}", event); }
        }
    }
}

include!("integration_test_util/util.rs");
