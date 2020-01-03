/// Helper for implementing `RequestConnection::extension_information()`.

use std::sync::Mutex;
use std::collections::HashMap;
use crate::connection::RequestConnection;
use crate::generated::xproto::{ConnectionExt, QueryExtensionReply};

/// Helper for implementing `RequestConnection::extension_information()`.
///
/// This helps with implementing `RequestConnection`. Most likely, you do not need this in your own
/// code, unless you really want to implement your own X11 connection.
#[derive(Debug, Default)]
pub struct ExtensionInformation(Mutex<HashMap<&'static str, Option<QueryExtensionReply>>>);

impl ExtensionInformation {
    /// An implementation of `RequestConnection::extension_information()`.
    ///
    /// The given connection is used for sending a `QueryExtension` request if needed.
    pub fn extension_information<C: RequestConnection>(&self, conn: &C, extension_name: &'static str)
            -> Option<QueryExtensionReply> {
        // If locking the mutex fails, just return None
        self.0.lock().ok()
            .and_then(|mut map| *map.entry(extension_name)
                // Insert the entry if it does not yet exist and get a copy
                .or_insert_with(|| {
                    let info = conn.query_extension(extension_name.as_bytes()).ok();
                    let info = info.and_then(|c| c.reply().ok());
                    if let Some(info) = info {
                        // If the extension is not present, we return None, else we box it
                        if info.present as u8 == 0 {
                            None
                        } else {
                            Some(info)
                        }
                    } else {
                        // There was an error. Pretend the extension is not present.
                        None
                    }
                }))
    }
}
