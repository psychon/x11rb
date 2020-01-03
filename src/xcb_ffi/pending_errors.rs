//! Management of pending errors for the XCB connection.
//!
//! We add our own error tracking ontop of what XCB already provides. For this reason, this module
//! contains a struct for tracking requests that were send and also tracking errors that were
//! received, but not yet given to the user of this library.

use std::cmp::Reverse;
use std::sync::Mutex;
use std::convert::TryInto;
use std::collections::{BinaryHeap, VecDeque};

use super::XCBConnection;
use crate::connection::SequenceNumber;
use crate::x11_utils::GenericError;

#[derive(Debug, Default)]
struct PendingErrorsInner {
    in_flight: BinaryHeap<Reverse<SequenceNumber>>,
    pending: VecDeque<GenericError>,
}

/// A management struct for pending X11 errors
#[derive(Debug, Default)]
pub(crate) struct PendingErrors {
    inner: Mutex<PendingErrorsInner>
}

impl PendingErrors {
    pub(crate) fn append_error(&self, error: GenericError) {
        self.inner.lock().unwrap().pending.push_back(error)
    }

    pub(crate) fn discard_reply(&self, sequence: SequenceNumber) {
        self.inner.lock().unwrap().in_flight.push(Reverse(sequence));
    }

    pub(crate) fn get(&self, conn: &XCBConnection) -> Option<GenericError> {
        let mut inner = self.inner.lock().unwrap();

        // Check if we already have an element at hand
        let err = inner.pending.pop_front();
        if err.is_some() {
            return err;
        }

        // Check if any of the still in-flight responses got a reply/error
        while let Some(&Reverse(seqno)) = inner.in_flight.peek() {
            let result = match conn.poll_for_reply(seqno) {
                Err(()) => {
                    // This request was not answered/errored yet, so later request will not
                    // have answers as well.
                    return None;
                },
                Ok(reply) => reply
            };

            let seqno2 = inner.in_flight.pop();
            assert_eq!(Some(Reverse(seqno)), seqno2);

            if let Some(result) = result {
                // Is this an error?
                if let Ok(error) = result.try_into() {
                    return Some(error);
                } else {
                    // It's a reply, just ignore it
                }
            }
        }

        None
    }
}
