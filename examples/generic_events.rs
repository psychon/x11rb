// This example tests support for generic events (XGE). It generates a window and uses the PRESENT
// extension to cause an XGE event to be sent.

use std::process::exit;
use std::convert::TryFrom;

use x11rb::connection::{RequestConnection as _, Connection as _};
use x11rb::xcb_ffi::XCBConnection;
use x11rb::generated::xproto::{CreateWindowAux, ConfigureWindowAux, WindowClass, ConnectionExt as _, GE_GENERIC_EVENT, GeGenericEvent};
use x11rb::generated::present;
use x11rb::errors::ConnectionErrorOrX11Error;
use x11rb::x11_utils::Event;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() -> Result<(), ConnectionErrorOrX11Error>
{
    let (conn, screen_num) = XCBConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let present_info = match conn.extension_information(present::X11_EXTENSION_NAME) {
        Some(info) => info,
        None => {
            eprintln!("Present extension is not supported");
            exit(1);
        }
    };

    // Create a window
    let win_id = conn.generate_id();
    let win_aux = CreateWindowAux::new()
        .background_pixel(screen.white_pixel);
    conn.create_window(COPY_DEPTH_FROM_PARENT, win_id, screen.root, 0, 0, 10, 10, 0, WindowClass::InputOutput, 0, &win_aux)?;

    // Ask for present ConfigureNotify events
    let event_id = conn.generate_id();
    present::select_input(&conn, event_id, win_id, present::EventMask::ConfigureNotify.into())?;

    // Cause an event
    conn.configure_window(win_id, &ConfigureWindowAux::new().width(20))?;

    // Wait for the event
    conn.flush();
    let event = conn.wait_for_event()?;

    // Now check that the event really is what we wanted to get
    assert_eq!(event.response_type(), GE_GENERIC_EVENT);
    let generic_event = GeGenericEvent::try_from(&event)?;
    assert_eq!(generic_event.extension, present_info.major_opcode);
    assert_eq!(generic_event.event_type, present::Event::ConfigureNotify.into());

    let event = present::ConfigureNotifyEvent::try_from(event)?;
    println!("Got a Present ConfigureNotify event for event ID 0x{:x} and window 0x{:x}.", event.event, event.window);
    println!("x={}, y={}, width={}, height={}, off_x={}, off_y={}, pixmap_width={}, pixmap_height={}, pixmap_flags={:x}",
             event.x, event.y, event.width, event.height, event.off_x, event.off_y, event.pixmap_width, event.pixmap_height, event.pixmap_flags);

    Ok(())
}
