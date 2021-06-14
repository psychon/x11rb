use crate::generator::output::Output;

use xcbgen::defs as xcbdefs;

pub(super) fn write_header(out: &mut Output, ns: &xcbdefs::Namespace) {
    if let Some(info) = &ns.ext_info {
        outln!(out, "//! Bindings to the `{}` X11 extension.", info.name);
    } else {
        outln!(out, "//! Bindings to the core X11 protocol.");
        outln!(out, "//!");
        outln!(
            out,
            "//! For more documentation on the X11 protocol, see the"
        );
        outln!(
            out,
            "//! [protocol reference manual](https://www.x.org/releases/X11R7.6/doc/xproto/x11protocol.html).",
        );
        outln!(
            out,
            "//! This is especially recommended for looking up the exact semantics of"
        );
        outln!(out, "//! specific errors, events, or requests.");
    }
    outln!(out, "");
    outln!(out, "#![allow(clippy::too_many_arguments)]");
    outln!(out, "");
    outln!(out, "#[allow(unused_imports)]");
    outln!(out, "use std::borrow::Cow;");
    outln!(out, "use std::convert::TryFrom;");
    outln!(out, "#[allow(unused_imports)]");
    outln!(out, "use std::convert::TryInto;");
    outln!(out, "use std::io::IoSlice;");
    outln!(out, "#[allow(unused_imports)]");
    outln!(
        out,
        "use crate::utils::{{RawFdContainer, pretty_print_bitmask, pretty_print_enum}};"
    );
    outln!(out, "#[allow(unused_imports)]");
    outln!(
        out,
        "use crate::x11_utils::{{Request, RequestHeader, Serialize, TryParse, TryParseFd, \
         TryIntoUSize}};"
    );
    outln!(
        out,
        "use crate::connection::{{BufWithFds, PiecewiseBuf, RequestConnection}};"
    );
    outln!(out, "#[allow(unused_imports)]");
    outln!(
        out,
        "use crate::cookie::{{Cookie, CookieWithFds, VoidCookie}};"
    );
    if ns.header == "xproto" {
        outln!(out, "use crate::cookie::ListFontsWithInfoCookie;");
    }
    if ns.header == "record" {
        outln!(out, "use crate::cookie::RecordEnableContextCookie;");
    }
    outln!(out, "use crate::errors::{{ConnectionError, ParseError}};");

    let mut imports = ns
        .imports
        .borrow()
        .values()
        .map(|import| import.name.clone())
        .collect::<Vec<_>>();
    imports.sort();
    for import in imports.iter() {
        outln!(out, "use super::{};", import);
    }

    if let Some(ref ext_info) = ns.ext_info {
        outln!(out, "");
        outln!(out, "/// The X11 name of the extension for QueryExtension");
        outln!(
            out,
            "pub const X11_EXTENSION_NAME: &str = \"{}\";",
            ext_info.xname,
        );
        outln!(out, "");
        outln!(
            out,
            "/// The version number of this extension that this client library supports.",
        );
        outln!(out, "///");
        outln!(
            out,
            "/// This constant contains the version number of this extension that is supported",
        );
        outln!(
            out,
            "/// by this build of x11rb. For most things, it does not make sense to use this",
        );
        outln!(
            out,
            "/// information. If you need to send a `QueryVersion`, it is recommended to \
             instead"
        );
        outln!(
            out,
            "/// send the maximum version of the extension that you need.",
        );
        outln!(
            out,
            "pub const X11_XML_VERSION: (u32, u32) = ({}, {});",
            ext_info.major_version,
            ext_info.minor_version,
        );

        outln!(out, "");
        outln!(out, "/// Get the major opcode of this extension");
        outln!(out, "fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {{");
        out.indented(|out| {
            outln!(
                out,
                "let info = conn.extension_information(X11_EXTENSION_NAME)?;",
            );
            outln!(
                out,
                "let info = info.ok_or(ConnectionError::UnsupportedExtension)?;",
            );
            outln!(out, "Ok(info.major_opcode)");
        });
        outln!(out, "}}");
    }
    outln!(out, "");
}
