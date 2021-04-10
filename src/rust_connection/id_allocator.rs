use crate::connection::RequestConnection;
use crate::errors::{ConnectError, ReplyOrIdError};
use crate::protocol::xc_misc::{self, ConnectionExt as _, GetXIDRangeReply};

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
pub(crate) struct IdAllocator {
    next_id: u32,
    max_id: u32,
    increment: u32,
}

impl IdAllocator {
    /// Create a new instance of an ID allocator.
    ///
    /// The arguments should be the `resource_id_base` and `resource_id_mask` values that the X11
    /// server sent in a `Setup` response.
    pub(crate) fn new(id_base: u32, id_mask: u32) -> Result<Self, ConnectError> {
        if id_mask == 0 {
            return Err(ConnectError::ZeroIDMask);
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
    ///
    /// The given connection is used to ask for more IDs if necessary.
    pub(crate) fn generate_id<C: RequestConnection>(
        &mut self,
        conn: &C,
    ) -> Result<u32, ReplyOrIdError> {
        self.generate_id_impl(|| {
            if conn
                .extension_information(xc_misc::X11_EXTENSION_NAME)?
                .is_none()
            {
                // IDs are exhausted and XC-MISC is not available
                Err(ReplyOrIdError::IdsExhausted)
            } else {
                Ok(conn.xc_misc_get_xid_range()?.reply()?)
            }
        })
    }

    /// Generate the next ID.
    ///
    /// The `get_xid_range` callback is used to request more IDs from the X11 server if necessary.
    fn generate_id_impl<F>(&mut self, get_xid_range: F) -> Result<u32, ReplyOrIdError>
    where
        F: FnOnce() -> Result<GetXIDRangeReply, ReplyOrIdError>,
    {
        if self.next_id > self.max_id {
            // Send an XC-MISC GetXIDRange request.
            let xidrange = get_xid_range()?;
            let (start, count) = (xidrange.start_id, xidrange.count);
            // Apparently (0, 1) is how the server signals "I am out of IDs".
            // The second case avoids an underflow below and should never happen.
            if (start, count) == (0, 1) || count == 0 {
                return Err(ReplyOrIdError::IdsExhausted);
            }
            self.next_id = start;
            self.max_id = start + (count - 1) * self.increment;
        }
        assert!(self.next_id <= self.max_id);
        let id = self.next_id;
        self.next_id += self.increment;
        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use crate::errors::ReplyOrIdError;
    use crate::protocol::xc_misc::GetXIDRangeReply;

    use super::IdAllocator;

    fn unreachable_cb() -> Result<GetXIDRangeReply, ReplyOrIdError> {
        unreachable!()
    }

    #[test]
    fn exhaustive() {
        let mut allocator = IdAllocator::new(0x2800, 0x1ff).unwrap();
        for expected in 0x2800..=0x29ff {
            assert_eq!(
                expected,
                allocator.generate_id_impl(unreachable_cb).unwrap()
            );
        }
        let cb = || -> Result<_, ReplyOrIdError> { Err(ReplyOrIdError::IdsExhausted) };
        assert!(allocator.generate_id_impl(cb).is_err());
    }

    #[test]
    fn increment() {
        let mut allocator = IdAllocator::new(0, 0b1100).unwrap();
        assert_eq!(0b0000, allocator.generate_id_impl(unreachable_cb).unwrap());
        assert_eq!(0b0100, allocator.generate_id_impl(unreachable_cb).unwrap());
        assert_eq!(0b1000, allocator.generate_id_impl(unreachable_cb).unwrap());
        assert_eq!(0b1100, allocator.generate_id_impl(unreachable_cb).unwrap());
        let cb = || -> Result<_, ReplyOrIdError> { Err(ReplyOrIdError::IdsExhausted) };
        assert!(allocator.generate_id_impl(cb).is_err());
    }

    #[test]
    fn new_range() {
        let reply = generate_get_xid_range_reply(0x13370, 3);
        let mut allocator = IdAllocator::new(0x420, 2).unwrap();
        assert_eq!(0x420, allocator.generate_id_impl(unreachable_cb).unwrap());
        assert_eq!(0x422, allocator.generate_id_impl(unreachable_cb).unwrap());
        // At this point the range is exhausted and a GetXIDRange request is sent
        let cb = || -> Result<_, ReplyOrIdError> { Ok(reply) };
        assert_eq!(0x13370, allocator.generate_id_impl(cb).unwrap());
        assert_eq!(0x13372, allocator.generate_id_impl(unreachable_cb).unwrap());
        assert_eq!(0x13374, allocator.generate_id_impl(unreachable_cb).unwrap());
        // At this point the range is exhausted and a GetXIDRange request is sent
        let cb = || -> Result<_, ReplyOrIdError> { Ok(reply) };
        assert_eq!(0x13370, allocator.generate_id_impl(cb).unwrap());
    }

    #[test]
    fn invalid_arg() {
        let err = IdAllocator::new(1234, 0).unwrap_err();
        if let super::ConnectError::ZeroIDMask = err {
        } else {
            panic!("Wrong error: {:?}", err);
        }
    }

    fn generate_get_xid_range_reply(start_id: u32, count: u32) -> GetXIDRangeReply {
        GetXIDRangeReply {
            sequence: 0,
            length: 0,
            start_id,
            count,
        }
    }
}
