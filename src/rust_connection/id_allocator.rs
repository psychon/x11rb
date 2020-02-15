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
pub(crate) struct IDAllocator
{
    next_id: u32,
    max_id: u32,
    increment: u32,
}

impl IDAllocator {
    /// Create a new instance of an ID allocator.
    ///
    /// The arguments should be the `resource_id_base` and `resource_id_mask` values that the X11
    /// server sent in a `Setup` response.
    pub(crate) fn new(id_base: u32, id_mask: u32) -> Self {
        assert_ne!(0, id_mask);
        // Find the right-most set bit in id_mask, e.g. for 0b110, this results in 0b010.
        let increment = id_mask & (1 + !id_mask);
        Self {
            next_id: id_base,
            max_id: id_base | id_mask,
            increment,
        }
    }

    /// Generate the next ID.
    pub(crate) fn generate_id(&mut self) -> Option<u32> {
        // FIXME: use the XC_MISC extension to get a new range when the old one is used up
        if self.next_id <= self.max_id {
            let id = self.next_id;
            self.next_id += self.increment;
            Some(id)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::IDAllocator;

    #[test]
    fn exhaustive() {
        let mut allocator = IDAllocator::new(0x2800, 0x1ff);
        for expected in 0x2800..=0x29ff {
            assert_eq!(Some(expected), allocator.generate_id());
        }
        assert_eq!(None, allocator.generate_id());
    }

    #[test]
    fn increment() {
        let mut allocator = IDAllocator::new(0, 0b1100);
        assert_eq!(Some(0b0000), allocator.generate_id());
        assert_eq!(Some(0b0100), allocator.generate_id());
        assert_eq!(Some(0b1000), allocator.generate_id());
        assert_eq!(Some(0b1100), allocator.generate_id());
        assert_eq!(None, allocator.generate_id());
    }
}
