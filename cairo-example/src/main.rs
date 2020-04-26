//! Example for using cairo-xcb together with x11rb.
//!
//! The main ingredients are:
//! - x11rb provides XCBConnection::get_raw_xcb_connection() to get a `*mut c_void` for the
//!   underlying `xcb_connection_t`.
//! - Only one XCB type is actually used in cairo-xcb's public API. This is `xcb_visualtype_t` for
//!   which we provide an inline definition below.
//!   (Alternatively, one could use `xcb::Visualtype` from the xcb crate; it's equivalent.)

use x11rb::atom_manager;
use x11rb::connection::Connection;
use x11rb::errors::ReplyOrIdError;
use x11rb::protocol::xproto::{ConnectionExt as _, *};
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt;
use x11rb::xcb_ffi::XCBConnection;

// A collection of the atoms we will need.
atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

/// A rust version of XCB's `xcb_visualtype_t` struct. This is used in a FFI-way.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: u32,
    pub class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}

impl From<Visualtype> for xcb_visualtype_t {
    fn from(value: Visualtype) -> xcb_visualtype_t {
        xcb_visualtype_t {
            visual_id: value.visual_id,
            class: value.class.into(),
            bits_per_rgb_value: value.bits_per_rgb_value,
            colormap_entries: value.colormap_entries,
            red_mask: value.red_mask,
            green_mask: value.green_mask,
            blue_mask: value.blue_mask,
            pad0: [0; 4],
        }
    }
}

/// Find a `xcb_visualtype_t` based on its ID number
fn find_xcb_visualtype(conn: &impl Connection, visual_id: u32) -> Option<xcb_visualtype_t> {
    for root in &conn.setup().roots {
        for depth in &root.allowed_depths {
            for visual in &depth.visuals {
                if visual.visual_id == visual_id {
                    return Some((*visual).into());
                }
            }
        }
    }
    None
}

/// Create a window for us.
fn create_window<C>(
    conn: &C,
    screen: &x11rb::protocol::xproto::Screen,
    atoms: &AtomCollection,
    (width, height): (u16, u16),
) -> Result<Window, ReplyOrIdError<C::Buf>>
where
    C: Connection,
{
    let window = conn.generate_id()?;
    let win_aux =
        CreateWindowAux::new().event_mask(EventMask::Exposure | EventMask::StructureNotify);
    conn.create_window(
        screen.root_depth,
        window,
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

    let title = "Simple Window";
    conn.change_property8(
        PropMode::Replace,
        window,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes(),
    )?;
    conn.change_property8(
        PropMode::Replace,
        window,
        atoms._NET_WM_NAME,
        atoms.UTF8_STRING,
        title.as_bytes(),
    )?;
    conn.change_property32(
        PropMode::Replace,
        window,
        atoms.WM_PROTOCOLS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )?;
    conn.change_property8(
        PropMode::Replace,
        window,
        AtomEnum::WM_CLASS,
        AtomEnum::STRING,
        b"simple_window\0simple_window\0",
    )?;

    conn.map_window(window)?;
    Ok(window)
}

/// Draw the window content
fn do_draw(cr: &cairo::Context, (width, height): (f64, f64)) {
    use std::f64::consts::PI;

    // Draw a background
    cr.set_source_rgb(0.9, 1.0, 0.9);
    cr.paint();

    // Everybody likes odd geometrical shapes, right?
    let radius = width.min(height) / 3.0;
    cr.arc(width / 2.0, height / 2.0, radius, 0.0, PI * 3.0 / 2.0);
    cr.rel_line_to(radius, 0.0);
    cr.close_path();
    cr.set_source_rgba(1.0, 0.0, 0.0, 0.3);
    cr.fill_preserve();
    cr.set_source_rgb(1.0, 0.0, 0.0);
    cr.stroke();

    // Draw a cross
    cr.move_to(0.0, 0.0);
    cr.line_to(width, height);
    cr.move_to(width, 0.0);
    cr.line_to(0.0, height);

    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.stroke();

    // Add some text somewhere
    cr.set_source_rgb(0.1, 0.1, 0.7);
    cr.move_to(10.0, 30.0);
    cr.set_font_size(30.0);
    cr.show_text("Hi there");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = XCBConnection::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let atoms = AtomCollection::new(&conn)?.reply()?;
    let (mut width, mut height) = (100, 100);
    let window = create_window(&conn, &screen, &atoms, (width, height))?;

    // Here comes all the interaction between cairo and x11rb:
    let mut visual = find_xcb_visualtype(&conn, screen.root_visual).unwrap();
    // SAFETY: cairo-rs just passes the pointer to C code and C code uses the xcb_connection_t, so
    // "nothing really" happens here, except that the borrow checked cannot check the lifetimes.
    let cairo_conn =
        unsafe { cairo::XCBConnection::from_raw_none(conn.get_raw_xcb_connection() as _) };
    let visual = unsafe { cairo::XCBVisualType::from_raw_none(&mut visual as *mut _ as _) };
    let surface = cairo::XCBSurface::create(
        &cairo_conn,
        &cairo::XCBDrawable(window),
        &visual,
        width.into(),
        height.into(),
    )
    .unwrap();

    loop {
        conn.flush()?;
        let event = conn.wait_for_event()?;
        let mut event_option = Some(event);
        let mut need_redraw = false;
        while let Some(event) = event_option {
            println!("{:?})", event);
            match event {
                Event::Expose(_) => {
                    need_redraw = true;
                }
                Event::ConfigureNotify(event) => {
                    width = event.width;
                    height = event.height;
                    surface.set_size(width as _, height as _).unwrap();
                    need_redraw = true;
                }
                Event::ClientMessage(event) => {
                    let data = event.data.as_data32();
                    if event.format == 32
                        && event.window == window
                        && data[0] == atoms.WM_DELETE_WINDOW
                    {
                        println!("Window was asked to close");
                        return Ok(());
                    }
                }
                Event::Error(_) => println!("Got an unexpected error"),
                _ => println!("Got an unknown event"),
            }
            event_option = conn.poll_for_event()?;
        }
        if need_redraw {
            let cr = cairo::Context::new(&surface);
            do_draw(&cr, (width as _, height as _));
            surface.flush();
        }
    }
}
