use std::collections::HashMap;

use xcbgen::defs as xcbdefs;

use super::{
    CaseInfo, DeducibleField, Derives, FieldContainer, NamespaceGenerator,
    Output, StructSizeConstraint, expr_to_str, gather_deducible_fields, parse,
    serialize, struct_type, to_rust_type_name, to_rust_variable_name,
};

pub(super) fn emit_switch_type(
    generator: &NamespaceGenerator<'_, '_>,
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
            if case_has_single_visible_field(generator, case, &case_deducible_fields) {
                Some(
                    case.fields
                        .borrow()
                        .iter()
                        .position(|field| generator.field_is_visible(field, &case_deducible_fields))
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
                generate_switch(
                    generator,
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

            generator.filter_derives_for_fields(&mut derives, &*case_fields, false);
            struct_type::emit_struct_type(
                generator,
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
        generator.filter_derives_for_fields(&mut derives, &*case.fields.borrow(), false);
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
                        generator.option_name,
                        generator.field_to_rust_type(single_field, name),
                    );
                }
                CaseInfo::MultiField(field_name, struct_name) => {
                    outln!(
                        out.indent(),
                        "pub {}: {}<{}>,",
                        to_rust_variable_name(field_name),
                        generator.option_name,
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
                        generator.field_to_rust_type(single_field, name),
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
        outln!(
            out.indent(),
            "/// This variant is returned when the server sends a discriminant",
        );
        outln!(
            out.indent(),
            "/// value that does not match any of the defined by the protocol.",
        );
        outln!(out.indent(), "///");
        outln!(
            out.indent(),
            "/// Usually, this should be considered a parsing error, but there",
        );
        outln!(
            out.indent(),
            "/// are some cases where the server violates the protocol.",
        );
        outln!(out.indent(), "///");
        outln!(
            out.indent(),
            "/// Trying to use `serialize` or `serialize_into` with this variant",
        );
        outln!(out.indent(), "/// will raise a panic.");
        outln!(out.indent(), "InvalidValue(u32),");
        outln!(out, "}}");
    }

    if generate_try_parse {
        emit_switch_try_parse(generator, switch, name, &case_infos, out);
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
                            generator.field_to_rust_type(single_field, name),
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
            emit_fixed_size_switch_serialize(generator, switch, name, &case_infos, size, out);
        } else {
            emit_variable_size_switch_serialize(generator, switch, name, &case_infos, out);
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
                            expr_to_str(
                                generator,
                                &case.exprs[0],
                                to_rust_variable_name,
                                true,
                                true,
                                false,
                            ),
                        );
                    }
                    outln!(
                        out.indent(),
                        "{}::InvalidValue(switch_expr) => *switch_expr,",
                        name,
                    );
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
                            expr_to_str(
                                generator,
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
    generator: &NamespaceGenerator<'_, '_>,
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
                    generator.type_to_rust_type(&ext_param.type_),
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
                expr_to_str(generator, &switch.expr, to_rust_variable_name, false, true, false),
            );
            outln!(out, "let mut outer_remaining = value;");
            if switch.kind == xcbdefs::SwitchKind::BitCase {
                let mut rust_case_names = Vec::new();
                for (case, case_info) in switch.cases.iter().zip(case_infos.iter()) {
                    let mut case_expr_str = format!(
                        "switch_expr & {} != 0",
                        expr_to_str(
                            generator,
                            &case.exprs[0],
                            to_rust_variable_name,
                            false,
                            true,
                            true,
                        ),
                    );
                    for expr in case.exprs[1..].iter() {
                        case_expr_str.push_str(" || switch_expr & ");
                        case_expr_str.push_str(&expr_to_str(
                            generator,
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
                                NamespaceGenerator::emit_let_value_for_dynamic_align(&*case_fields, out);
                                for field in case_fields.iter() {
                                    parse::emit_field_parse(
                                        generator,
                                        field,
                                        name,
                                        "remaining",
                                        FieldContainer::Other,
                                        out,
                                    );
                                }
                                for field in case_fields.iter() {
                                    parse::emit_field_post_parse(field, out);
                                }
                                outln!(out, "outer_remaining = remaining;");
                            }
                            CaseInfo::MultiField(_, struct_name) => {
                                let mut parse_params = vec![String::from("outer_remaining")];
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
                        expr_to_str(
                            generator,
                            &case.exprs[0],
                            to_rust_variable_name,
                            false,
                            true,
                            true,
                        ),
                    );
                    for expr in case.exprs[1..].iter() {
                        case_expr_str.push_str(" || switch_expr == ");
                        case_expr_str.push_str(&expr_to_str(
                            generator,
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
                                NamespaceGenerator::emit_let_value_for_dynamic_align(&*case_fields, out);
                                for field in case_fields.iter() {
                                    parse::emit_field_parse(
                                        generator,
                                        field,
                                        name,
                                        "remaining",
                                        FieldContainer::Other,
                                        out,
                                    );
                                }
                                for field in case_fields.iter() {
                                    parse::emit_field_post_parse(field, out);
                                }
                                outln!(out, "outer_remaining = remaining;");
                            }
                            CaseInfo::MultiField(_, struct_name) => {
                                let mut parse_params = vec![String::from("outer_remaining")];
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
                outln!(
                    out.indent(),
                    "None => Ok(({}::InvalidValue(switch_expr), &[])),",
                    name,
                );
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
    generator: &NamespaceGenerator<'_, '_>,
    switch: &xcbdefs::SwitchField,
    name: &str,
    case_infos: &[CaseInfo],
    size: u32,
    out: &mut Output,
) {
    assert!(switch.kind == xcbdefs::SwitchKind::Case);

    let external_params = switch.external_params.borrow();
    let ext_params_arg_defs = generator.ext_params_to_arg_defs(true, &*external_params);
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
            serialize::emit_assert_for_switch_serialize(generator, switch, out);
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
                                    serialize::emit_field_serialize(
                                        generator,
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
                                serialize::emit_field_serialize_into(
                                    generator,
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
    generator: &NamespaceGenerator<'_, '_>,
    switch: &xcbdefs::SwitchField,
    name: &str,
    case_infos: &[CaseInfo],
    out: &mut Output,
) {
    let external_params = switch.external_params.borrow();
    let ext_params_arg_defs = generator.ext_params_to_arg_defs(true, &*external_params);
    let ext_params_call_args =
        generator.ext_params_to_call_args(true, to_rust_variable_name, &*external_params);

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
            serialize::emit_assert_for_switch_serialize(generator, switch, out);
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
                                    serialize::emit_field_serialize_into(
                                        generator,
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
                            let ext_params_call_args = generator.ext_params_to_call_args(
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
                                        serialize::emit_field_serialize_into(
                                            generator,
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
                    outln!(
                        out,
                        "{}::InvalidValue(_) => panic!(\"attempted to serialize invalid \
                         switch case\"),",
                        name
                    );
                });
                outln!(out, "}}");
            }
        });
        outln!(out, "}}");
    });
    outln!(out, "}}");
}

pub(super) fn generate_switch(
    generator: &NamespaceGenerator<'_, '_>,
    name_prefix: &str,
    switch: &xcbdefs::SwitchField,
    generate_try_parse: bool,
    generate_serialize: bool,
    out: &mut Output,
) {
    let switch_name = format!("{}{}", name_prefix, to_rust_type_name(&switch.name));
    emit_switch_type(
        generator,
        switch,
        &switch_name,
        generate_try_parse,
        generate_serialize,
        None,
        out,
    );
    outln!(out, "");
}

/// Returns whether `case` has a single visible field.
fn case_has_single_visible_field(
    generator: &NamespaceGenerator<'_, '_>,
    case: &xcbdefs::SwitchCase,
    deducible_fields: &HashMap<String, DeducibleField>,
) -> bool {
    let num_visible_fields = case
        .fields
        .borrow()
        .iter()
        .filter(|case_field| generator.field_is_visible(case_field, &deducible_fields))
        .count();
    assert!(num_visible_fields > 0);
    num_visible_fields == 1
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
