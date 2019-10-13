// This is (quite loosely) based on the original xeyes

extern crate x11rb;

use std::convert::TryFrom;

use x11rb::xcb_ffi::XCBConnection;
use x11rb::connection::Connection;
use x11rb::x11_utils::Event;
use x11rb::generated::xproto::*;
use x11rb::generated::shape;
use x11rb::wrapper::*;
use x11rb::errors::ConnectionError;

const PUPIL_SIZE: i16 = 50;
const EYE_SIZE: i16 = 50;

// Draw the big background of the eyes
fn draw_eyes<C: Connection>(conn: &C, win_id: WINDOW, black: GCONTEXT, white: GCONTEXT,
                            window_size: (u16, u16))
-> Result<(), ConnectionError>
{
    // Draw the black outlines
    let mut arc1 = Arc {
        x: 0,
        y: 0,
        width: window_size.0 / 2,
        height: window_size.1,
        angle1: 0*64,
        angle2: 360*64
    };
    let mut arc2 = arc1.clone();
    arc2.x = arc2.width as i16;
    poly_fill_arc(conn, win_id, black, &[arc1, arc2])?;

    // Draw the white inner part
    for mut arc in [&mut arc1, &mut arc2].iter_mut() {
        arc.x += EYE_SIZE;
        arc.y += EYE_SIZE;
        arc.width -= 2 * EYE_SIZE as u16;
        arc.height -= 2 * EYE_SIZE as u16;
    }
    poly_fill_arc(conn, win_id, white, &[arc1, arc2])?;

    Ok(())
}

// Draw the pupils inside the eye
fn draw_pupils<C: Connection>(conn: &C, win_id: WINDOW, gc: GCONTEXT,
                              ((x1, y1), (x2, y2)): ((i16, i16), (i16, i16)))
-> Result<(), ConnectionError>
{
    // Transform center to top left corner
    let (x1, y1) = (x1 - PUPIL_SIZE / 2, y1 - PUPIL_SIZE / 2);
    let (x2, y2) = (x2 - PUPIL_SIZE / 2, y2 - PUPIL_SIZE / 2);

    let arc1 = Arc {
        x: x1,
        y: y1,
        width: PUPIL_SIZE as _,
        height: PUPIL_SIZE as _,
        angle1: 90 * 64,
        angle2: 360*64
    };
    let mut arc2 = arc1.clone();
    arc2.x = x2;
    arc2.y = y2;

    // Do the drawing
    poly_fill_arc(conn, win_id, gc, &[arc1, arc2])?;
    Ok(())
}

// Given two points, return their squared distance
fn distance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    dx * dx + dy * dy
}

// Compute the position of a pupil inside of the given area.
fn compute_pupil(area: (i16, i16, i16, i16), mouse: (i16, i16)) -> (i16, i16)
{
    // What is the center of the eye?
    let center_x = area.0 + area.2 / 2;
    let center_y = area.1 + area.3 / 2;
    let (w, h) = (area.2 as f64 / 2.0, area.3 as f64 / 2.0);

    // Is the mouse exactly on the center?
    if (center_x, center_y) == mouse {
        return mouse;
    }

    let center = (center_x as f64, center_y as f64);
    let mouse = (mouse.0 as f64, mouse.1 as f64);

    // Calculate the offset of the mouse position from the center
    let diff = (mouse.0 as f64 - center.0, mouse.1 as f64 - center.1);

    // An eclipse is described by this equation, where the angle 'a' is varied over all values, but
    // does not actually describe the angle from the center due to the different scaling in x and y
    // direction.
    //
    //  x = w * cos(a)
    //  y = h * sin(a)
    //
    // With tan(a) = sin(a)/cos(a), we get
    //
    //  tan(a) * x = w * sin(a) => sin(a) = tan(a) * x / w
    //  y = h * sin(a)          => sin(a) = y / h
    //
    // and thus
    //
    //   tan(a) * x / w = y / h
    //
    // which we can rearrange to
    //
    //   tan(a) = (y * w) / (x * h)
    //
    // And thus, the angle we are looking for is:
    //
    //   a = arctan((y * w) / (x * h))
    //
    // However, due to tan() being the way it is, we actually need:
    let a = (diff.1 * w).atan2(diff.0 * h);

    // Now compute the corresponding point on the ellipse (relative to the center)
    let (cx, cy) = (w * a.cos(), h * a.sin());

    // ...and also compute the actual point
    let (x, y) = ((center.0 + cx) as _, (center.1 + cy) as _);

    // Return the point that is closer to the center
    if distance_squared(center, mouse) < distance_squared(center, (x, y)) {
        (mouse.0 as _, mouse.1 as _)
    } else {
        (x as _, y as _)
    }
}

// Compute the position of both pupils.
fn compute_pupils(window_size: (u16, u16), mouse_position: (i16, i16)) -> ((i16, i16), (i16, i16))
{
    let border = PUPIL_SIZE + EYE_SIZE;
    let half_width = window_size.0 as i16 / 2;
    let width = half_width - 2 * border;
    let height = window_size.1 as i16 - 2 * border;

    (compute_pupil((border, border, width, height), mouse_position),
     compute_pupil((border + half_width, border, width, height), mouse_position))
}

struct FreePixmap<'c, C: Connection>(&'c C, PIXMAP);
impl<C: Connection> Drop for FreePixmap<'_, C> {
    fn drop(&mut self) {
        free_pixmap(self.0, self.1).unwrap();
    }
}
struct FreeGC<'c, C: Connection>(&'c C, GCONTEXT);
impl<C: Connection> Drop for FreeGC<'_, C> {
    fn drop(&mut self) {
        free_gc(self.0, self.1).unwrap();
    }
}

fn shape_window<C: Connection>(conn: &C, win_id: WINDOW, window_size: (u16, u16))
-> Result<(), ConnectionError>
{
    // Create a pixmap for the shape
    let pixmap = conn.generate_id();
    create_pixmap(conn, 1, pixmap, win_id, window_size.0, window_size.1)?;
    let _free_pixmap = FreePixmap(conn, pixmap);

    // Fill the pixmap with what will indicate "transparent"
    let gc = create_gc_with_foreground(conn, pixmap, 0)?;
    let _free_gc = FreeGC(conn, gc);

    let rect = Rectangle {
        x: 0,
        y: 0,
        width: window_size.0,
        height: window_size.1
    };
    poly_fill_rectangle(conn, pixmap, gc, &[rect])?;

    // Draw the eyes as "not transparent"
    let values = ChangeGCAux::new().foreground(1);
    change_gc(conn, gc, &values)?;
    draw_eyes(conn, pixmap, gc, gc, window_size)?;

    // Set the shape of the window
    shape::mask(conn, shape::SO::Set, shape::SK::Bounding, win_id, 0, 0, pixmap)?;
    Ok(())
}

fn setup_window<C: Connection>(conn: &C, screen: &Screen, window_size: (u16, u16),
                               wm_protocols: ATOM, wm_delete_window: ATOM)
-> Result<WINDOW, ConnectionError>
{
    let win_id = conn.generate_id();
    let win_aux = CreateWindowAux::new()
        .event_mask(EventMask::Exposure | EventMask::StructureNotify | EventMask::PointerMotion)
        .background_pixel(screen.white_pixel);

    create_window(conn, 24, win_id, screen.root, 0, 0, window_size.0, window_size.1, 0,
                  WindowClass::InputOutput, 0, &win_aux)?;

    let title = "xeyes";
    change_property8(conn, PropMode::Replace, win_id, Atom::WM_NAME.into(), Atom::STRING.into(), title.as_bytes()).unwrap();
    change_property32(conn, PropMode::Replace, win_id, wm_protocols, Atom::ATOM.into(), &[wm_delete_window]).unwrap();

    map_window(conn, win_id).unwrap();

    Ok(win_id)
}

fn create_gc_with_foreground<C: Connection>(conn: &C, win_id: WINDOW, foreground: u32)
-> Result<GCONTEXT, ConnectionError>
{
    let gc = conn.generate_id();
    let gc_aux = CreateGCAux::new()
        .foreground(foreground);
    create_gc(conn, gc, win_id, &gc_aux)?;
    Ok(gc)
}

fn main() {
    let (conn, screen_num) = XCBConnection::connect(None).expect("Failed to connect to the X11 server");
    let screen = &conn.setup().roots[screen_num];

    let (wm_protocols, wm_delete_window) = {
        let protocols = intern_atom(&conn, 0, "WM_PROTOCOLS".as_bytes()).unwrap();
        let delete = intern_atom(&conn, 0, "WM_DELETE_WINDOW".as_bytes()).unwrap();
        (protocols.reply().unwrap().atom, delete.reply().unwrap().atom)
    };

    let mut window_size = (700, 500);
    let has_shape = conn.extension_information(shape::X11_EXTENSION_NAME).is_some();
    let win_id = setup_window(&conn, screen, window_size, wm_protocols, wm_delete_window).unwrap();

    let black_gc = create_gc_with_foreground(&conn, win_id, screen.black_pixel).unwrap();
    let white_gc = create_gc_with_foreground(&conn, win_id, screen.white_pixel).unwrap();

    conn.flush();

    let mut need_repaint = false;
    let mut need_reshape = false;
    let mut mouse_position = (0, 0);

    loop {
        let event = conn.wait_for_event().unwrap();
        let mut event_option = Some(event);
        while let Some(event) = event_option {
            match event.response_type() {
                EXPOSE_EVENT => {
                    let event = ExposeEvent::try_from(event);
                    if let Ok(event) = event {
                        if event.count == 0 {
                            need_repaint = true;
                        }
                    }
                }
                CONFIGURE_NOTIFY_EVENT => {
                    let event = ConfigureNotifyEvent::try_from(event);
                    if let Ok(event) = event {
                        window_size = (event.width, event.height);
                        need_reshape = true;
                    }
                }
                MOTION_NOTIFY_EVENT => {
                    let event = MotionNotifyEvent::try_from(event);
                    if let Ok(event) = event {
                        mouse_position = (event.event_x, event.event_y);
                        need_repaint = true;
                    }
                }
                MAP_NOTIFY_EVENT => {
                    need_reshape = true;
                }
                CLIENT_MESSAGE_EVENT => {
                    let event = ClientMessageEvent::try_from(event);
                    if let Ok(event) = event {
                        let data = event.data.as_data32();
                        if event.format == 32 && event.window == win_id && data[0] == wm_delete_window {
                            println!("Window was asked to close");
                            return;
                        }
                    }
                }
                0 => { println!("Unknown error {:?}", event); },
                _ => { println!("Unknown event {:?}", event); }
            }

            event_option = conn.poll_for_event().unwrap();
        }

        if need_reshape && has_shape {
            shape_window(&conn, win_id, window_size).unwrap();
            need_reshape = false;
        }
        if need_repaint {
            clear_area(&conn, 0, win_id, 0, 0, 0, 0).unwrap();

            // Draw new pupils
            let pos = compute_pupils(window_size, mouse_position);
            draw_eyes(&conn, win_id, black_gc, white_gc, window_size).unwrap();
            draw_pupils(&conn, win_id, black_gc, pos).unwrap();

            conn.flush();
            need_repaint = false;
        }
    }
}
