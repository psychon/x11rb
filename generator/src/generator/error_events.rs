use super::get_ns_name_prefix;
use super::output::Output;

pub(super) fn generate(out: &mut Output, module: &xcbgen::defs::Module) {
    generate_errors(out, module);
    outln!(out, "");
    generate_events(out, module);
    outln!(out, "");
    outln!(
        out,
        "/// Get the response type out of the raw bytes of an X11 error or event."
    );
    outln!(
        out,
        "fn response_type(raw_bytes: &[u8]) -> Result<u8, ParseError> {{"
    );
    out.indented(|out| {
        outln!(out, "raw_bytes.get(0)");
        outln!(out.indent(), ".map(|x| x & 0x7f)");
        outln!(out.indent(), ".ok_or(ParseError::InsufficientData)");
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "/// Get the sequence number out of an X11 packet.");
    outln!(
        out,
        "fn sequence_number(raw_bytes: &[u8]) -> Result<u16, ParseError> {{"
    );
    out.indented(|out| {
        outln!(out, "raw_bytes.get(2..4)");
        outln!(
            out.indent(),
            ".map(|b| u16::from_ne_bytes(b.try_into().unwrap()))"
        );
        outln!(out.indent(), ".ok_or(ParseError::InsufficientData)");
    });
    outln!(out, "}}");
}

fn generate_errors(out: &mut Output, module: &xcbgen::defs::Module) {
    let namespaces = module.sorted_namespaces();

    outln!(out, "/// Enumeration of all possible X11 error kinds.");
    outln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
    outln!(out, "pub enum ErrorKind {{");
    out.indented(|out| {
        outln!(out, "Unknown(u8),");
        for ns in namespaces.iter() {
            let has_feature = super::ext_has_feature(&ns.header);
            let error_defs = sorted_errors(ns);

            for err_name in error_defs.iter().map(|def| def.name()) {
                if has_feature {
                    outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                }
                outln!(out, "{}{},", get_ns_name_prefix(ns), err_name);
            }
        }
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "impl ErrorKind {{");
    out.indented(|out| {
        outln!(out, "pub fn from_wire_error_code(");
        outln!(out.indent(), "error_code: u8,");
        outln!(out.indent(), "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, ") -> Self {{");
        out.indented(|out| {
            outln!(out, "// Check if this is a core protocol error");
            outln!(out, "match error_code {{");
            out.indented(|out| {
                let xproto_ns = module.namespace("xproto").unwrap();
                let error_defs = sorted_errors(&xproto_ns);
                for err_name in error_defs.iter().map(|def| def.name()) {
                    outln!(
                        out,
                        "xproto::{}_ERROR => return Self::{},",
                        super::camel_case_to_upper_snake(err_name),
                        err_name,
                    );
                }
                outln!(out, "_ => {{}}");
            });
            outln!(out, "}}");
            outln!(out, "");
            outln!(out, "// Find the extension that this error could belong to");
            outln!(
                out,
                "let ext_info = ext_info_provider.get_from_error_code(error_code);",
            );
            outln!(out, "match ext_info {{");
            out.indented(|out| {
                for ns in namespaces.iter() {
                    // skip xproto
                    if ns.ext_info.is_none() {
                        continue;
                    }

                    let error_defs = sorted_errors(ns);
                    if error_defs.is_empty() {
                        continue;
                    }
                    let has_feature = super::ext_has_feature(&ns.header);
                    if has_feature {
                        outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out,
                        "Some(({}::X11_EXTENSION_NAME, ext_info)) => {{",
                        ns.header
                    );
                    out.indented(|out| {
                        outln!(out, "match error_code - ext_info.first_error {{");
                        for err_name in error_defs.iter().map(|def| def.name()) {
                            outln!(
                                out.indent(),
                                "{}::{}_ERROR => Self::{}{},",
                                ns.header,
                                super::camel_case_to_upper_snake(err_name),
                                get_ns_name_prefix(ns),
                                err_name,
                            );
                        }
                        outln!(out.indent(), "_ => Self::Unknown(error_code),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => Self::Unknown(error_code),");
            });
            outln!(out, "}}")
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
}

fn generate_events(out: &mut Output, module: &xcbgen::defs::Module) {
    let namespaces = module.sorted_namespaces();

    outln!(out, "/// Enumeration of all possible X11 events.");
    outln!(out, "#[derive(Debug, Clone)]");
    outln!(out, "pub enum Event {{");
    out.indented(|out| {
        outln!(out, "Unknown(Vec<u8>),");
        outln!(out, "Error(X11Error),");

        for ns in namespaces.iter() {
            let has_feature = super::ext_has_feature(&ns.header);
            let event_defs = sorted_events(ns);
            for event_name in event_defs.iter().map(|def| def.name()) {
                if has_feature {
                    outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                }
                outln!(
                    out,
                    "{}{}({}::{}Event),",
                    get_ns_name_prefix(ns),
                    event_name,
                    ns.header,
                    event_name,
                );
            }
        }
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "impl Event {{");
    out.indented(|out| {
        outln!(
            out,
            "/// Parse a generic X11 event into a concrete event type."
        );
        outln!(
            out,
            "#[allow(clippy::cognitive_complexity, clippy::match_single_binding)]",
        );
        outln!(out, "pub fn parse(");
        outln!(out.indent(), "event: &[u8],");
        outln!(out.indent(), "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, ") -> Result<Self, ParseError> {{");
        out.indented(|out| {
            outln!(out, "let event_code = response_type(event)?;");
            outln!(out, "");
            outln!(
                out,
                "// Check if this is a core protocol event {}",
                "or error, or from the generic event extension",
            );
            outln!(out, "match event_code {{");
            out.indented(|out| {
                outln!(
                    out,
                    "0 => return Ok({}),",
                    "Self::Error(X11Error::try_parse(event, ext_info_provider)?)",
                );
                let xproto_ns = module.namespace("xproto").unwrap();
                let event_defs = sorted_events(&xproto_ns);
                for event_name in event_defs.iter().map(|def| def.name()) {
                    if event_name == "GeGeneric" {
                        // This does not really count and is parsed as an extension's event
                        continue;
                    }
                    outln!(
                        out,
                        "xproto::{}_EVENT => return Ok(Self::{}(event.try_into()?)),",
                        super::camel_case_to_upper_snake(event_name),
                        event_name,
                    );
                }
                outln!(
                    out,
                    "xproto::GE_GENERIC_EVENT => {},",
                    "return Self::from_generic_event(event, ext_info_provider)",
                );
                outln!(out, "_ => {{}}");
            });
            outln!(out, "}}");

            outln!(out, "// Find the extension that this event could belong to");
            outln!(
                out,
                "let ext_info = ext_info_provider.get_from_event_code(event_code);"
            );
            outln!(out, "match ext_info {{");
            out.indented(|out| {
                for ns in namespaces.iter() {
                    // skip xproto
                    if ns.ext_info.is_none() {
                        continue;
                    }
                    let event_defs = sorted_events(ns);
                    if event_defs.iter().all(|event_def| event_def.is_xge()) {
                        continue;
                    }

                    let has_feature = super::ext_has_feature(&ns.header);
                    if has_feature {
                        outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out,
                        "Some(({}::X11_EXTENSION_NAME, ext_info)) => {{",
                        ns.header
                    );
                    out.indented(|out| {
                        if ns.header == "xkb" {
                            outln!(out, "if event_code != ext_info.first_event {{");
                            outln!(out.indent(), "return Ok(Self::Unknown(event.to_vec()));");
                            outln!(out, "}}");
                            outln!(
                                out,
                                "match *event.get(1).ok_or(ParseError::InsufficientData)? {{"
                            );
                        } else {
                            outln!(out, "match event_code - ext_info.first_event {{");
                        }
                        for event_def in event_defs.iter() {
                            if event_def.is_xge() {
                                continue;
                            }
                            outln!(
                                out.indent(),
                                "{}::{}_EVENT => Ok(Self::{}{}(event.try_into()?)),",
                                ns.header,
                                super::camel_case_to_upper_snake(event_def.name()),
                                get_ns_name_prefix(ns),
                                event_def.name(),
                            );
                        }
                        outln!(out.indent(), "_ => Ok(Self::Unknown(event.to_vec())),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => Ok(Self::Unknown(event.to_vec())),");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");

        outln!(out, "#[allow(clippy::match_single_binding)]");
        outln!(out, "fn from_generic_event(");
        outln!(out.indent(), "event: &[u8],");
        outln!(out.indent(), "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, ") -> Result<Self, ParseError> {{");
        out.indented(|out| {
            outln!(
                out,
                "let ge_event = xproto::GeGenericEvent::try_from(event)?;"
            );
            outln!(out, "let ext_name = ext_info_provider");
            outln!(out.indent(), ".get_from_major_opcode(ge_event.extension)");
            outln!(out.indent(), ".map(|(name, _)| name);");
            outln!(out, "match ext_name {{");
            out.indented(|out| {
                for ns in namespaces.iter() {
                    // skip xproto
                    if ns.ext_info.is_none() {
                        continue;
                    }
                    let event_defs = sorted_events(ns);
                    if event_defs.iter().all(|event_def| !event_def.is_xge()) {
                        continue;
                    }

                    let has_feature = super::ext_has_feature(&ns.header);
                    if has_feature {
                        outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(out, "Some({}::X11_EXTENSION_NAME) => {{", ns.header);
                    out.indented(|out| {
                        outln!(out, "match ge_event.event_type {{");
                        for event_def in event_defs.iter() {
                            if !event_def.is_xge() {
                                continue;
                            }
                            outln!(
                                out.indent(),
                                "{}::{}_EVENT => Ok(Self::{}{}(event.try_into()?)),",
                                ns.header,
                                super::camel_case_to_upper_snake(event_def.name()),
                                get_ns_name_prefix(ns),
                                event_def.name(),
                            );
                        }
                        outln!(out.indent(), "_ => Ok(Self::Unknown(event.to_vec())),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => Ok(Self::Unknown(event.to_vec())),");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");

        outln!(
            out,
            "/// Get the sequence number contained in this X11 event",
        );
        outln!(out, "pub fn wire_sequence_number(&self) -> Option<u16> {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            outln!(
                out.indent(),
                "Event::Unknown(value) => sequence_number(value).ok(),",
            );
            outln!(out.indent(), "Event::Error(value) => Some(value.sequence),");
            for ns in namespaces.iter() {
                let event_defs = sorted_events(ns);
                let has_feature = super::ext_has_feature(&ns.header);
                for event_def in event_defs.iter() {
                    if has_feature {
                        outln!(out.indent(), "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    if event_def.get_original_full_def().no_sequence_number {
                        outln!(
                            out.indent(),
                            "Event::{}{}(_) => None,",
                            get_ns_name_prefix(ns),
                            event_def.name(),
                        );
                    } else {
                        outln!(
                            out.indent(),
                            "Event::{}{}(value) => Some(value.sequence),",
                            get_ns_name_prefix(ns),
                            event_def.name(),
                        );
                    }
                }
            }
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");

        outln!(out, "/// Get the raw response type of this X11 event");
        outln!(out, "///");
        outln!(
            out,
            "/// Response types have seven bits in X11. The eight bit indicates whether",
        );
        outln!(
            out,
            "/// the packet was generated through the `SendEvent` request. This function",
        );
        outln!(out, "/// returns all eight bits.");
        outln!(out, "///");
        outln!(
            out,
            "/// See also the `response_type()`, `server_generated()` and `sent_event()` methods.",
        );
        outln!(out, "pub fn raw_response_type(&self) -> u8 {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            outln!(
                out.indent(),
                "Event::Unknown(value) => response_type(value).unwrap(),",
            );
            outln!(out.indent(), "Event::Error(_) => 0,");
            for ns in namespaces.iter() {
                let event_defs = sorted_events(ns);
                let has_feature = super::ext_has_feature(&ns.header);
                for event_def in event_defs.iter() {
                    if has_feature {
                        outln!(out.indent(), "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out.indent(),
                        "Event::{}{}(value) => value.response_type,",
                        get_ns_name_prefix(ns),
                        event_def.name(),
                    );
                }
            }
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");

        outln!(
            out,
            r"/// Get the response type of this X11 event
pub fn response_type(&self) -> u8 {{
    self.raw_response_type() & 0x7f
}}

/// Was this event generated by the X11 server?
///
/// If this function returns true, then this event comes from the X11 server.
/// Otherwise, it was sent from another client via the `SendEvent` request.
pub fn server_generated(&self) -> bool {{
    self.raw_response_type() & 0x80 == 0
}}

/// Was this event generated by another X11 client?
///
/// If this function returns true, then this event comes from another client via
/// the `SendEvent` request. Otherwise, it was generated by the X11 server.
pub fn sent_event(&self) -> bool {{
    self.raw_response_type() & 0x80 != 0
}}"
        );
    });
    outln!(out, "}}");
}

fn sorted_errors(ns: &xcbgen::defs::Namespace) -> Vec<xcbgen::defs::ErrorDef> {
    let mut errors: Vec<_> = ns
        .error_defs
        .borrow()
        .values()
        .filter(|error_def| {
            let (ns, name) = match error_def {
                xcbgen::defs::ErrorDef::Full(error_full_def) => (
                    error_full_def.namespace.upgrade().unwrap(),
                    &error_full_def.name,
                ),
                xcbgen::defs::ErrorDef::Copy(error_copy_def) => (
                    error_copy_def.namespace.upgrade().unwrap(),
                    &error_copy_def.name,
                ),
            };
            // The XML for GLX has a comment saying "fake number"
            ns.header != "glx" || name != "Generic"
        })
        .cloned()
        .collect();
    errors.sort_by(|a, b| a.name().cmp(b.name()));
    errors
}

fn sorted_events(ns: &xcbgen::defs::Namespace) -> Vec<xcbgen::defs::EventDef> {
    let mut events: Vec<_> = ns.event_defs.borrow().values().cloned().collect();
    events.sort_by(|a, b| a.name().cmp(b.name()));
    events
}
