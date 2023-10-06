use std::collections::HashMap;

use xcbgen::defs as xcbdefs;

use super::{
    expr_to_str, gather_deducible_fields, parse, serialize, switch, to_rust_variable_name,
    DeducibleField, DeducibleLengthFieldOp, Derives, FieldContainer, NamespaceGenerator, Output,
    StructSizeConstraint,
};

// FIXME: `skip_length_field` is broken
pub(super) fn emit_struct_type(
    generator: &NamespaceGenerator<'_, '_>,
    name: &str,
    switch_prefix: &str,
    derives: Derives,
    fields: &[xcbdefs::FieldDef],
    external_params: &[xcbdefs::ExternalParam],
    skip_length_field: bool,
    generate_try_parse: bool,
    parse_size_constraint: StructSizeConstraint<'_>,
    generate_serialize: bool,
    fields_need_serialize: bool,
    doc: Option<&xcbdefs::Doc>,
    out: &mut Output,
) {
    assert!(!generate_serialize || fields_need_serialize);
    assert!(matches!(parse_size_constraint, StructSizeConstraint::None) || generate_try_parse);

    let deducible_fields = gather_deducible_fields(fields);

    generate_switches_for_fields(
        generator,
        switch_prefix,
        fields,
        generate_try_parse,
        fields_need_serialize,
        out,
    );

    let has_fds = fields.iter().any(|field| {
        matches!(
            field,
            xcbdefs::FieldDef::Fd(_) | xcbdefs::FieldDef::FdList(_)
        )
    });

    if let Some(doc) = doc {
        generator.emit_doc(doc, out, Some(&deducible_fields));
    }
    let extras = derives.extra_traits_list();
    let derives = derives.to_list();
    if !derives.is_empty() {
        outln!(out, "#[derive({})]", derives.join(", "));
    }
    if !extras.is_empty() {
        outln!(
            out,
            "#[cfg_attr(feature = \"extra-traits\", derive({}))]",
            extras.join(", ")
        );
    }
    if !has_fds {
        outln!(
            out,
            r#"#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]"#
        );
    }
    outln!(out, "pub struct {} {{", name);
    for field in fields.iter() {
        if generator.field_is_visible(field, &deducible_fields) {
            let field_name = field.name().unwrap();
            if !skip_length_field || field_name != "length" {
                let field_type = generator.field_to_rust_type(field, switch_prefix);
                outln!(
                    out.indent(),
                    "pub {}: {},",
                    to_rust_variable_name(field_name),
                    field_type
                );
            }
        }
    }
    outln!(out, "}}");

    super::helpers::default_debug_impl(name, out);

    if generate_try_parse {
        let input_name = if !matches!(parse_size_constraint, StructSizeConstraint::None) {
            "initial_value"
        } else {
            "remaining"
        };
        if has_fds {
            assert!(external_params.is_empty());
            outln!(out, "impl TryParseFd for {} {{", name);
            outln!(
                out.indent(),
                "fn try_parse_fd<'a>({}: &'a [u8], fds: &mut Vec<RawFdContainer>) -> \
                 Result<(Self, &'a [u8]), ParseError> {{",
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
                        generator.type_to_rust_type(&ext_param.type_),
                    )
                })
                .collect::<Vec<_>>();
            outln!(
                out.indent(),
                "pub fn try_parse({}: &[u8], {}) -> Result<(Self, &[u8]), ParseError> {{",
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
                if !matches!(parse_size_constraint, StructSizeConstraint::None) {
                    outln!(out, "let remaining = initial_value;");
                }
                NamespaceGenerator::emit_let_value_for_dynamic_align(fields, out);
                for field in fields.iter() {
                    parse::emit_field_parse(
                        generator,
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
                        parse::emit_field_post_parse(field, out);
                    }
                }
                let field_names = fields
                    .iter()
                    .filter_map(|field| {
                        if generator.field_is_visible(field, &deducible_fields) {
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
                    StructSizeConstraint::LengthExpr(length_expr) => {
                        outln!(
                            out,
                            "let length = {}.try_to_usize()?;",
                            expr_to_str(
                                generator,
                                length_expr,
                                to_rust_variable_name,
                                false,
                                None,
                                true,
                            ),
                        );
                        outln!(out, "let _ = remaining;");
                        outln!(out, "let remaining = initial_value.get(length..)");
                        outln!(out.indent(), ".ok_or(ParseError::InsufficientData)?;");
                    }
                }
                outln!(out, "Ok((result, remaining))");
            })
        });

        outln!(out.indent(), "}}");
        outln!(out, "}}");
    }

    if generate_serialize {
        let size = fields
            .iter()
            .try_fold(0, |sz, field| Some(sz + field.size()?));
        if let Some(size) = size {
            emit_fixed_size_struct_serialize(
                generator,
                name,
                fields,
                external_params,
                &deducible_fields,
                skip_length_field,
                size,
                out,
            );
        } else {
            emit_variable_size_struct_serialize(
                generator,
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
            let field_type = generator.field_to_rust_type(field, switch_prefix);
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
                    outln!(out, "self.{}.len()", to_rust_variable_name(list_name));
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

fn generate_switches_for_fields(
    generator: &NamespaceGenerator<'_, '_>,
    name_prefix: &str,
    fields: &[xcbdefs::FieldDef],
    generate_try_parse: bool,
    generate_serialize: bool,
    out: &mut Output,
) {
    for field in fields.iter() {
        if let xcbdefs::FieldDef::Switch(switch_field) = field {
            switch::generate_switch(
                generator,
                name_prefix,
                switch_field,
                generate_try_parse,
                generate_serialize,
                out,
            );
        }
    }
}

fn emit_fixed_size_struct_serialize(
    generator: &NamespaceGenerator<'_, '_>,
    name: &str,
    fields: &[xcbdefs::FieldDef],
    external_params: &[xcbdefs::ExternalParam],
    deducible_fields: &HashMap<String, DeducibleField>,
    skip_length_field: bool,
    size: u32,
    out: &mut Output,
) {
    let ext_params_arg_defs = generator.ext_params_to_arg_defs(true, external_params);

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
                serialize::emit_field_serialize(
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
                serialize::emit_field_serialize_into(
                    generator,
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
    generator: &NamespaceGenerator<'_, '_>,
    name: &str,
    fields: &[xcbdefs::FieldDef],
    external_params: &[xcbdefs::ExternalParam],
    deducible_fields: &HashMap<String, DeducibleField>,
    skip_length_field: bool,
    out: &mut Output,
) {
    let ext_params_arg_defs = generator.ext_params_to_arg_defs(true, external_params);
    let ext_params_call_args =
        generator.ext_params_to_call_args(true, to_rust_variable_name, external_params);

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
                serialize::emit_field_serialize_into(
                    generator,
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
