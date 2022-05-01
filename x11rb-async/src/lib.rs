/// Reexports of dependencies
pub mod reexports {
    pub use x11rb_protocol;
}

mod condvar;
mod connection;
pub(crate) mod extension_manager;
pub mod tokio;

use condvar::Condvar;

pub use connection::Connection;
