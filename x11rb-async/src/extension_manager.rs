use std::collections::{hash_map::Entry as HashMapEntry, HashMap};

use x11rb_protocol::protocol::xproto::QueryExtensionReply;
use x11rb_protocol::x11_utils::{ExtInfoProvider, ExtensionInformation};

#[derive(Debug, Default)]
pub(crate) struct ExtensionManager(HashMap<&'static str, CheckState>);

#[derive(Debug)]
enum CheckState {
    Present(ExtensionInformation),
    Missing,
}

#[derive(Debug)]
pub(crate) enum ExtensionResult {
    Present(ExtensionInformation),
    Missing,
    NotYetChecked,
}

impl ExtensionManager {
    // TODO: Proper error return type
    pub(crate) fn extension_information(
        &mut self,
        extension_name: &'static str,
    ) -> ExtensionResult {
        match self.0.entry(extension_name) {
            HashMapEntry::Vacant(_) => ExtensionResult::NotYetChecked,
            HashMapEntry::Occupied(entry) => match entry.get() {
                CheckState::Present(info) => ExtensionResult::Present(*info),
                CheckState::Missing => ExtensionResult::Missing,
            },
        }
    }

    pub(crate) fn insert_if_unknown(
        &mut self,
        extension_name: &'static str,
        reply: &QueryExtensionReply,
    ) {
        self.0.entry(extension_name).or_insert_with(move || {
            if reply.present {
                // TODO: Add impl TryFrom<QueryExtensionReply> for ExtensionInformation to
                // x11rb-protocol
                let info = ExtensionInformation {
                    major_opcode: reply.major_opcode,
                    first_event: reply.first_event,
                    first_error: reply.first_error,
                };
                CheckState::Present(info)
            } else {
                CheckState::Missing
            }
        });
    }
}

impl ExtInfoProvider for ExtensionManager {
    fn get_from_major_opcode(&self, major_opcode: u8) -> Option<(&str, ExtensionInformation)> {
        self.0
            .iter()
            .filter_map(|(name, state)| {
                if let CheckState::Present(info) = state {
                    Some((*name, *info))
                } else {
                    None
                }
            })
            .find(|(_, info)| info.major_opcode == major_opcode)
    }

    fn get_from_event_code(&self, event_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.0
            .iter()
            .filter_map(|(name, state)| {
                if let CheckState::Present(info) = state {
                    if info.first_event <= event_code {
                        Some((*name, *info))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .max_by_key(|(_, info)| info.first_event)
    }

    fn get_from_error_code(&self, error_code: u8) -> Option<(&str, ExtensionInformation)> {
        self.0
            .iter()
            .filter_map(|(name, state)| {
                if let CheckState::Present(info) = state {
                    if info.first_error <= error_code {
                        Some((*name, *info))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .max_by_key(|(_, info)| info.first_error)
    }
}
