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
    match e {
        nix::Error::Sys(errno) => errno.into(),
        nix::Error::InvalidPath | nix::Error::InvalidUtf8 => {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
        }
        nix::Error::UnsupportedOperation => std::io::Error::new(std::io::ErrorKind::Other, e),
    }
}

#[cfg(unix)]
mod raw_fd_container {
    use std::mem::forget;
    #[cfg(unix)]
    use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};

    use super::nix_error_to_io;

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
            Ok(Self::new(
                nix::unistd::dup(self.0).map_err(nix_error_to_io)?,
            ))
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
            nix::unistd::close(fd).map_err(nix_error_to_io)
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
    #[allow(missing_copy_implementations)]
    #[derive(Debug, Hash, PartialEq, Eq)]
    pub struct RawFdContainer(());
}

pub use raw_fd_container::RawFdContainer;
