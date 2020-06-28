use x11rb::errors::ParseError;
use x11rb::protocol::{Error, Event, Reply, Request, xproto};
use x11rb::x11_utils::{BigRequests, ExtInfoProvider, ExtensionInformation, TryParse, ReplyParsingFunction, parse_request_header};

use std::collections::VecDeque;
use std::convert::TryInto;

fn print_obj_remaining(obj: &impl std::fmt::Debug, data: &[u8], remaining: &[u8]) {
    println!("{:?}", obj);
    if !remaining.is_empty() {
        println!("Left-over data while parsing:");
        println!("Input:    {:x?}", data);
        println!("Remaining {:x?}", remaining);
    }
}

fn print_parse_return<T: TryParse + std::fmt::Debug>(data: &[u8]) -> Result<T, ParseError> {
    match T::try_parse(data) {
        Err(e) => {
            println!("Error while parsing: {:?}", e);
            Err(e)
        }
        Ok((obj, remaining)) => {
            print_obj_remaining(&obj, data, remaining);
            Ok(obj)
        }
    }
}

fn print_parse<T: TryParse + std::fmt::Debug>(data: &[u8]) {
    let _ = print_parse_return::<T>(data);
}

#[derive(Debug, Default)]
pub struct ConnectionInner {
    ext_info: ExtInfo,
    next_client_request: u16,
    pending_replies: VecDeque<PendingReply>,
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
        assert_eq!(self.next_client_request, 0);
        self.next_client_request = 1;
    }

    /// Handle the server's Setup (or SetupFailed, SetupAuthenticate)
    pub fn server_setup(&mut self, packet: &[u8]) {
        print!("server: ");
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
        fn do_parse(inner: &mut ConnectionInner, packet: &[u8]) -> Result<(), ParseError> {
            let seqno = inner.next_client_request;
            inner.next_client_request = seqno.wrapping_add(1);

            let (header, remaining) = parse_request_header(packet, BigRequests::Enabled)?;
            let request = Request::parse(header, remaining, &mut Vec::new(), &inner.ext_info)?;
            // TODO: Can we get some "remaining" data somehow?
            //print_obj_remaining(&request, packet, remaining);
            println!("client ({}): {:?}", seqno, request);

            // Is this a QueryExtension?
            let queried_extension = if let Request::QueryExtension(ref request) = request {
                match String::from_utf8(request.name.to_vec()) {
                    Ok(name) => {
                        println!("Extension name: {}", name);
                        Some(name)
                    },
                    Err(e) => {
                        println!("Extension name is not utf8: {:?}", e);
                        None
                    }
                }
            } else {
                None
            };

            // Does the request have a reply? If so, remember it.
            if let Some(parser) = request.reply_parser() {
                inner.pending_replies.push_back(PendingReply::new(seqno, parser, queried_extension));
            }

            Ok(())
        }
        if let Err(e) = do_parse(self, packet) {
            eprintln!("Error while parsing a client request: {:?}", e);
        }
    }

    /// Handle an X11 error sent by the server
    pub fn server_error(&mut self, packet: &[u8]) {
        fn do_parse(inner: &mut ConnectionInner, packet: &[u8]) -> Result<(), ParseError> {
            let err = Error::parse(packet, &inner.ext_info)?;
            println!("server ({}): {:?}", err.wire_sequence_number(), err);

            // Remove a pending request if it failed
            if inner.pending_replies.front().map(|r| r.seqno) == Some(err.wire_sequence_number()) {
                let _ = inner.pending_replies.pop_front();
            }

            Ok(())
        }
        if let Err(e) = do_parse(self, packet) {
            eprintln!("Error while parsing an X11 error: {:?}", e);
        }
    }

    /// Handle an X11 event sent by the server
    pub fn server_event(&mut self, packet: &[u8]) {
        fn do_parse(inner: &mut ConnectionInner, packet: &[u8]) -> Result<(), ParseError> {
            let event = Event::parse(packet, &inner.ext_info)?;
            println!("server ({}): {:?}", event.wire_sequence_number().unwrap_or(0), event);
            Ok(())
        }
        if let Err(e) = do_parse(self, packet) {
            eprintln!("Error while parsing an X11 event: {:?}", e);
        }
    }

    /// Handle a reply sent by the server
    pub fn server_reply(&mut self, packet: &[u8]) {
        fn do_parse(inner: &mut ConnectionInner, packet: &[u8]) -> Result<(), ParseError> {
            let request = match inner.pending_replies.pop_front() {
                None => {
                    println!("server: Got unexpected reply {:?}", packet);
                    return Ok(())
                },
                Some(request) => request,
            };

            let seqno = u16::from_ne_bytes(packet[2..4].try_into().unwrap());
            if request.seqno != seqno {
                println!("Expected reply with seqno={}, but got seqno={}", request.seqno, seqno);
            }

            let (reply, remaining) = (request.parser)(packet, &mut Vec::new())?;
            print!("server ({}): ", seqno);
            print_obj_remaining(&reply, packet, remaining);

            if let Some(extension) = request.queried_extension {
                if let Reply::QueryExtension(reply) = reply {
                    if reply.present {
                        let info = ExtensionInformation {
                            major_opcode: reply.major_opcode,
                            first_event: reply.first_event,
                            first_error: reply.first_error,
                        };
                        inner.ext_info.add_extension(extension, info);
                    }
                }
            }

            Ok(())
        }
        if let Err(e) = do_parse(self, packet) {
            eprintln!("Error while parsing an X11 event: {:?}", e);
        }
    }
}

struct PendingReply {
    seqno: u16,
    parser: ReplyParsingFunction,
    queried_extension: Option<String>,
}

impl std::fmt::Debug for PendingReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingReply")
            .field("seqno", &self.seqno)
            .field("parser", &"<???>")
            .field("queried_extension", &self.queried_extension)
            .finish()
    }
}

impl PendingReply {
    fn new(seqno: u16, parser: ReplyParsingFunction, queried_extension: Option<String>) -> Self {
        Self { seqno, parser, queried_extension }
    }
}

/// Information about known extensions
#[derive(Debug, Default)]
struct ExtInfo {
    exts: Vec<(String, ExtensionInformation)>,
}

impl ExtInfo {
    /// Add a new extension to the state
    fn add_extension(&mut self, name: String, info: ExtensionInformation) {
        self.exts.push((name, info))
    }
}

impl ExtInfoProvider for ExtInfo {
    fn get_from_major_opcode(&self, major_opcode: u8) -> Option<(&str, ExtensionInformation)> {
        self.exts
            .iter()
            .find(|(_, ext)| ext.major_opcode == major_opcode)
            .map(|(s, ext)| (s.as_ref(), *ext))
    }

    fn get_from_event_code(&self, event_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.exts
            .iter()
            .filter(|(_, ext)| ext.first_event <= event_code)
            .max_by_key(|(_, ext)| ext.first_event)
            .map(|(s, ext)| (s.as_ref(), *ext))
    }

    fn get_from_error_code(&self, error_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.exts
            .iter()
            .filter(|(_, ext)| ext.first_error <= error_code)
            .max_by_key(|(_, ext)| ext.first_event)
            .map(|(s, ext)| (s.as_ref(), *ext))
    }
}
