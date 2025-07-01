use std::collections::HashMap;

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

fn generate_request_naming_internal(out: &mut Output, module: &xcbdefs::Module) {
    outln!(
        out,
        "/// Helper container for translating numeric request information to a string"
    );
    outln!(out, "#[derive(Debug)]");
    outln!(out, "enum RequestInfo {{");
    out.indented(|out| {
        outln!(out, "/// A core protocol request");
        outln!(out, "Xproto(&'static str),");
        outln!(out, "/// A known request from a known extension. String is of the form \"ExtName::RequestName\".");
        outln!(out, "KnownExt(&'static str),");
        outln!(out, "/// A request which could not be identified. The first entry is the extension name (or none for xproto). Second is opcode.");
        outln!(out, "UnknownRequest(Option<&'static str>, u8),");
        outln!(out, "/// A request from an extension that could not be identified");
        outln!(out, "UnknownExtension(u8, u8),");
    });
    outln!(out, "}}");
    outln!(out, "");

    outln!(
        out,
        "/// Get information about a request based on its major and minor code."
    );
    outln!(out, "///");
    outln!(
        out,
        "/// The major and minor opcode are the first and second byte of a request."
    );
    outln!(out, "/// Core requests do not have a minor opcode. For these, the minor opcode is ignored by this function.");
    outln!(out, "///");
    outln!(out, "/// This function returns the name of the extension to which the request belongs, if available, and information about the specific request.");
    outln!(out, "fn get_request_name_internal(");
    out.indented(|out| {
        outln!(out, "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, "major_opcode: u8,");
        outln!(out, "minor_opcode: u8,");
    });
    outln!(out, ") -> (Option<&str>, RequestInfo) {{");
    out.indented(|out| {
        outln!(out, "// From the X11 protocol reference manual:");
        outln!(out, "// Major opcodes 128 through 255 are reserved for extensions.");
        outln!(out, "if major_opcode < 128 {{");
        out.indented(|out| {
            outln!(out, "match major_opcode {{");
            out.indented(|out| {
                let xproto_ns = module.namespace("xproto").unwrap();
                for def in xproto_ns.src_order_defs.borrow().iter() {
                    if let xcbdefs::Def::Request(request) = def {
                        outln!(out, "xproto::{}_REQUEST => (None, RequestInfo::Xproto(\"{}\")),", super::camel_case_to_upper_snake(&request.name), request.name);
                    }
                }
                outln!(out, "_ => (None, RequestInfo::UnknownRequest(None, major_opcode)),");
            });
            outln!(out, "}}");
        });
        outln!(out, "}} else {{");
        out.indented(|out| {
            outln!(out, "// Figure out the extension name");
            outln!(out, "let ext_name = match ext_info_provider.get_from_major_opcode(major_opcode) {{");
            out.indented(|out| {
                outln!(out, "Some((name, _)) => name,");
                outln!(out, "None => return (None, RequestInfo::UnknownExtension(major_opcode, minor_opcode)),");
            });
            outln!(out, "}};");
            outln!(out, "let info = match ext_name {{");
            out.indented(|out| {
                for ns in module.sorted_namespaces() {
                    // xproto was already handled above
                    if let Some(ext_info) = &ns.ext_info {
                        let has_feature = super::ext_has_feature(&ns.header);
                        if has_feature {
                            outln!(out, "#[cfg(feature = \"{}\")]", ns.header);
                        }
                        outln!(out, "{}::X11_EXTENSION_NAME => {{", ns.header);
                        out.indented(|out| {
                            outln!(out, "match minor_opcode {{");
                            out.indented(|out| {
                                for def in ns.src_order_defs.borrow().iter() {
                                    if let xcbdefs::Def::Request(request) = def {
                                        outln!(out, "{}::{}_REQUEST => RequestInfo::KnownExt(\"{}::{}\"),", ns.header, super::camel_case_to_upper_snake(&request.name), ext_info.name, request.name);
                                    }
                                }
                                outln!(out, "_ => RequestInfo::UnknownRequest(Some(\"{}\"), minor_opcode),", ext_info.name);
                            });
                            outln!(out, "}}");
                        });
                        outln!(out, "}}");
                    }
                }
                outln!(out, "_ => RequestInfo::UnknownExtension(major_opcode, minor_opcode),");
            });
            outln!(out, "}};");
            outln!(out, "(Some(ext_name), info)");
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
}

/// Generate a function that figures out the name of a request
fn generate_request_naming(out: &mut Output) {
    outln!(
        out,
        "/// Get the name of a request based on its major and minor code."
    );
    outln!(out, "///");
    outln!(
        out,
        "/// The major and minor opcode are the first and second byte of a request."
    );
    outln!(out, "/// Core requests do not have a minor opcode. For these, the minor opcode is ignored by this function.");
    outln!(out, "pub fn get_request_name(");
    out.indented(|out| {
        outln!(out, "ext_info_provider: &dyn ExtInfoProvider,");
        outln!(out, "major_opcode: u8,");
        outln!(out, "minor_opcode: u8,");
    });
    outln!(out, ") -> Cow<'static, str> {{");
    out.indented(|out| {
        outln!(out, "let (ext_name, info) = get_request_name_internal(ext_info_provider, major_opcode, minor_opcode);");
        outln!(out, "match info {{");
        out.indented(|out| {
            outln!(out, "RequestInfo::Xproto(request) => request.into(),");
            outln!(out, "RequestInfo::KnownExt(ext_and_request) => ext_and_request.into(),");
            outln!(out, "RequestInfo::UnknownRequest(None, opcode) => alloc::format!(\"xproto::opcode {{opcode}}\").into(),");
            outln!(out, "RequestInfo::UnknownRequest(Some(ext), opcode) => alloc::format!(\"{{ext}}::opcode {{opcode}}\").into(),");
            outln!(out, "RequestInfo::UnknownExtension(major_opcode, minor_opcode) => match ext_name {{");
            out.indented(|out| {
                outln!(out, "None => alloc::format!(\"ext {{major_opcode}}::opcode {{minor_opcode}}\").into(),");
                outln!(out, "Some(ext_name) => alloc::format!(\"ext {{ext_name}}::opcode {{minor_opcode}}\").into(),");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
}

/// Generate the Request and Reply enums containing all possible requests and replies, respectively.
pub(super) fn generate(out: &mut Output, module: &xcbdefs::Module, mut enum_cases: EnumCases) {
    let namespaces = module.sorted_namespaces();

    generate_request_naming_internal(out, module);
    generate_request_naming(out);

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
        outln!(out, "#[cfg(feature = \"request-parsing\")]");
        outln!(out, "pub fn parse(");
        out.indented(|out| {
            outln!(out, "header: RequestHeader,");
            outln!(out, "body: &'input [u8],");
            outln!(
                out,
                "// Might not be used if none of the extensions that use FD passing is enabled",
            );
            outln!(out, "#[allow(unused_variables, clippy::ptr_arg)]");
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
    outln!(out, "///");
    outln!(
        out,
        "/// First result is the name of the extension, second the name of the request."
    );
    outln!(out, "pub(crate) fn request_name(ext_info_provider: &dyn ExtInfoProvider, major_opcode: u8, minor_opcode: u16) -> (Option<String>, Option<&'static str>) {{");
    out.indented(|out| {
        outln!(out, "// Don't ask me why X11 errors contain u16 for minor opcode, but requests are sent with u8.");
        outln!(out, "// We have to work around that incompatibility here.");
        outln!(out, "// From the X11 protocol reference manual:");
        outln!(out, "// Major opcodes 128 through 255 are reserved for extensions.");
        outln!(out, "let (ext, info) = if major_opcode < 128 || minor_opcode <= u16::from(u8::MAX) {{");
        out.indented(|out| {
            outln!(out, "get_request_name_internal(ext_info_provider, major_opcode, minor_opcode as u8)");
        });
        outln!(out, "}} else {{");
        out.indented(|out| {
            outln!(out, "let ext = ext_info_provider.get_from_major_opcode(major_opcode);");
            outln!(out, "return (ext.map(|(ext, _)| String::from(ext)), None);");
        });
        outln!(out, "}};");
        outln!(out, "let ext = ext.map(String::from);");
        outln!(out, "let info = match info {{");
        out.indented(|out| {
            outln!(out, "RequestInfo::Xproto(request) => request.into(),");
            outln!(out, "RequestInfo::KnownExt(ext_and_request) => ext_and_request.split_once(\"::\").map(|r| r.1),");
            outln!(out, "RequestInfo::UnknownRequest(_, _) => None,");
            outln!(out, "RequestInfo::UnknownExtension(_, _) => None,");
        });
        outln!(out, "}};");
        outln!(out, "(ext, info)");
    });
    outln!(out, "}}");
    outln!(out, "");
}
