extern crate x11rb;

use std::io::IoSlice;
use std::convert::TryFrom;
use std::error::Error;
use std::ptr::read_unaligned;
use x11rb::xcb_ffi::{Connection, CSlice};

#[derive(Debug)]
struct GetInputFocusReply(u32, u8);

impl TryFrom<CSlice> for GetInputFocusReply {
    type Error = Box<dyn Error>;

    fn try_from(value: CSlice) -> Result<Self, Self::Error> {
        let focus = unsafe {
            read_unaligned(value[8..12].as_ptr() as _)
        };
        let revert_to = value[1];
        Ok(GetInputFocusReply(focus, revert_to))
    }
}

fn no_operation(c: &Connection) -> Result<u64, Box<dyn Error>> {
    let request = [127, 0, 1, 0];
    let bufs = [IoSlice::new(&request)];
    c.send_request_without_reply(&bufs)
}

fn get_input_focus(c: &Connection) -> Result<GetInputFocusReply, Box<dyn Error>> {
    let request = [43, 0, 1, 0];
    let bufs = [IoSlice::new(&request)];
    let reply = c.send_request_with_reply::<GetInputFocusReply>(&bufs)?;
    let reply = reply.reply()?;
    Ok(reply)
}

fn main() {
    let conn = Connection::new();
    let conn = match conn {
        Ok(conn) => conn,
        Err(_) => panic!("")
    };
    println!("{:?}", no_operation(&conn));
    println!("{:?}", get_input_focus(&conn));
}
