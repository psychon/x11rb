// This program should produce output identical to `xlsfonts -lu`.

extern crate x11rb;

use x11rb::xcb_ffi::XCBConnection;
use x11rb::generated::xproto::*;

fn main() {
    let (conn, _) = XCBConnection::connect(None).unwrap();
    let (ltr, rtl): (u8, u8) = (FontDraw::LeftToRight.into(), FontDraw::RightToLeft.into());

    println!("DIR  MIN  MAX EXIST DFLT PROP ASC DESC NAME");
    for reply in list_fonts_with_info(&conn, u16::max_value(), "*".as_bytes()).unwrap() {
        let reply = reply.unwrap();

        let dir = if reply.draw_direction == ltr {
            "-->"
        } else if reply.draw_direction == rtl {
            "<--"
        } else {
            "???"
        };

        let (min, max, indicator) = if reply.min_byte1 == 0 && reply.max_byte1 == 0 {
            (reply.min_char_or_byte2, reply.max_char_or_byte2, ' ')
        } else {
            (reply.min_byte1 as u16, reply.max_byte1 as u16, '*')
        };

        let all = if reply.all_chars_exist != 0 { "all" } else { "some" };

        let name = String::from_utf8_lossy(&reply.name);

        println!("{} {}{:3} {}{:3} {:>5} {:4} {:4} {:3} {:4} {}", dir, indicator, min, indicator,
                 max, all, reply.default_char, reply.properties.len(), reply.font_ascent,
                 reply.font_descent, name);
    }
}
