use x11rb::atom_manager;
use x11rb::connection::{Connection as _, RequestConnection as _};
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::xkb::{self, ConnectionExt as _};
use x11rb::protocol::xproto::{
    self, ConnectionExt as _, CreateWindowAux, EventMask, PropMode, WindowClass,
};
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _;
use x11rb::xcb_ffi::XCBConnection;
use xkbcommon::xkb as xkbc;

// A collection of the atoms we will need.
atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

/// Handle a single key press or key release event by printing some details
fn handle_key(event: xproto::KeyPressEvent, press: bool, state: &xkbc::State) {
    let kind = if press { "  Pressed" } else { "Released" };
    let sym = state.key_get_one_sym(event.detail.into());
    let utf8 = state.key_get_utf8(event.detail.into());
    println!("{} keysym {sym:?} for utf8 '{utf8}'", kind);

    // Just as an example on how this works:
    if sym == xkbc::keysyms::KEY_BackSpace.into() {
        println!("Pressed key was backspace");
    }
}

/// Create and return a window
fn create_window(
    conn: &XCBConnection,
    screen_num: usize,
    atoms: &AtomCollection,
) -> Result<xproto::Window, ReplyOrIdError> {
    let screen = &conn.setup().roots[screen_num];
    let window = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        window,
        screen.root,
        0,
        0,
        100,
        100,
        0,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .event_mask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE),
    )?;
    let title = "Keyboard tester";
    conn.change_property8(
        PropMode::REPLACE,
        window,
        xproto::AtomEnum::WM_NAME,
        xproto::AtomEnum::STRING,
        title.as_bytes(),
    )?;
    conn.change_property8(
        PropMode::REPLACE,
        window,
        atoms._NET_WM_NAME,
        atoms.UTF8_STRING,
        title.as_bytes(),
    )?;
    conn.change_property32(
        PropMode::REPLACE,
        window,
        atoms.WM_PROTOCOLS,
        xproto::AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )?;
    conn.change_property8(
        PropMode::REPLACE,
        window,
        xproto::AtomEnum::WM_CLASS,
        xproto::AtomEnum::STRING,
        b"simple_window\0simple_window\0",
    )?;
    Ok(window)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = XCBConnection::connect(None)?;

    conn.prefetch_extension_information(xkb::X11_EXTENSION_NAME)?;
    let atoms = AtomCollection::new(&conn)?;
    let xkb = conn.xkb_use_extension(1, 0)?;
    let atoms = atoms.reply()?;
    let xkb = xkb.reply()?;
    assert!(
        xkb.supported,
        "This program requires the X11 server to support the XKB extension"
    );

    // Ask the X11 server to send us XKB events.
    // TODO: No idea what to pick here. I guess this is asking unnecessarily for too much?
    let events = xkb::EventType::NEW_KEYBOARD_NOTIFY
        | xkb::EventType::MAP_NOTIFY
        | xkb::EventType::STATE_NOTIFY;
    // TODO: No idea what to pick here. I guess this is asking unnecessarily for too much?
    let map_parts = xkb::MapPart::KEY_TYPES
        | xkb::MapPart::KEY_SYMS
        | xkb::MapPart::MODIFIER_MAP
        | xkb::MapPart::EXPLICIT_COMPONENTS
        | xkb::MapPart::KEY_ACTIONS
        | xkb::MapPart::KEY_BEHAVIORS
        | xkb::MapPart::VIRTUAL_MODS
        | xkb::MapPart::VIRTUAL_MOD_MAP;
    conn.xkb_select_events(
        xkb::ID::USE_CORE_KBD.into(),
        0u8.into(),
        events,
        map_parts,
        map_parts,
        &xkb::SelectEventsAux::new(),
    )?;

    // Set up xkbcommon state and get the current keymap.
    let context = xkbc::Context::new(xkbc::CONTEXT_NO_FLAGS);
    let device_id = xkbc::x11::get_core_keyboard_device_id(&conn);
    assert!(device_id >= 0);
    let keymap = xkbc::x11::keymap_new_from_device(
        &context,
        &conn,
        device_id,
        xkbc::KEYMAP_COMPILE_NO_FLAGS,
    );
    let mut state = xkbc::x11::state_new_from_device(&keymap, &conn, device_id);

    // Create and show a window
    let window = create_window(&conn, screen_num, &atoms)?;
    conn.map_window(window)?;
    conn.flush()?;

    // Main loop; wait for events
    loop {
        match conn.wait_for_event()? {
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32 && event.window == window && data[0] == atoms.WM_DELETE_WINDOW
                {
                    println!("Window was asked to close");
                    break;
                }
            }
            Event::XkbStateNotify(event) => {
                if i32::from(event.device_id) == device_id {
                    // Inform xkbcommon that the keyboard state changed
                    state.update_mask(
                        event.base_mods.into(),
                        event.latched_mods.into(),
                        event.locked_mods.into(),
                        event.base_group.try_into().unwrap(),
                        event.latched_group.try_into().unwrap(),
                        event.locked_group.into(),
                    );
                }
            }
            Event::KeyPress(event) => handle_key(event, true, &state),
            Event::KeyRelease(event) => handle_key(event, false, &state),
            event => println!("Ignoring event {event:?}"),
        }
    }

    Ok(())
}
