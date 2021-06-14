use std::collections::HashMap;
use std::rc::Rc;

use xcbgen::defs as xcbdefs;

use super::output::Output;

#[derive(Debug, Default)]
pub(super) struct PerModuleEnumCases {
    /// Lines that belong in the Request enum definition.
    pub(super) request_variants: Vec<String>,
    /// Lines that belong in definition of Request::parse.
    pub(super) request_parse_cases: Vec<String>,
    /// Lines that belong in the definition of Request::reply_parser.
    pub(super) reply_parse_cases: Vec<String>,
    /// Lines that belong in the definition of Request::into_owned.
    pub(super) request_into_owned_cases: Vec<String>,
    /// Lines that belong in the Reply enum definition.
    pub(super) reply_variants: Vec<String>,
    /// Impls for From<ReplyType> for Reply enum.
    pub(super) reply_from_cases: Vec<String>,
}

pub(super) type EnumCases = HashMap<String, PerModuleEnumCases>;

/// Generate the Request and Reply enums containing all possible requests and replies, respectively.
pub(super) fn generate(out: &mut Output, module: &xcbdefs::Module, mut enum_cases: EnumCases) {
    let namespaces = module.sorted_namespaces();

    outln!(out, "/// Enumeration of all possible X11 requests.");
    outln!(out, "#[derive(Debug)]");
    // clippy::large_enum_variant for XkbSetNamesRequest.
    outln!(out, "#[allow(clippy::large_enum_variant)]");
    outln!(out, "#[non_exhaustive]");
    outln!(out, "pub enum Request<'input> {{");
    out.indented(|out| {
        outln!(out, "Unknown(RequestHeader, Cow<'input, [u8]>),");
        for ns in namespaces.iter() {
            let has_feature = super::ext_has_feature(&ns.header);

            let request_cases = enum_cases
                .get_mut(&ns.header)
                .unwrap()
                .request_variants
                .drain(..);
            for case in request_cases {
                if has_feature {
                    outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                }
                outln!(out, "{}", case);
            }
        }
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "impl<'input> Request<'input> {{");
    out.indented(|out| {
        outln!(out, "// Parse a X11 request into a concrete type");
        outln!(
            out,
            "#[allow(clippy::cognitive_complexity, clippy::single_match)]"
        );
        outln!(out, "pub fn parse(");
        out.indented(|out| {
            outln!(out, "header: RequestHeader,");
            outln!(out, "body: &'input [u8],");
            outln!(
                out,
                "// Might not be used if none of the extensions that use FD passing is enabled",
            );
            outln!(out, "#[allow(unused_variables)]");
            outln!(out, "fds: &mut Vec<RawFdContainer>,");
            outln!(out, "ext_info_provider: &dyn ExtInfoProvider,");
        });
        outln!(out, ") -> Result<Self, ParseError> {{");
        out.indented(|out| {
            outln!(out, "let remaining = body;");
            outln!(out, "// Check if this is a core protocol request.");
            outln!(out, "match header.major_opcode {{");
            out.indented(|out| {
                let xproto_ns = module.namespace("xproto").unwrap();
                let xproto_cases = enum_cases
                    .get_mut(&xproto_ns.header)
                    .unwrap()
                    .request_parse_cases
                    .drain(..);
                for case in xproto_cases {
                    outln!(out, "{}", case);
                }
                outln!(out, "_ => (),");
            });
            outln!(out, "}}");
            outln!(
                out,
                "// Find the extension that this request could belong to"
            );
            outln!(
                out,
                "let ext_info = ext_info_provider.get_from_major_opcode(header.major_opcode);"
            );
            outln!(out, "match ext_info {{");
            out.indented(|out| {
                for ns in namespaces.iter() {
                    let parse_cases =
                        &mut enum_cases.get_mut(&ns.header).unwrap().request_parse_cases;
                    if parse_cases.is_empty() {
                        continue;
                    }

                    let has_feature = super::ext_has_feature(&ns.header);
                    if has_feature {
                        outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(out, "Some(({}::X11_EXTENSION_NAME, _)) => {{", ns.header);

                    out.indented(|out| {
                        let parse_cases = parse_cases.drain(..);
                        outln!(out, "match header.minor_opcode {{");
                        for case in parse_cases {
                            outln!(out.indent(), "{}", case);
                        }
                        outln!(out.indent(), "_ => (),");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
                outln!(out, "_ => (),");
            });
            outln!(out, "}}");
            outln!(
                out,
                "Ok(Request::Unknown(header, Cow::Borrowed(remaining)))"
            );
        });
        outln!(out, "}}");
        outln!(
            out,
            "/// Get the matching reply parser (if any) for this request."
        );
        outln!(out, "/// For `Request::Unknown`, `None` is also returned.");
        outln!(
            out,
            "pub fn reply_parser(&self) -> Option<ReplyParsingFunction> {{"
        );
        out.indented(|out| {
            outln!(out, "match self {{");
            out.indented(|out| {
                outln!(out, "Request::Unknown(_, _) => None,");
                for ns in namespaces.iter() {
                    let has_feature = super::ext_has_feature(&ns.header);

                    let reply_parse_cases = enum_cases
                        .get_mut(&ns.header)
                        .unwrap()
                        .reply_parse_cases
                        .drain(..);
                    for case in reply_parse_cases {
                        if has_feature {
                            outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                        }
                        outln!(out, "{}", case);
                    }
                }
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(
            out,
            "/// Convert this Request into an owned version with no borrows."
        );
        outln!(out, "pub fn into_owned(self) -> Request<'static> {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            out.indented(|out| {
                outln!(
                    out,
                    "Request::Unknown(header, body) => Request::Unknown(header, \
                     Cow::Owned(body.into_owned())),"
                );
                for ns in namespaces.iter() {
                    let has_feature = super::ext_has_feature(&ns.header);

                    let request_into_owned_cases = enum_cases
                        .get_mut(&ns.header)
                        .unwrap()
                        .request_into_owned_cases
                        .drain(..);
                    for case in request_into_owned_cases {
                        if has_feature {
                            outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                        }
                        outln!(out, "{}", case);
                    }
                }
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "/// Enumeration of all possible X11 replies.");
    outln!(out, "#[derive(Debug)]");
    // clippy::large_enum_variant for XkbGetKbdByNameReply.
    outln!(out, "#[allow(clippy::large_enum_variant)]");
    outln!(out, "#[non_exhaustive]");
    outln!(out, "pub enum Reply {{");
    out.indented(|out| {
        outln!(out, "Void,");
        for ns in namespaces.iter() {
            let has_feature = super::ext_has_feature(&ns.header);

            let reply_cases = enum_cases
                .get_mut(&ns.header)
                .unwrap()
                .reply_variants
                .drain(..);
            for case in reply_cases {
                if has_feature {
                    outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                }
                outln!(out, "{}", case);
            }
        }
    });
    outln!(out, "}}");
    outln!(out, "impl From<()> for Reply {{");
    out.indented(|out| {
        outln!(out, "fn from(_: ()) -> Reply {{");
        outln!(out.indent(), "Reply::Void");
        outln!(out, "}}");
    });
    outln!(out, "}}");
    for ns in namespaces.iter() {
        let has_feature = super::ext_has_feature(&ns.header);

        let reply_from_cases = enum_cases
            .get_mut(&ns.header)
            .unwrap()
            .reply_from_cases
            .drain(..);
        for case in reply_from_cases {
            if has_feature {
                outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
            }
            outln!(out, "{}", case);
        }
    }
    outln!(out, "");

    outln!(
        out,
        "/// Get the name of a request from its extension name and opcodes.",
    );
    outln!(out, "pub(crate) fn request_name(extension: Option<&str>, major_opcode: u8, minor_opcode: u16) -> Option<&'static str> {{");
    out.indented(|out| {
        outln!(out, "// Check if this is a core protocol request.");
        outln!(out, "match major_opcode {{");
        out.indented(|out| {
            let xproto_ns = module.namespace("xproto").unwrap();
            for def in sorted_requests(&*xproto_ns) {
                outln!(out, "{} => return Some(\"{}\"),", def.opcode, def.name);
            }
            outln!(out, "_ => (),");
        });
        outln!(out, "}}");

        outln!(out, "// Check the extension");
        outln!(out, "match (extension, minor_opcode) {{");
        out.indented(|out| {
            for ns in namespaces.iter() {
                if ns.header == "xproto" {
                    continue;
                }
                let has_feature = super::ext_has_feature(&ns.header);
                for def in sorted_requests(ns) {
                    if has_feature {
                        outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                    }
                    outln!(
                        out,
                        "(Some({}::X11_EXTENSION_NAME), {}) => Some(\"{}\"),",
                        ns.header,
                        def.opcode,
                        def.name,
                    );
                }
            }
            outln!(out, "_ => None,");
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
}

/// Get all requests in the namespace in a sorted order
fn sorted_requests(ns: &xcbgen::defs::Namespace) -> Vec<Rc<xcbgen::defs::RequestDef>> {
    let mut events: Vec<_> = ns.request_defs.borrow().values().cloned().collect();
    events.sort_by(|a, b| a.opcode.cmp(&b.opcode));
    events
}
