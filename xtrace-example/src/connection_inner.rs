#[derive(Debug, Default)]
pub struct ConnectionInner {
}

impl ConnectionInner {
    /// Handle the client's SetupRequest
    pub fn client_setup(&mut self, packet: &[u8]) {
        let _ = packet;
    }

    /// Handle a request sent by the client
    pub fn client_request(&mut self, packet: &[u8]) {
        let _ = packet;
    }

    /// Handle the server's Setup (or SetupFailed, SetupAuthenticate)
    pub fn server_setup(&mut self, packet: &[u8]) {
        let _ = packet;
    }

    /// Handle an X11 error sent by the server
    pub fn server_error(&mut self, packet: &[u8]) {
        let _ = packet;
    }

    /// Handle an X11 event sent by the server
    pub fn server_event(&mut self, packet: &[u8]) {
        let _ = packet;
    }

    /// Handle a reply sent by the server
    pub fn server_reply(&mut self, packet: &[u8]) {
        let _ = packet;
    }
}
