use std::borrow::Cow;
use std::collections::HashMap;

use super::{
    expr_to_str, gather_deducible_fields, get_ns_name_prefix, parse, serialize, special_cases,
    struct_type, switch, to_rust_type_name, to_rust_variable_name, CaseInfo, DeducibleField,
    Derives, FieldContainer, NamespaceGenerator, Output, PerModuleEnumCases, StructSizeConstraint,
};

use xcbgen::defs as xcbdefs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IovecConversion {
    // No conversion is required.
    None,
    // A simple .into() is enough.
    Into,
    // Call cow_strip_length.
    CowStripLength,
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

pub(super) fn generate_request(
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    out: &mut Output,
    trait_out: &mut Output,
    enum_cases: &mut PerModuleEnumCases,
) {
    let name = to_rust_type_name(&request_def.name);

    let mut function_name = super::super::camel_case_to_lower_snake(&name);
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
        generator.ns.header,
        request_def.name,
    );

    let deducible_fields = gather_deducible_fields(&*request_fields);

    if switch_fields.len() == 1 {
        if let Some(aux_start_align) = switch_fields[0].required_start_align {
            assert_eq!(aux_start_align.offset(), 0);
        }
        generate_aux(
            generator,
            request_def,
            switch_fields[0],
            &function_name,
            out,
        );
    }

    outln!(out, "/// Opcode for the {} request", name);
    outln!(
        out,
        "pub const {}_REQUEST: u8 = {};",
        super::super::camel_case_to_upper_snake(&name),
        request_def.opcode,
    );

    let gathered = gather_request_fields(generator, request_def, &deducible_fields);

    emit_request_struct(
        generator,
        request_def,
        &name,
        &deducible_fields,
        &gathered,
        out,
    );
    let ns_prefix = get_ns_name_prefix(generator.ns);
    let lifetime_block = if gathered.needs_lifetime {
        "<'input>"
    } else {
        ""
    };
    enum_cases.request_variants.push(format!(
        "{ns_prefix}{name}({header}::{name}Request{lifetime}),",
        ns_prefix = ns_prefix,
        name = name,
        header = generator.ns.header,
        lifetime = lifetime_block
    ));
    if gathered.has_fds() {
        enum_cases.request_parse_cases.push(format!(
            "{header}::{opcode_name}_REQUEST => return \
             Ok(Request::{ns_prefix}{name}({header}::{name}Request::\
             try_parse_request_fd(header, remaining, fds)?)),",
            header = generator.ns.header,
            opcode_name = super::super::camel_case_to_upper_snake(&name),
            ns_prefix = ns_prefix,
            name = name,
        ));
    } else {
        enum_cases.request_parse_cases.push(format!(
            "{header}::{opcode_name}_REQUEST => return \
             Ok(Request::{ns_prefix}{name}({header}::{name}Request::try_parse_request(header, \
             remaining)?)),",
            header = generator.ns.header,
            opcode_name = super::super::camel_case_to_upper_snake(&name),
            ns_prefix = ns_prefix,
            name = name,
        ));
    }
    emit_request_function(
        generator,
        request_def,
        &name,
        &function_name,
        &gathered,
        out,
    );
    emit_request_trait_function(
        generator,
        request_def,
        &name,
        &function_name,
        &gathered,
        trait_out,
    );

    super::special_cases::handle_request(request_def, out);

    outln!(out, "");

    if let Some(ref reply) = request_def.reply {
        let reply_struct_name = format!("{}Reply", name);
        enum_cases.reply_variants.push(format!(
            "{ns_prefix}{name}({header}::{name}Reply),",
            ns_prefix = ns_prefix,
            name = name,
            header = generator.ns.header,
        ));
        enum_cases.reply_parse_cases.push(format!(
            "Request::{ns_prefix}{name}(_) => Some({header}::{name}Request::parse_reply),",
            ns_prefix = ns_prefix,
            name = name,
            header = generator.ns.header,
        ));
        enum_cases.reply_from_cases.push(format!(
            r#"impl From<{header}::{name}Reply> for Reply {{
  fn from(reply: {header}::{name}Reply) -> Reply {{
    Reply::{ns_prefix}{name}(reply)
  }}
}}"#,
            ns_prefix = ns_prefix,
            name = name,
            header = generator.ns.header,
        ));
        let reply_fields = reply.fields.borrow();
        let mut reply_derives = Derives::all();
        generator.filter_derives_for_fields(&mut reply_derives, &*reply_fields, false);
        struct_type::emit_struct_type(
            generator,
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
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    switch_field: &xcbdefs::SwitchField,
    function_name: &str,
    out: &mut Output,
) {
    let aux_name = format!("{}Aux", request_def.name);

    if switch_field.kind == xcbdefs::SwitchKind::Case {
        switch::emit_switch_type(generator, switch_field, &aux_name, true, true, None, out);
    } else {
        let doc = format!(
            "Auxiliary and optional information for the `{}` function",
            function_name,
        );
        let cases_infos = switch::emit_switch_type(
            generator,
            switch_field,
            &aux_name,
            true,
            true,
            Some(&doc),
            out,
        );

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
                            generator.field_to_rust_type(single_field, &aux_name),
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
                outln!(out, "#[must_use]");
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

    special_cases::handle_request_switch(request_def, switch_field, &aux_name, out);

    outln!(out, "");
}

fn emit_request_struct(
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    name: &str,
    deducible_fields: &HashMap<String, DeducibleField>,
    gathered: &GatheredRequestFields,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    let is_xproto = ns.header == "xproto";
    let is_send_event = is_xproto && request_def.name == "SendEvent";

    if let Some(ref doc) = request_def.doc {
        generator.emit_doc(doc, out, Some(deducible_fields));
    }

    let mut derives = Derives::all();
    generator.filter_derives_for_fields(&mut derives, &request_def.fields.borrow(), true);
    let derives = derives.to_list();
    if !derives.is_empty() {
        outln!(out, "#[derive({})]", derives.join(", "));
    }

    let (struct_lifetime_block, serialize_lifetime_return, parse_lifetime_block) =
        if gathered.needs_lifetime {
            ("<'input>", "'input", "'input ")
        } else {
            ("", "'static", "")
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
            "fn serialize(self{opcode}) -> BufWithFds<PiecewiseBuf<{lifetime}>> {{",
            opcode = if is_xproto { "" } else { ", major_opcode: u8" },
            lifetime = serialize_lifetime_return,
        );
        out.indented(|out| {
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
                serialize::emit_assert_for_field_serialize(
                    generator,
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
                                fixed_fields_bytes.push(String::from("major_opcode"));
                            } else {
                                fixed_fields_bytes.push(format!(
                                    "{}_REQUEST",
                                    super::super::camel_case_to_upper_snake(name),
                                ));
                            }
                        } else if normal_field.name == "minor_opcode" {
                            assert!(ns.ext_info.is_some());
                            fixed_fields_bytes.push(format!(
                                "{}_REQUEST",
                                super::super::camel_case_to_upper_snake(name),
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
                                generator.emit_calc_deducible_field(
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

                            let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
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
                                    serialize::emit_value_serialize(
                                        generator,
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
                        if generator.rust_value_type_is_u8(&list_field.element_type) {
                            let conversion = if list_length.is_some() {
                                // If this is a fixed length array we need to erase its length.
                                IovecConversion::CowStripLength
                            } else {
                                IovecConversion::None
                            };
                            next_slice = Some((format!("self.{}", rust_field_name), conversion));
                        } else {
                            assert_eq!(
                                list_length, None,
                                "fixed length arrays in requests must be [u8; N]"
                            );
                            let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                            if parse::can_use_simple_list_parsing(
                                generator,
                                &list_field.element_type,
                            ) {
                                outln!(
                                    tmp_out,
                                    "let {} = self.{}.serialize();",
                                    bytes_name,
                                    rust_field_name,
                                );
                            } else {
                                outln!(tmp_out, "let mut {} = Vec::new();", bytes_name);
                                outln!(tmp_out, "for element in {}.iter() {{", rust_field_name);
                                tmp_out.indented(|tmp_out| {
                                    serialize::emit_value_serialize_into(
                                        generator,
                                        &list_field.element_type,
                                        "element",
                                        false,
                                        &bytes_name,
                                        tmp_out,
                                    );
                                });
                                outln!(tmp_out, "}}");
                            }
                            next_slice = Some((bytes_name, IovecConversion::Into));
                        }
                    }
                    xcbdefs::FieldDef::Switch(switch_field) => {
                        let rust_field_name = to_rust_variable_name(&switch_field.name);
                        let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                        outln!(
                            tmp_out,
                            "let {} = self.{}.serialize({});",
                            bytes_name,
                            rust_field_name,
                            generator.ext_params_to_call_args(
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
                        let bytes_name = super::postfix_var_name(&rust_field_name, "bytes");
                        let type_ = generator.field_value_type_to_rust_type(&expr_field.type_);
                        if type_ == "bool" {
                            outln!(
                                out,
                                "let {} = {} != 0;",
                                rust_field_name,
                                expr_to_str(
                                    generator,
                                    &expr_field.expr,
                                    to_rust_variable_name,
                                    true,
                                    Some("u32"),
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
                            serialize::emit_value_serialize(
                                generator,
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
                            IovecConversion::CowStripLength => {
                                request_slices.push(format!("Cow::Owned({}.to_vec())", next_slice))
                            }
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

            let result = format!(
                "(vec![{slices}], {fds})",
                slices = slices_arg,
                fds = fds_arg,
            );
            outln!(out, "{}", result);
        });
        outln!(out, "}}");

        // Sending method
        let is_xproto = ns.header == "xproto";
        let is_list_fonts_with_info = request_def.name == "ListFontsWithInfo" && is_xproto;
        let is_record_enable_context = request_def.name == "EnableContext" && ns.header == "record";
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
            outln!(
                out,
                "let (bytes, fds) = self.serialize({});",
                if is_xproto { "" } else { "major_opcode(conn)?" }
            );
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
                    outln!(out, "conn.send_request_with_reply_with_fds(&slices, fds)");
                } else {
                    outln!(out, "conn.send_request_with_reply(&slices, fds)",);
                }
            } else {
                outln!(out, "conn.send_request_without_reply(&slices, fds)",);
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
                    super::super::camel_case_to_upper_snake(name),
                );
                outln!(out.indent(), "return Err(ParseError::InvalidValue);");
                outln!(out, "}}");
            } else {
                // The minor opcode could be repurposed to store data for this request, so there's
                // no validation of it to be done.
                outln!(
                    out,
                    "if header.major_opcode != {}_REQUEST {{",
                    super::super::camel_case_to_upper_snake(name),
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
                parse::emit_field_parse(
                    generator,
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
                    parse::emit_field_post_parse(field, out);
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
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    name: &str,
    function_name: &str,
    gathered: &GatheredRequestFields,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    let is_list_fonts_with_info = request_def.name == "ListFontsWithInfo" && ns.header == "xproto";
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
        generator.emit_doc(doc, out, None);
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
    generator: &NamespaceGenerator<'_, '_>,
    request_def: &xcbdefs::RequestDef,
    name: &str,
    function_name: &str,
    gathered: &GatheredRequestFields,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    let is_list_fonts_with_info = request_def.name == "ListFontsWithInfo" && ns.header == "xproto";
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
        generator.emit_doc(doc, out, Default::default());
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

/// Gathers information about the fields of a request,
/// returning a `GatheredRequestFields`.
fn gather_request_fields(
    generator: &NamespaceGenerator<'_, '_>,
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
        if !generator.field_is_visible(field, deducible_fields) {
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
                let rust_field_type = generator.field_value_type_to_rust_type(&normal_field.type_);
                let use_into = if ((is_change_property || is_get_property)
                    && normal_field.name == "property")
                    || (is_change_property && normal_field.name == "type")
                {
                    true
                } else if generator
                    .use_enum_type_in_field(&normal_field.type_)
                    .is_none()
                {
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
                } else {
                    let element_type =
                        generator.field_value_type_to_rust_type(&list_field.element_type);
                    let rust_field_name = to_rust_variable_name(&list_field.name);
                    let rust_field_type =
                        if generator.rust_value_type_is_u8(&list_field.element_type) {
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
                }
                needs_lifetime = true;
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                let rust_field_name = to_rust_variable_name(&switch_field.name);
                let rust_field_type =
                    Type::VariableOwnership(format!("{}Aux", to_rust_type_name(&request_def.name)));
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
                request_args.push((rust_field_name, Type::Simple("Vec<RawFdContainer>".into())));
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
