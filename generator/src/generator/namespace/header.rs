use super::async_switch::ImplMode;
use crate::generator::output::Output;

use xcbgen::defs as xcbdefs;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(super) enum Mode {
    Protocol,
    X11rb,
}

pub(super) fn write_header(
    out: &mut Output,
    ns: &xcbdefs::Namespace,
    mode: Mode,
    poll_mode: ImplMode,
) {
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

    let (alloc_name, core_name) = match mode {
        Mode::Protocol => ("alloc", "core"),
        Mode::X11rb => ("std", "std"),
    };

    outln!(out, "");
    outln!(out, "#![allow(clippy::too_many_arguments)]");
    if mode == Mode::Protocol {
        outln!(
            out,
            "// The code generator is simpler if it can always use conversions"
        );
        outln!(out, "#![allow(clippy::useless_conversion)]");
    }
    outln!(out, "");
    outln!(out, "#[allow(unused_imports)]");
    outln!(out, "use {}::borrow::Cow;", alloc_name);
    outln!(out, "#[allow(unused_imports)]");
    outln!(out, "use {}::convert::TryInto;", core_name);
    if mode == Mode::Protocol {
        outln!(out, "use alloc::vec;");
        outln!(out, "use alloc::vec::Vec;");
        outln!(out, "use core::convert::TryFrom;");
        outln!(out, "use crate::errors::ParseError;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::x11_utils::TryIntoUSize;");
        outln!(out, "use crate::BufWithFds;");
    }
    outln!(out, "#[allow(unused_imports)]");
    match mode {
        Mode::Protocol => outln!(
            out,
            "use crate::utils::{{RawFdContainer, pretty_print_bitmask, pretty_print_enum}};"
        ),
        Mode::X11rb => outln!(out, "use crate::utils::RawFdContainer;"),
    }
    outln!(out, "#[allow(unused_imports)]");
    outln!(
        out,
        "use crate::x11_utils::{{Request, RequestHeader, Serialize, TryParse, TryParseFd}};"
    );
    if mode == Mode::X11rb {
        outln!(out, "use std::io::IoSlice;");
        outln!(out, "use crate::connection::RequestConnection;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::connection::Connection as X11Connection;");
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
        outln!(out, "use crate::errors::ConnectionError;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::errors::ReplyOrIdError;");

        if poll_mode == ImplMode::Async {
            outln!(out, "use std::future::Future;");
            outln!(out, "use std::pin::Pin;");
        }
    }

    let mut imports = ns
        .imports
        .borrow()
        .values()
        .map(|import| import.name.clone())
        .collect::<Vec<_>>();
    imports.sort();
    for import in imports.iter() {
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use super::{};", import);
    }

    if mode == Mode::X11rb {
        outln!(out, "");
        outln!(out, "pub use x11rb_protocol::protocol::{}::*;", ns.header);
    }

    if let Some(ref ext_info) = ns.ext_info {
        if mode == Mode::Protocol {
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
        }

        if mode == Mode::X11rb {
            outln!(out, "");
            outln!(out, "/// Get the major opcode of this extension");
            outln!(out, "{}fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {{", poll_mode.fn_async());
            out.indented(|out| {
                outln!(
                    out,
                    "let info = conn.extension_information(X11_EXTENSION_NAME){}?;",
                    poll_mode.dot_await()
                );
                outln!(
                    out,
                    "let info = info.ok_or(ConnectionError::UnsupportedExtension)?;",
                );
                outln!(out, "Ok(info.major_opcode)");
            });
            outln!(out, "}}");
        }
    }
    outln!(out, "");
}
