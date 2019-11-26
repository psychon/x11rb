// This program shows error handling. It causes some X11 errors and shows where they end up.

extern crate x11rb;

use x11rb::xcb_ffi::XCBConnection;
use x11rb::connection::Connection;
use x11rb::generated::xproto::ConnectionExt;
use x11rb::errors::ConnectionErrorOrX11Error;
use x11rb::x11_utils::Event;

const INVALID_WINDOW: u32 = 0;

fn main() -> Result<(), ConnectionErrorOrX11Error> {
    let (conn, _) = XCBConnection::connect(None).unwrap();

    // For requests with responses, there are four possibilities:

    // We can just normally get the response or error to the request via reply()
    let res = conn.get_geometry(INVALID_WINDOW)?.reply();
    assert!(res.is_err());

    // We can decide that we do not care about the response and also do not care about errors via
    // discard_reply_and_errors()
    conn.get_geometry(INVALID_WINDOW)?.discard_reply_and_errors();

    // Errors can show up as 'events' in wait_for_event() via reply_unchecked()
    let cookie = conn.get_geometry(INVALID_WINDOW)?;
    let seq1 = cookie.sequence_number();
    let res = cookie.reply_unchecked()?;
    assert!(res.is_none());

    // Errors can show up as 'events' in wait_for_event() by just dropping the cookie
    let cookie = conn.get_geometry(INVALID_WINDOW)?;
    let seq2 = cookie.sequence_number();
    std::mem::drop(cookie);

    // For requests without responses, there are three possibilities

    // We can check for errors explicitly
    let res = conn.destroy_window(INVALID_WINDOW)?.check()?;
    assert!(res.is_some());

    // We can silently ignore the error
    conn.destroy_window(INVALID_WINDOW)?.ignore_error();

    // An error can be handled as an event.
    let cookie = conn.destroy_window(INVALID_WINDOW)?;
    let seq3 = cookie.sequence_number();
    std::mem::drop(cookie);

    // Synchronise with the server so that all errors are already received.
    // FIXME: Add a wrapper to simplify this.
    conn.get_input_focus()?.reply()?;

    // Now check if the things above really caused errors
    for &seq in &[seq1, seq2, seq3] {
        let event = conn.wait_for_event()?;
        assert_eq!(0, event.response_type());
        assert_eq!(Some(seq as _), event.raw_sequence_number());
    }
    assert!(conn.poll_for_event()?.is_none());

    println!("Done");

    Ok(())
}
