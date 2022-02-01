use std::collections::hash_map::{Entry as HashMapEntry, HashMap};
use x11rb::x11_utils::{ExtInfoProvider, ExtensionInformation};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExtState {
    Present(ExtensionInformation),
    Missing,
}

#[derive(Debug, Default)]
pub(crate) struct ExtensionManager(HashMap<&'static str, ExtState>);

impl ExtensionManager {
    pub(crate) fn insert(&mut self, extension_name: &'static str, state: ExtState) {
        self.0.insert(extension_name, state);
    }

    pub(crate) fn extension_information(
        &mut self,
        extension_name: &'static str,
    ) -> Option<ExtState> {
        match self.0.entry(extension_name) {
            HashMapEntry::Occupied(info) => Some(*info.get()),
            HashMapEntry::Vacant(entry) => None,
        }
    }
}

impl ExtInfoProvider for ExtensionManager {
    fn get_from_major_opcode(&self, major_opcode: u8) -> Option<(&str, ExtensionInformation)> {
        self.0
            .iter()
            .filter_map(|(name, state)| {
                if let ExtState::Present(info) = state {
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
                if let ExtState::Present(info) = state {
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
                if let ExtState::Present(info) = state {
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
