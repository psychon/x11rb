use x11rb::errors::ParseError;
use x11rb::protocol::xproto;
use x11rb::x11_utils::TryParse;

fn print_parse_return<T: TryParse + std::fmt::Debug>(data: &[u8]) -> Result<T, ParseError> {
    match T::try_parse(data) {
        Err(e) => {
            println!("Error while parsing: {:?}", e);
            Err(e)
        }
        Ok((obj, remaining)) => {
            println!("{:?}", obj);
            if !remaining.is_empty() {
                println!("Left-over data while parsing:");
                println!("Input:    {:x?}", data);
                println!("Remaining {:x?}", remaining);
            }
            Ok(obj)
        }
    }
}

fn print_parse<T: TryParse + std::fmt::Debug>(data: &[u8]) {
    let _ = print_parse_return::<T>(data);
}

#[derive(Debug, Default)]
pub struct ConnectionInner {
}

impl ConnectionInner {
    /// Handle the client's SetupRequest
    pub fn client_setup(&mut self, packet: &[u8]) {
        // Check the byte order before parsing, because we cannot parse length fields with the
        // wrong byte order
        #[cfg(target_endian = "little")]
        let byte_order = 0x6c;
        #[cfg(target_endian = "big")]
        let byte_order = 0x42;
        if byte_order != packet[0] {
            eprintln!(
                "Client is unexpected byte order {:x} != {:x}, only native byte order is supported!",
                byte_order,
                packet[0],
            );
        }
        print_parse::<xproto::SetupRequest>(packet);
    }

    /// Handle the server's Setup (or SetupFailed, SetupAuthenticate)
    pub fn server_setup(&mut self, packet: &[u8]) {
        match packet[0] {
            0 => print_parse::<xproto::SetupFailed>(packet),
            1 => {
                if let Ok(setup) = print_parse_return::<xproto::Setup>(packet) {
                    let expected = (11, 0);
                    let actual = (setup.protocol_major_version, setup.protocol_minor_version);
                    if expected != actual {
                        println!(
                            "Unexpected protocol version: {}.{} != {}.{}",
                             expected.0,
                             expected.1,
                             actual.0,
                             actual.1,
                         );
                    }
                }
            }
            2 => print_parse::<xproto::SetupAuthenticate>(packet),
            _ => eprintln!("Unknown server setup response: {:?}", packet),
        }
    }

    /// Handle a request sent by the client
    pub fn client_request(&mut self, packet: &[u8]) {
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
