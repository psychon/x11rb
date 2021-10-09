//! Utility functions that are not specific to X11.
//!
//!
//! # CSlice
//!
//! [`CSlice`] is a wrapper around some bytes in memory. It is unsafe to construct, but takes
//! ownership of the bytes and allows accessing them as a `[u8]`. When dropped, the underlying
//! memory is freed via [`libc::free`].
//!
//! `CSlice` is only available when the `allow-unsafe-code` feature is enabled.
//!
//!
//! # RawFdContainer
//!
//! [`RawFdContainer`] is a variant of [`std::os::unix::io::RawFd`] with ownership semantics. This
//! means that the `RawFd` will be closed when the `RawFdContainer` is dropped.
//!
//! On non-`cfg(unix)`-systems, this is an empty type without methods. It still exists as a type so
//! that it can appear in interfaces, but it is not actually possible to construct an instance of
//! `RawFdContainer`.

#[cfg(feature = "allow-unsafe-code")]
mod unsafe_code {
    use std::mem::forget;
    use std::ops::{Deref, Index};
    use std::ptr::NonNull;
    use std::slice::{from_raw_parts, SliceIndex};

    use libc::free;

    /// Wrapper around a slice that was allocated in C code.
    ///
    /// `CSlice` is only available when the `allow-unsafe-code` feature is enabled.
    pub struct CSlice {
        ptr: NonNull<[u8]>,
    }

    // `CSlice` is `Send` and `Sync` because, once created, it is
    // completely immutable (it does not have interior mutability),
    // so it can be safely accessed from different threads simultaneously.
    unsafe impl Send for CSlice {}
    unsafe impl Sync for CSlice {}

    impl CSlice {
        /// Constructs a new `CSlice` from the given parts. `libc::free` will be called on the given
        /// pointer when the slice is dropped.
        ///
        /// # Safety
        ///
        /// The same rules as for `std::slice::from_raw_parts` apply. Additionally, the given pointer
        /// must be safe to free with `libc::free`.
        pub unsafe fn new(ptr: *const u8, len: usize) -> CSlice {
            CSlice {
                ptr: NonNull::from(from_raw_parts(ptr, len)),
            }
        }

        /// Convert `self` into a raw part.
        ///
        /// Ownership of the returned pointer is given to the caller. Specifically, `libc::free` will
        /// not be called on it by `CSlice`.
        pub fn into_ptr(self) -> *const u8 {
            let ptr = self.ptr.as_ptr() as *const u8;
            forget(self);
            ptr
        }
    }

    impl Drop for CSlice {
        fn drop(&mut self) {
            unsafe { free(self.ptr.as_ptr() as _) }
        }
    }

    impl Deref for CSlice {
        type Target = [u8];

        fn deref(&self) -> &[u8] {
            unsafe { self.ptr.as_ref() }
        }
    }

    impl AsRef<[u8]> for CSlice {
        fn as_ref(&self) -> &[u8] {
            &**self
        }
    }

    impl<I> Index<I> for CSlice
    where
        I: SliceIndex<[u8]>,
    {
        type Output = I::Output;

        fn index(&self, index: I) -> &I::Output {
            (**self).index(index)
        }
    }

    impl std::fmt::Debug for CSlice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Debug::fmt(&**self, f)
        }
    }
}

#[cfg(feature = "allow-unsafe-code")]
pub use unsafe_code::CSlice;

#[cfg(unix)]
pub(crate) fn nix_error_to_io(e: nix::Error) -> std::io::Error {
    std::io::Error::from_raw_os_error(e as i32)
}

#[cfg(unix)]
mod raw_fd_container {
    use std::mem::forget;
    #[cfg(unix)]
    use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};

    /// A simple wrapper around RawFd that closes the fd on drop.
    ///
    /// On non-unix systems, this type is empty and does not provide
    /// any method.
    #[derive(Debug, Hash, PartialEq, Eq)]
    pub struct RawFdContainer(RawFd);

    impl Drop for RawFdContainer {
        fn drop(&mut self) {
            let _ = nix::unistd::close(self.0);
        }
    }

    impl RawFdContainer {
        /// Create a new `RawFdContainer` for the given `RawFd`.
        ///
        /// The `RawFdContainer` takes ownership of the `RawFd` and closes it on drop.
        pub fn new(fd: RawFd) -> Self {
            RawFdContainer(fd)
        }

        /// Tries to clone the `RawFdContainer` creating a new FD
        /// with `dup`. The new `RawFdContainer` will take ownership
        /// of the `dup`ed version, whereas the original `RawFdContainer`
        /// will keep the ownership of its FD.
        pub fn try_clone(&self) -> Result<Self, std::io::Error> {
            Ok(Self::new(nix::unistd::dup(self.0)?))
        }

        /// Get the `RawFd` out of this `RawFdContainer`.
        ///
        /// This function would be an implementation of `IntoRawFd` if that were possible. However, it
        /// causes a conflict with an `impl` from libcore...
        pub fn into_raw_fd(self) -> RawFd {
            let fd = self.0;
            forget(self);
            fd
        }

        /// Consumes the `RawFdContainer` and closes the wrapped FD with
        /// the `close` system call.
        ///
        /// This is similar to dropping the `RawFdContainer`, but it allows
        /// the caller to handle errors.
        pub fn close(self) -> Result<(), std::io::Error> {
            let fd = self.into_raw_fd();
            nix::unistd::close(fd).map_err(|e| e.into())
        }
    }

    impl<T: IntoRawFd> From<T> for RawFdContainer {
        fn from(fd: T) -> Self {
            Self::new(fd.into_raw_fd())
        }
    }

    impl AsRawFd for RawFdContainer {
        fn as_raw_fd(&self) -> RawFd {
            self.0
        }
    }
}

#[cfg(not(unix))]
mod raw_fd_container {
    /// A simple wrapper around RawFd that closes the fd on drop.
    ///
    /// On non-unix systems, this type is empty and does not provide
    /// any method.
    #[derive(Debug, Hash, PartialEq, Eq)]
    pub struct RawFdContainer(());

    impl Drop for RawFdContainer {
        fn drop(&mut self) {
            // This function exists for symmetry with cfg(unix)
        }
    }
}

mod pretty_printer {
    use std::fmt::{Debug, Formatter, Result};

    /// A helper to pretty-print an enumeration value.
    ///
    /// This function prints the given number. If it matches one of the provided variants, that
    /// match is used. Otherwise, the number is printed as a decimal.
    ///
    /// In alternate mode, the second string in the given array is used, else the first.
    pub(crate) fn pretty_print_enum(
        fmt: &mut Formatter<'_>,
        value: u32,
        cases: &[(u32, &str, &str)],
    ) -> Result {
        for (variant, name1, name2) in cases {
            if &value == variant {
                if fmt.alternate() {
                    return fmt.write_str(name2);
                } else {
                    return fmt.write_str(name1);
                }
            }
        }
        Debug::fmt(&value, fmt)
    }

    /// A helper to pretty-print a bitmask.
    ///
    /// This function prints the given number. All bit-matches with the given variants are printed.
    /// Any left-over number is printed as a decimal.
    ///
    /// In alternate mode, the second string in the given array is used, else the first.
    pub(crate) fn pretty_print_bitmask(
        fmt: &mut Formatter<'_>,
        value: u32,
        cases: &[(u32, &str, &str)],
    ) -> Result {
        // First, figure out if there are any bits not covered by any case
        let known_bits = cases.iter().fold(0, |acc, (value, _, _)| acc | value);
        let remaining = value & !known_bits;
        let mut already_printed = if value == 0 || remaining != 0 {
            Debug::fmt(&remaining, fmt)?;
            true
        } else {
            false
        };
        for (variant, name1, name2) in cases {
            if variant & value != 0 {
                if already_printed {
                    fmt.write_str(" | ")?;
                }
                already_printed = true;
                if fmt.alternate() {
                    fmt.write_str(name2)?;
                } else {
                    fmt.write_str(name1)?;
                }
            }
        }
        Ok(())
    }

    #[cfg(test)]
    mod test {
        use super::{pretty_print_bitmask, pretty_print_enum};
        use std::fmt::{Display, Formatter, Result};

        type CallbackType = fn(&mut Formatter<'_>, u32, &[(u32, &str, &str)]) -> Result;

        struct CallbackFormating<'a, 'b> {
            callback: CallbackType,
            value: u32,
            cases: &'a [(u32, &'b str, &'b str)],
        }

        fn new_enum<'a, 'b>(
            value: u32,
            cases: &'a [(u32, &'b str, &'b str)],
        ) -> CallbackFormating<'a, 'b> {
            CallbackFormating {
                callback: pretty_print_enum,
                value,
                cases,
            }
        }

        fn new_bitmask<'a, 'b>(
            value: u32,
            cases: &'a [(u32, &'b str, &'b str)],
        ) -> CallbackFormating<'a, 'b> {
            CallbackFormating {
                callback: pretty_print_bitmask,
                value,
                cases,
            }
        }

        impl Display for CallbackFormating<'_, '_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                (self.callback)(f, self.value, self.cases)
            }
        }

        #[test]
        fn test_enum() {
            let cases = [(0, "zero", "ZERO"), (42, "the answer", "ANSWER")];
            let printer = new_enum(0, &cases);
            assert_eq!(&format!("{}", printer), "zero");
            assert_eq!(&format!("{:#}", printer), "ZERO");
            let printer = new_enum(1, &cases);
            assert_eq!(&format!("{}", printer), "1");
            assert_eq!(&format!("{:#}", printer), "1");
            let printer = new_enum(42, &cases);
            assert_eq!(&format!("{}", printer), "the answer");
            assert_eq!(&format!("{:#}", printer), "ANSWER");
        }

        #[test]
        fn test_bitmask() {
            let bits = [
                (1 << 5, "b5", "B5"),
                (1 << 1, "b1", "B1"),
                (1 << 0, "unused", "UNUSED"),
            ];
            let printer = new_bitmask(8, &bits);
            assert_eq!(&format!("{}", printer), "8");
            assert_eq!(&format!("{:#}", printer), "8");
            let printer = new_bitmask(32, &bits);
            assert_eq!(&format!("{}", printer), "b5");
            assert_eq!(&format!("{:#}", printer), "B5");
            let printer = new_bitmask(34, &bits);
            assert_eq!(&format!("{}", printer), "b5 | b1");
            assert_eq!(&format!("{:#}", printer), "B5 | B1");
            let printer = new_bitmask(42, &bits);
            assert_eq!(&format!("{}", printer), "8 | b5 | b1");
            assert_eq!(&format!("{:#}", printer), "8 | B5 | B1");
        }
    }
}

pub(crate) use pretty_printer::{pretty_print_bitmask, pretty_print_enum};
pub use raw_fd_container::RawFdContainer;
