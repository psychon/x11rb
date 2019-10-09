use std::ops::Deref;
use std::slice::from_raw_parts;
use std::mem::forget;
use libc::free;

/// Wrapper around a slice that was allocated in C code.
#[derive(Debug)]
pub struct CSlice {
    slice: &'static [u8]
}

impl CSlice {
    /// Constructs a new `CSlice` from the given parts. `libc::free` will be called on the given
    /// pointer when the slice is dropped.
    ///
    /// # Safety
    ///
    /// The same rules as for `std::slice::from_raw_parts` apply. Additionally, the given pointer
    /// must be safe to free with `libc::free`.
    pub unsafe fn new(ptr: *const u8, len: usize) -> CSlice {
        let slice = from_raw_parts(ptr, len);
        CSlice{ slice }
    }

    /// Convert `self` into a raw part.
    ///
    /// Ownership of the returned pointer is given to the caller. Specifically, `libc::free` will
    /// not be called on it by `CSlice`.
    pub fn into_ptr(self) -> *const u8 {
        let ptr = self.slice.as_ptr();
        forget(self);
        ptr
    }
}

impl Deref for CSlice {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

impl Drop for CSlice {
    fn drop(&mut self) {
        unsafe { free(self.slice.as_ptr() as _) }
    }
}

