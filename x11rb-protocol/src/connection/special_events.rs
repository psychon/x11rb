//! Contains a management strategy for special events.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;

/// A series of selectors, used to classify events as special.
#[derive(Debug, Default)]
pub(crate) struct SpecialEventClassifier {
    /// The event classes.
    classes: Vec<SpecialEventClass>,
}

/// A class of event.
struct SpecialEventClass {
    /// The number that is used to select the queue.
    queue_selector: u32,
    /// The condition for an event being selected.
    condition: Condition<'static>,
}

impl fmt::Debug for SpecialEventClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SpecialEventClass")
            .field(&self.queue_selector)
            .finish()
    }
}

/// The condition that is used to select events.
struct Condition<'f> {
    callback: Box<dyn FnMut(&[u8]) -> bool + Send + Sync + 'f>,
}

impl SpecialEventClassifier {
    /// Classifies an event, fetching the given EID.
    pub(crate) fn classify(&mut self, event: &[u8]) -> Option<u32> {
        self.classes.iter_mut().find_map(|class| {
            if class.condition.satisfied(event) {
                Some(class.queue_selector)
            } else {
                None
            }
        })
    }

    /// Add a new classification that uses a callback.
    pub(crate) fn add_class(
        &mut self,
        eid: u32,
        callback: Box<dyn FnMut(&[u8]) -> bool + Send + Sync + 'static>,
    ) {
        self.classes.push(SpecialEventClass {
            queue_selector: eid,
            condition: Condition { callback },
        })
    }

    /// Remove a classification based on its EID.
    pub(crate) fn remove_class(&mut self, eid: u32) {
        self.classes.retain(|class| class.queue_selector != eid)
    }
}

impl<'f> Condition<'f> {
    /// Tell if the condition is satisfied by the given event.
    fn satisfied(&mut self, event: &[u8]) -> bool {
        (self.callback)(event)
    }
}

#[cfg(test)]
mod tests {
    use super::SpecialEventClassifier;
    use crate::protocol::xproto::GE_GENERIC_EVENT;
    use alloc::boxed::Box;
    use alloc::vec;

    #[test]
    fn unmatches_arbitrary() {
        let mut class = SpecialEventClassifier::default();
        class.add_class(0x87, Box::new(|event| event[2] == 0x01));

        // shouldn't match this one
        let mut packet = vec![0; 32];
        packet[0] = 0x33;

        assert!(class.classify(&packet).is_none());
    }

    #[test]
    fn matches_callback() {
        let mut class = SpecialEventClassifier::default();
        class.add_class(0x1337, Box::new(|event| event[2] == 0x01));

        // craft a packet for that
        let mut packet = vec![0; 32];
        packet[0] = GE_GENERIC_EVENT | 0x18;
        packet[2] = 0x01;

        assert_eq!(class.classify(&packet), Some(0x1337));
    }
}
