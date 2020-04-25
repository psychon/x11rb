#![allow(clippy::cognitive_complexity, clippy::option_as_ref_deref, clippy::too_many_arguments)]

use std::cell::RefCell;
use std::collections::hash_map::Entry as HashMapEntry;
use std::rc::Rc;

use fxhash::{FxHashMap, FxHashSet};
use xcbgen::defs as xcbdefs;

use super::output::Output;
use super::special_cases;

/// Generate a Rust module for namespace `ns`.
pub(super) fn generate(ns: &xcbdefs::Namespace, caches: &RefCell<Caches>, out: &mut Output) {
    NamespaceGenerator::new(ns, caches).generate(out);
}

/// Caches to avoid repeating some operations.
#[derive(Default)]
pub(super) struct Caches {
    derives: FxHashMap<usize, Derives>,
    enum_has_repeated_values: FxHashMap<usize, bool>,
    rust_type_names: FxHashMap<usize, String>,
}

struct NamespaceGenerator<'ns, 'c> {
    ns: &'ns xcbdefs::Namespace,
    caches: &'c RefCell<Caches>,

    /// `Option` or `std::option::Option`
    option_name: &'static str,

    /// `GenericEvent` or `crate::x11_utils::GenericEvent`
    generic_event_name: &'static str,

    /// `GenericError` or `crate::x11_utils::GenericError`
    generic_error_name: &'static str,
}

impl<'ns, 'c> NamespaceGenerator<'ns, 'c> {
    #[inline]
    fn new(ns: &'ns xcbdefs::Namespace, caches: &'c RefCell<Caches>) -> Self {
        let option_name = if ns.header == "present" {
            "std::option::Option"
        } else {
            "Option"
        };
        let generic_event_name = if ns.header == "present" {
            "crate::x11_utils::GenericEvent"
        } else {
            "GenericEvent"
        };
        let generic_error_name = if ns.header == "glx" {
            "crate::x11_utils::GenericError"
        } else {
            "GenericError"
        };
        NamespaceGenerator {
            ns,
            caches,
            option_name,
            generic_event_name,
            generic_error_name,
        }
    }

    fn generate(&self, out: &mut Output) {
        super::write_code_header(out);
        outln!(out, "#![allow(clippy::too_many_arguments)]");
        outln!(out, "#![allow(clippy::identity_op)]");
        outln!(out, "#![allow(clippy::trivially_copy_pass_by_ref)]");
        outln!(out, "#![allow(clippy::eq_op)]");
        outln!(out, "");
        outln!(out, "use std::convert::TryFrom;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use std::convert::TryInto;");
        outln!(out, "use std::io::IoSlice;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::utils::RawFdContainer;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::x11_utils::Event as _;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use crate::x11_utils::{{Serialize, TryParse}};");
        outln!(out, "use crate::connection::RequestConnection;");
        outln!(out, "#[allow(unused_imports)]");
        outln!(
            out,
            "use crate::cookie::{{Cookie, CookieWithFds, VoidCookie}};"
        );
        if self.ns.header == "xproto" {
            outln!(out, "use crate::cookie::ListFontsWithInfoCookie;");
        }
        outln!(out, "use crate::errors::{{ConnectionError, ParseError}};");
        if self.ns.header != "present" {
            outln!(out, "#[allow(unused_imports)]");
            outln!(out, "use crate::x11_utils::GenericEvent;");
        }
        if self.ns.header != "glx" {
            outln!(out, "#[allow(unused_imports)]");
            outln!(out, "use crate::x11_utils::GenericError;");
        }

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
            outln!(out, "/// information. If you need to send a `QueryVersion`, it is recommended to instead");
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
                    self.generate_request(request_def, out, &mut trait_out)
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
    ) {
        let name = to_rust_type_name(&request_def.name);

        outln!(out, "/// Opcode for the {} request", name);
        outln!(
            out,
            "pub const {}_REQUEST: u8 = {};",
            super::camel_case_to_upper_snake(&name),
            request_def.opcode,
        );

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

        if switch_fields.len() == 1 {
            if let Some(aux_start_align) = switch_fields[0].required_start_align {
                assert_eq!(aux_start_align.offset, 0);
            }
            self.generate_aux(request_def, switch_fields[0], &function_name, out);
        }

        let deducible_fields = gather_deducible_fields(&*request_fields);

        let gathered = self.gather_request_fields(request_def, &deducible_fields);

        self.emit_request_function(
            request_def,
            &name,
            &function_name,
            &deducible_fields,
            &gathered,
            out,
        );
        self.emit_request_trait_function(request_def, &name, &function_name, &gathered, trait_out);

        special_cases::handle_request(request_def, out);

        outln!(out, "");

        if let Some(ref reply) = request_def.reply {
            let reply_struct_name = format!("{}Reply", name);
            let reply_fields = reply.fields.borrow();
            let mut reply_derives = Derives::all();
            self.filter_derives_for_fields(&mut reply_derives, &*reply_fields);
            self.emit_struct_type(
                &reply_struct_name,
                &name,
                reply_derives,
                &*reply_fields,
                &[],
                false,
                true,
                false,
                reply.doc.as_ref(),
                out,
            );

            outln!(out, "");
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
            self.emit_switch_type(switch_field, &aux_name, false, true, None, out);
        } else {
            let doc = format!(
                "Auxiliary and optional information for the `{}` function",
                function_name,
            );
            let cases_infos =
                self.emit_switch_type(switch_field, &aux_name, false, true, Some(&doc), out);

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

    fn emit_request_function(
        &self,
        request_def: &xcbdefs::RequestDef,
        name: &str,
        function_name: &str,
        deducible_fields: &FxHashMap<String, DeducibleField>,
        gathered: &GatheredRequestFields,
        out: &mut Output,
    ) {
        let mut generic_params = String::new();
        if gathered.needs_lifetime {
            generic_params.push_str("'c, Conn");
        } else {
            generic_params.push_str("Conn");
        }
        for (param_name, _) in gathered.generics.iter() {
            generic_params.push_str(", ");
            generic_params.push_str(param_name);
        }

        let ns = request_def.namespace.upgrade().unwrap();
        let is_send_event = request_def.name == "SendEvent" && ns.header == "xproto";
        let is_list_fonts_with_info =
            request_def.name == "ListFontsWithInfo" && ns.header == "xproto";

        let ret_lifetime = if gathered.needs_lifetime { "'c" } else { "'_" };
        let ret_type = if is_list_fonts_with_info {
            assert!(request_def.reply.is_some());
            assert!(!gathered.reply_has_fds);
            format!("ListFontsWithInfoCookie<{}, Conn>", ret_lifetime)
        } else {
            match (request_def.reply.is_some(), gathered.reply_has_fds) {
                (false, _) => format!("VoidCookie<{}, Conn>", ret_lifetime),
                (true, false) => format!("Cookie<{}, Conn, {}Reply>", ret_lifetime, name),
                (true, true) => format!("CookieWithFds<{}, Conn, {}Reply>", ret_lifetime, name),
            }
        };

        let mut args = String::new();
        if gathered.needs_lifetime {
            args.push_str("conn: &'c Conn");
        } else {
            args.push_str("conn: &Conn");
        }
        for (arg_name, arg_type) in gathered.args.iter() {
            args.push_str(", ");
            args.push_str(arg_name);
            args.push_str(": ");
            args.push_str(arg_type);
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
            if ns.ext_info.is_some() {
                outln!(
                    out,
                    "let extension_information = conn.extension_information(X11_EXTENSION_NAME)?",
                );
                outln!(
                    out.indent(),
                    ".ok_or(ConnectionError::UnsupportedExtension)?;"
                );
            }

            for preamble in gathered.preamble.iter() {
                outln!(out, "{}", preamble);
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
                            "let {} = u32::try_from({}.len()).unwrap();",
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
                self.emit_assert_for_field_serialize(field, deducible_fields, "", &mut tmp_out);
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
                            next_slice = Some(format!("&padding{}", pad_count));
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
                                    super::camel_case_to_upper_snake(name),
                                ));
                            }
                        } else if normal_field.name == "minor_opcode" {
                            assert!(ns.ext_info.is_some());
                            fixed_fields_bytes.push(format!(
                                "{}_REQUEST",
                                super::camel_case_to_upper_snake(name),
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
                                    "",
                                    &rust_field_name,
                                    out,
                                );
                                true
                            } else {
                                false
                            };

                            let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                            if let Some(field_size) = normal_field.type_.size() {
                                outln!(
                                    out,
                                    "let {} = {};",
                                    bytes_name,
                                    self.emit_value_serialize(
                                        &normal_field.type_,
                                        &rust_field_name,
                                        was_deduced,
                                    ),
                                );
                                for i in 0..field_size {
                                    fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                                }
                            } else {
                                outln!(
                                    tmp_out,
                                    "let {} = {}.serialize();",
                                    bytes_name,
                                    rust_field_name
                                );
                                next_slice = Some(format!("&{}", bytes_name));
                            }
                        }
                    }
                    xcbdefs::FieldDef::List(list_field) => {
                        let rust_field_name = to_rust_variable_name(&list_field.name);
                        let list_length = list_field.length();
                        if self.rust_value_type_is_u8(&list_field.element_type) {
                            if list_length.is_some() {
                                if is_send_event && list_field.name == "event" {
                                    next_slice = Some(format!("&{}", rust_field_name));
                                } else {
                                    next_slice = Some(rust_field_name);
                                }
                            } else {
                                next_slice = Some(rust_field_name);
                            }
                        } else {
                            let element_size = list_field.element_type.size();
                            if let (Some(list_length), Some(element_size)) =
                                (list_length, element_size)
                            {
                                for i in 0..list_length {
                                    let src_value = format!("{}[{}]", rust_field_name, i);
                                    let bytes_name =
                                        postfix_var_name(&rust_field_name, &format!("{}_bytes", i));
                                    outln!(
                                        out,
                                        "let {} = {};",
                                        bytes_name,
                                        self.emit_value_serialize(
                                            &list_field.element_type,
                                            &src_value,
                                            false,
                                        ),
                                    );
                                    for j in 0..element_size {
                                        fixed_fields_bytes.push(format!("{}[{}]", bytes_name, j));
                                    }
                                }
                            } else if self.can_use_simple_list_parsing(&list_field.element_type) {
                                let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                                outln!(
                                    tmp_out,
                                    "let {} = {}.serialize();",
                                    bytes_name,
                                    rust_field_name,
                                );
                                next_slice = Some(format!("&{}", bytes_name));
                            } else {
                                let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                                outln!(tmp_out, "let mut {} = Vec::new();", bytes_name);
                                outln!(tmp_out, "for element in {}.iter() {{", rust_field_name,);
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
                                next_slice = Some(format!("&{}", bytes_name));
                            }
                        }
                    }
                    xcbdefs::FieldDef::Switch(switch_field) => {
                        let rust_field_name = to_rust_variable_name(&switch_field.name);
                        let bytes_name = postfix_var_name(&rust_field_name, "bytes");
                        outln!(
                            tmp_out,
                            "let {} = {}.serialize();",
                            bytes_name,
                            rust_field_name,
                        );
                        if let Some(field_size) = switch_field.size() {
                            for i in 0..field_size {
                                fixed_fields_bytes.push(format!("{}[{}]", bytes_name, i));
                            }
                        } else {
                            next_slice = Some(format!("&{}", bytes_name));
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
                                self.expr_to_str(&expr_field.expr, true, true, true),
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
                            self.emit_value_serialize(&expr_field.type_, &rust_field_name, false),
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
                        outln!(out, "let {}request{} = [", maybe_mut, num_fixed_len_slices);
                        for byte in fixed_fields_bytes.iter() {
                            outln!(out.indent(), "{},", byte);
                        }
                        outln!(out, "];");
                        outln!(
                            out,
                            "let length_so_far = length_so_far + request{}.len();",
                            num_fixed_len_slices,
                        );
                        request_slices.push(format!("&request{}", num_fixed_len_slices));
                        fixed_fields_bytes.clear();
                        num_fixed_len_slices += 1;
                    }
                    if let Some(next_slice) = next_slice {
                        let next_slice_for_len = if next_slice.starts_with('&') {
                            &next_slice[1..]
                        } else {
                            next_slice.as_str()
                        };
                        outln!(
                            tmp_out,
                            "let length_so_far = length_so_far + {}.len();",
                            next_slice_for_len,
                        );
                        request_slices.push(next_slice);
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
                    request_slices.push(String::from("&padding"));
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
                request_slices.push(format!("&padding{}", pad_count));
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
                format!("vec![{}]", gathered.single_fds.join(", "))
            } else if gathered.fd_lists.len() == 1 && gathered.single_fds.is_empty() {
                gathered.fd_lists[0].clone()
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
                slices_arg.push_str("IoSlice::new(");
                slices_arg.push_str(request_slices);
                slices_arg.push(')');
            }

            if is_list_fonts_with_info {
                outln!(
                    out,
                    "Ok(ListFontsWithInfoCookie::new(conn.send_request_with_reply(&[{}], {})?))",
                    slices_arg,
                    fds_arg,
                )
            } else if request_def.reply.is_some() {
                if gathered.reply_has_fds {
                    outln!(
                        out,
                        "Ok(conn.send_request_with_reply_with_fds(&[{}], {})?)",
                        slices_arg,
                        fds_arg,
                    );
                } else {
                    outln!(
                        out,
                        "Ok(conn.send_request_with_reply(&[{}], {})?)",
                        slices_arg,
                        fds_arg,
                    );
                }
            } else {
                outln!(
                    out,
                    "Ok(conn.send_request_without_reply(&[{}], {})?)",
                    slices_arg,
                    fds_arg
                );
            }
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
        let mut generic_params = String::new();
        if gathered.needs_lifetime || !gathered.generics.is_empty() {
            generic_params.push('<');
            if gathered.needs_lifetime {
                generic_params.push_str("'c");
            }
            for (i, (param_name, _)) in gathered.generics.iter().enumerate() {
                if i != 0 || gathered.needs_lifetime {
                    generic_params.push_str(", ");
                }
                generic_params.push_str(param_name);
            }
            generic_params.push('>');
        }

        let ns = request_def.namespace.upgrade().unwrap();
        let is_list_fonts_with_info =
            request_def.name == "ListFontsWithInfo" && ns.header == "xproto";

        let ret_lifetime = if gathered.needs_lifetime { "'c" } else { "'_" };
        let ret_type = if is_list_fonts_with_info {
            assert!(request_def.reply.is_some());
            assert!(!gathered.reply_has_fds);
            format!("ListFontsWithInfoCookie<{}, Self>", ret_lifetime)
        } else {
            match (request_def.reply.is_some(), gathered.reply_has_fds) {
                (false, _) => format!("VoidCookie<{}, Self>", ret_lifetime),
                (true, false) => format!("Cookie<{}, Self, {}Reply>", ret_lifetime, name),
                (true, true) => format!("CookieWithFds<{}, Self, {}Reply>", ret_lifetime, name),
            }
        };

        let mut args = String::new();
        if gathered.needs_lifetime {
            args.push_str("&'c self");
        } else {
            args.push_str("&self");
        }
        for (arg_name, arg_type) in gathered.args.iter() {
            args.push_str(", ");
            args.push_str(arg_name);
            args.push_str(": ");
            args.push_str(arg_type);
        }

        let ns = request_def.namespace.upgrade().unwrap();
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
        let number = event_copy_def.number;

        let event_full_def = event_copy_def.get_original_full_def();
        self.emit_event(&name, number, &event_full_def, out);
    }

    fn emit_event(
        &self,
        name: &str,
        number: u16,
        event_full_def: &xcbdefs::EventFullDef,
        out: &mut Output,
    ) {
        let full_name = format!("{}Event", name);

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

        let fields = event_full_def.fields.borrow();
        let mut derives = Derives::all();
        self.filter_derives_for_fields(&mut derives, &*fields);

        self.emit_struct_type(
            &full_name,
            name,
            derives,
            &*fields,
            &[],
            false,
            true,
            false,
            event_full_def.doc.as_ref(),
            out,
        );

        outln!(
            out,
            "impl<B: AsRef<[u8]>> TryFrom<{}<B>> for {} {{",
            self.generic_event_name,
            full_name,
        );
        out.indented(|out| {
            outln!(out, "type Error = ParseError;");
            outln!(out, "");
            outln!(
                out,
                "fn try_from(value: {}<B>) -> Result<Self, Self::Error> {{",
                self.generic_event_name,
            );
            outln!(out.indent(), "Self::try_from(value.raw_bytes())");
            outln!(out, "}}");
        });
        outln!(out, "}}");

        outln!(
            out,
            "impl<B: AsRef<[u8]>> TryFrom<&{}<B>> for {} {{",
            self.generic_event_name,
            full_name,
        );
        out.indented(|out| {
            outln!(out, "type Error = ParseError;");
            outln!(
                out,
                "fn try_from(value: &{}<B>) -> Result<Self, Self::Error> {{",
                self.generic_event_name
            );
            outln!(out.indent(), "Self::try_from(value.raw_bytes())");
            outln!(out, "}}");
        });
        outln!(out, "}}");

        if !event_full_def.xge {
            let deducible_fields = gather_deducible_fields(&*fields);
            self.emit_event_or_error_serialize(&full_name, &*fields, &deducible_fields, out);
        }

        outln!(out, "");
    }

    fn generate_error_full_def(&self, error_full_def: &xcbdefs::ErrorFullDef, out: &mut Output) {
        let name = to_rust_type_name(&error_full_def.name);
        self.emit_error(&name, error_full_def.number, error_full_def, out);
    }

    fn generate_error_copy_def(&self, error_copy_def: &xcbdefs::ErrorCopyDef, out: &mut Output) {
        let name = to_rust_type_name(&error_copy_def.name);
        let number = error_copy_def.number;

        let error_full_def = error_copy_def.get_original_full_def();
        self.emit_error(&name, number, &error_full_def, out);
    }

    fn emit_error(
        &self,
        name: &str,
        number: i16,
        error_full_def: &xcbdefs::ErrorFullDef,
        out: &mut Output,
    ) {
        let full_name = format!("{}Error", name);

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

        let fields = error_full_def.fields.borrow();
        let mut derives = Derives::all();
        self.filter_derives_for_fields(&mut derives, &*fields);

        self.emit_struct_type(
            &full_name,
            name,
            derives,
            &*fields,
            &[],
            false,
            true,
            false,
            None,
            out,
        );

        let msg = "Buffer should be large enough so that parsing cannot fail";
        outln!(
            out,
            "impl<B: AsRef<[u8]>> From<{}<B>> for {} {{",
            self.generic_error_name,
            full_name,
        );
        out.indented(|out| {
            outln!(
                out,
                "fn from(value: {}<B>) -> Self {{",
                self.generic_error_name,
            );
            outln!(
                out.indent(),
                "Self::try_from(value.raw_bytes()).expect(\"{}\")",
                msg,
            );
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(
            out,
            "impl<B: AsRef<[u8]>> From<&{}<B>> for {} {{",
            self.generic_error_name,
            full_name
        );
        out.indented(|out| {
            outln!(
                out,
                "fn from(value: &{}<B>) -> Self {{",
                self.generic_error_name,
            );
            outln!(
                out.indent(),
                "Self::try_from(value.raw_bytes()).expect(\"{}\")",
                msg
            );
            outln!(out, "}}");
        });
        outln!(out, "}}");

        let deducible_fields = gather_deducible_fields(&*fields);
        self.emit_event_or_error_serialize(&full_name, &*fields, &deducible_fields, out);

        outln!(out, "");
    }

    fn emit_event_or_error_serialize(
        &self,
        name: &str,
        fields: &[xcbdefs::FieldDef],
        deducible_fields: &FxHashMap<String, DeducibleField>,
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
                        "input.",
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
        assert!(struct_align.internal_align <= struct_align.begin.align);
        assert_eq!(struct_align.begin.offset % struct_align.internal_align, 0);

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
                        self.emit_field_parse(field, &rust_name, out);
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
            outln!(out, "fn serialize(&self) -> Self::Bytes {{");
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
                outln!(out.indent(), ".ok_or(ParseError::ParseError)?");
                outln!(out.indent(), ".try_into()");
                outln!(out.indent(), ".unwrap();");
                outln!(out, "let result = {}(inner);", rust_name);
                outln!(out, "Ok((result, &value[{}..]))", union_size);
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");

        let mut seen_field_types = FxHashSet::default();

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
                            &normal_field
                                .type_
                                .type_
                                .def
                                .get()
                                .unwrap()
                                .get_original_type(),
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
                                .def
                                .get()
                                .unwrap()
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
                        &FxHashMap::default(),
                        "",
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
                    let event = event.as_event_def();
                    let event_name = event.name();
                    let event_ns = event.namespace();
                    let rust_event_name = format!("{}Event", to_rust_type_name(event_name));
                    let rust_event_type = self.type_name_to_rust_type(&rust_event_name, &event_ns);
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
            outln!(out, "fn serialize(&self) -> Self::Bytes {{");
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
                outln!(out.indent(), ".ok_or(ParseError::ParseError)?");
                outln!(out.indent(), ".try_into()");
                outln!(out.indent(), ".unwrap();");
                outln!(out, "let result = {}(inner);", rust_name);
                outln!(out, "Ok((result, &value[32..]))");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");

        for allowed in event_struct_def.alloweds.iter() {
            for event in allowed.resolved.borrow().iter() {
                let event = event.as_event_def();
                let event_name = event.name();
                let event_ns = event.namespace();
                let rust_event_name = format!("{}Event", to_rust_type_name(event_name));
                let rust_event_type = self.type_name_to_rust_type(&rust_event_name, &event_ns);
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
        // TODO: `From` impl for each event.

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
        let assign_discriminators = !self.enum_has_repeated_values(enum_def);

        // Guess which types this enum can be represented in. We do this based on the
        // highest value that appears in any of the variants
        let max_value = enum_def
            .items
            .iter()
            .map(|enum_item| match enum_item.value {
                xcbdefs::EnumValue::Value(value) => value,
                xcbdefs::EnumValue::Bit(bit) => 1 << bit,
            })
            .max()
            .unwrap();

        let (to_type, larger_types): (&str, &[&str]) = if max_value <= 0xFF {
            ("u8", &["u16", "u32"])
        } else if max_value <= 0xFFFF {
            ("u16", &["u32"])
        } else {
            ("u32", &[])
        };

        if let Some(ref doc) = enum_def.doc {
            self.emit_doc(doc, out);
        }

        outln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]");
        let needs_lint_allow = enum_def.items.iter().any(|enum_item| {
            enum_item
                .name
                .bytes()
                .all(|c| !c.is_ascii_lowercase() && !c.is_ascii_digit())
        });
        if needs_lint_allow {
            outln!(out, "#[allow(non_camel_case_types)]");
        }
        if assign_discriminators {
            // Specify the representation if we are assigning discriminators, so we make sure that
            // all the values fit. This prevents the 'enum_clike_unportable_variant' clippy warning.
            outln!(out, "#[repr({})]", to_type);
        }
        outln!(out, "pub enum {} {{", rust_name);
        for enum_item in enum_def.items.iter() {
            let rust_item_name = ename_to_rust(&enum_item.name);
            if assign_discriminators {
                match enum_item.value {
                    xcbdefs::EnumValue::Value(value) => {
                        outln!(
                            out.indent(),
                            "{} = {},",
                            rust_item_name,
                            format_literal_integer(value),
                        );
                    }
                    xcbdefs::EnumValue::Bit(bit) => {
                        outln!(out.indent(), "{} = 1 << {},", rust_item_name, bit);
                    }
                }
            } else {
                outln!(out.indent(), "{},", rust_item_name);
            }
        }
        outln!(out, "}}");

        if max_value == 1 && enum_def.items.len() == 2 {
            outln!(out, "impl From<{}> for bool {{", rust_name);
            out.indented(|out| {
                outln!(out, "fn from(input: {}) -> Self {{", rust_name);
                out.indented(|out| {
                    outln!(out, "match input {{");
                    for enum_item in enum_def.items.iter() {
                        let bool_value = match enum_item.value {
                            xcbdefs::EnumValue::Value(0) => "false",
                            xcbdefs::EnumValue::Value(1) => "true",
                            _ => unreachable!(),
                        };
                        outln!(
                            out.indent(),
                            "{}::{} => {},",
                            rust_name,
                            ename_to_rust(&enum_item.name),
                            bool_value,
                        );
                    }
                    outln!(out, "}}");
                });
                outln!(out, "}}");
            });
            outln!(out, "}}");
        }

        outln!(out, "impl From<{}> for {} {{", rust_name, to_type);
        out.indented(|out| {
            outln!(out, "fn from(input: {}) -> Self {{", rust_name);
            out.indented(|out| {
                outln!(out, "match input {{");
                for enum_item in enum_def.items.iter() {
                    let rust_item_name = ename_to_rust(&enum_item.name);
                    match enum_item.value {
                        xcbdefs::EnumValue::Value(value) => {
                            outln!(
                                out.indent(),
                                "{}::{} => {},",
                                rust_name,
                                rust_item_name,
                                format_literal_integer(value),
                            );
                        }
                        xcbdefs::EnumValue::Bit(bit) => {
                            outln!(
                                out.indent(),
                                "{}::{} => 1 << {},",
                                rust_name,
                                rust_item_name,
                                bit,
                            );
                        }
                    }
                }
                outln!(out, "}}");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
        outln!(
            out,
            "impl From<{}> for {}<{}> {{",
            rust_name,
            self.option_name,
            to_type
        );
        out.indented(|out| {
            outln!(out, "fn from(input: {}) -> Self {{", rust_name);
            outln!(out.indent(), "Some({}::from(input))", to_type);
            outln!(out, "}}");
        });
        outln!(out, "}}");
        for larger_type in larger_types.iter() {
            outln!(out, "impl From<{}> for {} {{", rust_name, larger_type);
            out.indented(|out| {
                outln!(out, "fn from(input: {}) -> Self {{", rust_name);
                outln!(out.indent(), "Self::from({}::from(input))", to_type);
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
                outln!(out, "fn from(input: {}) -> Self {{", rust_name);
                outln!(out.indent(), "Some({}::from(input))", larger_type);
                outln!(out, "}}");
            });
            outln!(out, "}}");
        }

        // Values can only be parsed if they are unique
        if !self.enum_has_repeated_values(enum_def) {
            outln!(out, "impl TryFrom<{}> for {} {{", to_type, rust_name);
            out.indented(|out| {
                outln!(out, "type Error = ParseError;");
                outln!(
                    out,
                    "fn try_from(value: {}) -> Result<Self, Self::Error> {{",
                    to_type,
                );
                out.indented(|out| {
                    outln!(out, "match value {{");
                    for enum_item in enum_def.items.iter() {
                        let rust_item_name = ename_to_rust(&enum_item.name);
                        match enum_item.value {
                            xcbdefs::EnumValue::Value(value) => {
                                outln!(
                                    out.indent(),
                                    "{} => Ok({}::{}),",
                                    format_literal_integer(value),
                                    rust_name,
                                    rust_item_name,
                                );
                            }
                            xcbdefs::EnumValue::Bit(bit) => {
                                outln!(
                                    out.indent(),
                                    "{} => Ok({}::{}),",
                                    format_literal_integer(1u32 << bit),
                                    rust_name,
                                    rust_item_name,
                                );
                            }
                        }
                    }
                    outln!(out.indent(), "_ => Err(ParseError::ParseError),");
                    outln!(out, "}}");
                });
                outln!(out, "}}");
            });
            outln!(out, "}}");

            for larger_type in larger_types.iter() {
                outln!(out, "impl TryFrom<{}> for {} {{", larger_type, rust_name);
                out.indented(|out| {
                    outln!(out, "type Error = ParseError;");
                    outln!(
                        out,
                        "fn try_from(value: {}) -> Result<Self, Self::Error> {{",
                        larger_type,
                    );
                    outln!(
                        out.indent(),
                        "Self::try_from({}::try_from(value).or(Err(ParseError::ParseError))?)",
                        to_type,
                    );
                    outln!(out, "}}");
                });
                outln!(out, "}}");
            }
        }

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
            outln!(out, "bitmask_binop!({}, {});", rust_name, to_type);
        }

        outln!(out, "");
    }

    fn generate_type_alias_def(&self, type_alias_def: &xcbdefs::TypeAliasDef, out: &mut Output) {
        let rust_new_name = self.get_type_alias_rust_name(&type_alias_def);
        outln!(
            out,
            "pub type {} = {};",
            rust_new_name,
            self.type_to_rust_type(type_alias_def.old_name.def.get().unwrap()),
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
        generate_serialize: bool,
        doc: Option<&xcbdefs::Doc>,
        out: &mut Output,
    ) {
        self.generate_switches_for_fields(
            switch_prefix,
            fields,
            generate_try_parse,
            generate_serialize,
            out,
        );

        let deducible_fields = gather_deducible_fields(fields);

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
            if has_fds {
                assert!(external_params.is_empty());
                outln!(out, "impl {} {{", name);
                outln!(
                    out.indent(),
                    "fn try_parse_fd<'a>({}) -> Result<(Self, &'a [u8]), ParseError> {{",
                    "remaining: &'a [u8], fds: &mut Vec<RawFdContainer>",
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
                    "pub fn try_parse(remaining: &[u8], {}) \
                    -> Result<(Self, &[u8]), ParseError> {{",
                    p.join(", "),
                );
            } else {
                outln!(out, "impl TryParse for {} {{", name);
                outln!(
                    out.indent(),
                    "fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {{",
                );
            }

            out.indented(|out| {
                out.indented(|out| {
                    Self::emit_let_value_for_dynamic_align(fields, out);
                    for field in fields.iter() {
                        self.emit_field_parse(field, switch_prefix, out);
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
                    &deducible_fields,
                    skip_length_field,
                    size,
                    out,
                );
            } else {
                self.emit_variable_size_struct_serialize(
                    name,
                    fields,
                    &deducible_fields,
                    skip_length_field,
                    out,
                );
            }
        }
    }

    fn emit_fixed_size_struct_serialize(
        &self,
        name: &str,
        fields: &[xcbdefs::FieldDef],
        deducible_fields: &FxHashMap<String, DeducibleField>,
        skip_length_field: bool,
        size: u32,
        out: &mut Output,
    ) {
        outln!(out, "impl Serialize for {} {{", name);
        out.indented(|out| {
            outln!(out, "type Bytes = [u8; {}];", size);
            outln!(out, "fn serialize(&self) -> Self::Bytes {{");
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
                        "self.",
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
            outln!(out, "fn serialize_into(&self, bytes: &mut Vec<u8>) {{");
            out.indented(|out| {
                outln!(out, "bytes.reserve({});", size);
                for field in fields.iter() {
                    if skip_length_field && field.name() == Some("length") {
                        continue;
                    }
                    self.emit_field_serialize_into(field, deducible_fields, "self.", "bytes", out);
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
        deducible_fields: &FxHashMap<String, DeducibleField>,
        skip_length_field: bool,
        out: &mut Output,
    ) {
        outln!(out, "impl Serialize for {} {{", name);
        out.indented(|out| {
            outln!(out, "type Bytes = Vec<u8>;");
            outln!(out, "fn serialize(&self) -> Self::Bytes {{");
            out.indented(|out| {
                outln!(out, "let mut result = Vec::new();");
                outln!(out, "self.serialize_into(&mut result);");
                outln!(out, "result");
            });
            outln!(out, "}}");
            outln!(out, "fn serialize_into(&self, bytes: &mut Vec<u8>) {{");
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
                    self.emit_field_serialize_into(field, deducible_fields, "self.", "bytes", out);
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
        assert!(switch_align.internal_align <= switch_align.begin.align);
        assert_eq!(switch_align.begin.offset % switch_align.internal_align, 0);

        let mut case_infos = Vec::new();

        for (i, case) in switch.cases.iter().enumerate() {
            let case_align = case.alignment.get().unwrap();
            // serializing / parsing wouldn't work correctly otherwise
            assert!(case_align.internal_align <= case_align.begin.align);
            assert_eq!(case_align.begin.offset % case_align.internal_align, 0);

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

                self.filter_derives_for_fields(&mut derives, &*case_fields);
                self.emit_struct_type(
                    &type_name,
                    &type_name,
                    derives,
                    &*case_fields,
                    &*ext_params,
                    false,
                    generate_try_parse,
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
            self.filter_derives_for_fields(&mut derives, &*case.fields.borrow());
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
                        outln!(out, "match self {{",);
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
                                self.expr_to_str(&case.exprs[0], true, true, false),
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
                                self.expr_to_str(&case.exprs[0], true, true, false),
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
                    self.expr_to_str(&switch.expr, false, true, false),
                );
                outln!(out, "let mut outer_remaining = value;");
                if switch.kind == xcbdefs::SwitchKind::BitCase {
                    let mut rust_case_names = Vec::new();
                    for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                        let mut case_expr_str = format!(
                            "switch_expr & {} != 0",
                            self.expr_to_str(&case.exprs[0], false, true, true),
                        );
                        for expr in case.exprs[1..].iter() {
                            use std::fmt::Write as _;
                            write!(
                                case_expr_str,
                                " || switch_expr & {} != 0",
                                self.expr_to_str(expr, false, true, true),
                            )
                            .unwrap();
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
                                        self.emit_field_parse(field, name, out);
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
                            self.expr_to_str(&case.exprs[0], false, true, true),
                        );
                        for expr in case.exprs[1..].iter() {
                            use std::fmt::Write as _;
                            write!(
                                case_expr_str,
                                " || switch_expr == {}",
                                self.expr_to_str(expr, false, true, true),
                            )
                            .unwrap();
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
                                        self.emit_field_parse(field, name, out);
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
                    outln!(out.indent(), "None => Err(ParseError::ParseError),");
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
        outln!(out, "impl Serialize for {} {{", name);
        out.indented(|out| {
            outln!(out, "type Bytes = [u8; {}];", size);
            outln!(out, "fn serialize(&self) -> Self::Bytes {{");
            out.indented(|out| {
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
                                            "",
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
            outln!(out, "fn serialize_into(&self, bytes: &mut Vec<u8>) {{");
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
                                        "",
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
        outln!(out, "impl Serialize for {} {{", name);
        out.indented(|out| {
            outln!(out, "type Bytes = Vec<u8>;");
            outln!(out, "fn serialize(&self) -> Self::Bytes {{");
            out.indented(|out| {
                outln!(out, "let mut result = Vec::new();");
                outln!(out, "self.serialize_into(&mut result);");
                outln!(out, "result");
            });
            outln!(out, "}}");
            outln!(out, "fn serialize_into(&self, bytes: &mut Vec<u8>) {{");
            out.indented(|out| {
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
                                            "",
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
                                    "if let Some(ref {}) = self.{} {{",
                                    rust_field_name,
                                    rust_field_name,
                                );
                                outln!(out.indent(), "{}.serialize_into(bytes);", rust_field_name);
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
                                                "",
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

    fn emit_field_parse(&self, field: &xcbdefs::FieldDef, switch_prefix: &str, out: &mut Output) {
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
                    "let remaining = remaining.get({}..).ok_or(ParseError::ParseError)?;",
                    pad_size
                );
            }
            xcbdefs::FieldDef::Normal(normal_field) => {
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                outln!(
                    out,
                    "let ({}, remaining) = {};",
                    rust_field_name,
                    self.emit_value_parse(&normal_field.type_),
                );
            }
            xcbdefs::FieldDef::List(list_field) => {
                let rust_field_name = to_rust_variable_name(&list_field.name);

                if self.rust_value_type_is_u8(&list_field.element_type) {
                    // List of `u8`, use simple parsing
                    if let Some(list_length) = list_field.length() {
                        outln!(
                            out,
                            "let ({}, remaining) = crate::x11_utils::parse_u8_list(remaining, {})?;",
                            rust_field_name,
                            list_length,
                        );
                        outln!(
                            out,
                            "let {} = <[u8; {}]>::try_from({}).unwrap();",
                            rust_field_name,
                            list_length,
                            rust_field_name,
                        );
                    } else if let Some(ref length_expr) = list_field.length_expr {
                        outln!(
                            out,
                            "let ({}, remaining) = crate::x11_utils\
                            ::parse_u8_list(remaining, {}.try_into().or(Err(ParseError::ParseError))?)?;",
                            rust_field_name,
                            self.expr_to_str(length_expr, false, false, true),
                        );
                        outln!(
                            out,
                            "let {} = {}.to_vec();",
                            rust_field_name,
                            rust_field_name
                        );
                    } else {
                        outln!(out, "let {} = remaining.to_vec();", rust_field_name);
                        outln!(out, "let remaining = &remaining[remaining.len()..];");
                    }
                } else if self.can_use_simple_list_parsing(&list_field.element_type)
                    && list_field.length_expr.is_some()
                    && list_field.length().is_none()
                {
                    let rust_element_type =
                        self.type_to_rust_type(list_field.element_type.type_.def.get().unwrap());
                    outln!(
                        out,
                        "let ({}, remaining) = crate::x11_utils\
                        ::parse_list::<{}>(remaining, {}.try_into().or(Err(ParseError::ParseError))?)?;",
                        rust_field_name,
                        rust_element_type,
                        self.expr_to_str(
                            list_field.length_expr.as_ref().unwrap(),
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
                            self.emit_value_parse(&list_field.element_type),
                        );
                        self.emit_value_post_parse(&list_field.element_type, &tmp_name, out);
                    }
                    outln!(out, "let {} = [", rust_field_name);
                    for i in 0..list_len {
                        outln!(out.indent(), "{}_{},", rust_field_name, i);
                    }
                    outln!(out, "];");
                } else {
                    outln!(out, "let mut remaining = remaining;");
                    if let Some(ref length_expr) = list_field.length_expr {
                        outln!(
                            out,
                            "let list_length = usize::try_from({}).or(Err(ParseError::ParseError))?;",
                            self.expr_to_str(length_expr, false, false, false),
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
                            self.emit_value_parse(&list_field.element_type),
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
                let switch_struct_name =
                    format!("{}{}", switch_prefix, to_rust_type_name(&switch_field.name));
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
                    "if fds.is_empty() {{ return Err(ParseError::ParseError) }}"
                );
                outln!(out, "let {} = fds.remove(0);", rust_field_name);
            }
            xcbdefs::FieldDef::FdList(fd_list_field) => {
                let rust_field_name = to_rust_variable_name(&fd_list_field.name);

                outln!(
                    out,
                    "let fds_len = usize::try_from({}).or(Err(ParseError::ParseError))?;",
                    self.expr_to_str(&fd_list_field.length_expr, false, false, false),
                );
                outln!(
                    out,
                    "if fds.len() < fds_len {{ return Err(ParseError::ParseError) }}",
                );
                outln!(out, "let mut {} = fds.split_off(fds_len);", rust_field_name);
                outln!(out, "std::mem::swap(fds, &mut {});", rust_field_name);
            }
            xcbdefs::FieldDef::Expr(_) => {
                // Only supported in requests
                unreachable!();
            }
            xcbdefs::FieldDef::VirtualLen(_) => {}
        }
    }

    fn emit_field_post_parse(&self, field: &xcbdefs::FieldDef, out: &mut Output) {
        if let xcbdefs::FieldDef::Normal(normal_field) = field {
            let rust_field_name = to_rust_variable_name(&normal_field.name);
            self.emit_value_post_parse(&normal_field.type_, &rust_field_name, out);
        }
    }

    fn emit_value_parse(&self, type_: &xcbdefs::FieldValueType) -> String {
        // Don't handle enum values yet
        let type_type = type_.type_.def.get().unwrap();
        let rust_type = self.type_to_rust_type(type_type);
        let params = self.get_type_parse_params(type_type, "remaining");
        format!("{}::try_parse({})?", rust_type, params.join(", "))
    }

    fn emit_value_post_parse(
        &self,
        type_: &xcbdefs::FieldValueType,
        var_name: &str,
        out: &mut Output,
    ) {
        if let xcbdefs::FieldValueSet::Enum(ref enum_) = type_.value_set {
            // Handle turning things into enum instances where necessary.
            let enum_def = match enum_.def.get().unwrap() {
                xcbdefs::TypeRef::Enum(enum_def) => enum_def.upgrade().unwrap(),
                _ => unreachable!(),
            };
            if !self.enum_has_repeated_values(&enum_def) {
                outln!(out, "let {} = {}.try_into()?;", var_name, var_name);
            }
        }
    }

    fn needs_post_parse(&self, type_: &xcbdefs::FieldValueType) -> bool {
        if let xcbdefs::FieldValueSet::Enum(ref enum_) = type_.value_set {
            let enum_def = match enum_.def.get().unwrap() {
                xcbdefs::TypeRef::Enum(enum_def) => enum_def.upgrade().unwrap(),
                _ => unreachable!(),
            };
            !self.enum_has_repeated_values(&enum_def)
        } else {
            false
        }
    }

    fn can_use_simple_list_parsing(&self, type_: &xcbdefs::FieldValueType) -> bool {
        self.get_type_parse_params(&type_.type_.def.get().unwrap(), "")
            .len()
            == 1
            && !self.needs_post_parse(type_)
    }

    /// Returns `Some(bytes)` if the serialized result is a single
    /// array of bytes.
    fn emit_field_serialize(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &FxHashMap<String, DeducibleField>,
        obj_name: &str,
        result_bytes: &mut Vec<String>,
        out: &mut Output,
    ) -> Option<String> {
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
                let field_size = normal_field.type_.type_.def.get().unwrap().size().unwrap();
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                let bytes_name = postfix_var_name(&rust_field_name, "bytes");

                if let Some(deducible_field) = deducible_fields.get(&normal_field.name) {
                    self.emit_calc_deducible_field(
                        field,
                        deducible_field,
                        obj_name,
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
                    let src_value = format!("{}{}", obj_name, rust_field_name);
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
                let rust_field_name = to_rust_variable_name(&list_field.name);
                if self.rust_value_type_is_u8(&list_field.element_type) {
                    // Fixed-sized list with `u8` members
                    for i in 0..list_length {
                        result_bytes.push(format!("{}{}[{}]", obj_name, rust_field_name, i));
                    }
                    Some(format!("{}{}", obj_name, rust_field_name))
                } else {
                    let element_size = list_field.element_type.size().unwrap();
                    for i in 0..list_length {
                        let src_value = format!("{}{}[{}]", obj_name, rust_field_name, i);
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
                    "let {} = {}{}.serialize();",
                    bytes_name,
                    obj_name,
                    rust_field_name,
                );
                for i in 0..field_size {
                    result_bytes.push(format!("{}[{}]", bytes_name, i));
                }
                Some(bytes_name)
            }
            // the reamining field types are only used in request and replies,
            // which do not implement serialize
            _ => unreachable!(),
        }
    }

    fn emit_field_serialize_into(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &FxHashMap<String, DeducibleField>,
        obj_name: &str,
        bytes_name: &str,
        out: &mut Output,
    ) {
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
                        obj_name,
                        &rust_field_name,
                        out,
                    );
                    self.emit_value_serialize_into(
                        &normal_field.type_,
                        &rust_field_name,
                        true,
                        bytes_name,
                        out,
                    );
                } else {
                    let src_value = format!("{}{}", obj_name, rust_field_name);
                    self.emit_value_serialize_into(
                        &normal_field.type_,
                        &src_value,
                        false,
                        bytes_name,
                        out,
                    );
                }
            }
            xcbdefs::FieldDef::List(list_field) => {
                let rust_field_name = to_rust_variable_name(&list_field.name);
                if self.rust_value_type_is_u8(&list_field.element_type) {
                    // Fixed-sized list with `u8` members
                    outln!(
                        out,
                        "{}.extend_from_slice(&{}{});",
                        bytes_name,
                        obj_name,
                        rust_field_name,
                    );
                } else if self.can_use_simple_list_parsing(&list_field.element_type) {
                    outln!(
                        out,
                        "{}{}.serialize_into({});",
                        obj_name,
                        rust_field_name,
                        bytes_name
                    );
                } else {
                    outln!(
                        out,
                        "for element in {}{}.iter() {{",
                        obj_name,
                        rust_field_name
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
                let rust_field_name = to_rust_variable_name(&switch_field.name);
                outln!(
                    out,
                    "{}{}.serialize_into({});",
                    obj_name,
                    rust_field_name,
                    bytes_name
                );
            }
            // the reamining field types are only used in request and replies,
            // which do not implement serialize
            _ => unreachable!(),
        }
    }

    /// Emits an assert that checks the consistency of list lengths
    /// or switch discriminator.
    fn emit_assert_for_field_serialize(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &FxHashMap<String, DeducibleField>,
        obj_name: &str,
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
                            DeducibleField::CaseSwitchExpr(_) => false,
                            DeducibleField::BitCaseSwitchExpr(_) => false,
                        })
                        && list_field.length_expr.is_some()
                        && list_field.length().is_none();

                if needs_assert {
                    let rust_field_name = to_rust_variable_name(&list_field.name);
                    let length_expr_str = self.expr_to_str(
                        list_field.length_expr.as_ref().unwrap(),
                        true,
                        false,
                        false,
                    );
                    outln!(
                        out,
                        "assert_eq!({}{}.len(), \
                        usize::try_from({}).unwrap(), \
                        \"`{}` has an incorrect length\");",
                        obj_name,
                        rust_field_name,
                        length_expr_str,
                        rust_field_name,
                    );
                }
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                let needs_assert =
                    !deducible_fields
                        .values()
                        .any(|deducible_field| match deducible_field {
                            DeducibleField::LengthOf(_, _) => false,
                            DeducibleField::CaseSwitchExpr(switch_name) => {
                                *switch_name == switch_field.name
                            }
                            DeducibleField::BitCaseSwitchExpr(switch_name) => {
                                *switch_name == switch_field.name
                            }
                        });

                if needs_assert {
                    let rust_field_name = to_rust_variable_name(&switch_field.name);
                    let switch_expr_str = self.expr_to_str(&switch_field.expr, true, true, false);
                    outln!(
                        out,
                        "assert_eq!({}{}.switch_expr(), {}, \
                        \"expression `{}` must match the value of `{}`\");",
                        obj_name,
                        rust_field_name,
                        switch_expr_str,
                        switch_expr_str,
                        rust_field_name,
                    );
                }
            }
            xcbdefs::FieldDef::Fd(_) => {}
            xcbdefs::FieldDef::FdList(fd_list_field) => {
                let needs_assert =
                    !deducible_fields
                        .values()
                        .any(|deducible_field| match deducible_field {
                            DeducibleField::LengthOf(list_name, _) => {
                                *list_name == fd_list_field.name
                            }
                            DeducibleField::CaseSwitchExpr(_) => false,
                            DeducibleField::BitCaseSwitchExpr(_) => false,
                        })
                        && fd_list_field.length().is_none();

                if needs_assert {
                    let rust_field_name = to_rust_variable_name(&fd_list_field.name);
                    let length_expr_str =
                        self.expr_to_str(&fd_list_field.length_expr, true, false, false);
                    outln!(
                        out,
                        "assert_eq!({}{}.len(), \
                        usize::try_from({}).unwrap(), \
                        \"`{}` has an incorrect length\");",
                        obj_name,
                        rust_field_name,
                        length_expr_str,
                        rust_field_name,
                    );
                }
            }
            xcbdefs::FieldDef::Expr(_) => {}
            xcbdefs::FieldDef::VirtualLen(_) => {}
        }
    }

    fn emit_value_serialize(
        &self,
        type_: &xcbdefs::FieldValueType,
        value: &str,
        was_deduced: bool,
    ) -> String {
        // Deduced fields are not converter to their enum value
        if !was_deduced && self.use_enum_type_in_field(type_).is_some() {
            let rust_wire_type = self.type_to_rust_type(type_.type_.def.get().unwrap());
            format!("{}::from({}).serialize()", rust_wire_type, value)
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
        // Deduced fields are not converter to their enum value
        if !was_deduced && self.use_enum_type_in_field(type_).is_some() {
            let rust_wire_type = self.type_to_rust_type(type_.type_.def.get().unwrap());
            outln!(
                out,
                "{}::from({}).serialize_into({});",
                rust_wire_type,
                value,
                bytes_var
            );
        } else {
            outln!(out, "{}.serialize_into({});", value, bytes_var);
        }
    }

    fn emit_calc_deducible_field(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_field: &DeducibleField,
        obj_name: &str,
        dst_var_name: &str,
        out: &mut Output,
    ) {
        match deducible_field {
            DeducibleField::LengthOf(list_field_name, op) => {
                if let xcbdefs::FieldDef::Normal(normal_field) = field {
                    let rust_type = self.field_value_type_to_rust_type(&normal_field.type_);
                    let rust_list_field_name = to_rust_variable_name(list_field_name);
                    let msg = format!("`{}` has too many elements", rust_list_field_name);
                    let list_len = format!("{}{}.len()", obj_name, rust_list_field_name);
                    let value = match op {
                        DeducibleLengthFieldOp::None => {
                            format!("{}::try_from({}).expect(\"{}\")", rust_type, list_len, msg)
                        }
                        DeducibleLengthFieldOp::Mul(n) => format!(
                            "{}::try_from({}).ok()\
                            .and_then(|len| len.checked_mul({})).expect(\"{}\")",
                            rust_type, list_len, n, msg
                        ),
                        DeducibleLengthFieldOp::Div(n) => {
                            outln!(
                                out,
                                "assert_eq!({} % {}, 0, \
                                \"`{}` has an incorrect length, must be a multiple of {}\");",
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
            DeducibleField::CaseSwitchExpr(switch_field_name) => {
                if let xcbdefs::FieldDef::Normal(_) = field {
                    outln!(
                        out,
                        "let {} = {}{}.switch_expr();",
                        dst_var_name,
                        obj_name,
                        to_rust_variable_name(switch_field_name),
                    );
                } else {
                    unreachable!();
                }
            }
            DeducibleField::BitCaseSwitchExpr(switch_field_name) => {
                if let xcbdefs::FieldDef::Normal(_) = field {
                    // TODO: Try to handle xkb::SelectEvents
                    outln!(
                        out,
                        "let {} = {}{}.switch_expr();",
                        dst_var_name,
                        obj_name,
                        to_rust_variable_name(switch_field_name),
                    )
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
            let original_type = match type_.type_.def.get().unwrap() {
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
            let enum_def = match enum_.def.get().unwrap() {
                xcbdefs::TypeRef::Enum(enum_def) => enum_def.upgrade().unwrap(),
                _ => unreachable!(),
            };
            if !self.enum_has_repeated_values(&enum_def) {
                // The field can only have the values from the enum,
                // use its type.
                Some(enum_def)
            } else {
                None
            }
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
                    field.doc.as_ref().map(String::as_str).unwrap_or("").trim(),
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
                    error.doc.as_ref().map(String::as_str).unwrap_or("").trim(),
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

    fn expr_to_str(
        &self,
        expr: &xcbdefs::Expression,
        panic_on_overflow: bool,
        cast_to_u32: bool,
        needs_parens: bool,
    ) -> String {
        match expr {
            xcbdefs::Expression::BinaryOp(bin_op_expr) => {
                let err_handler = if panic_on_overflow {
                    ".unwrap()"
                } else {
                    ".ok_or(ParseError::ParseError)?"
                };
                match bin_op_expr.operator {
                    xcbdefs::BinaryOperator::Add => format!(
                        "{}.checked_add({}){}",
                        self.expr_to_str(&bin_op_expr.lhs, panic_on_overflow, true, true),
                        self.expr_to_str(&bin_op_expr.rhs, panic_on_overflow, true, false),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::Sub => format!(
                        "{}.checked_sub({}){}",
                        self.expr_to_str(&bin_op_expr.lhs, panic_on_overflow, true, true),
                        self.expr_to_str(&bin_op_expr.rhs, panic_on_overflow, true, false),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::Mul => format!(
                        "{}.checked_mul({}){}",
                        self.expr_to_str(&bin_op_expr.lhs, panic_on_overflow, true, true),
                        self.expr_to_str(&bin_op_expr.rhs, panic_on_overflow, true, false),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::Div => format!(
                        "{}.checked_div({}){}",
                        self.expr_to_str(&bin_op_expr.lhs, panic_on_overflow, true, true),
                        self.expr_to_str(&bin_op_expr.rhs, panic_on_overflow, true, false),
                        err_handler,
                    ),
                    xcbdefs::BinaryOperator::And => {
                        let lhs_str =
                            self.expr_to_str(&bin_op_expr.lhs, panic_on_overflow, true, true);
                        let rhs_str =
                            self.expr_to_str(&bin_op_expr.rhs, panic_on_overflow, true, true);
                        if needs_parens {
                            format!("({} & {})", lhs_str, rhs_str)
                        } else {
                            format!("{} & {}", lhs_str, rhs_str)
                        }
                    }
                    // I'm not sure know how to handle overflow here,
                    // but this operator never appears in the XMLs
                    xcbdefs::BinaryOperator::Shl => unimplemented!(),
                }
            }
            xcbdefs::Expression::UnaryOp(unary_op_expr) => match unary_op_expr.operator {
                xcbdefs::UnaryOperator::Not => {
                    let rhs_str =
                        self.expr_to_str(&unary_op_expr.rhs, panic_on_overflow, true, true);
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
                    xcbdefs::FieldRefKind::LocalField => {
                        to_rust_variable_name(&field_ref_expr.field_name)
                    }
                    xcbdefs::FieldRefKind::ExtParam => {
                        to_rust_variable_name(&field_ref_expr.field_name)
                    }
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
                let param_is_card32 = match param_ref_expr.type_.def.get().unwrap() {
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
                let rust_enum_type = self.type_to_rust_type(enum_ref_expr.enum_.def.get().unwrap());
                let rust_variant_name = ename_to_rust(&enum_ref_expr.variant);
                format!("u32::from({}::{})", rust_enum_type, rust_variant_name,)
            }
            xcbdefs::Expression::PopCount(pop_count_expr) => {
                let arg = self.expr_to_str(pop_count_expr, panic_on_overflow, false, true);
                format!("{}.count_ones()", arg)
            }
            xcbdefs::Expression::SumOf(sum_of_expr) => {
                // nested sum-of not supported
                assert_ne!(
                    sum_of_expr.resolved_field.get().unwrap().ref_kind,
                    xcbdefs::FieldRefKind::SumOfRef,
                );
                let rust_field_name = to_rust_variable_name(&sum_of_expr.field_name);
                if panic_on_overflow {
                    if let Some(ref operand) = sum_of_expr.operand {
                        format!(
                            "{}.iter().fold(0u32, |acc, x| acc.checked_add({}).unwrap())",
                            rust_field_name,
                            self.expr_to_str(operand, panic_on_overflow, true, true),
                        )
                    } else {
                        format!(
                            "{}.iter().try(0u32, |acc, &x| acc.checked_add(u32::from(x)).unwrap())",
                            rust_field_name,
                        )
                    }
                } else {
                    if let Some(ref operand) = sum_of_expr.operand {
                        format!(
                            "{}.iter().try_fold(0u32, |acc, x| \
                            acc.checked_add({}).ok_or(ParseError::ParseError))?",
                            rust_field_name,
                            self.expr_to_str(operand, panic_on_overflow, true, true),
                        )
                    } else {
                        format!(
                            "{}.iter().try_fold(0u32, |acc, &x| \
                            acc.checked_add(u32::from(x)).ok_or(ParseError::ParseError))?",
                            rust_field_name,
                        )
                    }
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

    /// Returns whether `enum_def` has repeated values.
    fn enum_has_repeated_values(&self, enum_def: &xcbdefs::EnumDef) -> bool {
        let id = enum_def as *const xcbdefs::EnumDef as usize;
        match self.caches.borrow_mut().enum_has_repeated_values.entry(id) {
            HashMapEntry::Occupied(entry) => *entry.get(),
            HashMapEntry::Vacant(entry) => {
                let mut value_set = FxHashSet::default();
                let mut has_repeated = false;
                for enum_item in enum_def.items.iter() {
                    let value = match enum_item.value {
                        xcbdefs::EnumValue::Value(v) => v,
                        xcbdefs::EnumValue::Bit(bit) => 1 << bit,
                    };
                    if !value_set.insert(value) {
                        // value already present
                        has_repeated = true;
                        break;
                    }
                }
                entry.insert(has_repeated);
                has_repeated
            }
        }
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

    /// Gathers information about the fields of a request,
    /// returning a `GatheredRequestFields`.
    fn gather_request_fields(
        &self,
        request_def: &xcbdefs::RequestDef,
        deducible_fields: &FxHashMap<String, DeducibleField>,
    ) -> GatheredRequestFields {
        let mut letter_iter = b'A'..=b'Z';

        let mut needs_lifetime = false;
        let mut args = Vec::new();
        let mut generics = Vec::new();
        let mut preamble = Vec::new();
        let mut single_fds = Vec::new();
        let mut fd_lists = Vec::new();

        let ns = request_def.namespace.upgrade().unwrap();
        let is_change_propery = request_def.name == "ChangeProperty" && ns.header == "xproto";
        let is_get_propery = request_def.name == "GetProperty" && ns.header == "xproto";
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
                    let use_into = if ((is_change_propery || is_get_propery)
                        && normal_field.name == "property")
                        || (is_change_propery && normal_field.name == "type")
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
                        args.push((rust_field_name, generic_param.clone()));
                        generics.push((generic_param, where_));
                        preamble.push(preamble_part);
                    } else {
                        args.push((rust_field_name, rust_field_type));
                    }
                }
                xcbdefs::FieldDef::List(list_field) => {
                    if is_send_event && list_field.name == "event" {
                        let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                        args.push((list_field.name.clone(), generic_param.clone()));
                        generics.push((generic_param, String::from("Into<[u8; 32]>")));
                        preamble.push(String::from("let event: [u8; 32] = event.into();"));
                    } else {
                        let element_type =
                            self.field_value_type_to_rust_type(&list_field.element_type);
                        let rust_field_name = to_rust_variable_name(&list_field.name);
                        let rust_field_type = if let Some(list_len) = list_field.length() {
                            format!("&[{}; {}]", element_type, list_len)
                        } else {
                            format!("&[{}]", element_type)
                        };
                        args.push((rust_field_name, rust_field_type));
                        needs_lifetime = true;
                    }
                }
                xcbdefs::FieldDef::Switch(switch_field) => {
                    let rust_field_name = to_rust_variable_name(&switch_field.name);
                    let rust_field_type = format!("&{}Aux", to_rust_type_name(&request_def.name));
                    args.push((rust_field_name, rust_field_type));
                    needs_lifetime = true;
                }
                xcbdefs::FieldDef::Fd(fd_field) => {
                    let rust_field_name = to_rust_variable_name(&fd_field.name);
                    let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                    let preamble_part = format!(
                        "let {}: RawFdContainer = {}.into();",
                        rust_field_name, rust_field_name,
                    );
                    args.push((rust_field_name, generic_param.clone()));
                    generics.push((generic_param, "Into<RawFdContainer>".into()));
                    preamble.push(preamble_part);
                    single_fds.push(fd_field.name.clone());
                }
                xcbdefs::FieldDef::FdList(fd_list_field) => {
                    let rust_field_name = to_rust_variable_name(&fd_list_field.name);
                    args.push((rust_field_name, "Vec<RawFdContainer>".into()));
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

        GatheredRequestFields {
            reply_has_fds,
            needs_lifetime,
            args,
            generics,
            preamble,
            single_fds,
            fd_lists,
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
            self.type_to_rust_type(value_type.type_.def.get().unwrap())
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
            self.filter_derives_for_fields(&mut derives, &*struct_def.fields.borrow());
            self.caches.borrow_mut().derives.insert(id, derives);
            derives
        }
    }

    /// Clears traits from `derives` that cannot be implemented by
    /// `fields`.
    fn filter_derives_for_fields(&self, derives: &mut Derives, fields: &[xcbdefs::FieldDef]) {
        for field in fields.iter() {
            match field {
                xcbdefs::FieldDef::Pad(_) => {}
                xcbdefs::FieldDef::Normal(normal_field) => {
                    self.filter_derives_for_type(
                        derives,
                        normal_field.type_.type_.def.get().unwrap(),
                    );
                }
                xcbdefs::FieldDef::List(list_field) => {
                    self.filter_derives_for_type(
                        derives,
                        list_field.element_type.type_.def.get().unwrap(),
                    );

                    if !list_field.has_fixed_length() {
                        // Variable length list, represented as Vec
                        derives.copy = false;
                    }
                }
                xcbdefs::FieldDef::Switch(switch_field) => {
                    for case in switch_field.cases.iter() {
                        self.filter_derives_for_fields(derives, &*case.fields.borrow());
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
                // Only appears in requests
                unreachable!();
            }
            xcbdefs::TypeRef::Xid(_) => {}
            xcbdefs::TypeRef::XidUnion(_) => {}
            xcbdefs::TypeRef::Enum(_) => {}
            xcbdefs::TypeRef::Alias(type_alias_def) => {
                let type_alias_def = type_alias_def.upgrade().unwrap();
                self.filter_derives_for_type(derives, type_alias_def.old_name.def.get().unwrap())
            }
        }
    }

    /// Whether the field is visible (i.e., appears in parsed rust structs)
    fn field_is_visible(
        &self,
        field: &xcbdefs::FieldDef,
        deducible_fields: &FxHashMap<String, DeducibleField>,
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
        deducible_fields: &FxHashMap<String, DeducibleField>,
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
    CaseSwitchExpr(String),
    /// The value is the discriminant of a bitcase switch
    ///
    /// `(switch name)`
    BitCaseSwitchExpr(String),
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

/// Gathers deducible fields (fields whose value can be calculated
/// from other fields) from a list of fields.
fn gather_deducible_fields(fields: &[xcbdefs::FieldDef]) -> FxHashMap<String, DeducibleField> {
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

    let mut deducible_fields = FxHashMap::default();
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
                            DeducibleField::CaseSwitchExpr(switch_field.name.clone()),
                        ))
                    } else {
                        None
                    }
                } else if switch_field.kind == xcbdefs::SwitchKind::BitCase {
                    if let xcbdefs::Expression::FieldRef(ref field_ref_expr) = switch_field.expr {
                        Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleField::BitCaseSwitchExpr(switch_field.name.clone()),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some((field_name, deducible_field)) = deducible_field {
            match deducible_fields.entry(field_name) {
                HashMapEntry::Occupied(mut entry) => {
                    // field used more than once,
                    // do not deduce it
                    *entry.get_mut() = None;
                }
                HashMapEntry::Vacant(entry) => {
                    entry.insert(Some(deducible_field));
                }
            }
        }
    }

    // filter out `None`s
    deducible_fields
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect()
}

/// Some information about the fields of a request.
struct GatheredRequestFields {
    reply_has_fds: bool,
    /// Whether the request function needs a lifetime parameter
    /// for the connection object.
    needs_lifetime: bool,
    /// Function arguments
    /// `(name, type)`
    args: Vec<(String, String)>,
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
            match normal_field
                .type_
                .type_
                .def
                .get()
                .unwrap()
                .get_original_type()
            {
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
    name
}
