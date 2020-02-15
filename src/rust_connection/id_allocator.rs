/// An allocator for X11 IDs.
///
/// This struct handles the client-side generation of X11 IDs.
#[derive(Debug)]
pub(crate) struct IDAllocator
{
    next_id: u32,
    max_id: u32,
}

impl IDAllocator {
    /// Create a new instance of an ID allocator.
    ///
    /// The arguments should be the `resource_id_base` and `resource_id_mask` values that the X11
    /// server sent in a `Setup` response.
    pub(crate) fn new(id_base: u32, id_mask: u32) -> Self {
        Self {
            next_id: id_base,
            max_id: id_base | id_mask
        }
    }

    /// Generate the next ID.
    pub(crate) fn generate_id(&mut self) -> u32 {
        // FIXME: use the XC_MISC extension to get a new range when the old one is used up
        assert!(self.next_id < self.max_id);
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
