//! Helper for implementing `RequestConnection::extension_information()`.

use std::collections::{hash_map::Entry as HashMapEntry, HashMap};

use crate::connection::{RequestConnection, SequenceNumber};
use crate::cookie::Cookie;
use crate::errors::{ConnectionError, ReplyError};
use crate::generated::xproto::{ConnectionExt, QueryExtensionReply};

/// Helper for implementing `RequestConnection::extension_information()`.
///
/// This helps with implementing `RequestConnection`. Most likely, you do not need this in your own
/// code, unless you really want to implement your own X11 connection.
#[derive(Debug, Default)]
pub struct ExtensionInformation(HashMap<&'static str, CheckState>);

#[derive(Debug)]
enum CheckState {
    Prefetched(SequenceNumber),
    Present(QueryExtensionReply),
    Missing,
}

impl ExtensionInformation {
    /// If the extension has not prefetched yet, sends a `QueryExtension`
    /// requests, adds a field to the hash map and returns a reference to it.
    fn prefetch_extension_information_aux<C: RequestConnection>(
        &mut self,
        conn: &C,
        extension_name: &'static str,
    ) -> Result<&mut CheckState, ConnectionError> {
        match self.0.entry(extension_name) {
            // Extension already checked, return the cached value
            HashMapEntry::Occupied(entry) => Ok(entry.into_mut()),
            HashMapEntry::Vacant(entry) => {
                let cookie = conn.query_extension(extension_name.as_bytes())?;
                Ok(entry.insert(CheckState::Prefetched(cookie.into_sequence_number())))
            }
        }
    }

    /// Prefetchs an extension sending a `QueryExtension` without waiting for
    /// the reply.
    pub fn prefetch_extension_information<C: RequestConnection>(
        &mut self,
        conn: &C,
        extension_name: &'static str,
    ) -> Result<(), ConnectionError> {
        // We are not interested on the reference to the entry.
        let _ = self.prefetch_extension_information_aux(conn, extension_name)?;
        Ok(())
    }

    /// An implementation of `RequestConnection::extension_information()`.
    ///
    /// The given connection is used for sending a `QueryExtension` request if needed.
    pub fn extension_information<C: RequestConnection>(
        &mut self,
        conn: &C,
        extension_name: &'static str,
    ) -> Result<Option<QueryExtensionReply>, ConnectionError> {
        let entry = self.prefetch_extension_information_aux(conn, extension_name)?;
        match entry {
            CheckState::Prefetched(sequence_number) => {
                let info = Cookie::<C, QueryExtensionReply>::new(conn, *sequence_number)
                    .reply()
                    .map_err(|e| match e {
                        ReplyError::ConnectionError(e) => e,
                        // The X11 protocol specification does not specify any error
                        // for the QueryExtension request, so this should not happen.
                        ReplyError::X11Error(_) => ConnectionError::UnknownError,
                    })?;
                // If the extension is not present, we return None, else we box it
                #[allow(trivial_numeric_casts)]
                let present = info.present as u8;
                if present != 0 {
                    *entry = CheckState::Present(info);
                    Ok(Some(info))
                } else {
                    *entry = CheckState::Missing;
                    Ok(None)
                }
            }
            CheckState::Present(info) => Ok(Some(*info)),
            CheckState::Missing => Ok(None),
        }
    }
}
