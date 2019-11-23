// As far as I can see, I cannot easily share code between different examples. The following code
// is used by several examples to react to the $X11RB_EXAMPLE_TIMEOUT variable. This code is
// include!()d in the examples

mod util {
    use std::thread;
    use std::env;
    use std::sync::Arc;
    use std::time::Duration;

    use x11rb::connection::Connection;
    use x11rb::x11_utils::TryParse;
    use x11rb::generated::xproto::{WINDOW, ConnectionExt as _, EventMask, ClientMessageData, ClientMessageEvent, CLIENT_MESSAGE_EVENT};

    pub fn start_timeout_thread<C>(conn: Arc<C>, window: WINDOW)
    where C: Connection + Send + Sync + 'static
    {
        let timeout = match env::var("X11RB_EXAMPLE_TIMEOUT").ok()
                .and_then(|str| str.parse().ok()) {
            None => return,
            Some(timeout) => timeout
        };

        let (wm_protocols, wm_delete_window) = {
            let protocols = conn.intern_atom(false, "WM_PROTOCOLS".as_bytes()).unwrap();
            let delete = conn.intern_atom(false, "WM_DELETE_WINDOW".as_bytes()).unwrap();
            (protocols.reply().unwrap().atom, delete.reply().unwrap().atom)
        };

        let mut data = [0; 20];
        data[..4].copy_from_slice(&wm_delete_window.to_ne_bytes());
        let (data, _): (ClientMessageData, _) = TryParse::try_parse(&data).unwrap();
        let event = ClientMessageEvent {
            response_type: CLIENT_MESSAGE_EVENT,
            format: 32,
            sequence: 0,
            window: window,
            type_: wm_protocols,
            data
        };

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            if let Err(err) = conn.send_event(0, window, EventMask::NoEvent.into(), &event) {
                eprintln!("Error while sending event: {:?}", err);
            }
            if let Err(err) = conn.send_event(0, window, EventMask::SubstructureRedirect.into(), &event) {
                eprintln!("Error while sending event: {:?}", err);
            }
            conn.flush();
        });
    }
}
