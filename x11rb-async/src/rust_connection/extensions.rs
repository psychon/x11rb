//! Extension state for the connection

use crate::connection::Connection;
use crate::Cookie;
use std::collections::hash_map::{Entry, HashMap};
use x11rb::errors::{ConnectionError, ReplyError};
use x11rb_protocol::protocol::xproto::QueryExtensionReply;
use x11rb_protocol::x11_utils::{ExtInfoProvider, ExtensionInformation};
use x11rb_protocol::SequenceNumber;

/// Cache for X11 extensions supported by the server
#[derive(Debug, Default)]
pub(super) struct Extensions(HashMap<&'static str, ExtensionState>);

#[derive(Debug)]
enum ExtensionState {
    /// Currently loading the extension.
    Loading(SequenceNumber),

    /// The extension is loaded.
    Loaded(Option<ExtensionInformation>),
}

impl Extensions {
    fn iter(&self) -> impl Iterator<Item = (&str, ExtensionInformation)> {
        self.0.iter().filter_map(|(name, state)| match state {
            ExtensionState::Loaded(ref ext) => ext.map(|ext| (*name, ext)),
            _ => None,
        })
    }

    /// Prefetch information for a given extension, if this was not yet done.
    pub(super) async fn prefetch<C: Connection>(
        &mut self,
        conn: &C,
        name: &'static str,
    ) -> Result<(), ConnectionError> {
        // Check if there is already a cache entry.
        if let Entry::Vacant(entry) = self.0.entry(name) {
            // Send a QueryExtension request.
            let cookie = crate::protocol::xproto::query_extension(conn, name.as_bytes()).await?;

            // Add the extension to the cache.
            entry.insert(ExtensionState::Loading(cookie.sequence_number()));

            std::mem::forget(cookie);
        }

        Ok(())
    }

    /// Get information about an X11 extension.
    pub(super) async fn information<C: Connection>(
        &mut self,
        conn: &C,
        name: &'static str,
    ) -> Result<Option<ExtensionInformation>, ConnectionError> {
        // Prefetch the implementation in case this was not yet done
        self.prefetch(conn, name).await?;

        // Get the entry from the cache
        let mut entry = match self.0.entry(name) {
            Entry::Occupied(o) => o,
            _ => unreachable!("We just prefetched this."),
        };

        // Complete the request if we need to.
        match entry.get() {
            ExtensionState::Loaded(info) => Ok(*info),

            ExtensionState::Loading(cookie) => {
                // Load the extension information.
                let cookie = Cookie::<'_, _, QueryExtensionReply>::new(conn, *cookie);

                // Get the reply.
                let reply = cookie.reply().await.map_err(|e| match e {
                    ReplyError::ConnectionError(e) => e,
                    ReplyError::X11Error(_) => ConnectionError::UnknownError,
                })?;

                let ext_info = if reply.present {
                    Some(ExtensionInformation {
                        major_opcode: reply.major_opcode,
                        first_event: reply.first_event,
                        first_error: reply.first_error,
                    })
                } else {
                    None
                };

                // Update the cache.
                *entry.get_mut() = ExtensionState::Loaded(ext_info);

                Ok(ext_info)
            }
        }
    }
}

impl ExtInfoProvider for Extensions {
    fn get_from_major_opcode(&self, major_opcode: u8) -> Option<(&str, ExtensionInformation)> {
        self.iter()
            .find(|(_, info)| info.major_opcode == major_opcode)
    }

    fn get_from_event_code(&self, event_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.iter()
            .filter(|(_, info)| info.first_event <= event_code)
            .max_by_key(|(_, info)| info.first_event)
    }

    fn get_from_error_code(&self, error_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.iter()
            .filter(|(_, info)| info.first_error <= error_code)
            .max_by_key(|(_, info)| info.first_error)
    }
}
