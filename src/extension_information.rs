//! Helper for implementing `RequestConnection::extension_information()`.

use std::collections::{hash_map::Entry as HashMapEntry, HashMap};

use crate::connection::RequestConnection;
use crate::errors::{ConnectionError, ReplyError};
use crate::generated::xproto::{ConnectionExt, QueryExtensionReply};

/// Helper for implementing `RequestConnection::extension_information()`.
///
/// This helps with implementing `RequestConnection`. Most likely, you do not need this in your own
/// code, unless you really want to implement your own X11 connection.
#[derive(Debug, Default)]
pub struct ExtensionInformation(HashMap<&'static str, Option<QueryExtensionReply>>);

impl ExtensionInformation {
    /// An implementation of `RequestConnection::extension_information()`.
    ///
    /// The given connection is used for sending a `QueryExtension` request if needed.
    pub fn extension_information<C: RequestConnection>(
        &mut self,
        conn: &C,
        extension_name: &'static str,
    ) -> Result<Option<QueryExtensionReply>, ConnectionError> {
        match self.0.entry(extension_name) {
            // Extension already checked, return the cached value
            HashMapEntry::Occupied(entry) => Ok(*entry.get()),
            // Extension not checked, check now and cache the result
            HashMapEntry::Vacant(entry) => {
                let info = conn
                    .query_extension(extension_name.as_bytes())?
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
                let info = if present == 0 { None } else { Some(info) };
                Ok(*entry.insert(info))
            }
        }
    }
}
