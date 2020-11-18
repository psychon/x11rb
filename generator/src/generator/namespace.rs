#![allow(
    clippy::cognitive_complexity,
    clippy::match_like_matches_macro,
    clippy::too_many_arguments
)]

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::hash_map::Entry as HashMapEntry;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::rc::Rc;

use xcbgen::defs as xcbdefs;

use super::output::Output;
use super::{get_ns_name_prefix, special_cases};

#[derive(Debug, Default)]
pub(super) struct PerModuleEnumCases {
    /// Lines that belong in the Request enum definition.
    request_variants: Vec<String>,
    /// Lines that belong in definition of Request::parse.
    request_parse_cases: Vec<String>,
    /// Lines that belong in the definition of Request::reply_parser.
    reply_parse_cases: Vec<String>,
    /// Lines that belong in the definition of Request::into_owned.
    request_into_owned_cases: Vec<String>,
    /// Lines that belong in the Reply enum definition.
    reply_variants: Vec<String>,
    /// Impls for From<ReplyType> for Reply enum.
    reply_from_cases: Vec<String>,
}

type EnumCases = HashMap<String, PerModuleEnumCases>;

/// Generate a Rust module for namespace `ns`.
pub(super) fn generate(
    ns: &xcbdefs::Namespace,
    caches: &RefCell<Caches>,
    out: &mut Output,
    enum_cases: &mut EnumCases,
) {
    NamespaceGenerator::new(ns, caches).generate(out, enum_cases);
}

/// Generate the Request and Reply enums containing all possible requests and replies, respectively.
pub(super) fn generate_request_reply_enum(
    out: &mut Output,
    module: &xcbdefs::Module,
    mut enum_cases: EnumCases,
) {
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
        outln!(out, "/// Convert this Request into an owned version with no borrows.");
        outln!(out, "pub fn into_owned(self) -> Request<'static> {{");
        out.indented(|out| {
            outln!(out, "match self {{");
            out.indented(|out| {
                outln!(out, "Request::Unknown(header, body) => Request::Unknown(header, Cow::Owned(body.into_owned())),");
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IovecConversion {
    // No conversion is required.
    None,
    // A simple .into() is enough.
    Into,
    // Call cow_strip_length.
    CowStripLength,
}

#[derive(Copy, Clone)]
struct EnumInfo {
    // The size needed to hold all defined values for the enum
    max_value_size: Option<u8>,
    // The range of wire sizes used for the enum
    // min, max
    wire_size: Option<(u8, u8)>,
}

/// Caches to avoid repeating some operations.
#[derive(Default)]
pub(super) struct Caches {
    enum_infos: HashMap<usize, EnumInfo>,
    derives: HashMap<usize, Derives>,
    rust_type_names: HashMap<usize, String>,
}

impl Caches {
    #[inline]
    fn enum_info(&self, enum_def: &xcbdefs::EnumDef) -> EnumInfo {
        self.enum_infos[&(enum_def as *const xcbdefs::EnumDef as usize)]
    }

    fn put_enum_max_value_size(&mut self, enum_def: &xcbdefs::EnumDef, max_value_size: u8) {
        let id = enum_def as *const xcbdefs::EnumDef as usize;
        match self.enum_infos.entry(id) {
            HashMapEntry::Vacant(entry) => {
                entry.insert(EnumInfo {
                    max_value_size: Some(max_value_size),
                    wire_size: None,
                });
            }
            HashMapEntry::Occupied(mut entry) => {
                let entry_value = entry.get_mut();
                if entry_value.max_value_size.is_none() {
                    entry_value.max_value_size = Some(max_value_size);
                } else {
                    // Expected only once
                    unreachable!();
                }
            }
        }
    }

    fn put_enum_wire_size(&mut self, enum_def: &xcbdefs::EnumDef, wire_size: u8) {
        let id = enum_def as *const xcbdefs::EnumDef as usize;
        match self.enum_infos.entry(id) {
            HashMapEntry::Vacant(entry) => {
                entry.insert(EnumInfo {
                    max_value_size: None,
                    wire_size: Some((wire_size, wire_size)),
                });
            }
            HashMapEntry::Occupied(mut entry) => {
                let entry_value = entry.get_mut();
                match entry_value.wire_size {
                    None => entry_value.wire_size = Some((wire_size, wire_size)),
                    Some((ref mut min, ref mut max)) => {
                        *min = (*min).min(wire_size);
                        *max = (*max).max(wire_size)
                    }
                }
            }
        }
    }

    pub(super) fn gather_enum_infos(&mut self, module: &xcbdefs::Module) {
        for ns in module.namespaces.borrow().values() {
            for request_def in ns.request_defs.borrow().values() {
                for field in request_def.fields.borrow().iter() {
                    self.gather_enum_infos_in_field(field);
                }
                if let Some(ref reply_def) = request_def.reply {
                    for field in reply_def.fields.borrow().iter() {
                        self.gather_enum_infos_in_field(field);
                    }
                }
            }

            for event_def in ns.event_defs.borrow().values() {
                match event_def {
                    xcbdefs::EventDef::Full(event_def) => {
                        for field in event_def.fields.borrow().iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::EventDef::Copy(_) => {}
                }
            }

            for error_def in ns.error_defs.borrow().values() {
                match error_def {
                    xcbdefs::ErrorDef::Full(error_def) => {
                        for field in error_def.fields.borrow().iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::ErrorDef::Copy(_) => {}
                }
            }

            for type_def in ns.type_defs.borrow().values() {
                match type_def {
                    xcbdefs::TypeDef::Struct(struct_def) => {
                        for field in struct_def.fields.borrow().iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::TypeDef::Union(union_def) => {
                        for field in union_def.fields.iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::TypeDef::EventStruct(_) => {}
                    xcbdefs::TypeDef::Xid(_) => {}
                    xcbdefs::TypeDef::XidUnion(_) => {}
                    xcbdefs::TypeDef::Enum(enum_def) => {
                        self.gather_enum_infos_in_enum_def(enum_def);
                    }
                    xcbdefs::TypeDef::Alias(_) => {}
                }
            }
        }
    }

    fn gather_enum_infos_in_field(&mut self, field: &xcbdefs::FieldDef) {
        match field {
            xcbdefs::FieldDef::Pad(_) => {}
            xcbdefs::FieldDef::Normal(normal_field) => {
                self.gather_enum_infos_in_field_value_type(&normal_field.type_);
            }
            xcbdefs::FieldDef::List(list_field) => {
                self.gather_enum_infos_in_field_value_type(&list_field.element_type);
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                for case in switch_field.cases.iter() {
                    for field in case.fields.borrow().iter() {
                        self.gather_enum_infos_in_field(field);
                    }
                }
            }
            xcbdefs::FieldDef::Fd(_) => {}
            xcbdefs::FieldDef::FdList(_) => {}
            xcbdefs::FieldDef::Expr(expr_field) => {
                self.gather_enum_infos_in_field_value_type(&expr_field.type_);
            }
            xcbdefs::FieldDef::VirtualLen(_) => {}
        }
    }

    fn gather_enum_infos_in_field_value_type(&mut self, value_type: &xcbdefs::FieldValueType) {
        match value_type.value_set {
            xcbdefs::FieldValueSet::None => {}
            xcbdefs::FieldValueSet::Enum(ref enum_type) => {
                let enum_def = match enum_type.get_resolved().get_original_type() {
                    xcbdefs::TypeRef::Enum(enum_type) => enum_type.upgrade().unwrap(),
                    _ => unreachable!(),
                };

                let size = match value_type.type_.get_resolved().get_original_type() {
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Bool) => 1,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card8) => 8,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card16) => 16,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) => 32,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Byte) => 8,
                    _ => unreachable!(),
                };
                self.put_enum_wire_size(&enum_def, size);
            }
            xcbdefs::FieldValueSet::AltEnum(_) => {}
            xcbdefs::FieldValueSet::Mask(_) => {}
            xcbdefs::FieldValueSet::AltMask(_) => {}
        }
    }

    fn gather_enum_infos_in_enum_def(&mut self, enum_def: &xcbdefs::EnumDef) {
        // Get the maximum value of the defined variants for this enum
        let max_value = enum_def
            .items
            .iter()
            .map(|enum_item| match enum_item.value {
                xcbdefs::EnumValue::Value(value) => value,
                xcbdefs::EnumValue::Bit(bit) => 1 << bit,
            })
            .max()
            .unwrap();

        let size = if max_value == 1 && enum_def.items.len() == 2 {
            1
        } else if max_value <= 0xFF {
            8
        } else if max_value <= 0xFFFF {
            16
        } else {
            32
        };

        self.put_enum_max_value_size(enum_def, size);
    }
}

struct NamespaceGenerator<'ns, 'c> {
    ns: &'ns xcbdefs::Namespace,
    caches: &'c RefCell<Caches>,

    /// `Option` or `std::option::Option`
    option_name: &'static str,
}

impl<'ns, 'c> NamespaceGenerator<'ns, 'c> {
    #[inline]
    fn new(ns: &'ns xcbdefs::Namespace, caches: &'c RefCell<Caches>) -> Self {
        let option_name = if ns.header == "present" {
            "std::option::Option"
        } else {
            "Option"
        };
        NamespaceGenerator {
            ns,
            caches,
            option_name,
        }
    }

    fn generate(&self, out: &mut Output, enum_cases: &mut EnumCases) {
        super::write_code_header(out);
        if let Some(info) = &self.ns.ext_info {
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
        outln!(out, "#![allow(clippy::identity_op)]");
        outln!(out, "#![allow(clippy::trivially_copy_pass_by_ref)]");
        outln!(out, "#![allow(clippy::eq_op)]");
        outln!(out, "");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use std::borrow::Cow;");
        outln!(out, "use std::convert::TryFrom;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use std::convert::TryInto;");
        outln!(out, "use std::io::IoSlice;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::utils::RawFdContainer;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(
            out,
            "use crate::x11_utils::{{Request, RequestHeader, Serialize, TryParse, TryParseFd}};"
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
        if self.ns.header == "xproto" {
            outln!(out, "use crate::cookie::ListFontsWithInfoCookie;");
        }
        if self.ns.header == "record" {
            outln!(out, "use crate::cookie::RecordEnableContextCookie;");
        }
        outln!(out, "use crate::errors::{{ConnectionError, ParseError}};");

        let mut imports = self
            .ns
            .imports
            .borrow()
            .values()
            .map(|import| import.name.clone())
            .collect::<Vec<_>>();
        imports.sort();
        for import in imports.iter() {
            outln!(out, "use super::{};", import);
        }

        if let Some(ref ext_info) = self.ns.ext_info {
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
        outln!(out, "");

        let mut trait_out = Output::new();

        for def in self.ns.src_order_defs.borrow().iter() {
            match def {
                xcbdefs::Def::Request(request_def) => {
                    let cases_entry = enum_cases.entry(self.ns.header.clone()).or_default();
                    self.generate_request(request_def, out, &mut trait_out, cases_entry)
                }
                xcbdefs::Def::Event(event_def) => match event_def {
                    xcbdefs::EventDef::Full(event_full_def) => {
                        self.generate_event_full_def(event_full_def, out);
                    }
                    xcbdefs::EventDef::Copy(event_copy_def) => {
                        self.generate_event_copy_def(event_copy_def, out);
                    }
                },
                xcbdefs::Def::Error(error_def) => match error_def {
                    xcbdefs::ErrorDef::Full(error_full_def) => {
                        self.generate_error_full_def(error_full_def, out);
                    }
                    xcbdefs::ErrorDef::Copy(error_copy_def) => {
                        self.generate_error_copy_def(error_copy_def, out);
                    }
                },
                xcbdefs::Def::Type(type_def) => match type_def {
                    xcbdefs::TypeDef::Struct(struct_def) => {
                        self.generate_struct_def(struct_def, out)
                    }
                    xcbdefs::TypeDef::Union(union_def) => self.generate_union_def(union_def, out),
                    xcbdefs::TypeDef::EventStruct(event_struct_def) => {
                        self.generate_event_struct_def(event_struct_def, out);
                    }
                    xcbdefs::TypeDef::Xid(xid_type_def) => {
                        self.generate_xid_type_def(xid_type_def, out)
                    }
                    xcbdefs::TypeDef::XidUnion(xid_union_def) => {
                        self.generate_xid_union_def(xid_union_def, out)
                    }
                    xcbdefs::TypeDef::Enum(enum_def) => self.generate_enum_def(enum_def, out),
                    xcbdefs::TypeDef::Alias(type_alias_def) => {
                        self.generate_type_alias_def(type_alias_def, out)
                    }
                },
            }
        }

        let trait_out = trait_out.into_data();

        outln!(
            out,
            "/// Extension trait defining the requests of this extension.",
        );
        outln!(out, "pub trait ConnectionExt: RequestConnection {{");
        out!(out.indent(), "{}", trait_out);
        outln!(out, "}}");
        outln!(out, "");
        outln!(
            out,
            "impl<C: RequestConnection + ?Sized> ConnectionExt for C {{}}",
        );
    }

    fn generate_request(
        &self,
        request_def: &xcbdefs::RequestDef,
        out: &mut Output,
        trait_out: &mut Output,
        enum_cases: &mut PerModuleEnumCases,
    ) {
        let name = to_rust_type_name(&request_def.name);

        let mut function_name = super::camel_case_to_lower_snake(&name);
        if function_name == "await" {
            function_name.push('_');
        }

        // If this have a <switch>, we generate an *Aux structure
        let request_fields = request_def.fields.borrow();
        let switch_fields = request_fields
            .iter()
            .filter_map(|field| match field {
                xcbdefs::FieldDef::Switch(switch_field) => Some(switch_field),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(
            switch_fields.len() <= 1,
            "request {}::{} has more than one <switch> field",
            self.ns.header,
            request_def.name,
        );

        let deducible_fields = gather_deducible_fields(&*request_fields);

        if switch_fields.len() == 1 {
            if let Some(aux_start_align) = switch_fields[0].required_start_align {
                assert_eq!(aux_start_align.offset(), 0);
            }
            self.generate_aux(request_def, switch_fields[0], &function_name, out);
        }

        outln!(out, "/// Opcode for the {} request", name);
        outln!(
            out,
            "pub const {}_REQUEST: u8 = {};",
            super::camel_case_to_upper_snake(&name),
            request_def.opcode,
        );

        let gathered = self.gather_request_fields(request_def, &deducible_fields);

        self.emit_request_struct(request_def, &name, &deducible_fields, &gathered, out);
        let ns_prefix = get_ns_name_prefix(self.ns);
        let lifetime_block = if gathered.needs_lifetime {
            "<'input>"
        } else {
            ""
        };
        enum_cases.request_variants.push(format!(
            "{ns_prefix}{name}({header}::{name}Request{lifetime}),",
            ns_prefix = ns_prefix,
            name = name,
            header = self.ns.header,
            lifetime = lifetime_block
        ));
        if gathered.has_fds() {
            enum_cases.request_parse_cases.push(format!(
                "{header}::{opcode_name}_REQUEST => return \
                 Ok(Request::{ns_prefix}{name}({header}::{name}Request::\
                 try_parse_request_fd(header, remaining, fds)?)),",
                header = self.ns.header,
                opcode_name = super::camel_case_to_upper_snake(&name),
                ns_prefix = ns_prefix,
                name = name,
            ));
        } else {
            enum_cases.request_parse_cases.push(format!(
                "{header}::{opcode_name}_REQUEST => return \
                 Ok(Request::{ns_prefix}{name}({header}::{name}Request::try_parse_request(header, \
                 remaining)?)),",
                header = self.ns.header,
                opcode_name = super::camel_case_to_upper_snake(&name),
                ns_prefix = ns_prefix,
                name = name,
            ));
        }
        self.emit_request_function(request_def, &name, &function_name, &gathered, out);
        self.emit_request_trait_function(request_def, &name, &function_name, &gathered, trait_out);

        special_cases::handle_request(request_def, out);

        outln!(out, "");

        if let Some(ref reply) = request_def.reply {
            let reply_struct_name = format!("{}Reply", name);
            enum_cases.reply_variants.push(format!(
                "{ns_prefix}{name}({header}::{name}Reply),",
                ns_prefix = ns_prefix,
                name = name,
                header = self.ns.header,
            ));
            enum_cases.reply_parse_cases.push(format!(
                "Request::{ns_prefix}{name}(_) => Some({header}::{name}Request::parse_reply),",
                ns_prefix = ns_prefix,
                name = name,
                header = self.ns.header,
            ));
            enum_cases.reply_from_cases.push(format!(
                r#"impl From<{header}::{name}Reply> for Reply {{
  fn from(reply: {header}::{name}Reply) -> Reply {{
    Reply::{ns_prefix}{name}(reply)
  }}
}}"#,
                ns_prefix = ns_prefix,
                name = name,
                header = self.ns.header,
            ));
            let reply_fields = reply.fields.borrow();
            let mut reply_derives = Derives::all();
            self.filter_derives_for_fields(&mut reply_derives, &*reply_fields, false);
            self.emit_struct_type(
                &reply_struct_name,
                &name,
                reply_derives,
                &*reply_fields,
                &[],
                false,
                true,
                StructSizeConstraint::EmbeddedLength { minimum: 32 },
                false,
                false,
                reply.doc.as_ref(),
                out,
            );

            outln!(out, "");
        } else {
            enum_cases.reply_parse_cases.push(format!(
                "Request::{ns_prefix}{name}(_) => None,",
                ns_prefix = ns_prefix,
                name = name,
            ));
        }

        if gathered.needs_lifetime {
            enum_cases.request_into_owned_cases.push(format!(
                "Request::{ns_prefix}{name}(req) => Request::{ns_prefix}{name}(req.into_owned()),",
                ns_prefix = ns_prefix,
                name = name,
            ));
        } else {
            enum_cases.request_into_owned_cases.push(format!(
                "Request::{ns_prefix}{name}(req) => Request::{ns_prefix}{name}(req),",
                ns_prefix = ns_prefix,
                name = name,
            ));
        }
    }

    fn generate_aux(
        &self,
        request_def: &xcbdefs::RequestDef,
        switch_field: &xcbdefs::SwitchField,
        function_name: &str,
        out: &mut Output,
    ) {
        let aux_name = format!("{}Aux", request_def.name);

        if switch_field.kind == xcbdefs::SwitchKind::Case {
            self.emit_switch_type(switch_field, &aux_name, true, true, None, out);
        } else {
            let doc = format!(
                "Auxiliary and optional information for the `{}` function",
                function_name,
            );
            let cases_infos =
                self.emit_switch_type(switch_field, &aux_name, true, true, Some(&doc), out);

            outln!(out, "impl {} {{", aux_name);
            out.indented(|out| {
                outln!(
                    out,
                    "/// Create a new instance with all fields unset / not present."
                );
                outln!(out, "pub fn new() -> Self {{");
                outln!(out.indent(), "Default::default()");
                outln!(out, "}}");

                for (case, case_info) in switch_field.cases.iter().zip(cases_infos.iter()) {
                    let (rust_case_name, rust_case_type) = match case_info {
                        CaseInfo::SingleField(index) => {
                            let fields = case.fields.borrow();
                            let single_field = &fields[*index];
                            let field_name = single_field.name().unwrap();
                            (
                                to_rust_variable_name(field_name),
                                self.field_to_rust_type(single_field, &aux_name),
                            )
                        }
                        CaseInfo::MultiField(field_name, struct_name) => {
                            (to_rust_variable_name(field_name), struct_name.clone())
                        }
                    };
                    outln!(
                        out,
                        "/// Set the `{}` field of this structure.",
                        rust_case_name,
                    );
                    outln!(
                        out,
                        "pub fn {}<I>(mut self, value: I) -> Self where I: Into<Option<{}>> {{",
                        rust_case_name,
                        rust_case_type,
                    );
                    outln!(out.indent(), "self.{} = value.into();", rust_case_name);
                    outln!(out.indent(), "self");
                    outln!(out, "}}");
                }
            });
            outln!(out, "}}");
        }

        outln!(out, "");
    }

    fn emit_request_struct(
        &self,
        request_def: &xcbdefs::RequestDef,
        name: &str,
        deducible_fields: &HashMap<String, DeducibleField>,
        gathered: &GatheredRequestFields,
        out: &mut Output,
    ) {
        let ns = request_def.namespace.upgrade().unwrap();
        let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";

        if let Some(ref doc) = request_def.doc {
            self.emit_doc(doc, out);
        }

        let mut derives = Derives::all();
        self.filter_derives_for_fields(&mut derives, &request_def.fields.borrow(), true);
        let derives = derives.to_list();
        if !derives.is_empty() {
            outln!(out, "#[derive({})]", derives.join(", "));
        }

        let (struct_lifetime_block, serialize_lifetime_block, parse_lifetime_block) =
            if gathered.needs_lifetime {
                ("<'input>", "", "'input ")
            } else {
                ("", "'input, ", "")
            };

        let has_members = !gathered.request_args.is_empty();
        let has_cows = gathered
            .request_args
            .iter()
            .any(|arg| arg.1.needs_any_cow());
        let members_start = if has_members { " {" } else { ";" };
        outln!(
            out,
            "pub struct {name}Request{lifetime}{members_start}",
            name = name,
            lifetime = struct_lifetime_block,
            members_start = members_start
        );
        out.indented(|out| {
            for (member_name, member_type) in gathered.request_args.iter() {
                outln!(out, "pub {name}: {type},",
                       name=member_name,
                       type=member_type.as_field());
            }
        });
        if has_members {
            outln!(out, "}}",);
        }

        // Methods implemented on every request
        outln!(
            out,
            "impl{lifetime} {name}Request{lifetime} {{",
            lifetime = struct_lifetime_block,
            name = name
        );
        out.indented(|out| {
            outln!(
                out,
                "/// Serialize this request into bytes for the provided connection",
            );
            outln!(
                out,
                "fn serialize<{lifetime}Conn>(self, conn: &Conn) -> \
                 Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>",
                lifetime = serialize_lifetime_block,
            );
            outln!(out, "where");
            outln!(out.indent(), "Conn: RequestConnection + ?Sized,");
            outln!(out, "{{");
            out.indented(|out| {
                if ns.ext_info.is_some() {
                    outln!(
                        out,
                        "let extension_information = \
                         conn.extension_information(X11_EXTENSION_NAME)?",
                    );
                    outln!(
                        out.indent(),
                        ".ok_or(ConnectionError::UnsupportedExtension)?;"
                    );
                } else {
                    // Silence a warning about an unused `conn`.
                    outln!(out, "let _ = conn;");
                }

                let fields = request_def.fields.borrow();

                let has_expr_fields = fields.iter().any(|field| {
                    if let xcbdefs::FieldDef::Expr(_) = field {
                        true
                    } else {
                        false
                    }
                });
                // Calculate `VirtualLen` field values because they
                // may be used by <exprfield>s.
                if has_expr_fields {
                    for field in fields.iter() {
                        if let xcbdefs::FieldDef::VirtualLen(virtual_len_field) = field {
                            outln!(
                                out,
                                "let {} = u32::try_from(self.{}.len()).unwrap();",
                                to_rust_variable_name(&virtual_len_field.name),
                                to_rust_variable_name(&virtual_len_field.list_name),
                            );
                        }
                    }
                }

                outln!(out, "let length_so_far = 0;");

                let mut request_size = fields
                    .iter()
                    .try_fold(0, |sum, field| Some(sum + field.size()?));

                let mut request_slices = Vec::new();
                let mut fixed_fields_bytes = Vec::new();
                let mut num_fixed_len_slices = 0;
                let mut pad_count = 0;

                for (field_i, field) in fields.iter().enumerate() {
                    let mut next_slice = None;

                    let mut tmp_out = Output::new();
                    self.emit_assert_for_field_serialize(
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if !deducible_fields.contains_key(field_name) {
                                format!("self.{}", rust_field_name)
                            } else {
                                rust_field_name
                            }
                        },
                        &mut tmp_out,
                    );
                    match field {
                        xcbdefs::FieldDef::Pad(pad_field) => match pad_field.kind {
                            xcbdefs::PadKind::Bytes(bytes) => {
                                for _ in 0..bytes {
                                    fixed_fields_bytes.push(String::from("0"));
                                }
                            }
                            xcbdefs::PadKind::Align(align) => {
                                outln!(
                                    tmp_out,
                                    "let padding{} = &[0; {}][..({} - (length_so_far % {})) % {}];",
                                    pad_count,
                                    align - 1,
                                    align,
                                    align,
                                    align,
                                );
                                next_slice =
                                    Some((format!("padding{}", pad_count), IovecConversion::Into));
                                pad_count += 1;
                            }
                        },
                        xcbdefs::FieldDef::Normal(normal_field) => {
                            if normal_field.name == "major_opcode" {
                                if ns.ext_info.is_some() {
                                    fixed_fields_bytes
                                        .push(String::from("extension_information.major_opcode"));
                                } else {
                                    fixed_fields_bytes.push(format!(
                                        "{}_REQUEST",
                                        super::camel_case_to_upper_snake(&name),
                                    ));
                                }
                            } else if normal_field.name == "minor_opcode" {
                                assert!(ns.ext_info.is_some());
                                fixed_fields_bytes.push(format!(
                                    "{}_REQUEST",
                                    super::camel_case_to_upper_snake(&name),
                                ));
                            } else if normal_field.name == "length" {
                                // the actual length will be calculated later
                                fixed_fields_bytes.push(String::from("0"));
                                fixed_fields_bytes.push(String::from("0"));
                            } else {
                                let rust_field_name = to_rust_variable_name(&normal_field.name);

                                let was_deduced = if let Some(deducible_field) =
                                    deducible_fields.get(&normal_field.name)
                                {
                                    self.emit_calc_deducible_field(
                                        field,
                                        deducible_field,
                                        |field_name| {
                                            let rust_field_name = to_rust_variable_name(field_name);
                                            if !deducible_fields.contains_key(field_name) {
                                                format!("self.{}", rust_field_name)
                                            } else {
                                                rust_field_name
                                            }
                                        },
                                        &rust_field_name,
                                        out,
                                    );
                                    true
                                } else {
                                    false
                                };

                                let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                                if let Some(field_size) = normal_field.type_.size() {
                                    let field_name = if was_deduced {
                                        // If we deduced this value it comes from a local.
                                        rust_field_name
                                    } else {
                                        // Otherwise a member.
                                        format!("self.{}", rust_field_name)
                                    };

                                    outln!(
                                        out,
                                        "let {} = {};",
                                        bytes_name,
                                        self.emit_value_serialize(
                                            &normal_field.type_,
                                            &field_name,
                                            was_deduced,
                                        ),
                                    );
                                    for i in 0..field_size {
                                        fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                                    }
                                } else {
                                    outln!(
                                        tmp_out,
                                        "let {} = self.{}.serialize();",
                                        bytes_name,
                                        rust_field_name,
                                    );
                                    next_slice = Some((bytes_name, IovecConversion::Into));
                                }
                            }
                        }
                        xcbdefs::FieldDef::List(list_field) => {
                            let rust_field_name = to_rust_variable_name(&list_field.name);
                            let list_length = list_field.length();
                            if self.rust_value_type_is_u8(&list_field.element_type) {
                                let conversion = if list_length.is_some() {
                                    // If this is a fixed length array we need to erase its length.
                                    IovecConversion::CowStripLength
                                } else {
                                    IovecConversion::None
                                };
                                next_slice =
                                    Some((format!("self.{}", rust_field_name), conversion));
                            } else {
                                assert_eq!(
                                    list_length, None,
                                    "fixed length arrays in requests must be [u8; N]"
                                );
                                if self.can_use_simple_list_parsing(&list_field.element_type) {
                                    let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                                    outln!(
                                        tmp_out,
                                        "let {} = self.{}.serialize();",
                                        bytes_name,
                                        rust_field_name,
                                    );
                                    next_slice = Some((bytes_name, IovecConversion::Into));
                                } else {
                                    let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                                    outln!(tmp_out, "let mut {} = Vec::new();", bytes_name);
                                    outln!(tmp_out, "for element in {}.iter() {{", rust_field_name);
                                    tmp_out.indented(|tmp_out| {
                                        self.emit_value_serialize_into(
                                            &list_field.element_type,
                                            "element",
                                            false,
                                            &bytes_name,
                                            tmp_out,
                                        );
                                    });
                                    outln!(tmp_out, "}}");
                                    next_slice = Some((bytes_name, IovecConversion::Into));
                                }
                            }
                        }
                        xcbdefs::FieldDef::Switch(switch_field) => {
                            let rust_field_name = to_rust_variable_name(&switch_field.name);
                            let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                            outln!(
                                tmp_out,
                                "let {} = self.{}.serialize({});",
                                bytes_name,
                                rust_field_name,
                                self.ext_params_to_call_args(
                                    false,
                                    |name| {
                                        if deducible_fields.get(name).is_some() {
                                            to_rust_variable_name(name)
                                        } else {
                                            format!("self.{}", to_rust_variable_name(name))
                                        }
                                    },
                                    &*switch_field.external_params.borrow(),
                                )
                            );
                            if let Some(field_size) = switch_field.size() {
                                for i in 0..field_size {
                                    fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                                }
                            } else {
                                next_slice = Some((bytes_name, IovecConversion::Into));
                            }
                        }
                        xcbdefs::FieldDef::Fd(_) => {}
                        xcbdefs::FieldDef::FdList(_) => {}
                        xcbdefs::FieldDef::Expr(expr_field) => {
                            let rust_field_name = to_rust_variable_name(&expr_field.name);
                            let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                            let type_ = self.field_value_type_to_rust_type(&expr_field.type_);
                            if type_ == "bool" {
                                outln!(
                                    out,
                                    "let {} = {} != 0;",
                                    rust_field_name,
                                    self.expr_to_str(
                                        &expr_field.expr,
                                        to_rust_variable_name,
                                        true,
                                        true,
                                        true,
                                    ),
                                );
                            } else {
                                // the only case found in the XML definitions is with a bool
                                unreachable!();
                            }
                            let field_size = expr_field.type_.size().unwrap();
                            outln!(
                                out,
                                "let {} = {};",
                                bytes_name,
                                self.emit_value_serialize(
                                    &expr_field.type_,
                                    &rust_field_name,
                                    false,
                                ),
                            );
                            for i in 0..field_size {
                                fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                            }
                        }
                        xcbdefs::FieldDef::VirtualLen(_) => {}
                    }

                    // The XML does not describe trailing padding in requests. Requests
                    // are implicitly padded to a four byte boundary.
                    if next_slice.is_none() && field_i == (fields.len() - 1) {
                        if let Some(ref mut request_size) = request_size {
                            let req_size_rem = *request_size % 4;
                            if req_size_rem != 0 {
                                let pad_size = 4 - req_size_rem;
                                for _ in 0..pad_size {
                                    fixed_fields_bytes.push(String::from("0"));
                                }
                                *request_size += pad_size;
                            }
                        }
                    }

                    if next_slice.is_some() || field_i == (fields.len() - 1) {
                        if !fixed_fields_bytes.is_empty() {
                            let maybe_mut = if num_fixed_len_slices == 0 {
                                // contains the length field, which will be modified
                                "mut "
                            } else {
                                ""
                            };
                            outln!(
                                out,
                                "let {}request{} = vec![",
                                maybe_mut,
                                num_fixed_len_slices,
                            );
                            for byte in fixed_fields_bytes.iter() {
                                outln!(out.indent(), "{},", byte);
                            }
                            outln!(out, "];");
                            outln!(
                                out,
                                "let length_so_far = length_so_far + request{}.len();",
                                num_fixed_len_slices,
                            );
                            request_slices.push(format!("request{}.into()", num_fixed_len_slices));
                            fixed_fields_bytes.clear();
                            num_fixed_len_slices += 1;
                        }
                        if let Some((next_slice, conversion)) = next_slice {
                            outln!(
                                tmp_out,
                                "let length_so_far = length_so_far + {}.len();",
                                next_slice,
                            );
                            match conversion {
                                IovecConversion::None => request_slices.push(next_slice),
                                IovecConversion::Into => {
                                    request_slices.push(format!("{}.into()", next_slice))
                                }
                                IovecConversion::CowStripLength => request_slices
                                    .push(format!("Cow::Owned({}.to_vec())", next_slice)),
                            }
                        }
                    }

                    out!(out, "{}", tmp_out.into_data());
                }
                // The XML does not describe trailing padding in requests. Requests
                // are implicitly padded to a four byte boundary.
                if let Some(request_size) = request_size {
                    let req_size_rem = request_size % 4;
                    if req_size_rem != 0 {
                        assert_eq!(pad_count, 0);
                        outln!(out, "let padding = [0; {}];", 4 - req_size_rem);
                        outln!(out, "let length_so_far = length_so_far + padding.len();");
                        request_slices.push(String::from("(&padding).into()"));
                    }
                } else {
                    outln!(
                        out,
                        "let padding{} = &[0; 3][..(4 - (length_so_far % 4)) % 4];",
                        pad_count,
                    );
                    outln!(
                        out,
                        "let length_so_far = length_so_far + padding{}.len();",
                        pad_count,
                    );
                    request_slices.push(format!("padding{}.into()", pad_count));
                }

                outln!(out, "assert_eq!(length_so_far % 4, 0);");
                // Set the length in the request.
                // If it does not fit into u16, compute_length_field will use BigRequests.
                outln!(
                    out,
                    "let length = u16::try_from(length_so_far / 4).unwrap_or(0);",
                );
                outln!(
                    out,
                    "request0[2..4].copy_from_slice(&length.to_ne_bytes());",
                );

                let fds_arg = if gathered.fd_lists.is_empty() {
                    format!(
                        "vec![{}]",
                        gathered
                            .single_fds
                            .iter()
                            .enumerate()
                            .map(|(i, single_fd)| {
                                let sep = if i == 0 { "" } else { ", " };
                                format!("{}self.{}", sep, single_fd)
                            })
                            .collect::<String>(),
                    )
                } else if gathered.fd_lists.len() == 1 && gathered.single_fds.is_empty() {
                    format!("self.{}", gathered.fd_lists[0])
                } else {
                    outln!(out, "let mut fds = Vec::new();");
                    for field in fields.iter() {
                        match field {
                            xcbdefs::FieldDef::Fd(fd_field) => {
                                outln!(out, "fds.push({});", to_rust_variable_name(&fd_field.name));
                            }
                            xcbdefs::FieldDef::FdList(fd_list_field) => {
                                outln!(
                                    out,
                                    "fds.extend({});",
                                    to_rust_variable_name(&fd_list_field.name)
                                );
                            }
                            _ => {}
                        }
                    }
                    String::from("fds")
                };

                let mut slices_arg = String::new();
                for (i, request_slices) in request_slices.iter().enumerate() {
                    if i != 0 {
                        slices_arg.push_str(", ");
                    }
                    slices_arg.push_str(request_slices);
                }

                outln!(
                    out,
                    "Ok((vec![{slices}], {fds}))",
                    slices = slices_arg,
                    fds = fds_arg,
                );
            });
            outln!(out, "}}");

            // Sending method
            let is_list_fonts_with_info =
                request_def.name == "ListFontsWithInfo" && ns.header == "xproto";
            let is_record_enable_context =
                request_def.name == "EnableContext" && ns.header == "record";
            let ret_type = if is_list_fonts_with_info || is_record_enable_context {
                assert!(request_def.reply.is_some());
                match (is_list_fonts_with_info, is_record_enable_context) {
                    (true, _) => "ListFontsWithInfoCookie<'_, Conn>".to_string(),
                    (_, true) => "RecordEnableContextCookie<'_, Conn>".to_string(),
                    _ => unreachable!(),
                }
            } else {
                match (request_def.reply.is_some(), gathered.reply_has_fds) {
                    (false, _) => "VoidCookie<'_, Conn>".to_string(),
                    (true, false) => format!("Cookie<'_, Conn, {}Reply>", name),
                    (true, true) => format!("CookieWithFds<'_, Conn, {}Reply>", name),
                }
            };
            outln!(
                out,
                "pub fn send<Conn>(self, conn: &Conn) -> Result<{}, ConnectionError>",
                ret_type,
            );
            outln!(out, "where");
            outln!(out.indent(), "Conn: RequestConnection + ?Sized,");
            outln!(out, "{{");
            out.indented(|out| {
                outln!(out, "let (bytes, fds) = self.serialize(conn)?;");
                outln!(
                    out,
                    "let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();"
                );

                if is_list_fonts_with_info || is_record_enable_context {
                    let cookie = match (is_list_fonts_with_info, is_record_enable_context) {
                        (true, _) => "ListFontsWithInfoCookie",
                        (_, true) => "RecordEnableContextCookie",
                        _ => unreachable!(),
                    };
                    outln!(
                        out,
                        "Ok({}::new(conn.send_request_with_reply(&slices, fds)?))",
                        cookie,
                    )
                } else if request_def.reply.is_some() {
                    if gathered.reply_has_fds {
                        outln!(
                            out,
                            "Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)",
                        );
                    } else {
                        outln!(out, "Ok(conn.send_request_with_reply(&slices, fds)?)",);
                    }
                } else {
                    outln!(out, "Ok(conn.send_request_without_reply(&slices, fds)?)",);
                }
            });
            outln!(out, "}}");

            // Parsing implementation.
            outln!(
                out,
                "/// Parse this request given its header, its body, and any fds that go along \
                 with it"
            );
            if gathered.has_fds() {
                outln!(
                    out,
                    "pub fn try_parse_request_fd(header: RequestHeader, value: &{lifetime}[u8], \
                     fds: &mut Vec<RawFdContainer>) -> Result<Self, ParseError> {{",
                    lifetime = parse_lifetime_block,
                );
            } else {
                outln!(
                    out,
                    "pub fn try_parse_request(header: RequestHeader, value: &{lifetime}[u8]) -> \
                     Result<Self, ParseError> {{",
                    lifetime = parse_lifetime_block,
                );
            }
            out.indented(|out| {
                if ns.ext_info.is_some() {
                    // We don't have enough information here to validate the major opcode, that requires
                    // state from the connection.
                    outln!(
                        out,
                        "if header.minor_opcode != {}_REQUEST {{",
                        super::camel_case_to_upper_snake(&name),
                    );
                    outln!(out.indent(), "return Err(ParseError::InvalidValue);");
                    outln!(out, "}}");
                } else {
                    // The minor opcode could be repurposed to store data for this request, so there's
                    // no validation of it to be done.
                    outln!(
                        out,
                        "if header.major_opcode != {}_REQUEST {{",
                        super::camel_case_to_upper_snake(&name),
                    );
                    outln!(out.indent(), "return Err(ParseError::InvalidValue);");
                    outln!(out, "}}");
                };

                let fields = request_def.fields.borrow();
                let mut seen_complete_header = false;
                let mut is_first_body_field = true;
                for (_, field) in fields.iter().enumerate() {
                    match field.name() {
                        // These are all in the header. Ignore them.
                        Some("major_opcode") | Some("minor_opcode") => continue,
                        Some("length") => {
                            seen_complete_header = true;
                            continue;
                        }
                        _ => (),
                    }

                    let from = if !seen_complete_header {
                        assert_eq!(field.size(), Some(1));
                        outln!(out, "let remaining = &[header.minor_opcode];");
                        "remaining"
                    } else if is_first_body_field {
                        is_first_body_field = false;
                        "value"
                    } else {
                        "remaining"
                    };
                    self.emit_field_parse(
                        field,
                        "",
                        from,
                        FieldContainer::Request(name.to_string()),
                        out,
                    );
                    if !field
                        .name()
                        .map(|field_name| deducible_fields.contains_key(field_name))
                        .unwrap_or(false)
                    {
                        self.emit_field_post_parse(field, out);
                    }

                    if !seen_complete_header {
                        // Dispose of the "remaining" variable just generated.
                        outln!(out, "let _ = remaining;");
                    }
                }

                // Silence unused variable warnings for the last component.
                if !is_first_body_field {
                    outln!(out, "let _ = remaining;");
                } else {
                    outln!(out, "let _ = value;");
                }

                // If this is QueryTextExtents we need special handling for odd_length.
                if ns.header == "xproto" && name == "QueryTextExtents" {
                    outln!(out, "if odd_length {{");
                    out.indented(|out| {
                        outln!(out, "if string.is_empty() {{");
                        outln!(out.indent(), "return Err(ParseError::InvalidValue);");
                        outln!(out, "}}");
                        outln!(out, "string.truncate(string.len() - 1);");
                    });
                    outln!(out, "}}");
                }

                let has_members = !gathered.request_args.is_empty();
                let members_start = if has_members { " {" } else { "" };
                outln!(out, "Ok({}Request{}", name, members_start);
                out.indented(|out| {
                    for (arg_name, arg_type) in gathered.request_args.iter() {
                        if arg_type.needs_owned_cow() {
                            // Types that are parsed from raw bytes must become
                            // Cow::Owned here because they don't exist outside
                            // of this function, so we cannot return references
                            // to them.
                            outln!(out, "{name}: Cow::Owned({name}),", name = arg_name);
                        } else if arg_type.needs_borrowed_cow() {
                            // But raw buffers of bytes can become Cow::Borrowed,
                            // because the original buffer of bytes still exists
                            // after we return.
                            outln!(out, "{name}: Cow::Borrowed({name}),", name = arg_name);
                        } else {
                            outln!(out, "{name},", name = arg_name);
                        }
                    }
                });
                let members_end = if has_members { "}" } else { "" };
                outln!(out, "{})", members_end);
            });
            outln!(out, "}}");
            if has_cows {
                outln!(out, "/// Clone all borrowed data in this {}Request.", name);
                outln!(
                    out,
                    "pub fn into_owned(self) -> {}Request<'static> {{",
                    name
                );
                out.indented(|out| {
                    outln!(out, "{}Request {{", name);
                    out.indented(|out| {
                        for (arg_name, arg_type) in gathered.args.iter() {
                            if arg_type.needs_any_cow() || is_send_event && arg_name == "event" {
                                outln!(
                                    out,
                                    "{name}: Cow::Owned(self.{name}.into_owned()),",
                                    name = arg_name
                                );
                            } else {
                                outln!(out, "{name}: self.{name},", name = arg_name);
                            }
                        }
                    });
                    outln!(out, "}}");
                });
                outln!(out, "}}");
            }
        });
        outln!(out, "}}");
        outln!(
            out,
            "impl{lifetime} Request for {name}Request{lifetime} {{",
            lifetime = struct_lifetime_block,
            name = name
        );
        if request_def.reply.is_some() {
            outln!(out.indent(), "type Reply = {}Reply;", name);
        } else {
            outln!(out.indent(), "type Reply = ();");
        }
        outln!(out, "}}");
    }

    fn emit_request_function(
        &self,
        request_def: &xcbdefs::RequestDef,
        name: &str,
        function_name: &str,
        gathered: &GatheredRequestFields,
        out: &mut Output,
    ) {
        let ns = request_def.namespace.upgrade().unwrap();
        let is_list_fonts_with_info =
            request_def.name == "ListFontsWithInfo" && ns.header == "xproto";
        let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";
        let is_record_enable_context = request_def.name == "EnableContext" && ns.header == "record";

        let needs_lifetime = gathered.needs_lifetime && !is_send_event;

        let mut generic_params = String::new();
        if needs_lifetime {
            generic_params.push_str("'c, 'input, Conn");
        } else {
            generic_params.push_str("Conn");
        }
        for (param_name, _) in gathered.generics.iter() {
            generic_params.push_str(", ");
            generic_params.push_str(param_name);
        }

        let ret_lifetime = if needs_lifetime { "'c" } else { "'_" };
        let ret_type = if is_list_fonts_with_info || is_record_enable_context {
            assert!(request_def.reply.is_some());
            assert!(!gathered.reply_has_fds);
            if is_list_fonts_with_info {
                format!("ListFontsWithInfoCookie<{}, Conn>", ret_lifetime)
            } else {
                format!("RecordEnableContextCookie<{}, Conn>", ret_lifetime)
            }
        } else {
            match (request_def.reply.is_some(), gathered.reply_has_fds) {
                (false, _) => format!("VoidCookie<{}, Conn>", ret_lifetime),
                (true, false) => format!("Cookie<{}, Conn, {}Reply>", ret_lifetime, name),
                (true, true) => format!("CookieWithFds<{}, Conn, {}Reply>", ret_lifetime, name),
            }
        };

        let mut args = String::new();
        if needs_lifetime {
            args.push_str("conn: &'c Conn");
        } else {
            args.push_str("conn: &Conn");
        }
        for (arg_name, arg_type) in gathered.args.iter() {
            args.push_str(", ");
            args.push_str(arg_name);
            args.push_str(": ");
            args.push_str(&arg_type.as_argument());
        }

        if let Some(ref doc) = request_def.doc {
            self.emit_doc(doc, out);
        }
        outln!(
            out,
            "pub fn {}<{}>({}) -> Result<{}, ConnectionError>",
            function_name,
            generic_params,
            args,
            ret_type,
        );
        outln!(out, "where");
        outln!(out.indent(), "Conn: RequestConnection + ?Sized,");
        for (param_name, where_) in gathered.generics.iter() {
            outln!(out.indent(), "{}: {},", param_name, where_);
        }
        outln!(out, "{{");
        #[allow(clippy::cognitive_complexity)]
        out.indented(|out| {
            for preamble in gathered.preamble.iter() {
                outln!(out, "{}", preamble);
            }

            let has_members = !gathered.request_args.is_empty();
            let members_start = if has_members { " {" } else { ";" };
            outln!(out, "let request0 = {}Request{}", name, members_start);
            out.indented(|out| {
                for (arg_name, arg_type) in gathered.args.iter() {
                    if arg_type.needs_any_cow() {
                        // Because the argument is passed in from outside this function,
                        // it is always ok to use a Cow::Borrowed here.
                        outln!(out, "{name}: Cow::Borrowed({name}),", name = arg_name);
                    } else {
                        outln!(out, "{name},", name = arg_name);
                    }
                }
            });
            if has_members {
                outln!(out, "}};");
            }

            outln!(out, "request0.send(conn)")
        });
        outln!(out, "}}");
    }

    fn emit_request_trait_function(
        &self,
        request_def: &xcbdefs::RequestDef,
        name: &str,
        function_name: &str,
        gathered: &GatheredRequestFields,
        out: &mut Output,
    ) {
        let ns = request_def.namespace.upgrade().unwrap();
        let is_list_fonts_with_info =
            request_def.name == "ListFontsWithInfo" && ns.header == "xproto";
        let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";
        let is_record_enable_context = request_def.name == "EnableContext" && ns.header == "record";
        let needs_lifetime = gathered.needs_lifetime && !is_send_event;

        let mut generic_params = String::new();
        if needs_lifetime || !gathered.generics.is_empty() {
            generic_params.push('<');
            if needs_lifetime {
                generic_params.push_str("'c, 'input");
            }
            for (i, (param_name, _)) in gathered.generics.iter().enumerate() {
                if i != 0 || needs_lifetime {
                    generic_params.push_str(", ");
                }
                generic_params.push_str(param_name);
            }
            generic_params.push('>');
        }

        let ret_lifetime = if needs_lifetime { "'c" } else { "'_" };
        let ret_type = if is_list_fonts_with_info || is_record_enable_context {
            assert!(request_def.reply.is_some());
            assert!(!gathered.reply_has_fds);
            if is_list_fonts_with_info {
                format!("ListFontsWithInfoCookie<{}, Self>", ret_lifetime)
            } else {
                format!("RecordEnableContextCookie<{}, Self>", ret_lifetime)
            }
        } else {
            match (request_def.reply.is_some(), gathered.reply_has_fds) {
                (false, _) => format!("VoidCookie<{}, Self>", ret_lifetime),
                (true, false) => format!("Cookie<{}, Self, {}Reply>", ret_lifetime, name),
                (true, true) => format!("CookieWithFds<{}, Self, {}Reply>", ret_lifetime, name),
            }
        };

        let mut args = String::new();
        if needs_lifetime {
            args.push_str("&'c self");
        } else {
            args.push_str("&self");
        }
        for (arg_name, arg_type) in gathered.args.iter() {
            args.push_str(", ");
            args.push_str(arg_name);
            args.push_str(": ");
            args.push_str(&arg_type.as_argument());
        }

        let func_name_prefix = if ns.ext_info.is_some() {
            format!("{}_", ns.header)
        } else {
            String::new()
        };

        if let Some(ref doc) = request_def.doc {
            self.emit_doc(doc, out);
        }
        outln!(
            out,
            "fn {}{}{}({}) -> Result<{}, ConnectionError>",
            func_name_prefix,
            function_name,
            generic_params,
            args,
            ret_type,
        );
        if !gathered.generics.is_empty() {
            outln!(out, "where");
            for (param_name, where_) in gathered.generics.iter() {
                outln!(out.indent(), "{}: {},", param_name, where_);
            }
        }
        outln!(out, "{{");

        let mut call_args = String::from("self");
        for (arg_name, _) in gathered.args.iter() {
            call_args.push_str(", ");
            call_args.push_str(arg_name);
        }

        let func_name_same_as_field_name = request_def
            .fields
            .borrow()
            .iter()
            .any(|field| field.name() == Some(function_name));
        if func_name_same_as_field_name {
            outln!(out.indent(), "self::{}({})", function_name, call_args);
        } else {
            outln!(out.indent(), "{}({})", function_name, call_args);
        }

        outln!(out, "}}");
    }

    fn generate_event_full_def(&self, event_full_def: &xcbdefs::EventFullDef, out: &mut Output) {
        let name = to_rust_type_name(&event_full_def.name);
        self.emit_event(&name, event_full_def.number, event_full_def, out);
    }

    fn generate_event_copy_def(&self, event_copy_def: &xcbdefs::EventCopyDef, out: &mut Output) {
        let name = to_rust_type_name(&event_copy_def.name);
        let event_full_def = event_copy_def.get_original_full_def();
        self.emit_event_opcode(&name, event_copy_def.number, &event_full_def, out);
        outln!(
            out,
            "pub type {}Event = {};",
            name,
            self.event_to_rust_type(event_copy_def.ref_.get_resolved()),
        );
        outln!(out, "");
    }

    fn emit_event(
        &self,
        name: &str,
        number: u16,
        event_full_def: &xcbdefs::EventFullDef,
        out: &mut Output,
    ) {
        let size_constraint = if event_full_def.xge {
            StructSizeConstraint::EmbeddedLength { minimum: 32 }
        } else {
            StructSizeConstraint::Fixed(32)
        };
        self.emit_event_opcode(name, number, &event_full_def, out);

        let full_name = format!("{}Event", name);

        let fields = event_full_def.fields.borrow();
        let mut derives = Derives::all();
        self.filter_derives_for_fields(&mut derives, &*fields, false);

        self.emit_struct_type(
            &full_name,
            name,
            derives,
            &*fields,
            &[],
            false,
            true,
            size_constraint,
            false,
            true,
            event_full_def.doc.as_ref(),
            out,
        );

        if !event_full_def.xge {
            let deducible_fields = gather_deducible_fields(&*fields);
            self.emit_event_or_error_serialize(&full_name, &*fields, &deducible_fields, out);
        }

        outln!(out, "");
    }

    fn emit_event_opcode(
        &self,
        name: &str,
        number: u16,
        event_full_def: &xcbdefs::EventFullDef,
        out: &mut Output,
    ) {
        // The opcode specified for GeGeneric in xproto (35) is the value
        // in the 8-bit response_type field
        let opcode_type =
            if event_full_def.xge && (self.ns.header != "xproto" || name != "GeGeneric") {
                "u16"
            } else {
                "u8"
            };

        outln!(out, "/// Opcode for the {} event", name);
        outln!(
            out,
            "pub const {}_EVENT: {} = {};",
            super::camel_case_to_upper_snake(name),
            opcode_type,
            number,
        );
    }

    fn generate_error_full_def(&self, error_full_def: &xcbdefs::ErrorFullDef, out: &mut Output) {
        let name = to_rust_type_name(&error_full_def.name);
        self.emit_error(&name, error_full_def.number, out);
    }

    fn generate_error_copy_def(&self, error_copy_def: &xcbdefs::ErrorCopyDef, out: &mut Output) {
        let name = to_rust_type_name(&error_copy_def.name);
        self.emit_error_opcode(&name, error_copy_def.number, out);
        outln!(out, "");
    }

    fn emit_error(&self, name: &str, number: i16, out: &mut Output) {
        self.emit_error_opcode(name, number, out);
        outln!(out, "");
    }

    fn emit_error_opcode(&self, name: &str, number: i16, out: &mut Output) {
        // The XML has a comment saying "fake number"
        let emit_opcode = self.ns.header != "glx" || name != "Generic";
        if emit_opcode {
            outln!(out, "/// Opcode for the {} error", name);
            outln!(
                out,
                "pub const {}_ERROR: u8 = {};",
                super::camel_case_to_upper_snake(name),
                number,
            );
        }
    }

    fn emit_event_or_error_serialize(
        &self,
        name: &str,
        fields: &[xcbdefs::FieldDef],
        deducible_fields: &HashMap<String, DeducibleField>,
        out: &mut Output,
    ) {
        outln!(out, "impl From<&{}> for [u8; 32] {{", name);
        out.indented(|out| {
            outln!(out, "fn from(input: &{}) -> Self {{", name);
            out.indented(|out| {
                // This gathers the bytes of the result
                let mut result_bytes = Vec::new();
                for field in fields.iter() {
                    self.emit_field_serialize(
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if !deducible_fields.contains_key(field_name) {
                                format!("input.{}", rust_field_name)
                            } else {
                                rust_field_name
                            }
                        },
                        &mut result_bytes,
                        out,
                    );
                }
                assert!(result_bytes.len() <= 32);
                outln!(out, "[");
                for result_byte in result_bytes.iter() {
                    outln!(out.indent(), "{},", result_byte);
                }
                if result_bytes.len() != 32 {
                    outln!(out.indent(), "// trailing padding");
                    for _ in result_bytes.len()..32 {
                        outln!(out.indent(), "0,");
                    }
                }
                outln!(out, "]");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");

        outln!(out, "impl From<{}> for [u8; 32] {{", name);
        out.indented(|out| {
            outln!(out, "fn from(input: {}) -> Self {{", name);
            outln!(out.indent(), "Self::from(&input)");
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }

    fn generate_struct_def(&self, struct_def: &xcbdefs::StructDef, out: &mut Output) {
        let struct_align = struct_def.alignment.get().unwrap();
        // serializing / parsing wouldn't work correctly otherwise
        assert!(struct_align.internal_align <= struct_align.begin.align());
        assert_eq!(struct_align.begin.offset() % struct_align.internal_align, 0);

        let rust_name = self.get_struct_rust_name(struct_def);
        let derives = self.get_derives_for_struct_def(struct_def);
        self.emit_struct_type(
            &rust_name,
            &rust_name,
            derives,
            &*struct_def.fields.borrow(),
            &*struct_def.external_params.borrow(),
            false,
            true,
            StructSizeConstraint::None,
            true,
            true,
            None,
            out,
        );

        outln!(out, "");
    }

    fn generate_union_def(&self, union_def: &xcbdefs::UnionDef, out: &mut Output) {
        let union_align = union_def.alignment.get().unwrap();
        // serializing / parsing wouldn't work correctly otherwise
        assert_eq!(union_align.internal_align, 1);

        let rust_name = self.get_union_rust_name(union_def);
        let union_size = union_def.size();

        outln!(out, "#[derive(Debug, Copy, Clone)]");
        outln!(out, "pub struct {}([u8; {}]);", rust_name, union_size);

        let fields = union_def.fields.as_slice();

        outln!(out, "impl {} {{", rust_name);
        out.indented(|out| {
            for field in fields.iter() {
                let rust_field_type = self.field_to_rust_type(field, &rust_name);
                let rust_field_name = to_rust_variable_name(field.name().unwrap());
                outln!(
                    out,
                    "pub fn {}(&self) -> {} {{",
                    prefix_var_name(&rust_field_name, "as"),
                    rust_field_type,
                );
                out.indented(|out| {
                    outln!(
                        out,
                        "fn do_the_parse(remaining: &[u8]) -> Result<{}, ParseError> {{",
                        rust_field_type,
                    );
                    out.indented(|out| {
                        self.emit_field_parse(
                            field,
                            &rust_name,
                            "remaining",
                            FieldContainer::Other,
                            out,
                        );
                        self.emit_field_post_parse(field, out);
                        outln!(out, "let _ = remaining;");
                        outln!(out, "Ok({})", rust_field_name);
                    });
                    outln!(out, "}}");
                    outln!(out, "do_the_parse(&self.0).unwrap()");
                });
                outln!(out, "}}");
            }
        });
        outln!(out, "}}");

        outln!(out, "impl Serialize for {} {{", rust_name);
        out.indented(|out| {
            outln!(out, "type Bytes = [u8; {}];", union_size);
            outln!(out, "fn serialize(&self) -> [u8; {}] {{", union_size);
            outln!(out.indent(), "self.0");
            outln!(out, "}}");
            outln!(out, "fn serialize_into(&self, bytes: &mut Vec<u8>) {{");
            outln!(out.indent(), "bytes.extend_from_slice(&self.0);");
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(out, "impl TryParse for {} {{", rust_name);
        out.indented(|out| {
            outln!(
                out,
                "fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {{",
            );
            out.indented(|out| {
                outln!(
                    out,
                    "let inner: [u8; {}] = value.get(..{})",
                    union_size,
                    union_size,
                );
                outln!(out.indent(), ".ok_or(ParseError::InsufficientData)?");
                outln!(out.indent(), ".try_into()");
                outln!(out.indent(), ".unwrap();");
                outln!(out, "let result = {}(inner);", rust_name);
                outln!(out, "Ok((result, &value[{}..]))", union_size);
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");

        let mut seen_field_types = HashSet::new();

        for field in fields.iter() {
            // Get the original type (without type aliases)
            // to make sure there are not repeated `From`s.
            let orig_rust_field_type = match field {
                xcbdefs::FieldDef::Normal(normal_field) => {
                    if let Some(enum_def) = self.use_enum_type_in_field(&normal_field.type_) {
                        let ns = enum_def.namespace.upgrade().unwrap();
                        self.type_name_to_rust_type(&self.get_enum_rust_name(&enum_def), &ns)
                    } else {
                        self.type_to_rust_type(
                            &normal_field.type_.type_.get_resolved().get_original_type(),
                        )
                    }
                }
                xcbdefs::FieldDef::List(list_field) => {
                    let element_type = if let Some(enum_def) =
                        self.use_enum_type_in_field(&list_field.element_type)
                    {
                        let ns = enum_def.namespace.upgrade().unwrap();
                        self.type_name_to_rust_type(&self.get_enum_rust_name(&enum_def), &ns)
                    } else {
                        self.type_to_rust_type(
                            &list_field
                                .element_type
                                .type_
                                .get_resolved()
                                .get_original_type(),
                        )
                    };
                    if let Some(list_length) = list_field.length() {
                        format!("[{}; {}]", element_type, list_length)
                    } else {
                        unreachable!();
                    }
                }
                _ => unreachable!(),
            };
            if !seen_field_types.insert(orig_rust_field_type) {
                // `From` already implemented for this type
                continue;
            }

            let rust_field_type = self.field_to_rust_type(field, &rust_name);
            let rust_field_name = to_rust_variable_name(field.name().unwrap());
            outln!(out, "impl From<{}> for {} {{", rust_field_type, rust_name);
            out.indented(|out| {
                outln!(
                    out,
                    "fn from({}: {}) -> Self {{",
                    rust_field_name,
                    rust_field_type,
                );
                let mut result_bytes = Vec::new();
                out.indented(|out| {
                    let bytes_name = self.emit_field_serialize(
                        field,
                        &HashMap::new(),
                        |field| to_rust_variable_name(field),
                        &mut result_bytes,
                        out,
                    );
                    let field_size = field.size().unwrap();
                    if bytes_name.is_some() && field_size == union_size {
                        outln!(out, "Self({})", bytes_name.unwrap());
                    } else {
                        outln!(out, "let value = [");
                        for result_byte in result_bytes.iter() {
                            outln!(out.indent(), "{},", result_byte);
                        }
                        // This is needed to handle cases such as Behavior.type or
                        // Action.type from the XKB extension.
                        //
                        // FIXME: For those cases it might be better to omit the
                        // serialize implementation.
                        if field_size != union_size {
                            for _ in 0..(union_size - field_size) {
                                outln!(out.indent(), "0,");
                            }
                        }
                        outln!(out, "];");
                        outln!(out, "Self(value)");
                    }
                });
                outln!(out, "}}");
            });
            outln!(out, "}}");
        }

        outln!(out, "");
    }

    fn generate_event_struct_def(
        &self,
        event_struct_def: &xcbdefs::EventStructDef,
        out: &mut Output,
    ) {
        let rust_name = self.get_event_struct_rust_name(event_struct_def);

        for allowed in event_struct_def.alloweds.iter() {
            assert!(!allowed.xge, "xge in eventstruct not supported");
        }

        outln!(out, "#[derive(Debug, Copy, Clone)]");
        outln!(out, "pub struct {}([u8; 32]);", rust_name);

        outln!(out, "impl {} {{", rust_name);
        out.indented(|out| {
            for allowed in event_struct_def.alloweds.iter() {
                for event in allowed.resolved.borrow().iter() {
                    let rust_event_name =
                        format!("{}Event", to_rust_type_name(event.as_event_def().name()));
                    let rust_event_type = self.event_to_rust_type(event);
                    outln!(
                        out,
                        "pub fn {}(&self) -> {} {{",
                        prefix_var_name(&to_rust_variable_name(&rust_event_name), "as"),
                        rust_event_type,
                    );
                    // coerce `&[u8; 32]` to `&[u8]`
                    outln!(out.indent(), "let value: &[u8] = &self.0;");
                    outln!(out.indent(), "// FIXME: event parsing can fail");
                    outln!(
                        out.indent(),
                        "{}::try_from(value).unwrap()",
                        rust_event_type,
                    );
                    outln!(out, "}}");
                }
            }
        });
        outln!(out, "}}");

        outln!(out, "impl Serialize for {} {{", rust_name);
        out.indented(|out| {
            outln!(out, "type Bytes = [u8; 32];");
            outln!(out, "fn serialize(&self) -> [u8; 32] {{");
            outln!(out.indent(), "self.0");
            outln!(out, "}}");
            outln!(out, "fn serialize_into(&self, bytes: &mut Vec<u8>) {{");
            outln!(out.indent(), "bytes.extend_from_slice(&self.0);");
            outln!(out, "}}");
        });
        outln!(out, "}}");

        outln!(out, "impl TryParse for {} {{", rust_name);
        out.indented(|out| {
            outln!(
                out,
                "fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {{",
            );
            out.indented(|out| {
                outln!(out, "let inner: [u8; 32] = value.get(..32)");
                outln!(out.indent(), ".ok_or(ParseError::InsufficientData)?");
                outln!(out.indent(), ".try_into()");
                outln!(out.indent(), ".unwrap();");
                outln!(out, "let result = {}(inner);", rust_name);
                outln!(out, "Ok((result, &value[32..]))");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");

        let mut events_with_from = HashSet::new();

        for allowed in event_struct_def.alloweds.iter() {
            for event in allowed.resolved.borrow().iter() {
                let rust_original_event_type = self.event_def_to_rust_type(
                    &xcbdefs::EventDef::Full(event.get_original_full_def()),
                );
                if !events_with_from.insert(rust_original_event_type) {
                    // Avoid conflicting `From` implementations.
                    continue;
                }
                let rust_event_type = self.event_to_rust_type(event);
                outln!(out, "impl From<{}> for {} {{", rust_event_type, rust_name);
                out.indented(|out| {
                    outln!(out, "fn from(value: {}) -> Self {{", rust_event_type);
                    outln!(out.indent(), "Self(<[u8; 32]>::from(value))");
                    outln!(out, "}}");
                });
                outln!(out, "}}");
                outln!(out, "impl From<&{}> for {} {{", rust_event_type, rust_name);
                out.indented(|out| {
                    outln!(out, "fn from(value: &{}) -> Self {{", rust_event_type);
                    outln!(out.indent(), "Self(<[u8; 32]>::from(value))");
                    outln!(out, "}}");
                });
                outln!(out, "}}");
            }
        }

        outln!(out, "");
    }

    fn generate_xid_type_def(&self, xid_type_def: &xcbdefs::XidTypeDef, out: &mut Output) {
        let rust_name = self.get_xid_type_rust_name(xid_type_def);
        outln!(out, "pub type {} = u32;", rust_name);
        outln!(out, "");
    }

    fn generate_xid_union_def(&self, xid_union_def: &xcbdefs::XidUnionDef, out: &mut Output) {
        let rust_name = self.get_xid_union_rust_name(xid_union_def);
        outln!(out, "pub type {} = u32;", rust_name);
        outln!(out, "");
    }

    fn generate_enum_def(&self, enum_def: &xcbdefs::EnumDef, out: &mut Output) {
        let rust_name = self.get_enum_rust_name(enum_def);

        let enum_info = self.caches.borrow().enum_info(enum_def);
        let max_value_size = enum_info.max_value_size.unwrap();
        let global_enum_size = max_value_size.max(enum_info.wire_size.unwrap_or((0, 0)).1);

        if let Some((min_wire_size, max_wire_size)) = enum_info.wire_size {
            assert!(max_wire_size >= global_enum_size);
            if min_wire_size != max_wire_size {
                // if it is bool, it is always bool
                assert_ne!(min_wire_size, 1);
            }
        }

        let (raw_type, smaller_types, larger_types): (&str, &[&str], &[&str]) =
            match global_enum_size {
                1 => ("bool", &[], &["u8", "u16", "u32"]),
                8 => ("u8", &[], &["u16", "u32"]),
                16 => ("u16", &["u8"], &["u32"]),
                32 => ("u32", &["u8", "u16"], &[]),
                _ => unreachable!(),
            };

        if let Some(ref doc) = enum_def.doc {
            self.emit_doc(doc, out);
        }

        outln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        outln!(out, "pub struct {}({});", rust_name, raw_type);

        outln!(out, "impl {} {{", rust_name);
        for enum_item in enum_def.items.iter() {
            let rust_item_name = ename_to_rust(&enum_item.name);
            if global_enum_size != 1 {
                match enum_item.value {
                    xcbdefs::EnumValue::Value(value) => {
                        outln!(
                            out.indent(),
                            "pub const {}: Self = Self({});",
                            rust_item_name,
                            format_literal_integer(value),
                        );
                    }
                    xcbdefs::EnumValue::Bit(bit) => {
                        outln!(
                            out.indent(),
                            "pub const {}: Self = Self(1 << {});",
                            rust_item_name,
                            bit,
                        );
                    }
                }
            } else {
                let value = match enum_item.value {
                    xcbdefs::EnumValue::Value(value) => value,
                    xcbdefs::EnumValue::Bit(bit) => 1 << bit,
                };
                outln!(
                    out.indent(),
                    "pub const {}: Self = Self({});",
                    rust_item_name,
                    if value == 0 { "false" } else { "true" },
                );
            }
        }
        outln!(out, "}}");

        outln!(out, "impl From<{}> for {} {{", rust_name, raw_type);
        out.indented(|out| {
            outln!(out, "#[inline]");
            outln!(out, "fn from(input: {}) -> Self {{", rust_name);
            outln!(out.indent(), "input.0");
            outln!(out, "}}");
        });
        outln!(out, "}}");

        outln!(
            out,
            "impl From<{}> for {}<{}> {{",
            rust_name,
            self.option_name,
            raw_type,
        );
        out.indented(|out| {
            outln!(out, "#[inline]");
            outln!(out, "fn from(input: {}) -> Self {{", rust_name);
            outln!(out.indent(), "Some(input.0)");
            outln!(out, "}}");
        });
        outln!(out, "}}");

        for larger_type in larger_types.iter() {
            outln!(out, "impl From<{}> for {} {{", rust_name, larger_type);
            out.indented(|out| {
                outln!(out, "#[inline]");
                outln!(out, "fn from(input: {}) -> Self {{", rust_name);
                outln!(out.indent(), "{}::from(input.0)", larger_type);
                outln!(out, "}}");
            });
            outln!(out, "}}");
            outln!(
                out,
                "impl From<{}> for {}<{}> {{",
                rust_name,
                self.option_name,
                larger_type,
            );
            out.indented(|out| {
                outln!(out, "#[inline]");
                outln!(out, "fn from(input: {}) -> Self {{", rust_name);
                outln!(out.indent(), "Some({}::from(input.0))", larger_type);
                outln!(out, "}}");
            });
            outln!(out, "}}");
        }

        for smaller_type in smaller_types.iter() {
            outln!(out, "impl From<{}> for {} {{", smaller_type, rust_name);
            out.indented(|out| {
                outln!(out, "#[inline]");
                outln!(out, "fn from(value: {}) -> Self {{", smaller_type);
                outln!(out.indent(), "Self(value.into())");
                outln!(out, "}}");
            });
            outln!(out, "}}");
        }

        outln!(out, "impl From<{}> for {} {{", raw_type, rust_name);
        out.indented(|out| {
            outln!(out, "#[inline]");
            outln!(out, "fn from(value: {}) -> Self {{", raw_type);
            outln!(out.indent(), "Self(value)");
            outln!(out, "}}");
        });
        outln!(out, "}}");

        // An enum is ok for bitmask if all its values are <bit>
        // or have value zero (but not if all values are zero)
        let ok_for_bitmask = enum_def
            .items
            .iter()
            .all(|enum_item| match enum_item.value {
                xcbdefs::EnumValue::Value(0) | xcbdefs::EnumValue::Bit(_) => true,
                _ => false,
            })
            && enum_def
                .items
                .iter()
                .any(|enum_item| match enum_item.value {
                    xcbdefs::EnumValue::Value(_) => false,
                    xcbdefs::EnumValue::Bit(_) => true,
                });

        if ok_for_bitmask {
            outln!(out, "bitmask_binop!({}, {});", rust_name, raw_type);
        }

        outln!(out, "");
    }

    fn generate_type_alias_def(&self, type_alias_def: &xcbdefs::TypeAliasDef, out: &mut Output) {
        let rust_new_name = self.get_type_alias_rust_name(&type_alias_def);
        outln!(
            out,
            "pub type {} = {};",
            rust_new_name,
            self.type_to_rust_type(type_alias_def.old_name.get_resolved()),
        );
        outln!(out, "");
    }

    fn generate_switches_for_fields(
        &self,
        name_prefix: &str,
        fields: &[xcbdefs::FieldDef],
        generate_try_parse: bool,
        generate_serialize: bool,
        out: &mut Output,
    ) {
        for field in fields.iter() {
            if let xcbdefs::FieldDef::Switch(switch_field) = field {
                self.generate_switch(
                    name_prefix,
                    switch_field,
                    generate_try_parse,
                    generate_serialize,
                    out,
                );
            }
        }
    }

    fn generate_switch(
        &self,
        name_prefix: &str,
        switch: &xcbdefs::SwitchField,
        generate_try_parse: bool,
        generate_serialize: bool,
        out: &mut Output,
    ) {
        let switch_name = format!("{}{}", name_prefix, to_rust_type_name(&switch.name));
        self.emit_switch_type(
            switch,
            &switch_name,
            generate_try_parse,
            generate_serialize,
            None,
            out,
        );
        outln!(out, "");
    }

    // FIXME: `skip_length_field` is broken
    fn emit_struct_type(
        &self,
        name: &str,
        switch_prefix: &str,
        derives: Derives,
        fields: &[xcbdefs::FieldDef],
        external_params: &[xcbdefs::ExternalParam],
        skip_length_field: bool,
        generate_try_parse: bool,
        parse_size_constraint: StructSizeConstraint,
        generate_serialize: bool,
        fields_need_serialize: bool,
        doc: Option<&xcbdefs::Doc>,
        out: &mut Output,
    ) {
        assert!(!(generate_serialize && !fields_need_serialize));
        assert!(parse_size_constraint == StructSizeConstraint::None || generate_try_parse);

        let deducible_fields = gather_deducible_fields(fields);

        self.generate_switches_for_fields(
            switch_prefix,
            fields,
            generate_try_parse,
            fields_need_serialize,
            out,
        );

        if let Some(doc) = doc {
            self.emit_doc(doc, out);
        }
        let derives = derives.to_list();
        if !derives.is_empty() {
            outln!(out, "#[derive({})]", derives.join(", "));
        }
        outln!(out, "pub struct {} {{", name);
        for field in fields.iter() {
            if self.field_is_visible(field, &deducible_fields) {
                let field_name = field.name().unwrap();
                if !skip_length_field || field_name != "length" {
                    let field_type = self.field_to_rust_type(field, switch_prefix);
                    outln!(
                        out.indent(),
                        "pub {}: {},",
                        to_rust_variable_name(&field_name),
                        field_type
                    );
                }
            }
        }
        outln!(out, "}}");

        let has_fds = fields.iter().any(|field| match field {
            xcbdefs::FieldDef::Fd(_) | xcbdefs::FieldDef::FdList(_) => true,
            _ => false,
        });

        if generate_try_parse {
            let input_name = if parse_size_constraint != StructSizeConstraint::None {
                "initial_value"
            } else {
                "remaining"
            };
            if has_fds {
                assert!(external_params.is_empty());
                outln!(out, "impl TryParseFd for {} {{", name);
                outln!(
                    out.indent(),
                    "fn try_parse_fd<'a>({}: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {{",
                    input_name,
                );
            } else if !external_params.is_empty() {
                outln!(out, "impl {} {{", name);
                let p = external_params
                    .iter()
                    .map(|ext_param| {
                        format!(
                            "{}: {}",
                            to_rust_variable_name(&ext_param.name),
                            self.type_to_rust_type(&ext_param.type_),
                        )
                    })
                    .collect::<Vec<_>>();
                outln!(
                    out.indent(),
                    "pub fn try_parse({}: &[u8], {}) -> Result<(Self, &[u8]), ParseError> \
                     {{",
                    input_name,
                    p.join(", "),
                );
            } else {
                outln!(out, "impl TryParse for {} {{", name);
                outln!(
                    out.indent(),
                    "fn try_parse({}: &[u8]) -> Result<(Self, &[u8]), ParseError> {{",
                    input_name,
                );
            }

            out.indented(|out| {
                out.indented(|out| {
                    if parse_size_constraint != StructSizeConstraint::None {
                        outln!(out, "let remaining = initial_value;");
                    }
                    Self::emit_let_value_for_dynamic_align(fields, out);
                    for field in fields.iter() {
                        self.emit_field_parse(
                            field,
                            switch_prefix,
                            "remaining",
                            FieldContainer::Other,
                            out,
                        );
                    }
                    for field in fields.iter() {
                        if !field
                            .name()
                            .map(|field_name| deducible_fields.contains_key(field_name))
                            .unwrap_or(false)
                        {
                            self.emit_field_post_parse(field, out);
                        }
                    }
                    let field_names = fields
                        .iter()
                        .filter_map(|field| {
                            if self.field_is_visible(field, &deducible_fields) {
                                Some(to_rust_variable_name(field.name().unwrap()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    outln!(
                        out,
                        "let result = {} {{ {} }};",
                        name,
                        field_names.join(", ")
                    );
                    match parse_size_constraint {
                        StructSizeConstraint::None => (),
                        StructSizeConstraint::Fixed(fixed_size) => {
                            outln!(out, "let _ = remaining;");
                            outln!(out, "let remaining = initial_value.get({}..)", fixed_size);
                            outln!(out.indent(), ".ok_or(ParseError::InsufficientData)?;");
                        }
                        StructSizeConstraint::EmbeddedLength { minimum } => {
                            outln!(out, "let _ = remaining;");
                            outln!(
                                out,
                                "let remaining = initial_value.get({} + length as usize * 4..)",
                                minimum
                            );
                            outln!(out.indent(), ".ok_or(ParseError::InsufficientData)?;");
                        }
                    }
                    outln!(out, "Ok((result, remaining))");
                })
            });

            outln!(out.indent(), "}}");
            outln!(out, "}}");

            if external_params.is_empty() {
                let value = if has_fds {
                    "(&[u8], Vec<RawFdContainer>)"
                } else {
                    "&[u8]"
                };
                outln!(out, "impl TryFrom<{}> for {} {{", value, name);
                out.indented(|out| {
                    outln!(out, "type Error = ParseError;");
                    outln!(
                        out,
                        "fn try_from(value: {}) -> Result<Self, Self::Error> {{",
                        value
                    );
                    out.indented(|out| {
                        if has_fds {
                            outln!(out, "let (value, mut fds) = value;");
                            outln!(out, "Ok(Self::try_parse_fd(value, &mut fds)?.0)");
                        } else {
                            outln!(out, "Ok(Self::try_parse(value)?.0)");
                        }
                    });
                    outln!(out, "}}");
                });
                outln!(out, "}}");
            } else {
                outln!(
                    out,
                    "// Skipping TryFrom implementations because of unresolved members",
                );
            }
        }

        if generate_serialize {
            let size = fields
                .iter()
                .try_fold(0, |sz, field| Some(sz + field.size()?));
            if let Some(size) = size {
                self.emit_fixed_size_struct_serialize(
                    name,
                    fields,
                    external_params,
                    &deducible_fields,
                    skip_length_field,
                    size,
                    out,
                );
            } else {
                self.emit_variable_size_struct_serialize(
                    name,
                    fields,
                    external_params,
                    &deducible_fields,
                    skip_length_field,
                    out,
                );
            }
        }

        // Generate accessors for length fields that are not part of the struct.
        // First, collect all relevant fields.
        let deducible = fields
            .iter()
            .filter_map(|field| match field {
                xcbdefs::FieldDef::Normal(normal_field) => deducible_fields
                    .get(&normal_field.name)
                    .and_then(|d| match d {
                        DeducibleField::LengthOf(list_name, op) => Some((list_name, op)),
                        // switches are not yet supported
                        _ => None,
                    })
                    .map(|d| (field, d)),
                _ => None,
            })
            .collect::<Vec<_>>();
        if !deducible.is_empty() {
            if deducible
                .iter()
                .any(|(field, _)| field.name() == Some("len"))
            {
                let comment = "This is not a container and is_empty() makes no sense";
                outln!(out, "#[allow(clippy::len_without_is_empty)] // {}", comment);
            }
            outln!(out, "impl {} {{", name);
            for (field, (list_name, op)) in deducible {
                let field_type = self.field_to_rust_type(field, switch_prefix);
                let name = field.name().unwrap();
                let name = if name == "type" {
                    format!("r#{}", name)
                } else {
                    name.to_string()
                };
                out.indented(|out| {
                    outln!(out, "/// Get the value of the `{}` field.", &name);
                    outln!(out, "///");
                    outln!(
                        out,
                        "/// The `{}` field is used as the length field of the `{}` field.",
                        &name,
                        &list_name,
                    );
                    outln!(
                        out,
                        "/// This function computes the field's value again based on the length \
                         of the list.",
                    );
                    outln!(out, "///");
                    outln!(out, "/// # Panics");
                    outln!(out, "///");
                    outln!(
                        out,
                        "/// Panics if the value cannot be represented in the target type. This",
                    );
                    outln!(
                        out,
                        "/// cannot happen with values of the struct received from the X11 server.",
                    );
                    outln!(
                        out,
                        "pub fn {}(&self) -> {} {{",
                        to_rust_variable_name(&name),
                        field_type
                    );
                    out.indented(|out| {
                        outln!(out, "self.{}.len()", to_rust_variable_name(&list_name));
                        match op {
                            DeducibleLengthFieldOp::None => {}
                            DeducibleLengthFieldOp::Mul(n) => {
                                outln!(out.indent(), ".checked_mul({}).unwrap()", n);
                            }
                            DeducibleLengthFieldOp::Div(n) => {
                                outln!(out.indent(), ".checked_div({}).unwrap()", n);
                            }
                        }
                        outln!(out.indent(), ".try_into().unwrap()");
                    });
                    outln!(out, "}}");
                });
            }
            outln!(out, "}}");
        }
    }

    fn emit_fixed_size_struct_serialize(
        &self,
        name: &str,
        fields: &[xcbdefs::FieldDef],
        external_params: &[xcbdefs::ExternalParam],
        deducible_fields: &HashMap<String, DeducibleField>,
        skip_length_field: bool,
        size: u32,
        out: &mut Output,
    ) {
        let ext_params_arg_defs = self.ext_params_to_arg_defs(true, external_params);

        if external_params.is_empty() {
            outln!(out, "impl Serialize for {} {{", name);
        } else {
            outln!(out, "impl {} {{", name);
        }
        out.indented(|out| {
            if external_params.is_empty() {
                outln!(out, "type Bytes = [u8; {}];", size);
            }
            outln!(
                out,
                "fn serialize(&self{}) -> [u8; {}] {{",
                ext_params_arg_defs,
                size,
            );
            out.indented(|out| {
                // This gathers the bytes of the result
                let mut result_bytes = Vec::new();
                for field in fields.iter() {
                    if skip_length_field && field.name() == Some("length") {
                        continue;
                    }
                    self.emit_field_serialize(
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if !deducible_fields.contains_key(field_name) {
                                format!("self.{}", rust_field_name)
                            } else {
                                rust_field_name
                            }
                        },
                        &mut result_bytes,
                        out,
                    );
                }
                outln!(out, "[");
                for result_byte in result_bytes.iter() {
                    outln!(out.indent(), "{},", result_byte);
                }
                outln!(out, "]");
            });
            outln!(out, "}}");
            outln!(
                out,
                "fn serialize_into(&self, bytes: &mut Vec<u8>{}) {{",
                ext_params_arg_defs
            );
            out.indented(|out| {
                outln!(out, "bytes.reserve({});", size);
                for field in fields.iter() {
                    if skip_length_field && field.name() == Some("length") {
                        continue;
                    }
                    self.emit_field_serialize_into(
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if !deducible_fields.contains_key(field_name)
                                && !external_params.iter().any(|param| param.name == field_name)
                            {
                                format!("self.{}", rust_field_name)
                            } else {
                                rust_field_name
                            }
                        },
                        "bytes",
                        out,
                    );
                }
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }

    fn emit_variable_size_struct_serialize(
        &self,
        name: &str,
        fields: &[xcbdefs::FieldDef],
        external_params: &[xcbdefs::ExternalParam],
        deducible_fields: &HashMap<String, DeducibleField>,
        skip_length_field: bool,
        out: &mut Output,
    ) {
        let ext_params_arg_defs = self.ext_params_to_arg_defs(true, external_params);
        let ext_params_call_args =
            self.ext_params_to_call_args(true, to_rust_variable_name, external_params);

        if external_params.is_empty() {
            outln!(out, "impl Serialize for {} {{", name);
        } else {
            outln!(out, "impl {} {{", name);
        }
        out.indented(|out| {
            if external_params.is_empty() {
                outln!(out, "type Bytes = Vec<u8>;");
            } else {
                outln!(out, "#[allow(dead_code)]");
            }
            outln!(
                out,
                "fn serialize(&self{}) -> Vec<u8> {{",
                ext_params_arg_defs,
            );
            out.indented(|out| {
                outln!(out, "let mut result = Vec::new();");
                outln!(
                    out,
                    "self.serialize_into(&mut result{});",
                    ext_params_call_args,
                );
                outln!(out, "result");
            });
            outln!(out, "}}");
            outln!(
                out,
                "fn serialize_into(&self, bytes: &mut Vec<u8>{}) {{",
                ext_params_arg_defs
            );
            out.indented(|out| {
                // Variable size structures usually have some fixed size
                // fields at the beginning. Gather the total size of those
                // fields...
                let mut fixed_part_size = 0;
                let mut num_fixed_fields = 0;
                for field in fields.iter() {
                    if let Some(field_size) = field.size() {
                        fixed_part_size += field_size;
                        num_fixed_fields += 1;
                    } else {
                        break;
                    }
                }
                // ...and reserve that size into the output vector to reduce
                // the number of reallocations.
                if fixed_part_size != 0 && num_fixed_fields != 1 {
                    outln!(out, "bytes.reserve({});", fixed_part_size);
                }

                for field in fields.iter() {
                    if skip_length_field && field.name() == Some("length") {
                        continue;
                    }
                    self.emit_field_serialize_into(
                        field,
                        deducible_fields,
                        |field_name| {
                            let rust_field_name = to_rust_variable_name(field_name);
                            if !deducible_fields.contains_key(field_name)
                                && !external_params.iter().any(|param| param.name == field_name)
                            {
                                format!("self.{}", rust_field_name)
                            } else {
                                rust_field_name
                            }
                        },
                        "bytes",
                        out,
                    );
                }
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }

    fn emit_switch_type(
        &self,
        switch: &xcbdefs::SwitchField,
        name: &str,
        generate_try_parse: bool,
        generate_serialize: bool,
        doc: Option<&str>,
        out: &mut Output,
    ) -> Vec<CaseInfo> {
        let switch_align = switch.alignment.get().unwrap();
        // serializing / parsing wouldn't work correctly otherwise
        assert!(switch_align.internal_align <= switch_align.begin.align());
        assert_eq!(switch_align.begin.offset() % switch_align.internal_align, 0);

        let mut case_infos = Vec::new();

        for (i, case) in switch.cases.iter().enumerate() {
            let case_align = case.alignment.get().unwrap();
            // serializing / parsing wouldn't work correctly otherwise
            assert!(case_align.internal_align <= case_align.begin.align());
            assert_eq!(case_align.begin.offset() % case_align.internal_align, 0);

            let case_deducible_fields = gather_deducible_fields(&*case.fields.borrow());
            let mut single_field_index =
                if self.case_has_single_visible_field(case, &case_deducible_fields) {
                    Some(
                        case.fields
                            .borrow()
                            .iter()
                            .position(|field| self.field_is_visible(field, &case_deducible_fields))
                            .unwrap(),
                    )
                } else {
                    None
                };

            if let Some(field_index) = single_field_index {
                if let Some(ref case_name) = case.name {
                    let case_fields = case.fields.borrow();
                    let single_field_name = case_fields[field_index].name().unwrap();
                    if case_name != single_field_name {
                        // The <(bit)case> has a name, which is different from
                        // the name of the single field, so create a struct
                        // anyways.
                        single_field_index = None;
                    }
                }
            }

            if let Some(field_index) = single_field_index {
                if let xcbdefs::FieldDef::Switch(ref switch_field) =
                    case.fields.borrow()[field_index]
                {
                    self.generate_switch(
                        name,
                        switch_field,
                        generate_try_parse,
                        generate_serialize,
                        out,
                    );
                }

                case_infos.push(CaseInfo::SingleField(field_index));
            } else {
                // If the switch case has more than one visible
                // field, a helper struct to ensure that these
                // fields are only specified together.
                let field_name = case.name.as_ref().cloned().unwrap_or_else(|| {
                    if switch.kind == xcbdefs::SwitchKind::Case {
                        format!("case{}", i + 1)
                    } else {
                        format!("bitcase{}", i + 1)
                    }
                });
                let type_name = format!("{}{}", name, to_rust_type_name(&field_name));

                let mut derives = Derives::all();
                let case_fields = case.fields.borrow();
                let ext_params = case.external_params.borrow();

                self.filter_derives_for_fields(&mut derives, &*case_fields, false);
                self.emit_struct_type(
                    &type_name,
                    &type_name,
                    derives,
                    &*case_fields,
                    &*ext_params,
                    false,
                    generate_try_parse,
                    StructSizeConstraint::None,
                    generate_serialize,
                    generate_serialize,
                    None,
                    out,
                );

                case_infos.push(CaseInfo::MultiField(field_name, type_name));
            }
        }

        if let Some(doc) = doc {
            outln!(out, "/// {}", doc);
        }

        let mut derives = Derives::all();
        for case in switch.cases.iter() {
            self.filter_derives_for_fields(&mut derives, &*case.fields.borrow(), false);
        }
        let mut derives = derives.to_list();
        if switch.kind == xcbdefs::SwitchKind::BitCase {
            derives.push("Default");
        }
        if !derives.is_empty() {
            outln!(out, "#[derive({})]", derives.join(", "));
        }

        if switch.kind == xcbdefs::SwitchKind::BitCase {
            outln!(out, "pub struct {} {{", name);
            for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                match case_info {
                    CaseInfo::SingleField(index) => {
                        let fields = case.fields.borrow();
                        let single_field = &fields[*index];
                        let field_name = single_field.name().unwrap();
                        outln!(
                            out.indent(),
                            "pub {}: {}<{}>,",
                            to_rust_variable_name(field_name),
                            self.option_name,
                            self.field_to_rust_type(single_field, name),
                        );
                    }
                    CaseInfo::MultiField(field_name, struct_name) => {
                        outln!(
                            out.indent(),
                            "pub {}: {}<{}>,",
                            to_rust_variable_name(field_name),
                            self.option_name,
                            struct_name,
                        );
                    }
                }
            }
            outln!(out, "}}");
        } else {
            outln!(out, "pub enum {} {{", name);
            for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                match case_info {
                    CaseInfo::SingleField(index) => {
                        let fields = case.fields.borrow();
                        let single_field = &fields[*index];
                        let field_name = single_field.name().unwrap();
                        outln!(
                            out.indent(),
                            "{}({}),",
                            to_rust_type_name(field_name),
                            self.field_to_rust_type(single_field, name),
                        );
                    }
                    CaseInfo::MultiField(field_name, struct_name) => {
                        outln!(
                            out.indent(),
                            "{}({}),",
                            to_rust_type_name(field_name),
                            struct_name,
                        );
                    }
                }
            }
            outln!(out, "}}");
        }

        if generate_try_parse {
            self.emit_switch_try_parse(switch, name, &case_infos, out);
        }

        if switch.kind == xcbdefs::SwitchKind::Case {
            outln!(out, "impl {} {{", name);
            out.indented(|out| {
                for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                    let (rust_case_var_name, rust_case_type_name, rust_case_type) = match case_info
                    {
                        CaseInfo::SingleField(index) => {
                            let fields = case.fields.borrow();
                            let single_field = &fields[*index];
                            let field_name = single_field.name().unwrap();
                            (
                                to_rust_variable_name(field_name),
                                to_rust_type_name(field_name),
                                self.field_to_rust_type(single_field, name),
                            )
                        }
                        CaseInfo::MultiField(field_name, struct_name) => (
                            to_rust_variable_name(field_name),
                            to_rust_type_name(field_name),
                            String::from(struct_name),
                        ),
                    };
                    outln!(
                        out,
                        "pub fn as_{}(&self) -> Option<&{}> {{",
                        rust_case_var_name,
                        rust_case_type,
                    );
                    out.indented(|out| {
                        outln!(out, "match self {{");
                        outln!(
                            out.indent(),
                            "{}::{}(value) => Some(value),",
                            name,
                            rust_case_type_name,
                        );
                        outln!(out.indent(), "_ => None,");
                        outln!(out, "}}");
                    });
                    outln!(out, "}}");
                }
            });
            outln!(out, "}}");
        }

        if generate_serialize {
            if let Some(size) = switch.size() {
                self.emit_fixed_size_switch_serialize(switch, name, &case_infos, size, out);
            } else {
                self.emit_variable_size_switch_serialize(switch, name, &case_infos, out);
            }
        }

        let generate_switch_expr_fn =
            generate_serialize && switch.cases.iter().all(|case| case.exprs.len() == 1);

        if generate_switch_expr_fn {
            outln!(out, "impl {} {{", name);
            out.indented(|out| {
                outln!(out, "fn switch_expr(&self) -> u32 {{");
                out.indented(|out| {
                    if switch.kind == xcbdefs::SwitchKind::Case {
                        outln!(out, "match self {{");
                        for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                            assert_eq!(case.exprs.len(), 1);
                            let rust_field_name = match case_info {
                                CaseInfo::SingleField(index) => {
                                    to_rust_type_name(case.fields.borrow()[*index].name().unwrap())
                                }
                                CaseInfo::MultiField(field_name, _) => {
                                    to_rust_type_name(field_name)
                                }
                            };
                            outln!(
                                out.indent(),
                                "{}::{}(_) => {},",
                                name,
                                rust_field_name,
                                self.expr_to_str(
                                    &case.exprs[0],
                                    to_rust_variable_name,
                                    true,
                                    true,
                                    false,
                                ),
                            );
                        }
                        outln!(out, "}}");
                    } else {
                        outln!(out, "let mut expr_value = 0;");
                        for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                            assert_eq!(case.exprs.len(), 1);
                            let rust_field_name = match case_info {
                                CaseInfo::SingleField(index) => to_rust_variable_name(
                                    case.fields.borrow()[*index].name().unwrap(),
                                ),
                                CaseInfo::MultiField(field_name, _) => {
                                    to_rust_variable_name(field_name)
                                }
                            };
                            outln!(out, "if self.{}.is_some() {{", rust_field_name);
                            outln!(
                                out.indent(),
                                "expr_value |= {};",
                                self.expr_to_str(
                                    &case.exprs[0],
                                    to_rust_variable_name,
                                    true,
                                    true,
                                    false,
                                ),
                            );
                            outln!(out, "}}");
                        }
                        outln!(out, "expr_value");
                    }
                });
                outln!(out, "}}");
            });
            outln!(out, "}}");
        }

        case_infos
    }

    fn emit_switch_try_parse(
        &self,
        switch: &xcbdefs::SwitchField,
        name: &str,
        case_infos: &[CaseInfo],
        out: &mut Output,
    ) {
        let external_params = switch.external_params.borrow();

        if !external_params.is_empty() {
            outln!(out, "impl {} {{", name);
            let p = external_params
                .iter()
                .map(|ext_param| {
                    format!(
                        "{}: {}",
                        to_rust_variable_name(&ext_param.name),
                        self.type_to_rust_type(&ext_param.type_),
                    )
                })
                .collect::<Vec<_>>();
            outln!(
                out.indent(),
                "fn try_parse(value: &[u8], {}) -> Result<(Self, &[u8]), ParseError> {{",
                p.join(", "),
            );
        } else {
            outln!(out, "impl TryParse for {} {{", name);
            outln!(
                out.indent(),
                "fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {{",
            );
        }

        out.indented(|out| {
            out.indented(|out| {
                outln!(
                    out,
                    "let switch_expr = {};",
                    self.expr_to_str(&switch.expr, to_rust_variable_name, false, true, false),
                );
                outln!(out, "let mut outer_remaining = value;");
                if switch.kind == xcbdefs::SwitchKind::BitCase {
                    let mut rust_case_names = Vec::new();
                    for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                        let mut case_expr_str = format!(
                            "switch_expr & {} != 0",
                            self.expr_to_str(
                                &case.exprs[0],
                                to_rust_variable_name,
                                false,
                                true,
                                true,
                            ),
                        );
                        for expr in case.exprs[1..].iter() {
                            case_expr_str.push_str(" || switch_expr & ");
                            case_expr_str.push_str(&self.expr_to_str(
                                expr,
                                to_rust_variable_name,
                                false,
                                true,
                                true,
                            ));
                            case_expr_str.push_str(" != 0");
                        }

                        let rust_case_name = match case_info {
                            CaseInfo::SingleField(index) => {
                                let fields = case.fields.borrow();
                                let single_field = &fields[*index];
                                let field_name = single_field.name().unwrap();
                                to_rust_variable_name(field_name)
                            }
                            CaseInfo::MultiField(field_name, _) => {
                                to_rust_variable_name(field_name)
                            }
                        };

                        outln!(out, "let {} = if {} {{", rust_case_name, case_expr_str);
                        out.indented(|out| {
                            match case_info {
                                CaseInfo::SingleField(_) => {
                                    outln!(out, "let remaining = outer_remaining;");
                                    let case_fields = case.fields.borrow();
                                    Self::emit_let_value_for_dynamic_align(&*case_fields, out);
                                    for field in case_fields.iter() {
                                        self.emit_field_parse(
                                            field,
                                            name,
                                            "remaining",
                                            FieldContainer::Other,
                                            out,
                                        );
                                    }
                                    for field in case_fields.iter() {
                                        self.emit_field_post_parse(field, out);
                                    }
                                    outln!(out, "outer_remaining = remaining;");
                                }
                                CaseInfo::MultiField(_, struct_name) => {
                                    let mut parse_params = Vec::new();
                                    parse_params.push(String::from("outer_remaining"));
                                    for p in case.external_params.borrow().iter() {
                                        parse_params.push(to_rust_variable_name(&p.name));
                                    }
                                    outln!(
                                        out,
                                        "let ({}, new_remaining) = {}::try_parse({})?;",
                                        rust_case_name,
                                        struct_name,
                                        parse_params.join(", "),
                                    );
                                    outln!(out, "outer_remaining = new_remaining;");
                                }
                            }
                            outln!(out, "Some({})", rust_case_name);
                        });
                        outln!(out, "}} else {{");
                        outln!(out.indent(), "None");
                        outln!(out, "}};");

                        rust_case_names.push(rust_case_name);
                    }
                    outln!(
                        out,
                        "let result = {} {{ {} }};",
                        name,
                        rust_case_names.join(", "),
                    );
                    outln!(out, "Ok((result, outer_remaining))");
                } else {
                    outln!(out, "let mut parse_result = None;");
                    for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                        let mut case_expr_str = format!(
                            "switch_expr == {}",
                            self.expr_to_str(
                                &case.exprs[0],
                                to_rust_variable_name,
                                false,
                                true,
                                true,
                            ),
                        );
                        for expr in case.exprs[1..].iter() {
                            case_expr_str.push_str(" || switch_expr == ");
                            case_expr_str.push_str(&self.expr_to_str(
                                expr,
                                to_rust_variable_name,
                                false,
                                true,
                                true,
                            ));
                        }

                        let (rust_case_type_name, rust_case_var_name) = match case_info {
                            CaseInfo::SingleField(index) => {
                                let fields = case.fields.borrow();
                                let single_field = &fields[*index];
                                let field_name = single_field.name().unwrap();
                                (
                                    to_rust_type_name(field_name),
                                    to_rust_variable_name(field_name),
                                )
                            }
                            CaseInfo::MultiField(field_name, _) => (
                                to_rust_type_name(field_name),
                                to_rust_variable_name(field_name),
                            ),
                        };

                        outln!(out, "if {} {{", case_expr_str);
                        out.indented(|out| {
                            match case_info {
                                CaseInfo::SingleField(_) => {
                                    outln!(out, "let remaining = outer_remaining;");
                                    let case_fields = case.fields.borrow();
                                    Self::emit_let_value_for_dynamic_align(&*case_fields, out);
                                    for field in case_fields.iter() {
                                        self.emit_field_parse(
                                            field,
                                            name,
                                            "remaining",
                                            FieldContainer::Other,
                                            out,
                                        );
                                    }
                                    for field in case_fields.iter() {
                                        self.emit_field_post_parse(field, out);
                                    }
                                    outln!(out, "outer_remaining = remaining;");
                                }
                                CaseInfo::MultiField(_, struct_name) => {
                                    let mut parse_params = Vec::new();
                                    parse_params.push(String::from("outer_remaining"));
                                    for p in case.external_params.borrow().iter() {
                                        parse_params.push(to_rust_variable_name(&p.name));
                                    }
                                    outln!(
                                        out,
                                        "let ({}, new_remaining) = {}::try_parse({})?;",
                                        rust_case_var_name,
                                        struct_name,
                                        parse_params.join(", "),
                                    );
                                    outln!(out, "outer_remaining = new_remaining;");
                                }
                            }
                            let msg = "The XML should prevent more than one 'if' from matching";
                            outln!(out, "assert!(parse_result.is_none(), \"{}\");", msg);
                            outln!(
                                out,
                                "parse_result = Some({}::{}({}));",
                                name,
                                rust_case_type_name,
                                rust_case_var_name,
                            );
                        });
                        outln!(out, "}}");
                    }
                    outln!(out, "match parse_result {{");
                    outln!(out.indent(), "None => Err(ParseError::InvalidValue),");
                    outln!(
                        out.indent(),
                        "Some(result) => Ok((result, outer_remaining)),",
                    );
                    outln!(out, "}}");
                }
            });
        });

        outln!(out.indent(), "}}");
        outln!(out, "}}");
    }

    fn emit_fixed_size_switch_serialize(
        &self,
        switch: &xcbdefs::SwitchField,
        name: &str,
        case_infos: &[CaseInfo],
        size: u32,
        out: &mut Output,
    ) {
        assert!(switch.kind == xcbdefs::SwitchKind::Case);

        let external_params = switch.external_params.borrow();
        let ext_params_arg_defs = self.ext_params_to_arg_defs(true, &*external_params);
        //let ext_params_call_args =
        //    self.ext_params_to_call_args(true, to_rust_variable_name, &*external_params);

        if external_params.is_empty() {
            outln!(out, "impl Serialize for {} {{", name);
        } else {
            outln!(out, "impl {} {{", name);
        }
        out.indented(|out| {
            if external_params.is_empty() {
                outln!(out, "type Bytes = [u8; {}];", size);
            }
            outln!(
                out,
                "fn serialize(&self{}) -> [u8; {}] {{",
                ext_params_arg_defs,
                size,
            );
            out.indented(|out| {
                self.emit_assert_for_switch_serialize(switch, out);
                outln!(out, "match self {{");
                out.indented(|out| {
                    for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                        match case_info {
                            CaseInfo::SingleField(index) => {
                                let fields = case.fields.borrow();
                                let single_field = &fields[*index];
                                let single_field_name = single_field.name().unwrap();
                                let deducible_fields = gather_deducible_fields(&*fields);
                                outln!(
                                    out,
                                    "{}::{}({}) => {{",
                                    name,
                                    to_rust_type_name(single_field_name),
                                    to_rust_variable_name(single_field_name),
                                );
                                out.indented(|out| {
                                    let mut result_bytes = Vec::new();
                                    for field in fields.iter() {
                                        self.emit_field_serialize(
                                            field,
                                            &deducible_fields,
                                            |field| to_rust_variable_name(field),
                                            &mut result_bytes,
                                            out,
                                        );
                                    }
                                    outln!(out, "[[");
                                    for result_byte in result_bytes.iter() {
                                        outln!(out, "{},", result_byte);
                                    }
                                    outln!(out, "]]");
                                });
                                outln!(out, "}}");
                            }
                            CaseInfo::MultiField(field_name, _) => {
                                let rust_field_name = to_rust_variable_name(field_name);
                                outln!(
                                    out,
                                    "{}::{}({}) => {}.serialize(),",
                                    name,
                                    to_rust_type_name(field_name),
                                    rust_field_name,
                                    rust_field_name,
                                );
                            }
                        }
                    }
                });
                outln!(out, "}}");
            });
            outln!(out, "}}");
            outln!(
                out,
                "fn serialize_into(&self, bytes: &mut Vec<u8>{}) {{",
                ext_params_arg_defs
            );
            out.indented(|out| {
                outln!(out, "match *self {{");
                for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                    match case_info {
                        CaseInfo::SingleField(index) => {
                            let fields = case.fields.borrow();
                            let single_field = &fields[*index];
                            let single_field_name = single_field.name().unwrap();
                            let deducible_fields = gather_deducible_fields(&*fields);
                            let qual = if needs_match_by_value(single_field) {
                                ""
                            } else {
                                "ref "
                            };
                            outln!(
                                out,
                                "{}::{}({}{}) => {{",
                                name,
                                to_rust_type_name(single_field_name),
                                qual,
                                to_rust_variable_name(single_field_name),
                            );
                            out.indented(|out| {
                                for field in fields.iter() {
                                    self.emit_field_serialize_into(
                                        field,
                                        &deducible_fields,
                                        |field| to_rust_variable_name(field),
                                        "bytes",
                                        out,
                                    );
                                }
                            });
                            outln!(out, "}}");
                        }
                        CaseInfo::MultiField(field_name, _) => {
                            let rust_field_name = to_rust_variable_name(field_name);
                            outln!(
                                out,
                                "{}::{}(ref {}) => {}.serialize_into(bytes),",
                                name,
                                to_rust_type_name(field_name),
                                rust_field_name,
                                rust_field_name,
                            );
                        }
                    }
                }
                outln!(out, "}}");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }

    fn emit_variable_size_switch_serialize(
        &self,
        switch: &xcbdefs::SwitchField,
        name: &str,
        case_infos: &[CaseInfo],
        out: &mut Output,
    ) {
        let external_params = switch.external_params.borrow();
        let ext_params_arg_defs = self.ext_params_to_arg_defs(true, &*external_params);
        let ext_params_call_args =
            self.ext_params_to_call_args(true, to_rust_variable_name, &*external_params);

        if external_params.is_empty() {
            outln!(out, "impl Serialize for {} {{", name);
        } else {
            outln!(out, "impl {} {{", name);
        }
        out.indented(|out| {
            if external_params.is_empty() {
                outln!(out, "type Bytes = Vec<u8>;");
            } else {
                outln!(out, "#[allow(dead_code)]");
            }
            outln!(
                out,
                "fn serialize(&self{}) -> Vec<u8> {{",
                ext_params_arg_defs,
            );
            out.indented(|out| {
                outln!(out, "let mut result = Vec::new();");
                outln!(
                    out,
                    "self.serialize_into(&mut result{});",
                    ext_params_call_args,
                );
                outln!(out, "result");
            });
            outln!(out, "}}");
            outln!(
                out,
                "fn serialize_into(&self, bytes: &mut Vec<u8>{}) {{",
                ext_params_arg_defs
            );
            out.indented(|out| {
                self.emit_assert_for_switch_serialize(switch, out);
                if switch.kind == xcbdefs::SwitchKind::BitCase {
                    for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                        match case_info {
                            CaseInfo::SingleField(index) => {
                                let fields = case.fields.borrow();
                                let single_field = &fields[*index];
                                let single_field_name = single_field.name().unwrap();
                                let rust_single_field_name =
                                    to_rust_variable_name(single_field_name);
                                let deducible_fields = gather_deducible_fields(&*fields);
                                let qual = if needs_match_by_value(single_field) {
                                    ""
                                } else {
                                    "ref "
                                };
                                outln!(
                                    out,
                                    "if let Some({}{}) = self.{} {{",
                                    qual,
                                    rust_single_field_name,
                                    rust_single_field_name,
                                );
                                out.indented(|out| {
                                    // Gather the total size of the head fixed size fields
                                    // and reserve that size into the output vector to reduce
                                    // the number of reallocations.
                                    let mut fixed_part_size = 0;
                                    let mut num_fixed_fields = 0;
                                    for field in fields.iter() {
                                        if let Some(field_size) = field.size() {
                                            fixed_part_size += field_size;
                                            num_fixed_fields += 1;
                                        } else {
                                            break;
                                        }
                                    }
                                    if fixed_part_size != 0 && num_fixed_fields != 1 {
                                        outln!(out, "bytes.reserve({});", fixed_part_size);
                                    }

                                    for field in fields.iter() {
                                        self.emit_field_serialize_into(
                                            field,
                                            &deducible_fields,
                                            |field| to_rust_variable_name(field),
                                            "bytes",
                                            out,
                                        );
                                    }
                                });
                                outln!(out, "}}");
                            }
                            CaseInfo::MultiField(field_name, _) => {
                                let rust_field_name = to_rust_variable_name(field_name);
                                let ext_params_call_args = self.ext_params_to_call_args(
                                    true,
                                    to_rust_variable_name,
                                    &*case.external_params.borrow(),
                                );
                                outln!(
                                    out,
                                    "if let Some(ref {}) = self.{} {{",
                                    rust_field_name,
                                    rust_field_name,
                                );
                                outln!(
                                    out.indent(),
                                    "{}.serialize_into(bytes{});",
                                    rust_field_name,
                                    ext_params_call_args,
                                );
                                outln!(out, "}}");
                            }
                        }
                    }
                } else {
                    outln!(out, "match self {{");
                    out.indented(|out| {
                        for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                            match case_info {
                                CaseInfo::SingleField(index) => {
                                    let fields = case.fields.borrow();
                                    let single_field = &fields[*index];
                                    let single_field_name = single_field.name().unwrap();
                                    let rust_single_field_name =
                                        to_rust_variable_name(single_field_name);
                                    let deducible_fields = gather_deducible_fields(&*fields);
                                    outln!(
                                        out,
                                        "{}::{}({}) => {{",
                                        name,
                                        to_rust_type_name(single_field_name),
                                        rust_single_field_name,
                                    );
                                    out.indented(|out| {
                                        // Gather the total size of the head fixed size fields
                                        // and reserve that size into the output vector to reduce
                                        // the number of reallocations.
                                        let mut fixed_part_size = 0;
                                        let mut num_fixed_fields = 0;
                                        for field in fields.iter() {
                                            if let Some(field_size) = field.size() {
                                                fixed_part_size += field_size;
                                                num_fixed_fields += 1;
                                            } else {
                                                break;
                                            }
                                        }
                                        if fixed_part_size != 0 && num_fixed_fields != 1 {
                                            outln!(out, "bytes.reserve({});", fixed_part_size);
                                        }

                                        for field in fields.iter() {
                                            self.emit_field_serialize_into(
                                                field,
                                                &deducible_fields,
                                                |field| to_rust_variable_name(field),
                                                "bytes",
                                                out,
                                            );
                                        }
                                    });
                                    outln!(out, "}}");
                                }
                                CaseInfo::MultiField(field_name, _) => {
                                    let rust_field_name = to_rust_variable_name(field_name);
                                    outln!(
                                        out,
                                        "{}::{}({}) => {}.serialize_into(bytes),",
                                        name,
                                        to_rust_type_name(field_name),
                                        rust_field_name,
                                        rust_field_name,
                                    );
                                }
                            }
                        }
                    });
                    outln!(out, "}}");
                }
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }

    fn emit_let_value_for_dynamic_align(fields: &[xcbdefs::FieldDef], out: &mut Output) {
        // If there is any dynamic padding, remember the original position
        // to calculate the alignment.
        let has_dynamic_pad = fields.iter().any(|field| match field {
            xcbdefs::FieldDef::Pad(pad_field) => match pad_field.kind {
                xcbdefs::PadKind::Bytes(_) => false,
                xcbdefs::PadKind::Align(_) => true,
            },
            _ => false,
        });
        if has_dynamic_pad {
            outln!(out, "let value = remaining;");
        }
    }

    fn emit_field_parse(
        &self,
        field: &xcbdefs::FieldDef,
        switch_prefix: &str,
        from: &str,
        container: FieldContainer,
        out: &mut Output,
    ) {
        match field {
            xcbdefs::FieldDef::Pad(pad_field) => {
                let pad_size = match pad_field.kind {
                    xcbdefs::PadKind::Bytes(size) => size.to_string(),
                    xcbdefs::PadKind::Align(align) => {
                        outln!(out, "// Align offset to multiple of {}", align);
                        outln!(
                            out,
                            "let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;",
                        );
                        outln!(
                            out,
                            "let misalignment = ({} - (offset % {})) % {};",
                            align,
                            align,
                            align,
                        );
                        "misalignment".into()
                    }
                };
                outln!(
                    out,
                    "let remaining = {from}.get({pad}..).ok_or(ParseError::InsufficientData)?;",
                    from = from,
                    pad = pad_size,
                );
            }
            xcbdefs::FieldDef::Normal(normal_field) => {
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                outln!(
                    out,
                    "let ({}, remaining) = {};",
                    rust_field_name,
                    self.emit_value_parse(&normal_field.type_, from),
                );
            }
            xcbdefs::FieldDef::List(list_field) => {
                let rust_field_name = to_rust_variable_name(&list_field.name);

                if self.rust_value_type_is_u8(&list_field.element_type) {
                    // List of `u8`, use simple parsing
                    if let Some(list_length) = list_field.length() {
                        outln!(
                            out,
                            "let ({}, remaining) = crate::x11_utils::parse_u8_list({}, {})?;",
                            rust_field_name,
                            from,
                            list_length,
                        );
                        let ownership = if container == FieldContainer::Other {
                            // Only force taking ownership for non-request types.
                            ""
                        } else {
                            // Otherwise convert this into a reference to a fixed length array.
                            // It's basically dumb luck that the largest list in the X11 protocol
                            // and the largest fixed length array that Rust has a builtin conversion
                            // for going from &[T] to &[T; N] both happen to be 32.
                            assert!(list_length <= 32);
                            "&"
                        };
                        outln!(
                            out,
                            "let {field} = <{ownership}[u8; \
                             {length}]>::try_from({field}).unwrap();",
                            field = rust_field_name,
                            ownership = ownership,
                            length = list_length,
                        );
                    } else if let Some(ref length_expr) = list_field.length_expr {
                        outln!(
                            out,
                            "let ({}, remaining) = crate::x11_utils::parse_u8_list({}, \
                             {}.try_into().or(Err(ParseError::ConversionFailed))?)?;",
                            rust_field_name,
                            from,
                            self.expr_to_str(
                                length_expr,
                                to_rust_variable_name,
                                false,
                                false,
                                true,
                            ),
                        );
                        // Only force taking ownership for non-request types.
                        if container == FieldContainer::Other {
                            outln!(
                                out,
                                "let {} = {}.to_vec();",
                                rust_field_name,
                                rust_field_name
                            );
                        }
                    } else {
                        // Only force taking ownership for non-request types.
                        if container == FieldContainer::Other {
                            outln!(out, "let {} = remaining.to_vec();", rust_field_name);
                            outln!(out, "let remaining = &remaining[remaining.len()..];");
                        } else {
                            outln!(
                                out,
                                "let ({}, remaining) = remaining.split_at(remaining.len());",
                                rust_field_name
                            );
                        }
                    }
                } else if self.can_use_simple_list_parsing(&list_field.element_type)
                    && list_field.length_expr.is_some()
                    && list_field.length().is_none()
                {
                    let rust_element_type =
                        self.type_to_rust_type(list_field.element_type.type_.get_resolved());
                    outln!(
                        out,
                        "let ({}, remaining) = crate::x11_utils::parse_list::<{}>(remaining, \
                         {}.try_into().or(Err(ParseError::ConversionFailed))?)?;",
                        rust_field_name,
                        rust_element_type,
                        self.expr_to_str(
                            list_field.length_expr.as_ref().unwrap(),
                            to_rust_variable_name,
                            false,
                            false,
                            true,
                        ),
                    );
                } else if let Some(list_len) = list_field.length() {
                    for i in 0..list_len {
                        let tmp_name = format!("{}_{}", rust_field_name, i);
                        outln!(
                            out,
                            "let ({}_{}, remaining) = {};",
                            rust_field_name,
                            i,
                            self.emit_value_parse(&list_field.element_type, from),
                        );
                        self.emit_value_post_parse(&list_field.element_type, &tmp_name, out);
                    }
                    outln!(out, "let {} = [", rust_field_name);
                    for i in 0..list_len {
                        outln!(out.indent(), "{}_{},", rust_field_name, i);
                    }
                    outln!(out, "];");
                } else {
                    outln!(out, "let mut remaining = {};", from);
                    if let Some(ref length_expr) = list_field.length_expr {
                        outln!(
                            out,
                            "let list_length = \
                             usize::try_from({}).or(Err(ParseError::ConversionFailed))?;",
                            self.expr_to_str(
                                length_expr,
                                to_rust_variable_name,
                                false,
                                false,
                                false,
                            ),
                        );
                        outln!(
                            out,
                            "let mut {} = Vec::with_capacity(list_length);",
                            rust_field_name
                        );
                        outln!(out, "for _ in 0..list_length {{");
                    } else {
                        outln!(out, "// Length is 'everything left in the input'");
                        outln!(out, "let mut {} = Vec::new();", rust_field_name);
                        outln!(out, "while !remaining.is_empty() {{");
                    }
                    out.indented(|out| {
                        outln!(
                            out,
                            "let (v, new_remaining) = {};",
                            self.emit_value_parse(&list_field.element_type, from),
                        );
                        self.emit_value_post_parse(&list_field.element_type, "v", out);
                        outln!(out, "remaining = new_remaining;");
                        outln!(out, "{}.push(v);", rust_field_name);
                    });
                    outln!(out, "}}");
                }
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                let rust_field_name = to_rust_variable_name(&switch_field.name);
                let switch_struct_name = if let FieldContainer::Request(request_name) = container {
                    format!("{}Aux", request_name)
                } else {
                    format!("{}{}", switch_prefix, to_rust_type_name(&switch_field.name))
                };
                let mut parse_params = Vec::new();
                parse_params.push(String::from("remaining"));
                for ext_param in switch_field.external_params.borrow().iter() {
                    parse_params.push(to_rust_variable_name(&ext_param.name));
                }
                outln!(
                    out,
                    "let ({}, remaining) = {}::try_parse({})?;",
                    rust_field_name,
                    switch_struct_name,
                    parse_params.join(", "),
                );
            }
            xcbdefs::FieldDef::Fd(fd_field) => {
                let rust_field_name = to_rust_variable_name(&fd_field.name);
                outln!(
                    out,
                    "if fds.is_empty() {{ return Err(ParseError::MissingFileDescriptors) }}"
                );
                outln!(out, "let {} = fds.remove(0);", rust_field_name);
            }
            xcbdefs::FieldDef::FdList(fd_list_field) => {
                let rust_field_name = to_rust_variable_name(&fd_list_field.name);

                outln!(
                    out,
                    "let fds_len = usize::try_from({}).or(Err(ParseError::ConversionFailed))?;",
                    self.expr_to_str(
                        &fd_list_field.length_expr,
                        to_rust_variable_name,
                        false,
                        false,
                        false,
                    ),
                );
                outln!(
                    out,
                    "if fds.len() < fds_len {{ return Err(ParseError::MissingFileDescriptors) }}",
                );
                outln!(out, "let mut {} = fds.split_off(fds_len);", rust_field_name);
                outln!(out, "std::mem::swap(fds, &mut {});", rust_field_name);
            }
            xcbdefs::FieldDef::Expr(expr_ref) => {
                match expr_ref.expr {
                    xcbdefs::Expression::Value(_) => {
                        // Treat this as a normal field, that we'll validate in
                        // emit_field_post_parse.
                        let rust_field_name = to_rust_variable_name(&expr_ref.name);
                        outln!(
                            out,
                            "let ({}, remaining) = {};",
                            rust_field_name,
                            self.emit_value_parse(&expr_ref.type_, from),
                        );
                    }
                    _ => {
                        // The only non-constant expr field we ever need to parse is
                        // odd_length, so assert that we have that and then just write
                        // some one-off code for it.
                        assert_eq!(expr_ref.name, "odd_length");
                        outln!(
                            out,
                            "let (odd_length, remaining) = bool::try_parse(remaining)?;"
                        );
                        // And there is a matching special case to use this in request parsing.
                    }
                }
            }
            xcbdefs::FieldDef::VirtualLen(_) => {}
        }
    }

    fn emit_field_post_parse(&self, field: &xcbdefs::FieldDef, out: &mut Output) {
        match field {
            xcbdefs::FieldDef::Normal(normal_field) => {
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                self.emit_value_post_parse(&normal_field.type_, &rust_field_name, out);
            }
            xcbdefs::FieldDef::Expr(xcbdefs::ExprField {
                name,
                expr: xcbdefs::Expression::Value(v),
                ..
            }) => {
                assert!(u8::try_from(*v).is_ok());
                let rust_field_name = to_rust_variable_name(name);
                outln!(out, "if {} != {} {{", rust_field_name, v);
                outln!(out.indent(), "return Err(ParseError::InvalidValue);");
                outln!(out, "}}");
            }
            _ => (),
        }
    }

    fn emit_value_parse(&self, type_: &xcbdefs::FieldValueType, from: &str) -> String {
        let type_type = type_.type_.get_resolved();
        let rust_type = self.type_to_rust_type(type_type);
        let params = self.get_type_parse_params(type_type, from);
        format!("{}::try_parse({})?", rust_type, params.join(", "))
    }

    fn emit_value_post_parse(
        &self,
        type_: &xcbdefs::FieldValueType,
        var_name: &str,
        out: &mut Output,
    ) {
        if let xcbdefs::FieldValueSet::Enum(_) = type_.value_set {
            // Handle turning things into enum instances.
            outln!(out, "let {var} = {var}.into();", var = var_name);
        }
    }

    fn needs_post_parse(&self, type_: &xcbdefs::FieldValueType) -> bool {
        if let xcbdefs::FieldValueSet::Enum(_) = type_.value_set {
            true
        } else {
            false
        }
    }

    fn can_use_simple_list_parsing(&self, type_: &xcbdefs::FieldValueType) -> bool {
        self.get_type_parse_params(&type_.type_.get_resolved(), "")
            .len()
            == 1
            && !self.needs_post_parse(type_)
    }

    /// Returns `Some(bytes)` if the serialized result is a single
    /// array of bytes.
    fn emit_field_serialize(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &HashMap<String, DeducibleField>,
        mut wrap_field_ref: impl FnMut(&str) -> String,
        result_bytes: &mut Vec<String>,
        out: &mut Output,
    ) -> Option<String> {
        // Should only be used in fixed size fields
        assert!(field.size().is_some());
        // Just in case, but for fixed size fields it should not emit anything.
        self.emit_assert_for_field_serialize(field, deducible_fields, &mut wrap_field_ref, out);
        match field {
            xcbdefs::FieldDef::Pad(pad_field) => {
                match pad_field.kind {
                    xcbdefs::PadKind::Bytes(pad_size) => {
                        for _ in 0..pad_size {
                            result_bytes.push(String::from("0"));
                        }
                    }
                    // not fixed length
                    xcbdefs::PadKind::Align(_) => unreachable!(),
                }
                None
            }
            xcbdefs::FieldDef::Normal(normal_field) => {
                let field_size = normal_field.type_.type_.get_resolved().size().unwrap();
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                let bytes_name = postfix_var_name(&rust_field_name, "bytes");

                if let Some(deducible_field) = deducible_fields.get(&normal_field.name) {
                    self.emit_calc_deducible_field(
                        field,
                        deducible_field,
                        wrap_field_ref,
                        &rust_field_name,
                        out,
                    );
                    outln!(
                        out,
                        "let {} = {};",
                        bytes_name,
                        self.emit_value_serialize(&normal_field.type_, &rust_field_name, true),
                    );
                } else {
                    let src_value = wrap_field_ref(&normal_field.name);
                    outln!(
                        out,
                        "let {} = {};",
                        bytes_name,
                        self.emit_value_serialize(&normal_field.type_, &src_value, false),
                    );
                }
                for i in 0..field_size {
                    result_bytes.push(format!("{}[{}]", bytes_name, i));
                }
                Some(bytes_name)
            }
            xcbdefs::FieldDef::List(list_field) => {
                let list_length = list_field.length().unwrap();
                if self.rust_value_type_is_u8(&list_field.element_type) {
                    // Fixed-sized list with `u8` members
                    for i in 0..list_length {
                        result_bytes.push(format!("{}[{}]", wrap_field_ref(&list_field.name), i));
                    }
                    Some(wrap_field_ref(&list_field.name))
                } else {
                    let element_size = list_field.element_type.size().unwrap();
                    for i in 0..list_length {
                        let src_value = format!("{}[{}]", wrap_field_ref(&list_field.name), i);
                        let rust_field_name = to_rust_variable_name(&list_field.name);
                        let bytes_name =
                            postfix_var_name(&rust_field_name, &format!("{}_bytes", i));
                        outln!(
                            out,
                            "let {} = {};",
                            bytes_name,
                            self.emit_value_serialize(&list_field.element_type, &src_value, false),
                        );
                        for j in 0..element_size {
                            result_bytes.push(format!("{}[{}]", bytes_name, j));
                        }
                    }
                    None
                }
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                let field_size = field.size().unwrap(); // FIXME: use switch_field.size().unwrap()?
                let rust_field_name = to_rust_variable_name(&switch_field.name);
                let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                outln!(
                    out,
                    "let {} = {}.serialize();",
                    bytes_name,
                    wrap_field_ref(&switch_field.name),
                );
                for i in 0..field_size {
                    result_bytes.push(format!("{}[{}]", bytes_name, i));
                }
                Some(bytes_name)
            }
            xcbdefs::FieldDef::Expr(xcbdefs::ExprField {
                name,
                type_,
                expr: xcbdefs::Expression::Value(v),
            }) => {
                let field_size = type_.type_.get_resolved().size().unwrap();
                assert!(field_size == 1 && u8::try_from(*v).is_ok());
                let rust_field_name = to_rust_variable_name(name);
                let bytes_name = postfix_var_name(&rust_field_name, "bytes");

                outln!(out, "let {} = &[{}];", bytes_name, v,);
                result_bytes.push(format!("{}[{}]", bytes_name, 0));
                Some(bytes_name)
            }
            // the remaining field types are only used in request and replies,
            // which do not implement serialize
            _ => unreachable!(),
        }
    }

    fn emit_field_serialize_into(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &HashMap<String, DeducibleField>,
        mut wrap_field_ref: impl FnMut(&str) -> String,
        bytes_name: &str,
        out: &mut Output,
    ) {
        self.emit_assert_for_field_serialize(field, deducible_fields, &mut wrap_field_ref, out);
        match field {
            xcbdefs::FieldDef::Pad(pad_field) => match pad_field.kind {
                xcbdefs::PadKind::Bytes(pad_size) => {
                    outln!(out, "{}.extend_from_slice(&[0; {}]);", bytes_name, pad_size);
                }
                xcbdefs::PadKind::Align(pad_align) => outln!(
                    out,
                    "{}.extend_from_slice(&[0; {}][..({} - ({}.len() % {})) % {}]);",
                    bytes_name,
                    pad_align - 1,
                    pad_align,
                    bytes_name,
                    pad_align,
                    pad_align,
                ),
            },
            xcbdefs::FieldDef::Normal(normal_field) => {
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                if let Some(deducible_field) = deducible_fields.get(&normal_field.name) {
                    self.emit_calc_deducible_field(
                        field,
                        deducible_field,
                        &mut wrap_field_ref,
                        &rust_field_name,
                        out,
                    );
                    self.emit_value_serialize_into(
                        &normal_field.type_,
                        &wrap_field_ref(&normal_field.name),
                        true,
                        bytes_name,
                        out,
                    );
                } else {
                    self.emit_value_serialize_into(
                        &normal_field.type_,
                        &wrap_field_ref(&normal_field.name),
                        false,
                        bytes_name,
                        out,
                    );
                }
            }
            xcbdefs::FieldDef::List(list_field) => {
                if self.rust_value_type_is_u8(&list_field.element_type) {
                    // Fixed-sized list with `u8` members
                    outln!(
                        out,
                        "{}.extend_from_slice(&{});",
                        bytes_name,
                        wrap_field_ref(&list_field.name),
                    );
                } else if self.can_use_simple_list_parsing(&list_field.element_type) {
                    outln!(
                        out,
                        "{}.serialize_into({});",
                        wrap_field_ref(&list_field.name),
                        bytes_name
                    );
                } else {
                    outln!(
                        out,
                        "for element in {}.iter() {{",
                        wrap_field_ref(&list_field.name)
                    );
                    out.indented(|out| {
                        self.emit_value_serialize_into(
                            &list_field.element_type,
                            "element",
                            false,
                            "bytes",
                            out,
                        );
                    });
                    outln!(out, "}}");
                }
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                let ext_params_args = self.ext_params_to_call_args(
                    true,
                    to_rust_variable_name,
                    &*switch_field.external_params.borrow(),
                );
                outln!(
                    out,
                    "{}.serialize_into({}{});",
                    wrap_field_ref(&switch_field.name),
                    bytes_name,
                    ext_params_args,
                );
            }
            // the remaining field types are only used in request and replies,
            // which do not implement serialize
            _ => unreachable!(),
        }
    }

    /// Emits an assert that checks the consistency of expressions
    fn emit_assert_for_field_serialize(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &HashMap<String, DeducibleField>,
        mut wrap_field_ref: impl FnMut(&str) -> String,
        out: &mut Output,
    ) {
        match field {
            xcbdefs::FieldDef::Pad(_) => {}
            xcbdefs::FieldDef::Normal(_) => {}
            xcbdefs::FieldDef::List(list_field) => {
                let needs_assert =
                    !deducible_fields
                        .values()
                        .any(|deducible_field| match deducible_field {
                            DeducibleField::LengthOf(list_name, _) => *list_name == list_field.name,
                            DeducibleField::CaseSwitchExpr(_, _) => false,
                            DeducibleField::BitCaseSwitchExpr(_, _) => false,
                        })
                        && list_field.length_expr.is_some()
                        && list_field.length().is_none();

                if needs_assert {
                    let rust_field_name = to_rust_variable_name(&list_field.name);
                    let length_expr_str = self.expr_to_str(
                        list_field.length_expr.as_ref().unwrap(),
                        &mut wrap_field_ref,
                        true,
                        false,
                        false,
                    );
                    outln!(
                        out,
                        "assert_eq!({}.len(), usize::try_from({}).unwrap(), \"`{}` has an \
                         incorrect length\");",
                        wrap_field_ref(&list_field.name),
                        length_expr_str,
                        rust_field_name,
                    );
                }
            }
            xcbdefs::FieldDef::Switch(_) => {}
            xcbdefs::FieldDef::Fd(_) => {}
            xcbdefs::FieldDef::FdList(fd_list_field) => {
                let needs_assert =
                    !deducible_fields
                        .values()
                        .any(|deducible_field| match deducible_field {
                            DeducibleField::LengthOf(list_name, _) => {
                                *list_name == fd_list_field.name
                            }
                            DeducibleField::CaseSwitchExpr(_, _) => false,
                            DeducibleField::BitCaseSwitchExpr(_, _) => false,
                        })
                        && fd_list_field.length().is_none();

                if needs_assert {
                    let rust_field_name = to_rust_variable_name(&fd_list_field.name);
                    let length_expr_str = self.expr_to_str(
                        &fd_list_field.length_expr,
                        &mut wrap_field_ref,
                        true,
                        false,
                        false,
                    );
                    outln!(
                        out,
                        "assert_eq!({}.len(), usize::try_from({}).unwrap(), \"`{}` has an \
                         incorrect length\");",
                        wrap_field_ref(&fd_list_field.name),
                        length_expr_str,
                        rust_field_name,
                    );
                }
            }
            xcbdefs::FieldDef::Expr(_) => {}
            xcbdefs::FieldDef::VirtualLen(_) => {}
        }
    }

    /// Emits an assert that checks the consistency of switch expressions
    fn emit_assert_for_switch_serialize(&self, switch: &xcbdefs::SwitchField, out: &mut Output) {
        let rust_field_name = to_rust_variable_name(&switch.name);
        let switch_expr_str =
            self.expr_to_str(&switch.expr, to_rust_variable_name, true, true, false);
        outln!(
            out,
            "assert_eq!(self.switch_expr(), {}, \"switch `{}` has an inconsistent \
             discriminant\");",
            switch_expr_str,
            rust_field_name,
        );
    }

    fn emit_value_serialize(
        &self,
        type_: &xcbdefs::FieldValueType,
        value: &str,
        was_deduced: bool,
    ) -> String {
        // Deduced fields are not converted to their enum value
        if let (false, Some(enum_def)) = (was_deduced, self.use_enum_type_in_field(type_)) {
            let enum_info = self.caches.borrow().enum_info(&enum_def);
            let (_, max_wire_size) = enum_info.wire_size.unwrap();
            let rust_wire_type = self.type_to_rust_type(type_.type_.get_resolved());
            let current_wire_size = type_.type_.get_resolved().size().unwrap();

            if max_wire_size > 1 && u32::from(max_wire_size / 8) > current_wire_size {
                format!(
                    "(u{}::from({}) as {}).serialize()",
                    max_wire_size, value, rust_wire_type,
                )
            } else {
                format!("{}::from({}).serialize()", rust_wire_type, value)
            }
        } else {
            format!("{}.serialize()", value)
        }
    }

    fn emit_value_serialize_into(
        &self,
        type_: &xcbdefs::FieldValueType,
        value: &str,
        was_deduced: bool,
        bytes_var: &str,
        out: &mut Output,
    ) {
        // Deduced fields are not converted to their enum value
        if let (false, Some(enum_def)) = (was_deduced, self.use_enum_type_in_field(type_)) {
            let enum_info = self.caches.borrow().enum_info(&enum_def);
            let (_, max_wire_size) = enum_info.wire_size.unwrap();
            let rust_wire_type = self.type_to_rust_type(type_.type_.get_resolved());
            let current_wire_size = type_.type_.get_resolved().size().unwrap();

            if max_wire_size > 1 && u32::from(max_wire_size / 8) > current_wire_size {
                outln!(
                    out,
                    "(u{}::from({}) as {}).serialize_into({});",
                    max_wire_size,
                    value,
                    rust_wire_type,
                    bytes_var,
                );
            } else {
                outln!(
                    out,
                    "{}::from({}).serialize_into({});",
                    rust_wire_type,
                    value,
                    bytes_var,
                );
            }
        } else {
            outln!(out, "{}.serialize_into({});", value, bytes_var);
        }
    }

    fn emit_calc_deducible_field(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_field: &DeducibleField,
        mut wrap_field_ref: impl FnMut(&str) -> String,
        dst_var_name: &str,
        out: &mut Output,
    ) {
        match deducible_field {
            DeducibleField::LengthOf(list_field_name, op) => {
                if let xcbdefs::FieldDef::Normal(normal_field) = field {
                    let rust_type = self.field_value_type_to_rust_type(&normal_field.type_);
                    let rust_list_field_name = to_rust_variable_name(list_field_name);
                    let msg = format!("`{}` has too many elements", rust_list_field_name);
                    let list_len = format!("{}.len()", wrap_field_ref(&rust_list_field_name));
                    let value = match op {
                        DeducibleLengthFieldOp::None => {
                            format!("{}::try_from({}).expect(\"{}\")", rust_type, list_len, msg)
                        }
                        DeducibleLengthFieldOp::Mul(n) => format!(
                            "{}::try_from({}).ok().and_then(|len| \
                             len.checked_mul({})).expect(\"{}\")",
                            rust_type, list_len, n, msg,
                        ),
                        DeducibleLengthFieldOp::Div(n) => {
                            outln!(
                                out,
                                "assert_eq!({} % {}, 0, \"`{}` has an incorrect length, must be a \
                                 multiple of {}\");",
                                list_len,
                                n,
                                rust_list_field_name,
                                n,
                            );
                            format!(
                                "{}::try_from({} / {}).expect(\"{}\")",
                                rust_type, list_len, n, msg,
                            )
                        }
                    };
                    outln!(out, "let {} = {};", dst_var_name, value);
                } else {
                    unreachable!();
                }
            }
            DeducibleField::CaseSwitchExpr(switch_field_name, op) => {
                match op {
                    DeducibleFieldOp::None => (),
                    _ => unreachable!(),
                }
                if let xcbdefs::FieldDef::Normal(normal_field) = field {
                    let field_type = normal_field.type_.type_.get_resolved();
                    let rust_field_type = self.type_to_rust_type(field_type);
                    if let xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) = field_type {
                        outln!(
                            out,
                            "let {} = {}.switch_expr();",
                            dst_var_name,
                            wrap_field_ref(&switch_field_name),
                        );
                    } else {
                        outln!(
                            out,
                            "let {} = {}::try_from({}.switch_expr()).unwrap();",
                            dst_var_name,
                            rust_field_type,
                            wrap_field_ref(&switch_field_name),
                        );
                    }
                } else {
                    unreachable!();
                }
            }
            DeducibleField::BitCaseSwitchExpr(switch_field_name, op) => {
                if let xcbdefs::FieldDef::Normal(normal_field) = field {
                    let field_type = normal_field.type_.type_.get_resolved();
                    let rust_field_type = self.type_to_rust_type(field_type);
                    let op_str = match op {
                        DeducibleFieldOp::None => String::new(),
                        DeducibleFieldOp::Or(or_expr) => format!(
                            " | {expr}",
                            expr = self.expr_to_str(
                                &*or_expr,
                                &mut wrap_field_ref,
                                true,
                                false,
                                true,
                            )
                        ),
                    };
                    if let xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) = field_type {
                        assert!(op_str.is_empty());
                        outln!(
                            out,
                            "let {} = {}.switch_expr();",
                            dst_var_name,
                            wrap_field_ref(&switch_field_name),
                        );
                    } else {
                        outln!(
                            out,
                            "let {} = {}::try_from({}.switch_expr(){}).unwrap();",
                            dst_var_name,
                            rust_field_type,
                            wrap_field_ref(&switch_field_name),
                            op_str,
                        );
                    }
                } else {
                    unreachable!();
                }
            }
        }
    }

    fn rust_value_type_is_u8(&self, type_: &xcbdefs::FieldValueType) -> bool {
        if self.use_enum_type_in_field(type_).is_some() {
            false
        } else {
            let original_type = match type_.type_.get_resolved() {
                xcbdefs::TypeRef::Alias(type_alias_def) => {
                    type_alias_def.upgrade().unwrap().get_original_type()
                }
                type_ => type_.clone(),
            };
            match original_type {
                xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card8)
                | xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Byte)
                | xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Char)
                | xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Void) => true,
                _ => false,
            }
        }
    }

    fn use_enum_type_in_field(
        &self,
        type_: &xcbdefs::FieldValueType,
    ) -> Option<Rc<xcbdefs::EnumDef>> {
        if let xcbdefs::FieldValueSet::Enum(ref enum_) = type_.value_set {
            let enum_def = match enum_.get_resolved() {
                xcbdefs::TypeRef::Enum(enum_def) => enum_def.upgrade().unwrap(),
                _ => unreachable!(),
            };
            Some(enum_def)
        } else {
            None
        }
    }

    fn emit_doc(&self, doc: &xcbdefs::Doc, out: &mut Output) {
        let mut has_doc = false;
        if let Some(ref brief) = doc.brief {
            outln!(out, "/// {}.", brief);
            has_doc = true;
        }
        if let Some(ref description) = doc.description {
            if has_doc {
                outln!(out, "///");
            }
            has_doc = true;
            for line in description.trim().split('\n') {
                if line.trim().is_empty() {
                    outln!(out, "///");
                } else {
                    outln!(out, "/// {}", line.trim_end());
                }
            }
        }
        if !doc.fields.is_empty() {
            if has_doc {
                outln!(out, "///");
            }
            has_doc = true;
            outln!(out, "/// # Fields");
            outln!(out, "///");
            for field in doc.fields.iter() {
                let text = format!(
                    " * `{}` - {}",
                    field.name,
                    field.doc.as_deref().unwrap_or("").trim(),
                );
                // Prevent rustdoc interpreting many leading spaces as code examples (?)
                for line in text.trim().split('\n') {
                    let line = line.trim();
                    if line.trim().is_empty() {
                        outln!(out, "///");
                    } else {
                        outln!(out, "/// {}", line);
                    }
                }
            }
        }
        if !doc.errors.is_empty() {
            if has_doc {
                outln!(out, "///");
            }
            has_doc = true;
            outln!(out, "/// # Errors");
            outln!(out, "///");
            for error in doc.errors.iter() {
                let text = format!(
                    "* `{}` - {}",
                    error.type_,
                    error.doc.as_deref().unwrap_or("").trim(),
                );
                for line in text.split('\n') {
                    outln!(out, "/// {}", line.trim_end());
                }
            }
        }
        if !doc.sees.is_empty() {
            if has_doc {
                outln!(out, "///");
            }
            has_doc = true;
            outln!(out, "/// # See");
            outln!(out, "///");
            for see in doc.sees.iter() {
                outln!(out, "/// * `{}`: {}", see.name, see.type_);
            }
        }
        if let Some(ref example) = doc.example {
            if has_doc {
                outln!(out, "///");
            }
            has_doc = true;
            outln!(out, "/// # Example");
            outln!(out, "///");
            outln!(out, "/// ```text");
            for line in example.trim().split('\n') {
                if line.trim().is_empty() {
                    outln!(out, "///");
                } else {
                    outln!(out, "/// {}", line.trim_end());
                }
            }
            outln!(out, "/// ```");
        }
        // Empty doc?
        assert!(has_doc);
    }

    #[inline]
    fn expr_to_str(
        &self,
        expr: &xcbdefs::Expression,
        mut wrap_field_ref: impl FnMut(&str) -> String,
        panic_on_overflow: bool,
        cast_to_u32: bool,
        needs_parens: bool,
    ) -> String {
        self.expr_to_str_impl(
            expr,
            &mut wrap_field_ref,
            panic_on_overflow,
            cast_to_u32,
            needs_parens,
        )
    }

    fn expr_to_str_impl(
        &self,
        expr: &xcbdefs::Expression,
        wrap_field_ref: &mut dyn FnMut(&str) -> String,
        panic_on_overflow: bool,
        cast_to_u32: bool,
        needs_parens: bool,
    ) -> String {
        match expr {
            xcbdefs::Expression::BinaryOp(bin_op_expr) => {
                let err_handler = if panic_on_overflow {
                    ".unwrap()"
                } else {
                    ".ok_or(ParseError::InvalidExpression)?"
                };
                match bin_op_expr.operator {
                    xcbdefs::BinaryOperator::Add => format!(
                        "{}.checked_add({}){}",
                        self.expr_to_str_impl(
                            &bin_op_expr.lhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        ),
                        self.expr_to_str_impl(
                            &bin_op_expr.rhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            false,
                        ),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::Sub => format!(
                        "{}.checked_sub({}){}",
                        self.expr_to_str_impl(
                            &bin_op_expr.lhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        ),
                        self.expr_to_str_impl(
                            &bin_op_expr.rhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            false,
                        ),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::Mul => format!(
                        "{}.checked_mul({}){}",
                        self.expr_to_str_impl(
                            &bin_op_expr.lhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        ),
                        self.expr_to_str_impl(
                            &bin_op_expr.rhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            false,
                        ),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::Div => format!(
                        "{}.checked_div({}){}",
                        self.expr_to_str_impl(
                            &bin_op_expr.lhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        ),
                        self.expr_to_str_impl(
                            &bin_op_expr.rhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            false,
                        ),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::And => {
                        let lhs_str = self.expr_to_str_impl(
                            &bin_op_expr.lhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        );
                        let rhs_str = self.expr_to_str_impl(
                            &bin_op_expr.rhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        );
                        if needs_parens {
                            format!("({} & {})", lhs_str, rhs_str)
                        } else {
                            format!("{} & {}", lhs_str, rhs_str)
                        }
                    }
                    xcbdefs::BinaryOperator::Or => {
                        let lhs_str = self.expr_to_str_impl(
                            &bin_op_expr.lhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        );
                        let rhs_str = self.expr_to_str_impl(
                            &bin_op_expr.rhs,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        );
                        if needs_parens {
                            format!("({} | {})", lhs_str, rhs_str)
                        } else {
                            format!("{} | {}", lhs_str, rhs_str)
                        }
                    }
                    // I'm not sure know how to handle overflow here,
                    // but this operator never appears in the XMLs
                    xcbdefs::BinaryOperator::Shl => unimplemented!(),
                }
            }
            xcbdefs::Expression::UnaryOp(unary_op_expr) => match unary_op_expr.operator {
                xcbdefs::UnaryOperator::Not => {
                    let rhs_str = self.expr_to_str_impl(
                        &unary_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        true,
                        true,
                    );
                    if needs_parens {
                        format!("(!{})", rhs_str)
                    } else {
                        format!("!{}", rhs_str)
                    }
                }
            },
            xcbdefs::Expression::FieldRef(field_ref_expr) => {
                let resolved_field_ref = field_ref_expr.resolved.get().unwrap();
                let value = match resolved_field_ref.ref_kind {
                    xcbdefs::FieldRefKind::LocalField => wrap_field_ref(&field_ref_expr.field_name),
                    xcbdefs::FieldRefKind::ExtParam => wrap_field_ref(&field_ref_expr.field_name),
                    xcbdefs::FieldRefKind::SumOfRef => {
                        format!("x.{}", to_rust_variable_name(&field_ref_expr.field_name))
                    }
                };
                let field_is_card32 = match resolved_field_ref.field_type {
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) => true,
                    _ => false,
                };
                if cast_to_u32 && !field_is_card32 {
                    format!("u32::from({})", value)
                } else {
                    value
                }
            }
            xcbdefs::Expression::ParamRef(param_ref_expr) => {
                let rust_field_name = to_rust_variable_name(&param_ref_expr.field_name);
                let param_is_card32 = match param_ref_expr.type_.get_resolved() {
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) => true,
                    _ => false,
                };
                if cast_to_u32 && !param_is_card32 {
                    format!("u32::from({})", rust_field_name)
                } else {
                    rust_field_name
                }
            }
            xcbdefs::Expression::EnumRef(enum_ref_expr) => {
                let rust_enum_type = self.type_to_rust_type(enum_ref_expr.enum_.get_resolved());
                let rust_variant_name = ename_to_rust(&enum_ref_expr.variant);
                format!("u32::from({}::{})", rust_enum_type, rust_variant_name)
            }
            xcbdefs::Expression::PopCount(pop_count_expr) => {
                let arg = self.expr_to_str_impl(
                    pop_count_expr,
                    wrap_field_ref,
                    panic_on_overflow,
                    false,
                    true,
                );
                format!("{}.count_ones()", arg)
            }
            xcbdefs::Expression::SumOf(sum_of_expr) => {
                // nested sum-of not supported
                assert_ne!(
                    sum_of_expr.resolved_field.get().unwrap().ref_kind,
                    xcbdefs::FieldRefKind::SumOfRef
                );
                let field_value = match sum_of_expr.resolved_field.get().unwrap().ref_kind {
                    xcbdefs::FieldRefKind::LocalField => wrap_field_ref(&sum_of_expr.field_name),
                    xcbdefs::FieldRefKind::ExtParam => {
                        to_rust_variable_name(&sum_of_expr.field_name)
                    }
                    // nested sum-of not supported
                    xcbdefs::FieldRefKind::SumOfRef => unreachable!(),
                };
                if panic_on_overflow {
                    format!(
                        "{}.iter().fold(0u32, |acc, x| acc.checked_add({}).unwrap())",
                        field_value,
                        self.expr_to_str_impl(
                            &sum_of_expr.operand,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        ),
                    )
                } else {
                    format!(
                        "{}.iter().try_fold(0u32, |acc, x| \
                         acc.checked_add({}).ok_or(ParseError::InvalidExpression))?",
                        field_value,
                        self.expr_to_str_impl(
                            &sum_of_expr.operand,
                            wrap_field_ref,
                            panic_on_overflow,
                            true,
                            true,
                        ),
                    )
                }
            }
            xcbdefs::Expression::ListElementRef => {
                if cast_to_u32 {
                    "u32::from(*x)".into()
                } else if needs_parens {
                    "(*x)".into()
                } else {
                    "*x".into()
                }
            }
            xcbdefs::Expression::Value(value) => {
                let value_str = format_literal_integer(*value);
                format!("{}u32", value_str)
            }
            xcbdefs::Expression::Bit(bit) => {
                let value_str = format_literal_integer(1u32 << *bit);
                format!("{}u32", value_str)
            }
        }
    }

    fn name_is_used_by_non_enum(&self, name: &str, ns: &xcbdefs::Namespace) -> bool {
        for type_def in ns.type_defs.borrow().values() {
            match type_def {
                xcbdefs::TypeDef::Struct(struct_def) => {
                    if self.get_struct_rust_name(struct_def) == *name {
                        return true;
                    }
                }
                xcbdefs::TypeDef::Union(union_def) => {
                    if self.get_union_rust_name(union_def) == *name {
                        return true;
                    }
                }
                xcbdefs::TypeDef::EventStruct(event_struct_def) => {
                    if self.get_event_struct_rust_name(event_struct_def) == *name {
                        return true;
                    }
                }
                xcbdefs::TypeDef::Xid(xid_type_def) => {
                    if self.get_xid_type_rust_name(xid_type_def) == *name {
                        return true;
                    }
                }
                xcbdefs::TypeDef::XidUnion(xid_union_def) => {
                    if self.get_xid_union_rust_name(xid_union_def) == *name {
                        return true;
                    }
                }
                xcbdefs::TypeDef::Enum(_) => {}
                xcbdefs::TypeDef::Alias(type_alias_def) => {
                    if self.get_type_alias_rust_name(type_alias_def) == *name {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_normal_type_name(&self, id: usize, name: &str) -> String {
        let mut caches = self.caches.borrow_mut();
        match caches.rust_type_names.entry(id) {
            HashMapEntry::Occupied(entry) => entry.get().clone(),
            HashMapEntry::Vacant(entry) => {
                let name = to_rust_type_name(name);
                entry.insert(name.clone());
                name
            }
        }
    }

    fn get_struct_rust_name(&self, struct_def: &xcbdefs::StructDef) -> String {
        let id = struct_def as *const _ as usize;
        self.get_normal_type_name(id, &struct_def.name)
    }

    fn get_union_rust_name(&self, union_def: &xcbdefs::UnionDef) -> String {
        let id = union_def as *const _ as usize;
        self.get_normal_type_name(id, &union_def.name)
    }

    fn get_event_struct_rust_name(&self, event_struct_def: &xcbdefs::EventStructDef) -> String {
        let id = event_struct_def as *const _ as usize;
        self.get_normal_type_name(id, &event_struct_def.name)
    }

    fn get_xid_type_rust_name(&self, xid_type_def: &xcbdefs::XidTypeDef) -> String {
        let id = xid_type_def as *const _ as usize;
        self.get_normal_type_name(id, &xid_type_def.name)
    }

    fn get_xid_union_rust_name(&self, xid_union_def: &xcbdefs::XidUnionDef) -> String {
        let id = xid_union_def as *const _ as usize;
        self.get_normal_type_name(id, &xid_union_def.name)
    }

    fn get_enum_rust_name(&self, enum_def: &xcbdefs::EnumDef) -> String {
        let ns = enum_def.namespace.upgrade().unwrap();
        let mut name = to_rust_enum_type_name(&enum_def.name);
        if self.name_is_used_by_non_enum(&name, &ns) {
            name.push_str("Enum");
        }
        name
    }

    fn get_type_alias_rust_name(&self, type_alias_def: &xcbdefs::TypeAliasDef) -> String {
        let id = type_alias_def as *const _ as usize;
        self.get_normal_type_name(id, &type_alias_def.new_name)
    }

    /// Gathers information about the fields of a request,
    /// returning a `GatheredRequestFields`.
    fn gather_request_fields(
        &self,
        request_def: &xcbdefs::RequestDef,
        deducible_fields: &HashMap<String, DeducibleField>,
    ) -> GatheredRequestFields {
        let mut letter_iter = b'A'..=b'Z';

        let mut needs_lifetime = false;
        let mut args = Vec::new();
        let mut request_args = Vec::new();
        let mut generics = Vec::new();
        let mut preamble = Vec::new();
        let mut single_fds = Vec::new();
        let mut fd_lists = Vec::new();

        let ns = request_def.namespace.upgrade().unwrap();
        let is_change_property = request_def.name == "ChangeProperty" && ns.header == "xproto";
        let is_get_property = request_def.name == "GetProperty" && ns.header == "xproto";
        let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";

        for field in request_def.fields.borrow().iter() {
            if !self.field_is_visible(field, deducible_fields) {
                continue;
            }
            match field.name() {
                Some("major_opcode") | Some("minor_opcode") | Some("length") => continue,
                _ => {}
            }

            match field {
                xcbdefs::FieldDef::Pad(_) => unreachable!(),
                xcbdefs::FieldDef::Normal(normal_field) => {
                    let rust_field_name = to_rust_variable_name(&normal_field.name);
                    let rust_field_type = self.field_value_type_to_rust_type(&normal_field.type_);
                    let use_into = if ((is_change_property || is_get_property)
                        && normal_field.name == "property")
                        || (is_change_property && normal_field.name == "type")
                    {
                        true
                    } else if self.use_enum_type_in_field(&normal_field.type_).is_none() {
                        match normal_field.type_.value_set {
                            xcbdefs::FieldValueSet::None => false,
                            _ => true,
                        }
                    } else {
                        false
                    };

                    if use_into {
                        let preamble_part = format!(
                            "let {}: {} = {}.into();",
                            rust_field_name, rust_field_type, rust_field_name,
                        );
                        let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                        let where_ = format!("Into<{}>", rust_field_type);
                        args.push((rust_field_name.clone(), Type::Simple(generic_param.clone())));
                        request_args.push((rust_field_name, Type::Simple(rust_field_type)));
                        generics.push((generic_param, where_));
                        preamble.push(preamble_part);
                    } else {
                        args.push((
                            rust_field_name.clone(),
                            Type::Simple(rust_field_type.clone()),
                        ));
                        request_args.push((rust_field_name, Type::Simple(rust_field_type)));
                    }
                }
                xcbdefs::FieldDef::List(list_field) => {
                    if is_send_event && list_field.name == "event" {
                        let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                        args.push((list_field.name.clone(), Type::Simple(generic_param.clone())));
                        request_args.push((
                            list_field.name.clone(),
                            Type::VariableOwnershipRawBytes("[u8; 32]".into()),
                        ));
                        generics.push((generic_param, String::from("Into<[u8; 32]>")));
                        preamble.push(String::from("let event = Cow::Owned(event.into());"));
                        needs_lifetime = true;
                    } else {
                        let element_type =
                            self.field_value_type_to_rust_type(&list_field.element_type);
                        let rust_field_name = to_rust_variable_name(&list_field.name);
                        let rust_field_type =
                            if self.rust_value_type_is_u8(&list_field.element_type) {
                                if let Some(list_len) = list_field.length() {
                                    Type::VariableOwnershipRawBytes(format!(
                                        "[{}; {}]",
                                        element_type, list_len
                                    ))
                                } else {
                                    Type::VariableOwnershipRawBytes(format!("[{}]", element_type))
                                }
                            } else {
                                assert_eq!(
                                list_field.length(),
                                None,
                                "Fixed length arrays of types other than u8 are not implemented"
                            );

                                Type::VariableOwnership(format!("[{}]", element_type))
                            };
                        args.push((rust_field_name.clone(), rust_field_type.clone()));
                        request_args.push((rust_field_name, rust_field_type));
                        needs_lifetime = true;
                    }
                }
                xcbdefs::FieldDef::Switch(switch_field) => {
                    let rust_field_name = to_rust_variable_name(&switch_field.name);
                    let rust_field_type = Type::VariableOwnership(format!(
                        "{}Aux",
                        to_rust_type_name(&request_def.name)
                    ));
                    args.push((rust_field_name.clone(), rust_field_type.clone()));
                    request_args.push((rust_field_name, rust_field_type));
                    needs_lifetime = true;
                }
                xcbdefs::FieldDef::Fd(fd_field) => {
                    let rust_field_name = to_rust_variable_name(&fd_field.name);
                    let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                    let preamble_part = format!(
                        "let {}: RawFdContainer = {}.into();",
                        rust_field_name, rust_field_name,
                    );
                    args.push((rust_field_name.clone(), Type::Simple(generic_param.clone())));
                    request_args.push((rust_field_name, Type::Simple("RawFdContainer".into())));
                    generics.push((generic_param, "Into<RawFdContainer>".into()));
                    preamble.push(preamble_part);
                    single_fds.push(fd_field.name.clone());
                }
                xcbdefs::FieldDef::FdList(fd_list_field) => {
                    let rust_field_name = to_rust_variable_name(&fd_list_field.name);
                    args.push((
                        rust_field_name.clone(),
                        Type::Simple("Vec<RawFdContainer>".into()),
                    ));
                    request_args
                        .push((rust_field_name, Type::Simple("Vec<RawFdContainer>".into())));
                    fd_lists.push(fd_list_field.name.clone());
                }
                xcbdefs::FieldDef::Expr(_) => unreachable!(),
                xcbdefs::FieldDef::VirtualLen(_) => unreachable!(),
            }
        }

        let reply_has_fds = request_def
            .reply
            .as_ref()
            .map(|reply_def| {
                reply_def.fields.borrow().iter().any(|field| match field {
                    xcbdefs::FieldDef::Fd(_) => true,
                    xcbdefs::FieldDef::FdList(_) => true,
                    _ => false,
                })
            })
            .unwrap_or(false);

        assert_eq!(args.len(), request_args.len());
        GatheredRequestFields {
            reply_has_fds,
            needs_lifetime,
            args,
            request_args,
            generics,
            preamble,
            single_fds,
            fd_lists,
        }
    }

    fn ext_params_to_arg_defs(
        &self,
        begin_with_comma: bool,
        ext_params: &[xcbdefs::ExternalParam],
    ) -> String {
        let mut s = String::new();
        for (i, ext_param) in ext_params.iter().enumerate() {
            if i != 0 || begin_with_comma {
                s.push_str(", ");
            }
            s.push_str(&to_rust_variable_name(&ext_param.name));
            s.push_str(": ");
            s.push_str(&self.type_to_rust_type(&ext_param.type_))
        }
        s
    }

    fn ext_params_to_call_args(
        &self,
        begin_with_comma: bool,
        mut wrap_name: impl FnMut(&str) -> String,
        ext_params: &[xcbdefs::ExternalParam],
    ) -> String {
        let mut s = String::new();
        for (i, ext_param) in ext_params.iter().enumerate() {
            if i != 0 || begin_with_comma {
                s.push_str(", ");
            }
            s.push_str(&wrap_name(&ext_param.name));
        }
        s
    }

    /// Returns the Rust type for `field`.
    fn field_to_rust_type(&self, field: &xcbdefs::FieldDef, switch_prefix: &str) -> String {
        match field {
            xcbdefs::FieldDef::Pad(_) => unreachable!(),
            xcbdefs::FieldDef::Normal(normal_field) => {
                self.field_value_type_to_rust_type(&normal_field.type_)
            }
            xcbdefs::FieldDef::List(list_field) => {
                let element_type = self.field_value_type_to_rust_type(&list_field.element_type);
                if let Some(list_len) = list_field.length() {
                    format!("[{}; {}]", element_type, list_len)
                } else {
                    format!("Vec<{}>", element_type)
                }
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                format!("{}{}", switch_prefix, to_rust_type_name(&switch_field.name))
            }
            xcbdefs::FieldDef::Fd(_) => "RawFdContainer".into(),
            xcbdefs::FieldDef::FdList(_) => "Vec<RawFdContainer>".into(),
            xcbdefs::FieldDef::Expr(_) => unreachable!(),
            xcbdefs::FieldDef::VirtualLen(_) => unreachable!(),
        }
    }

    /// Returns the Rust type for `type_`.
    fn type_to_rust_type(&self, type_: &xcbdefs::TypeRef) -> String {
        match type_ {
            xcbdefs::TypeRef::BuiltIn(builtin_type) => match builtin_type {
                xcbdefs::BuiltInType::Card8 => "u8".into(),
                xcbdefs::BuiltInType::Card16 => "u16".into(),
                xcbdefs::BuiltInType::Card32 => "u32".into(),
                xcbdefs::BuiltInType::Card64 => "u64".into(),
                xcbdefs::BuiltInType::Int8 => "i8".into(),
                xcbdefs::BuiltInType::Int16 => "i16".into(),
                xcbdefs::BuiltInType::Int32 => "i32".into(),
                xcbdefs::BuiltInType::Int64 => "i64".into(),
                xcbdefs::BuiltInType::Byte => "u8".into(),
                xcbdefs::BuiltInType::Bool => "bool".into(),
                xcbdefs::BuiltInType::Char => "u8".into(),
                xcbdefs::BuiltInType::Float => "f32".into(),
                xcbdefs::BuiltInType::Double => "f64".into(),
                xcbdefs::BuiltInType::Void => "u8".into(),
            },
            xcbdefs::TypeRef::Struct(struct_def) => {
                let struct_def = struct_def.upgrade().unwrap();
                let ns = struct_def.namespace.upgrade().unwrap();
                let rust_name = self.get_struct_rust_name(&struct_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
            xcbdefs::TypeRef::Union(union_def) => {
                let union_def = union_def.upgrade().unwrap();
                let ns = union_def.namespace.upgrade().unwrap();
                let rust_name = self.get_union_rust_name(&union_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
            xcbdefs::TypeRef::EventStruct(event_struct_def) => {
                let event_struct_def = event_struct_def.upgrade().unwrap();
                let ns = event_struct_def.namespace.upgrade().unwrap();
                let rust_name = self.get_event_struct_rust_name(&event_struct_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
            xcbdefs::TypeRef::Xid(xid_type_def) => {
                let xid_type_def = xid_type_def.upgrade().unwrap();
                let ns = xid_type_def.namespace.upgrade().unwrap();
                let rust_name = self.get_xid_type_rust_name(&xid_type_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
            xcbdefs::TypeRef::XidUnion(xid_union_def) => {
                let xid_union_def = xid_union_def.upgrade().unwrap();
                let ns = xid_union_def.namespace.upgrade().unwrap();
                let rust_name = self.get_xid_union_rust_name(&xid_union_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
            xcbdefs::TypeRef::Enum(enum_def) => {
                let enum_def = enum_def.upgrade().unwrap();
                let ns = enum_def.namespace.upgrade().unwrap();
                let rust_name = self.get_enum_rust_name(&enum_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
            xcbdefs::TypeRef::Alias(type_alias_def) => {
                let type_alias_def = type_alias_def.upgrade().unwrap();
                let ns = type_alias_def.namespace.upgrade().unwrap();
                let rust_name = self.get_type_alias_rust_name(&type_alias_def);
                self.type_name_to_rust_type(&rust_name, &ns)
            }
        }
    }

    /// Returns the Rust type for `value_type`.
    fn field_value_type_to_rust_type(&self, value_type: &xcbdefs::FieldValueType) -> String {
        if let Some(enum_def) = self.use_enum_type_in_field(value_type) {
            let ns = enum_def.namespace.upgrade().unwrap();
            self.type_name_to_rust_type(&self.get_enum_rust_name(&enum_def), &ns)
        } else {
            self.type_to_rust_type(value_type.type_.get_resolved())
        }
    }

    /// Gets the Rust type for a type with name `name` and defined
    /// in `ns`.
    fn type_name_to_rust_type(&self, name: &str, ns: &xcbdefs::Namespace) -> String {
        if ns.header == self.ns.header {
            // Same namespace, can be accessed directly
            name.into()
        } else {
            // Different namespace, requires specfying the module
            format!("{}::{}", ns.header, name)
        }
    }

    fn event_to_rust_type(&self, event: &xcbdefs::EventRef) -> String {
        self.event_def_to_rust_type(&event.as_event_def())
    }

    fn event_def_to_rust_type(&self, event: &xcbdefs::EventDef) -> String {
        let name = event.name();
        let namespace = event.namespace();
        let rust_name = format!("{}Event", to_rust_type_name(name));
        self.type_name_to_rust_type(&rust_name, &namespace)
    }

    /// Returns the parameters needed by the `try_parse` function for `type_`.
    fn get_type_parse_params(&self, type_: &xcbdefs::TypeRef, remaining: &str) -> Vec<String> {
        let mut params = Vec::new();
        params.push(String::from(remaining));

        let original_type = if let xcbdefs::TypeRef::Alias(type_alias_def) = type_ {
            type_alias_def.upgrade().unwrap().get_original_type()
        } else {
            type_.clone()
        };

        if let xcbdefs::TypeRef::Struct(struct_def) = original_type {
            let struct_def = struct_def.upgrade().unwrap();
            for ext_param in struct_def.external_params.borrow().iter() {
                params.push(to_rust_variable_name(&ext_param.name));
            }
        }

        params
    }

    /// Gets the traits that can be implemented for `struct_def`.
    fn get_derives_for_struct_def(&self, struct_def: &xcbdefs::StructDef) -> Derives {
        let id = struct_def as *const xcbdefs::StructDef as usize;

        let caches = self.caches.borrow();
        if let Some(derives) = caches.derives.get(&id).cloned() {
            derives
        } else {
            drop(caches);
            let mut derives = Derives::all();
            self.filter_derives_for_fields(&mut derives, &*struct_def.fields.borrow(), false);
            self.caches.borrow_mut().derives.insert(id, derives);
            derives
        }
    }

    /// Clears traits from `derives` that cannot be implemented by
    /// `fields`.
    fn filter_derives_for_fields(
        &self,
        derives: &mut Derives,
        fields: &[xcbdefs::FieldDef],
        will_use_cows: bool,
    ) {
        for field in fields.iter() {
            match field {
                xcbdefs::FieldDef::Pad(_) => {}
                xcbdefs::FieldDef::Normal(normal_field) => {
                    self.filter_derives_for_type(derives, normal_field.type_.type_.get_resolved());
                }
                xcbdefs::FieldDef::List(list_field) => {
                    self.filter_derives_for_type(
                        derives,
                        list_field.element_type.type_.get_resolved(),
                    );

                    if !list_field.has_fixed_length() {
                        // Variable length list, represented as Vec
                        derives.copy = false;
                    }
                    if will_use_cows {
                        // Lists will all be CoWs which are not Copy.
                        derives.copy = false;
                    }
                }
                xcbdefs::FieldDef::Switch(switch_field) => {
                    for case in switch_field.cases.iter() {
                        self.filter_derives_for_fields(
                            derives,
                            &*case.fields.borrow(),
                            will_use_cows,
                        );
                    }
                    if will_use_cows {
                        // Switches will be CoWs for the Aux struct so it cannot be Copy.
                        derives.copy = false;
                    }
                }
                xcbdefs::FieldDef::Fd(_) | xcbdefs::FieldDef::FdList(_) => {
                    // RawFdContainer cannot be cloned
                    derives.clone = false;
                    derives.copy = false;
                }
                xcbdefs::FieldDef::Expr(_) => {}
                xcbdefs::FieldDef::VirtualLen(_) => {}
            }
        }
    }

    /// Clears traits from `derives` that cannot be implemented by
    /// `type_`.
    fn filter_derives_for_type(&self, derives: &mut Derives, type_: &xcbdefs::TypeRef) {
        match type_ {
            xcbdefs::TypeRef::BuiltIn(builtin_type) => {
                match builtin_type {
                    xcbdefs::BuiltInType::Float | xcbdefs::BuiltInType::Double => {
                        // f32/f64 do not implement Eq
                        derives.eq = false;
                    }
                    _ => {}
                }
            }
            xcbdefs::TypeRef::Struct(struct_def) => {
                let struct_def = struct_def.upgrade().unwrap();
                let struct_derives = self.get_derives_for_struct_def(&struct_def);
                derives.intersect(struct_derives);
            }
            xcbdefs::TypeRef::Union(_) => {
                // Comparing unions makes no sense (in the current implementation)
                derives.partial_eq = false;
                derives.eq = false;
            }
            xcbdefs::TypeRef::EventStruct(_) => {
                // Event structs don't support equality tests.
                derives.partial_eq = false;
                derives.eq = false;
            }
            xcbdefs::TypeRef::Xid(_) => {}
            xcbdefs::TypeRef::XidUnion(_) => {}
            xcbdefs::TypeRef::Enum(_) => {}
            xcbdefs::TypeRef::Alias(type_alias_def) => {
                let type_alias_def = type_alias_def.upgrade().unwrap();
                self.filter_derives_for_type(derives, type_alias_def.old_name.get_resolved())
            }
        }
    }

    /// Whether the field is visible (i.e., appears in parsed rust structs)
    fn field_is_visible(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &HashMap<String, DeducibleField>,
    ) -> bool {
        match field {
            xcbdefs::FieldDef::Pad(_) => false,
            xcbdefs::FieldDef::Normal(normal_field) => {
                !deducible_fields.contains_key(&normal_field.name)
            }
            xcbdefs::FieldDef::List(_) => true,
            xcbdefs::FieldDef::Switch(_) => true,
            xcbdefs::FieldDef::Fd(_) => true,
            xcbdefs::FieldDef::FdList(_) => true,
            xcbdefs::FieldDef::Expr(_) => false,
            xcbdefs::FieldDef::VirtualLen(_) => false,
        }
    }

    /// Returns whether `case` has a single visible field.
    fn case_has_single_visible_field(
        &self,
        case: &xcbdefs::SwitchCase,
        deducible_fields: &HashMap<String, DeducibleField>,
    ) -> bool {
        let num_visible_fields = case
            .fields
            .borrow()
            .iter()
            .filter(|case_field| self.field_is_visible(case_field, &deducible_fields))
            .count();
        assert!(num_visible_fields > 0);
        num_visible_fields == 1
    }
}

#[derive(Copy, Clone)]
struct Derives {
    debug: bool,
    clone: bool,
    copy: bool,
    partial_eq: bool,
    eq: bool,
}

impl Derives {
    #[inline]
    fn all() -> Self {
        Self {
            debug: true,
            clone: true,
            copy: true,
            partial_eq: true,
            eq: true,
        }
    }

    fn intersect(&mut self, other: Self) {
        self.debug &= other.debug;
        self.clone &= other.clone;
        self.copy &= other.copy;
        self.partial_eq &= other.partial_eq;
        self.eq &= other.eq;
    }

    fn to_list(self) -> Vec<&'static str> {
        let mut list = Vec::new();
        if self.debug {
            list.push("Debug");
        }
        if self.clone {
            list.push("Clone");
        }
        if self.copy {
            list.push("Copy");
        }
        if self.partial_eq {
            list.push("PartialEq");
        }
        if self.eq {
            list.push("Eq");
        }
        list
    }
}

/// Constraints on the wire format of a struct.
#[derive(Debug, PartialEq, Eq)]
enum StructSizeConstraint {
    /// No code constraining the size is emitted.
    None,
    /// This is a fixed size struct.
    Fixed(u8),
    /// This struct has a "length" field embedded that tells us how long it
    /// should be (in 4-byte units).
    EmbeddedLength {
        /// The minimum size.
        minimum: u8,
    },
}

/// Information about a switch case
enum CaseInfo {
    /// The case contains a single visible field.
    ///
    /// The `usize` specifies the index of such field.
    SingleField(usize),
    /// The case contains many visible fields.
    ///
    /// The first `String` is the name of the field
    /// and the second `String` is the name of the
    /// created struct that contains all fields.
    MultiField(String, String),
}

/// Specifies how the value of a field can be calculated
/// from other fields.
#[derive(Debug)]
enum DeducibleField {
    /// The value is the length of a list.
    ///
    /// `(list name, operation)`
    LengthOf(String, DeducibleLengthFieldOp),
    /// The value is the discriminant of a case switch
    ///
    /// `(switch name)`
    CaseSwitchExpr(String, DeducibleFieldOp),
    /// The value is the discriminant of a bitcase switch
    ///
    /// `(switch name)`
    BitCaseSwitchExpr(String, DeducibleFieldOp),
}

#[derive(Copy, Clone, Debug)]
enum DeducibleLengthFieldOp {
    /// `deduced field = list length`
    None,
    /// `deduced field = list length * n`
    Mul(u32),
    /// `deduced field = list length / n`
    Div(u32),
}

#[derive(Debug, Clone)]
enum DeducibleFieldOp {
    /// `deduced field = value`.
    None,
    /// `deduced field = value | expr`.
    Or(Box<xcbdefs::Expression>),
}

/// Gathers deducible fields (fields whose value can be calculated
/// from other fields) from a list of fields.
fn gather_deducible_fields(fields: &[xcbdefs::FieldDef]) -> HashMap<String, DeducibleField> {
    fn extract_length(expr: &xcbdefs::Expression) -> Option<(String, DeducibleLengthFieldOp)> {
        match expr {
            xcbdefs::Expression::FieldRef(field_ref_expr) => Some((
                field_ref_expr.field_name.clone(),
                DeducibleLengthFieldOp::None,
            )),
            xcbdefs::Expression::BinaryOp(bin_op_expr) => {
                if bin_op_expr.operator == xcbdefs::BinaryOperator::Mul {
                    match (&*bin_op_expr.lhs, &*bin_op_expr.rhs) {
                        (
                            xcbdefs::Expression::FieldRef(field_ref_expr),
                            xcbdefs::Expression::Value(value),
                        ) => Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleLengthFieldOp::Div(*value),
                        )),
                        (
                            xcbdefs::Expression::Value(value),
                            xcbdefs::Expression::FieldRef(field_ref_expr),
                        ) => Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleLengthFieldOp::Div(*value),
                        )),
                        _ => None,
                    }
                } else if bin_op_expr.operator == xcbdefs::BinaryOperator::Div {
                    match (&*bin_op_expr.lhs, &*bin_op_expr.rhs) {
                        (
                            xcbdefs::Expression::FieldRef(field_ref_expr),
                            xcbdefs::Expression::Value(value),
                        ) => Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleLengthFieldOp::Mul(*value),
                        )),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    let mut deducible_fields = HashMap::new();
    for field in fields.iter() {
        let deducible_field = match field {
            xcbdefs::FieldDef::List(list_field) => list_field
                .length_expr
                .as_ref()
                .and_then(extract_length)
                .map(|(len_field_name, op)| {
                    (
                        len_field_name,
                        DeducibleField::LengthOf(list_field.name.clone(), op),
                    )
                }),
            xcbdefs::FieldDef::FdList(fd_list_field) => extract_length(&fd_list_field.length_expr)
                .map(|(len_field_name, op)| {
                    (
                        len_field_name,
                        DeducibleField::LengthOf(fd_list_field.name.clone(), op),
                    )
                }),
            xcbdefs::FieldDef::Switch(switch_field) => {
                if switch_field.cases.iter().any(|case| case.exprs.len() != 1) {
                    None
                } else if switch_field.kind == xcbdefs::SwitchKind::Case {
                    if let xcbdefs::Expression::FieldRef(ref field_ref_expr) = switch_field.expr {
                        Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleField::CaseSwitchExpr(
                                switch_field.name.clone(),
                                DeducibleFieldOp::None,
                            ),
                        ))
                    } else {
                        unreachable!("Can't figure out deducible field of {:#?}", switch_field);
                    }
                } else if switch_field.kind == xcbdefs::SwitchKind::BitCase {
                    if let xcbdefs::Expression::FieldRef(ref field_ref_expr) = switch_field.expr {
                        Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleField::BitCaseSwitchExpr(
                                switch_field.name.clone(),
                                DeducibleFieldOp::None,
                            ),
                        ))
                    } else if let xcbdefs::Expression::BinaryOp(ref binary_op_expr) =
                        switch_field.expr
                    {
                        if let xcbdefs::Expression::FieldRef(ref field_ref_expr) =
                            *binary_op_expr.lhs
                        {
                            match binary_op_expr.operator {
                                xcbdefs::BinaryOperator::And => {
                                    // This appears in XKB:SelectEvents.
                                    // There we're provided the additional constraint that
                                    // the right hand side of this operation is a strict superset of
                                    // the left hand side. Therefore, we can negate the right
                                    // hand side and OR it with the switch field to undo the
                                    // AND and deduce affectWhich.
                                    // Because this is not true in general, we assert this is
                                    // the field we expect.
                                    assert_eq!(field_ref_expr.field_name, "affectWhich");
                                    let rhs = binary_op_expr.rhs.clone();
                                    Some((
                                        field_ref_expr.field_name.clone(),
                                        DeducibleField::BitCaseSwitchExpr(
                                            switch_field.name.clone(),
                                            DeducibleFieldOp::Or(rhs.negate()),
                                        ),
                                    ))
                                }
                                // No other operators are actually used.
                                _ => unreachable!(),
                            }
                        } else {
                            unreachable!("Can't figure out deducible field of {:#?}", switch_field);
                        }
                    } else {
                        unreachable!("Can't figure out deducible field of {:#?}", switch_field);
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some((field_name, deducible_field)) = deducible_field {
            let is_not_ext_param = fields
                .iter()
                .any(|field| field.name() == Some(field_name.as_str()));

            if is_not_ext_param {
                match deducible_fields.entry(field_name) {
                    HashMapEntry::Occupied(_) => {
                        // field used more than once,
                        // deduce it from the first use
                        // (do not replace entry)
                    }
                    HashMapEntry::Vacant(entry) => {
                        entry.insert(deducible_field);
                    }
                }
            }
        }
    }

    deducible_fields
}

/// A helper to note what owns a field.
#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldContainer {
    /// This field is part of a request.
    Request(String),
    /// The field belongs to something else.
    Other,
}

/// A helper to represent either simple or borrowed types.
#[derive(Debug, Clone)]
enum Type {
    /// This type is simple can we can just use the string provided.
    Simple(String),
    /// This type has a lifetime and needs to be a borrow or a Cow, depending
    /// on the context. But during parsing it has to be a Cow::Owned because
    /// it is generated.
    VariableOwnership(String),
    /// This type has a lifetime and needs to be a borrow or a Cow, but
    /// when parsing it can be borrowed from the byte stream.
    VariableOwnershipRawBytes(String),
}

impl Type {
    /// Does this type need a Cow::Owned?
    fn needs_owned_cow(&self) -> bool {
        match self {
            Type::Simple(_) | Type::VariableOwnershipRawBytes(_) => false,
            Type::VariableOwnership(_) => true,
        }
    }

    /// Does this type need a Cow::Borrowed?
    fn needs_borrowed_cow(&self) -> bool {
        match self {
            Type::Simple(_) | Type::VariableOwnership(_) => false,
            Type::VariableOwnershipRawBytes(_) => true,
        }
    }

    /// Does this type need a Cow?
    fn needs_any_cow(&self) -> bool {
        match self {
            Type::Simple(_) => false,
            Type::VariableOwnership(_) | Type::VariableOwnershipRawBytes(_) => true,
        }
    }

    /// Render this type in a form suitable for function arguments.
    fn as_argument(&self) -> Cow<'_, str> {
        match self {
            Type::Simple(ref type_) => type_.into(),
            Type::VariableOwnership(ref type_) | Type::VariableOwnershipRawBytes(ref type_) => {
                format!("&'input {}", type_).into()
            }
        }
    }

    /// Render this type in a form suitable for struct fields.
    fn as_field(&self) -> Cow<'_, str> {
        match self {
            Type::Simple(ref type_) => type_.into(),
            Type::VariableOwnership(ref type_) | Type::VariableOwnershipRawBytes(ref type_) => {
                format!("Cow<'input, {}>", type_).into()
            }
        }
    }
}

/// Some information about the fields of a request.
struct GatheredRequestFields {
    reply_has_fds: bool,
    /// Whether lifetimes in the request need to be explicitly
    /// specified. If a request function takes any other
    /// references other than the connection object, we'll need
    /// to disambiguate the lifetimes for rustc.
    needs_lifetime: bool,
    /// Function arguments
    /// `(name, type)`
    args: Vec<(String, Type)>,
    /// Request arguments
    /// The type here has been converted as necessary.
    /// `(name, type)`
    request_args: Vec<(String, Type)>,
    /// Generic type parameters
    ///
    /// `(name, where clause)`
    generics: Vec<(String, String)>,
    /// Code at the beginning of the function.
    preamble: Vec<String>,
    /// Single FD fields
    single_fds: Vec<String>,
    /// FD list fields
    fd_lists: Vec<String>,
}

impl GatheredRequestFields {
    /// Does the request have fds in it?
    fn has_fds(&self) -> bool {
        !self.single_fds.is_empty() || !self.fd_lists.is_empty()
    }
}

/// Formats an integer such as clippy does not complain.
///
/// `1234567` produces `"1_234_567"`
fn format_literal_integer(value: u32) -> String {
    let value = value.to_string();
    if value.len() > 5 {
        let offset = value.len() % 3;
        let mut result = String::new();
        let mut i = offset;
        while i < value.len() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push_str(&value[i..(i + 3)]);
            i += 3;
        }
        if offset != 0 {
            result.insert(0, '_');
            result.insert_str(0, &value[0..offset]);
        }
        result
    } else {
        value
    }
}

fn needs_match_by_value(field: &xcbdefs::FieldDef) -> bool {
    match field {
        xcbdefs::FieldDef::Normal(normal_field) => {
            match normal_field.type_.type_.get_resolved().get_original_type() {
                xcbdefs::TypeRef::BuiltIn(_) => true,
                xcbdefs::TypeRef::Xid(_) => true,
                xcbdefs::TypeRef::XidUnion(_) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

/// Converts a type name from the XML to a rust
/// type name (in CamelCase).
///
/// If the name is all uppercase, all but the first
/// letter are converter to lowercase.
fn to_rust_type_name(name: &str) -> String {
    let mut name = String::from(name);
    if name.bytes().all(|c| !c.is_ascii_lowercase()) {
        name.make_ascii_lowercase();
    }

    // Convert to camel case
    let mut r = String::new();
    for chunk in name.split('_') {
        r.push_str(&chunk[..1]);
        let r_len = r.len();
        r[(r_len - 1)..].make_ascii_uppercase();
        r.push_str(&chunk[1..]);
    }
    r
}

/// Converts a type name from the XML to a rust
/// enum type name (in CamelCase).
///
/// If the name is not snake_case and is all uppercase,
/// it is left as is.
fn to_rust_enum_type_name(name: &str) -> String {
    if name.contains('_') {
        to_rust_type_name(name)
    } else {
        name.into()
    }
}

/// Converts a name from the XML to a Rust variable
/// name (snake_case).
fn to_rust_variable_name(name: &str) -> String {
    if name == "type" {
        "type_".into()
    } else if name == "match" {
        "match_".into()
    } else if name.bytes().any(|c| c.is_ascii_uppercase()) {
        // Deal with CamelCase
        super::camel_case_to_lower_snake(name)
    } else {
        name.into()
    }
}

/// Adds a prefix to `name`, spearated by an underscore.
///
/// If name ends with an underscore, it will be trimmed.
fn prefix_var_name(name: &str, prefix: &str) -> String {
    let mut prefixed = format!("{}_{}", prefix, name);
    if prefixed.ends_with('_') {
        prefixed.truncate(prefixed.len() - 1);
    }
    prefixed
}

/// Adds a postfix to `name`, spearated by an underscore.
///
/// If `name` ends with an underscore, it will not insert an
/// extra one.
fn postfix_var_name(name: &str, postfix: &str) -> String {
    if name.ends_with('_') {
        format!("{}{}", name, postfix)
    } else {
        format!("{}_{}", name, postfix)
    }
}

/// Converts the name of a enum value from the XML
/// to a Rust name.
fn ename_to_rust(name: &str) -> String {
    if name == "DECnet" {
        // Special case
        "DEC_NET".into()
    } else {
        // First convert to proper camel-case
        let mut name = String::from(name);
        if name.as_bytes()[0].is_ascii_digit() {
            name.insert(0, 'M');
        }
        if name.contains('_') && name.bytes().any(|c| c.is_ascii_lowercase()) {
            // xf86vidmode has a ModeFlag enum with items like
            // Positive_HSync. Turn this into PositiveHSync.
            name = name.replace('_', "");
        }
        name[..1].make_ascii_uppercase();

        // Now convert to upper-snake-case
        super::camel_case_to_upper_snake(&name)
    }
}
