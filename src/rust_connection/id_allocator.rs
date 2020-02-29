use crate::connection::RequestConnection;
use crate::generated::xc_misc::ConnectionExt as _;

/// An allocator for X11 IDs.
///
/// This struct handles the client-side generation of X11 IDs. The ID allocation is based on a
/// range of IDs that the server assigned us. This range is described by a base and a mask. From
/// the X11 protocol reference manual:
///
/// > The resource-id-mask contains a single contiguous set of bits (at least 18). The client
/// > allocates resource IDs [..] by choosing a value with only some subset of these bits set and
/// > ORing it with resource-id-base.
#[derive(Debug)]
pub(crate) struct IDAllocator {
    next_id: u32,
    max_id: u32,
    increment: u32,
}

// FIXME: Clean up this error stuff... somehow
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum MaskError {
    ZeroMask
}

impl std::error::Error for MaskError {}

impl std::fmt::Display for MaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MaskError::ZeroMask => write!(f, "XID mask was zero"),
        }
    }
}

impl IDAllocator {
    /// Create a new instance of an ID allocator.
    ///
    /// The arguments should be the `resource_id_base` and `resource_id_mask` values that the X11
    /// server sent in a `Setup` response.
    pub(crate) fn new(id_base: u32, id_mask: u32) -> Result<Self, MaskError> {
        if id_mask == 0 {
            return Err(MaskError::ZeroMask);
        }
        // Find the right-most set bit in id_mask, e.g. for 0b110, this results in 0b010.
        let increment = id_mask & (1 + !id_mask);
        Ok(Self {
            next_id: id_base,
            max_id: id_base | id_mask,
            increment,
        })
    }

    /// Generate the next ID.
    pub(crate) fn generate_id(&mut self, conn: &impl RequestConnection) -> Option<u32> {
        if self.next_id > self.max_id {
            // Send an XC-MISC GetXIDRange request. Any failure is turned into None via .ok().
            let xidrange = conn
                .get_xidrange()
                .ok()
                .and_then(|cookie| cookie.reply().ok());
            match xidrange {
                Some(reply) => {
                    let (start, count) = (reply.start_id, reply.count);
                    // Apparently (0, 1) is how the server signals "I am out of IDs".
                    // The second case avoids an underflow below and should never happen.
                    if (start, count) == (0, 1) || count == 0 {
                        return None;
                    }
                    self.next_id = start;
                    self.max_id = start + (count - 1) * self.increment;
                }
                None => return None,
            }
        }
        assert!(self.next_id <= self.max_id);
        let id = self.next_id;
        self.next_id += self.increment;
        Some(id)
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use std::io::IoSlice;

    use crate::connection::{DiscardMode, RequestConnection, RequestKind, SequenceNumber};
    use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
    use crate::errors::{ConnectionError, ConnectionErrorOrX11Error, ParseError};
    use crate::generated::xproto::QueryExtensionReply;
    use crate::utils::{Buffer, RawFdContainer};
    use crate::x11_utils::GenericError;

    use super::IDAllocator;

    #[test]
    fn exhaustive() {
        let conn = DummyConnection(None);
        let mut allocator = IDAllocator::new(0x2800, 0x1ff).unwrap();
        for expected in 0x2800..=0x29ff {
            assert_eq!(Some(expected), allocator.generate_id(&conn));
        }
        assert_eq!(None, allocator.generate_id(&conn));
    }

    #[test]
    fn increment() {
        let conn = DummyConnection(None);
        let mut allocator = IDAllocator::new(0, 0b1100).unwrap();
        assert_eq!(Some(0b0000), allocator.generate_id(&conn));
        assert_eq!(Some(0b0100), allocator.generate_id(&conn));
        assert_eq!(Some(0b1000), allocator.generate_id(&conn));
        assert_eq!(Some(0b1100), allocator.generate_id(&conn));
        assert_eq!(None, allocator.generate_id(&conn));
    }

    #[test]
    fn new_range() {
        let conn = DummyConnection(Some(generate_get_xid_range_reply(0x13370, 3)));
        let mut allocator = IDAllocator::new(0x420, 2).unwrap();
        assert_eq!(Some(0x420), allocator.generate_id(&conn));
        assert_eq!(Some(0x420 + 2), allocator.generate_id(&conn));
        // At this point the range is exhausted and a GetXIDRange request is sent
        assert_eq!(Some(0x13370), allocator.generate_id(&conn));
        assert_eq!(Some(0x13370 + 2), allocator.generate_id(&conn));
        assert_eq!(Some(0x13370 + 4), allocator.generate_id(&conn));
        // At this point the range is exhausted and a GetXIDRange request is sent
        assert_eq!(Some(0x13370), allocator.generate_id(&conn));
    }

    #[test]
    fn invalid_arg() {
        let allocator = IDAllocator::new(1234, 0).unwrap_err();
        assert_eq!(super::MaskError::ZeroMask, allocator);
    }

    fn generate_get_xid_range_reply(start_id: u32, count: u32) -> Vec<u8> {
        let mut reply = vec![0; 8];
        reply.extend(&start_id.to_ne_bytes());
        reply.extend(&count.to_ne_bytes());
        reply
    }

    // If the Option is None, the GetXIDRange request fails (unsupported extension). Otherwise,
    // this is the raw reply that is received for that request.
    struct DummyConnection(Option<Vec<u8>>);

    impl RequestConnection for DummyConnection {
        fn send_request_with_reply<R>(
            &self,
            _bufs: &[IoSlice],
            _fds: Vec<RawFdContainer>,
        ) -> Result<Cookie<Self, R>, ConnectionError>
        where
            R: TryFrom<Buffer, Error = ParseError>,
        {
            Ok(Cookie::new(self, 0))
        }

        fn send_request_with_reply_with_fds<R>(
            &self,
            _bufs: &[IoSlice],
            _fds: Vec<RawFdContainer>,
        ) -> Result<CookieWithFds<Self, R>, ConnectionError>
        where
            R: TryFrom<(Buffer, Vec<RawFdContainer>), Error = ParseError>,
        {
            unimplemented!()
        }

        fn send_request_without_reply(
            &self,
            _bufs: &[IoSlice],
            _fds: Vec<RawFdContainer>,
        ) -> Result<VoidCookie<Self>, ConnectionError> {
            unimplemented!()
        }

        fn discard_reply(&self, _sequence: SequenceNumber, _kind: RequestKind, _mode: DiscardMode) {
            unimplemented!()
        }

        fn extension_information(
            &self,
            _extension_name: &'static str,
        ) -> Option<QueryExtensionReply> {
            #[allow(trivial_casts, clippy::unnecessary_cast)]
            let present = true as _;
            self.0.as_ref().map(|_| QueryExtensionReply {
                response_type: 1,
                sequence: 0,
                length: 0,
                present,
                major_opcode: 127,
                first_event: 0,
                first_error: 0,
            })
        }

        fn wait_for_reply_or_error(
            &self,
            _sequence: SequenceNumber,
        ) -> Result<Buffer, ConnectionErrorOrX11Error> {
            Ok(Buffer::from_vec(self.0.as_ref().unwrap().clone()))
        }

        fn wait_for_reply(
            &self,
            _sequence: SequenceNumber,
        ) -> Result<Option<Buffer>, ConnectionError> {
            unimplemented!()
        }

        fn wait_for_reply_with_fds(
            &self,
            _sequence: SequenceNumber,
        ) -> Result<(Buffer, Vec<RawFdContainer>), ConnectionErrorOrX11Error> {
            unimplemented!()
        }

        fn check_for_error(
            &self,
            _sequence: SequenceNumber,
        ) -> Result<Option<GenericError>, ConnectionError> {
            unimplemented!()
        }

        fn maximum_request_bytes(&self) -> usize {
            unimplemented!()
        }
    }
}
