use std::collections::HashMap;
use std::convert::TryFrom;

use xcbgen::defs as xcbdefs;

use super::{
    expr_to_str, parse, postfix_var_name, to_rust_variable_name, DeducibleField,
    NamespaceGenerator, Output,
};

/// Returns `Some(bytes)` if the serialized result is a single
/// array of bytes.
pub(super) fn emit_field_serialize(
    generator: &NamespaceGenerator<'_, '_>,
    field: &xcbdefs::FieldDef,
    deducible_fields: &HashMap<String, DeducibleField>,
    mut wrap_field_ref: impl FnMut(&str) -> String,
    result_bytes: &mut Vec<String>,
    out: &mut Output,
) -> Option<String> {
    // Should only be used in fixed size fields
    assert!(field.size().is_some());
    // Just in case, but for fixed size fields it should not emit anything.
    emit_assert_for_field_serialize(generator, field, deducible_fields, &mut wrap_field_ref, out);
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
                generator.emit_calc_deducible_field(
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
                    emit_value_serialize(generator, &normal_field.type_, &rust_field_name, true),
                );
            } else {
                let src_value = wrap_field_ref(&normal_field.name);
                outln!(
                    out,
                    "let {} = {};",
                    bytes_name,
                    emit_value_serialize(generator, &normal_field.type_, &src_value, false),
                );
            }
            for i in 0..field_size {
                result_bytes.push(format!("{}[{}]", bytes_name, i));
            }
            Some(bytes_name)
        }
        xcbdefs::FieldDef::List(list_field) => {
            let list_length = list_field.length().unwrap();
            if generator.rust_value_type_is_u8(&list_field.element_type) {
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
                    let bytes_name = postfix_var_name(&rust_field_name, &format!("{}_bytes", i));
                    outln!(
                        out,
                        "let {} = {};",
                        bytes_name,
                        emit_value_serialize(
                            generator,
                            &list_field.element_type,
                            &src_value,
                            false,
                        ),
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

pub(super) fn emit_field_serialize_into(
    generator: &NamespaceGenerator<'_, '_>,
    field: &xcbdefs::FieldDef,
    deducible_fields: &HashMap<String, DeducibleField>,
    mut wrap_field_ref: impl FnMut(&str) -> String,
    bytes_name: &str,
    out: &mut Output,
) {
    emit_assert_for_field_serialize(generator, field, deducible_fields, &mut wrap_field_ref, out);
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
                generator.emit_calc_deducible_field(
                    field,
                    deducible_field,
                    &mut wrap_field_ref,
                    &rust_field_name,
                    out,
                );
                emit_value_serialize_into(
                    generator,
                    &normal_field.type_,
                    &wrap_field_ref(&normal_field.name),
                    true,
                    bytes_name,
                    out,
                );
            } else {
                emit_value_serialize_into(
                    generator,
                    &normal_field.type_,
                    &wrap_field_ref(&normal_field.name),
                    false,
                    bytes_name,
                    out,
                );
            }
        }
        xcbdefs::FieldDef::List(list_field) => {
            if generator.rust_value_type_is_u8(&list_field.element_type) {
                // Fixed-sized list with `u8` members
                outln!(
                    out,
                    "{}.extend_from_slice(&{});",
                    bytes_name,
                    wrap_field_ref(&list_field.name),
                );
            } else if parse::can_use_simple_list_parsing(generator, &list_field.element_type) {
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
                    emit_value_serialize_into(
                        generator,
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
            let ext_params_args = generator.ext_params_to_call_args(
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
pub(super) fn emit_assert_for_field_serialize(
    generator: &NamespaceGenerator<'_, '_>,
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
                let length_expr_str = expr_to_str(
                    generator,
                    list_field.length_expr.as_ref().unwrap(),
                    &mut wrap_field_ref,
                    true,
                    None,
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
                        DeducibleField::LengthOf(list_name, _) => *list_name == fd_list_field.name,
                        DeducibleField::CaseSwitchExpr(_, _) => false,
                        DeducibleField::BitCaseSwitchExpr(_, _) => false,
                    })
                    && fd_list_field.length().is_none();

            if needs_assert {
                let rust_field_name = to_rust_variable_name(&fd_list_field.name);
                let length_expr_str = expr_to_str(
                    generator,
                    &fd_list_field.length_expr,
                    &mut wrap_field_ref,
                    true,
                    None,
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
pub(super) fn emit_assert_for_switch_serialize(
    generator: &NamespaceGenerator<'_, '_>,
    switch: &xcbdefs::SwitchField,
    out: &mut Output,
) {
    let rust_field_name = to_rust_variable_name(&switch.name);
    let switch_expr_str = expr_to_str(
        generator,
        &switch.expr,
        to_rust_variable_name,
        true,
        Some("u32"),
        false,
    );
    outln!(
        out,
        "assert_eq!(self.switch_expr(), {}, \"switch `{}` has an inconsistent discriminant\");",
        switch_expr_str,
        rust_field_name,
    );
}

pub(super) fn emit_value_serialize(
    generator: &NamespaceGenerator<'_, '_>,
    type_: &xcbdefs::FieldValueType,
    value: &str,
    was_deduced: bool,
) -> String {
    // Deduced fields are not converted to their enum value
    if let (false, Some(enum_def)) = (was_deduced, generator.use_enum_type_in_field(type_)) {
        let enum_info = generator.caches.borrow().enum_info(&enum_def);
        let (_, max_wire_size) = enum_info.wire_size.unwrap();
        let rust_wire_type = generator.type_to_rust_type(type_.type_.get_resolved());
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

pub(super) fn emit_value_serialize_into(
    generator: &NamespaceGenerator<'_, '_>,
    type_: &xcbdefs::FieldValueType,
    value: &str,
    was_deduced: bool,
    bytes_var: &str,
    out: &mut Output,
) {
    // Deduced fields are not converted to their enum value
    if let (false, Some(enum_def)) = (was_deduced, generator.use_enum_type_in_field(type_)) {
        let enum_info = generator.caches.borrow().enum_info(&enum_def);
        let (_, max_wire_size) = enum_info.wire_size.unwrap();
        let rust_wire_type = generator.type_to_rust_type(type_.type_.get_resolved());
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
