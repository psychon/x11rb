#![allow(clippy::cognitive_complexity, clippy::too_many_arguments)]

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use xcbgen::defs as xcbdefs;

use super::output::Output;
use super::requests_replies::{EnumCases, PerModuleEnumCases};
use super::{get_ns_name_prefix, special_cases};

mod expr_to_str;
mod header;
pub(super) mod helpers;
mod parse;
mod request;
mod resource_wrapper;
mod serialize;
mod struct_type;
mod switch;

use expr_to_str::{expr_to_str, expr_type};

use helpers::{
    ename_to_camel_case, ename_to_rust, gather_deducible_fields, postfix_var_name, prefix_var_name,
    to_rust_enum_type_name, to_rust_type_name, to_rust_variable_name, Caches, CaseInfo,
    DeducibleField, DeducibleFieldOp, DeducibleLengthFieldOp, Derives, FieldContainer,
    StructSizeConstraint,
};

/// Generate a Rust module for namespace `ns`.
pub(super) fn generate(
    module: &xcbgen::defs::Module,
    ns: &xcbdefs::Namespace,
    caches: &RefCell<Caches>,
    proto_out: &mut Output,
    x11rb_out: &mut Output,
    enum_cases: &mut EnumCases,
    resource_info: &[super::ResourceInfo<'_>],
) {
    NamespaceGenerator::new(module, ns, caches).generate(
        proto_out,
        x11rb_out,
        enum_cases,
        resource_info,
    );
}

struct NamespaceGenerator<'ns, 'c> {
    module: &'ns xcbgen::defs::Module,
    ns: &'ns xcbdefs::Namespace,
    caches: &'c RefCell<Caches>,

    /// `Option` or `core::option::Option`
    option_name: &'static str,
}

impl<'ns, 'c> NamespaceGenerator<'ns, 'c> {
    #[inline]
    fn new(
        module: &'ns xcbgen::defs::Module,
        ns: &'ns xcbdefs::Namespace,
        caches: &'c RefCell<Caches>,
    ) -> Self {
        let option_name = if ns.header == "present" {
            "core::option::Option"
        } else {
            "Option"
        };
        NamespaceGenerator {
            module,
            ns,
            caches,
            option_name,
        }
    }

    fn generate(
        &self,
        proto_out: &mut Output,
        x11rb_out: &mut Output,
        enum_cases: &mut EnumCases,
        resource_info: &[super::ResourceInfo<'_>],
    ) {
        super::write_code_header(proto_out);
        super::write_code_header(x11rb_out);
        header::write_header(proto_out, self.ns, header::Mode::Protocol);
        header::write_header(x11rb_out, self.ns, header::Mode::X11rb);

        let mut trait_out = Output::new();

        for def in self.ns.src_order_defs.borrow().iter() {
            match def {
                xcbdefs::Def::Request(request_def) => {
                    let cases_entry = enum_cases.entry(self.ns.header.clone()).or_default();
                    request::generate_request(
                        self,
                        request_def,
                        proto_out,
                        x11rb_out,
                        &mut trait_out,
                        cases_entry,
                    )
                }
                xcbdefs::Def::Event(event_def) => match event_def {
                    xcbdefs::EventDef::Full(event_full_def) => {
                        self.generate_event_full_def(event_full_def, proto_out);
                    }
                    xcbdefs::EventDef::Copy(event_copy_def) => {
                        self.generate_event_copy_def(event_copy_def, proto_out);
                    }
                },
                xcbdefs::Def::Error(error_def) => match error_def {
                    xcbdefs::ErrorDef::Full(error_full_def) => {
                        self.generate_error_full_def(error_full_def, proto_out);
                    }
                    xcbdefs::ErrorDef::Copy(error_copy_def) => {
                        self.generate_error_copy_def(error_copy_def, proto_out);
                    }
                },
                xcbdefs::Def::Type(type_def) => match type_def {
                    xcbdefs::TypeDef::Struct(struct_def) => {
                        self.generate_struct_def(struct_def, proto_out)
                    }
                    xcbdefs::TypeDef::Union(union_def) => {
                        self.generate_union_def(union_def, proto_out)
                    }
                    xcbdefs::TypeDef::EventStruct(event_struct_def) => {
                        self.generate_event_struct_def(event_struct_def, proto_out);
                    }
                    xcbdefs::TypeDef::Xid(xid_type_def) => {
                        self.generate_xid_type_def(xid_type_def, proto_out)
                    }
                    xcbdefs::TypeDef::XidUnion(xid_union_def) => {
                        self.generate_xid_union_def(xid_union_def, proto_out)
                    }
                    xcbdefs::TypeDef::Enum(enum_def) => self.generate_enum_def(enum_def, proto_out),
                    xcbdefs::TypeDef::Alias(type_alias_def) => {
                        self.generate_type_alias_def(type_alias_def, proto_out)
                    }
                },
            }
        }

        let trait_out = trait_out.into_data();

        outln!(
            x11rb_out,
            "/// Extension trait defining the requests of this extension.",
        );
        outln!(x11rb_out, "pub trait ConnectionExt: RequestConnection {{");
        out!(x11rb_out.indent(), "{}", trait_out);
        outln!(x11rb_out, "}}");
        outln!(x11rb_out, "");
        outln!(
            x11rb_out,
            "impl<C: RequestConnection + ?Sized> ConnectionExt for C {{}}",
        );

        for info in resource_info {
            resource_wrapper::generate(self, x11rb_out, info);
        }
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
        self.emit_event_opcode(name, number, event_full_def, out);

        let full_name = format!("{}Event", name);

        let fields = event_full_def.fields.borrow();
        let mut derives = Derives::all();
        self.filter_derives_for_fields(&mut derives, &*fields, false);

        struct_type::emit_struct_type(
            self,
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

        special_cases::handle_event(name, event_full_def, out);

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
                    serialize::emit_field_serialize(
                        self,
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
        struct_type::emit_struct_type(
            self,
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
        outln!(
            out,
            r#"#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]"#
        );
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
                        parse::emit_field_parse(
                            self,
                            field,
                            &rust_name,
                            "remaining",
                            FieldContainer::Other,
                            out,
                        );
                        parse::emit_field_post_parse(field, out);
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
                    let bytes_name = serialize::emit_field_serialize(
                        self,
                        field,
                        &HashMap::new(),
                        to_rust_variable_name,
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
        outln!(
            out,
            r#"#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]"#
        );
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
                        "{}::try_parse(value).unwrap().0",
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
            self.emit_doc(doc, out, None);
        }

        outln!(
            out,
            "#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]"
        );
        outln!(
            out,
            r#"#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]"#
        );
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
                            expr_to_str::format_literal_integer(value),
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
        let ok_for_bitmask = enum_def.items.iter().all(|enum_item| {
            matches!(
                enum_item.value,
                xcbdefs::EnumValue::Value(0) | xcbdefs::EnumValue::Bit(_)
            )
        }) && enum_def
            .items
            .iter()
            .any(|enum_item| match enum_item.value {
                xcbdefs::EnumValue::Value(_) => false,
                xcbdefs::EnumValue::Bit(_) => true,
            });

        outln!(out, "impl core::fmt::Debug for {}  {{", rust_name);
        out.indented(|out| {
            outln!(
                out,
                "fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
            );
            out.indented(|out| {
                let into = match global_enum_size {
                    32 => "",
                    _ => ".into()",
                };
                outln!(out, "let variants = [");
                for enum_item in enum_def.items.iter() {
                    let rust_item_name = ename_to_rust(&enum_item.name);
                    let camel_item_name = ename_to_camel_case(&enum_item.name);
                    outln!(
                        out.indent(),
                        "(Self::{}.0{}, \"{}\", \"{}\"),",
                        rust_item_name,
                        into,
                        rust_item_name,
                        camel_item_name
                    );
                }
                outln!(out, "];");
                if ok_for_bitmask {
                    outln!(out, "pretty_print_bitmask(fmt, self.0{}, &variants)", into);
                } else {
                    outln!(out, "pretty_print_enum(fmt, self.0{}, &variants)", into);
                }
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");

        if ok_for_bitmask {
            outln!(out, "bitmask_binop!({}, {});", rust_name, raw_type);
        }

        outln!(out, "");
    }

    fn generate_type_alias_def(&self, type_alias_def: &xcbdefs::TypeAliasDef, out: &mut Output) {
        let rust_new_name = self.get_type_alias_rust_name(type_alias_def);
        outln!(
            out,
            "pub type {} = {};",
            rust_new_name,
            self.type_to_rust_type(type_alias_def.old_name.get_resolved()),
        );
        outln!(out, "");
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
                    outln!(
                        out,
                        "let {}: {} = {}.switch_expr();",
                        dst_var_name,
                        rust_field_type,
                        wrap_field_ref(switch_field_name),
                    );
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
                            expr = expr_to_str(
                                self,
                                &*or_expr,
                                &mut wrap_field_ref,
                                true,
                                Some(&rust_field_type),
                                true,
                            )
                        ),
                    };
                    outln!(
                        out,
                        "let {}: {} = {}.switch_expr(){};",
                        dst_var_name,
                        rust_field_type,
                        wrap_field_ref(switch_field_name),
                        op_str,
                    );
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
            matches!(
                original_type,
                xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card8)
                    | xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Byte)
                    | xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Char)
                    | xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Void)
            )
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

    fn emit_doc(
        &self,
        doc: &xcbdefs::Doc,
        out: &mut Output,
        deducible_fields: Option<&HashMap<String, DeducibleField>>,
    ) {
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
                if deducible_fields
                    .map(|deducible_fields| deducible_fields.contains_key(&field.name))
                    .unwrap_or(false)
                {
                    continue;
                }
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
        caches
            .rust_type_names
            .entry(id)
            .or_insert_with(|| to_rust_type_name(name))
            .clone()
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
        let mut params = vec![String::from(remaining)];

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
                    // non-bitcase switch cases can't be Default
                    if switch_field.kind != xcbdefs::SwitchKind::BitCase {
                        derives.default_ = false;
                    }
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
                    // RawFdContainer cannot be cloned, compared, hashed or made default
                    derives.clone = false;
                    derives.copy = false;
                    derives.partial_ord = false;
                    derives.ord = false;
                    derives.hash = false;
                    derives.default_ = false;
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
                        // f32/f64 do not implement Eq, Ord or Hash
                        derives.eq = false;
                        derives.ord = false;
                        derives.hash = false;
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
                derives.partial_ord = false;
                derives.ord = false;
                derives.hash = false;
                derives.default_ = false;
            }
            xcbdefs::TypeRef::EventStruct(_) => {
                // Event structs don't support equality tests.
                derives.partial_eq = false;
                derives.eq = false;
                derives.partial_ord = false;
                derives.ord = false;
                derives.hash = false;
                derives.default_ = false;
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
}
