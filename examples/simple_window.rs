extern crate x11rb;

use x11rb::xcb_ffi::Connection;
use x11rb::generated::xproto::{no_operation, get_input_focus};


fn main() {
    let conn = Connection::new();
    let conn = match conn {
        Ok(conn) => conn,
        Err(_) => panic!("")
    };
    println!("{:?}", no_operation(&conn).unwrap());
    println!("{:?}", get_input_focus(&conn).unwrap().reply());
}
