// Regression test for https://github.com/psychon/x11rb/issues/231

use std::sync::Arc;

use x11rb::connection::Connection as _;
use x11rb::generated::xproto::{
    ClientMessageData, ClientMessageEvent, ConnectionExt as _, CreateWindowAux, EventMask,
    WindowClass, CLIENT_MESSAGE_EVENT,
};
use x11rb::x11_utils::Event as _;
use x11rb::COPY_DEPTH_FROM_PARENT;

fn main() {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    let conn = Arc::new(conn);

    let screen = &conn.setup().roots[screen_num];
    let window = conn.generate_id();
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        window,
        screen.root,
        0, // x
        0, // y
        1, // width
        1, // height
        0,
        WindowClass::InputOutput,
        screen.root_visual,
        &CreateWindowAux::new().event_mask(EventMask::NoEvent),
    )
    .unwrap();
    conn.flush().unwrap();

    // Auxiliary thread: send requests and wait for replies
    let conn1 = conn.clone();
    let join = std::thread::spawn(move || {
        // Bug #231 sometimes caused `reply` to hang forever.
        // Send a huge amount of requests and wait for the reply
        // to check if it hangs at some point.
        for i in 1..=1_000_000 {
            let cookie = conn1.get_input_focus().unwrap();
            cookie.reply().unwrap();

            if (i % 50_000) == 0 {
                eprintln!("{}", i);
            }
        }
        eprintln!("all replies received successfully");

        let event = ClientMessageEvent {
            response_type: CLIENT_MESSAGE_EVENT,
            format: 32,
            sequence: 0,
            window,
            // Just anything, we don't care
            type_: 1,
            data: ClientMessageData::from([0, 0, 0, 0, 0]),
        };

        conn1
            .send_event(false, window, EventMask::NoEvent.into(), &event)
            .unwrap();
        conn1
            .send_event(
                false,
                window,
                EventMask::SubstructureRedirect.into(),
                &event,
            )
            .unwrap();
        conn1.flush().unwrap();
    });

    // Main thread: wait for events until finished
    loop {
        let event = conn.wait_for_event().unwrap();
        if event.response_type() == CLIENT_MESSAGE_EVENT {
            break;
        }
    }

    join.join().unwrap();
}
