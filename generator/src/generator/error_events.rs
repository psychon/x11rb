use super::output::Output;

pub(super) fn generate(out: &mut Output, module: &xcbgen::defs::Module) {
    generate_errors(out, module);
    outln!(out, "");
    generate_events(out, module);
}

fn generate_errors(out: &mut Output, module: &xcbgen::defs::Module) {
    let namespaces = module.sorted_namespaces();

    outln!(out, "/// Enumeration of all possible X11 errors.");
    outln!(out, "#[derive(Debug, Clone)]");
    outln!(out, "pub enum Error<B: std::fmt::Debug + AsRef<[u8]>> {{");
    out.indented(|out| {
        outln!(out, "Unknown(GenericError<B>),");

        for ns in namespaces.iter() {
            let has_feature = super::ext_has_feature(&ns.header);
            let error_defs = sorted_errors(ns);

            for err_name in error_defs.iter().map(|def| def.name()) {
                if has_feature {
                    outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                }
                outln!(
                    out,
                    "{}{}({}::{}Error),",
                    get_ns_name_prefix(ns),
                    err_name,
                    ns.header,
                    err_name,
                );
            }
        }
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "impl<B: std::fmt::Debug + AsRef<[u8]>> Error<B> {{");
    out.indented(|out| {
        outln!(
            out,
            "/// Parse a generic X11 error into a concrete error type."
        );
        outln!(out, "pub fn parse(");
        outln!(out.indent(), "error: GenericError<B>,");
        outln!(out.indent(), "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, ") -> Result<Self, ParseError> {{");
        out.indented(|out| {
            outln!(out, "let error_code = error.error_code();");
            outln!(out, "");
            outln!(out, "// Check if this is a core protocol error");
            outln!(out, "match error_code {{");
            out.indented(|out| {
                let xproto_ns = module.namespace("xproto").unwrap();
                let error_defs = sorted_errors(&xproto_ns);
                for err_name in error_defs.iter().map(|def| def.name()) {
                    outln!(
                        out,
                        "xproto::{}_ERROR => return Ok(Self::{}(error.into())),",
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
                                "{}::{}_ERROR => Ok(Self::{}{}(error.into())),",
                                ns.header,
                                super::camel_case_to_upper_snake(err_name),
                                get_ns_name_prefix(ns),
                                err_name,
                            );
                        }
                        outln!(out.indent(), "_ => Ok(Self::Unknown(error)),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => Ok(Self::Unknown(error)),");
            });
            outln!(out, "}}")
        });
        outln!(out, "}}");
        outln!(out, "");
        outln!(
            out,
            "/// Get the sequence number contained in this X11 error",
        );
        outln!(out, "pub fn wire_sequence_number(&self) -> u16 {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            let msg = "Errors should always have a sequence number";
            outln!(
                out.indent(),
                "Error::Unknown(value) => value.raw_sequence_number().expect(\"{}\"),",
                msg,
            );
            for ns in namespaces.iter() {
                let has_feature = super::ext_has_feature(&ns.header);
                let error_defs = sorted_errors(ns);

                for err_name in error_defs.iter().map(|def| def.name()) {
                    if has_feature {
                        outln!(out.indent(), "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out.indent(),
                        "Error::{}{}(value) => value.sequence,",
                        get_ns_name_prefix(ns),
                        err_name,
                    );
                }
            }
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");
        outln!(out, "/// Get the error code of this X11 error");
        outln!(out, "pub fn error_code(&self) -> u8 {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            outln!(out.indent(), "Error::Unknown(value) => value.error_code(),");
            for ns in namespaces.iter() {
                let has_feature = super::ext_has_feature(&ns.header);
                let error_defs = sorted_errors(ns);
                for err_name in error_defs.iter().map(|def| def.name()) {
                    if has_feature {
                        outln!(out.indent(), "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out.indent(),
                        "Error::{}{}(value) => value.error_code,",
                        get_ns_name_prefix(ns),
                        err_name,
                    );
                }
            }
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");
        outln!(out, "/// Get the response type of this X11 error");
        outln!(out, "///");
        outln!(
            out,
            "/// This is not `pub` because it should always be `0` for errors.",
        );
        outln!(out, "fn raw_response_type(&self) -> u8 {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            outln!(
                out.indent(),
                "Error::Unknown(value) => value.response_type(),"
            );
            for ns in namespaces.iter() {
                let has_feature = super::ext_has_feature(&ns.header);
                let error_defs = sorted_errors(ns);
                for err_name in error_defs.iter().map(|def| def.name()) {
                    if has_feature {
                        outln!(out.indent(), "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out.indent(),
                        "Error::{}{}(value) => value.response_type,",
                        get_ns_name_prefix(ns),
                        err_name,
                    );
                }
            }
            outln!(out, "}}");
        });
        outln!(out, "}}")
    });
    outln!(out, "}}");
}

fn generate_events(out: &mut Output, module: &xcbgen::defs::Module) {
    let namespaces = module.sorted_namespaces();

    outln!(out, "/// Enumeration of all possible X11 events.");
    outln!(out, "#[derive(Debug, Clone)]");
    outln!(out, "pub enum Event<B: std::fmt::Debug + AsRef<[u8]>> {{");
    out.indented(|out| {
        outln!(out, "Unknown(GenericEvent<B>),");
        outln!(out, "Error(Error<B>),");

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
    outln!(out, "impl<B: std::fmt::Debug + AsRef<[u8]>> Event<B> {{");
    out.indented(|out| {
        outln!(
            out,
            "/// Parse a generic X11 event into a concrete event type."
        );
        outln!(out, "#[allow(clippy::cognitive_complexity)]");
        outln!(out, "pub fn parse(");
        outln!(out.indent(), "event: GenericEvent<B>,");
        outln!(out.indent(), "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, ") -> Result<Self, ParseError> {{");
        out.indented(|out| {
            outln!(out, "let event_code = event.response_type();");
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
                    "Self::Error(Error::parse(event.try_into()?, ext_info_provider)?)",
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
                            outln!(out.indent(), "return Ok(Self::Unknown(event));");
                            outln!(out, "}}");
                            outln!(out, "match event.raw_bytes()[1] {{");
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
                        outln!(out.indent(), "_ => Ok(Self::Unknown(event)),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => Ok(Self::Unknown(event)),");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "");

        outln!(out, "fn from_generic_event(");
        outln!(out.indent(), "event: GenericEvent<B>,");
        outln!(out.indent(), "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, ") -> Result<Self, ParseError> {{");
        out.indented(|out| {
            outln!(
                out,
                "let ge_event = xproto::GeGenericEvent::try_from(event.raw_bytes())?;"
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
                        outln!(out.indent(), "_ => Ok(Self::Unknown(event)),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => Ok(Self::Unknown(event)),");
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
                "Event::Unknown(value) => value.raw_sequence_number(),",
            );
            outln!(
                out.indent(),
                "Event::Error(value) => Some(value.wire_sequence_number()),",
            );
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
                "Event::Unknown(value) => value.raw_response_type(),",
            );
            outln!(
                out.indent(),
                "Event::Error(value) => value.raw_response_type(),",
            );
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

/// Get the prefix that should be used for enum variants from this module.
fn get_ns_name_prefix(ns: &xcbgen::defs::Namespace) -> String {
    if ns.ext_info.is_some() {
        let (first, remaining) = ns.header.split_at(1);
        let mut r = first.to_ascii_uppercase();
        r.push_str(remaining);
        r
    } else {
        String::new()
    }
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
