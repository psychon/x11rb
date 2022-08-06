use xcbgen::defs as xcbdefs;

use super::{ename_to_rust, to_rust_variable_name, NamespaceGenerator};

#[inline]
pub(super) fn expr_to_str(
    generator: &NamespaceGenerator<'_, '_>,
    expr: &xcbdefs::Expression,
    mut wrap_field_ref: impl FnMut(&str) -> String,
    panic_on_overflow: bool,
    cast_to_type: Option<&str>,
    needs_parens: bool,
) -> String {
    expr_to_str_impl(
        generator,
        expr,
        &mut wrap_field_ref,
        panic_on_overflow,
        cast_to_type,
        needs_parens,
    )
}

fn expr_to_str_impl(
    generator: &NamespaceGenerator<'_, '_>,
    expr: &xcbdefs::Expression,
    wrap_field_ref: &mut dyn FnMut(&str) -> String,
    panic_on_overflow: bool,
    cast_to_type: Option<&str>,
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
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.lhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    ),
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        false,
                    ),
                    err_handler,
                ),
                xcbdefs::BinaryOperator::Sub => format!(
                    "{}.checked_sub({}){}",
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.lhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    ),
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        false,
                    ),
                    err_handler,
                ),
                xcbdefs::BinaryOperator::Mul => format!(
                    "{}.checked_mul({}){}",
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.lhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    ),
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        false,
                    ),
                    err_handler,
                ),
                xcbdefs::BinaryOperator::Div => format!(
                    "{}.checked_div({}){}",
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.lhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    ),
                    expr_to_str_impl(
                        generator,
                        &bin_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        false,
                    ),
                    err_handler,
                ),
                xcbdefs::BinaryOperator::And => {
                    let lhs_str = expr_to_str_impl(
                        generator,
                        &bin_op_expr.lhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    );
                    let rhs_str = expr_to_str_impl(
                        generator,
                        &bin_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    );
                    if needs_parens {
                        format!("({} & {})", lhs_str, rhs_str)
                    } else {
                        format!("{} & {}", lhs_str, rhs_str)
                    }
                }
                xcbdefs::BinaryOperator::Or => {
                    let lhs_str = expr_to_str_impl(
                        generator,
                        &bin_op_expr.lhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
                        true,
                    );
                    let rhs_str = expr_to_str_impl(
                        generator,
                        &bin_op_expr.rhs,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some(cast_to_type.unwrap_or("u32")),
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
                let rhs_str = expr_to_str_impl(
                    generator,
                    &unary_op_expr.rhs,
                    wrap_field_ref,
                    panic_on_overflow,
                    Some(cast_to_type.unwrap_or("u32")),
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
            if let Some(t) = cast_to_type {
                format!("{}::from({})", t, value)
            } else {
                value
            }
        }
        xcbdefs::Expression::ParamRef(param_ref_expr) => {
            let rust_field_name = to_rust_variable_name(&param_ref_expr.field_name);
            if let Some(t) = cast_to_type {
                format!("{}::from({})", t, rust_field_name)
            } else {
                rust_field_name
            }
        }
        xcbdefs::Expression::EnumRef(enum_ref_expr) => {
            let rust_enum_type = generator.type_to_rust_type(enum_ref_expr.enum_.get_resolved());
            let rust_variant_name = ename_to_rust(&enum_ref_expr.variant);
            format!(
                "{}::from({}::{})",
                cast_to_type.unwrap_or("u32"),
                rust_enum_type,
                rust_variant_name,
            )
        }
        xcbdefs::Expression::PopCount(pop_count_expr) => {
            let arg = expr_to_str_impl(
                generator,
                pop_count_expr,
                wrap_field_ref,
                panic_on_overflow,
                None,
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
                xcbdefs::FieldRefKind::ExtParam => to_rust_variable_name(&sum_of_expr.field_name),
                // nested sum-of not supported
                xcbdefs::FieldRefKind::SumOfRef => unreachable!(),
            };
            if panic_on_overflow {
                format!(
                    "{}.iter().fold(0u32, |acc, x| acc.checked_add({}).unwrap())",
                    field_value,
                    expr_to_str_impl(
                        generator,
                        &sum_of_expr.operand,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some("u32"),
                        true,
                    ),
                )
            } else {
                format!(
                    "{}.iter().try_fold(0u32, |acc, x| \
                     acc.checked_add({}).ok_or(ParseError::InvalidExpression))?",
                    field_value,
                    expr_to_str_impl(
                        generator,
                        &sum_of_expr.operand,
                        wrap_field_ref,
                        panic_on_overflow,
                        Some("u32"),
                        true,
                    ),
                )
            }
        }
        xcbdefs::Expression::ListElementRef => {
            if let Some(t) = cast_to_type {
                format!("{}::from(*x)", t)
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

/// Formats an integer such as clippy does not complain.
///
/// `1234567` produces `"1_234_567"`
pub(super) fn format_literal_integer(value: u32) -> String {
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

pub(super) fn expr_type<'a>(expr: &xcbdefs::Expression, fallback: &'a str) -> &'a str {
    match expr {
        xcbdefs::Expression::FieldRef(field) => match field.resolved.get().unwrap().field_type {
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card8) => "u8",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card16) => "u16",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) => "u32",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card64) => "u64",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Int8) => "i8",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Int16) => "i16",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Int32) => "i32",
            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Int64) => "i64",
            _ => fallback,
        },
        xcbdefs::Expression::UnaryOp(op) => expr_type(&op.rhs, fallback),
        xcbdefs::Expression::BinaryOp(op) => {
            let lhs = expr_type(&op.lhs, fallback);
            let rhs = expr_type(&op.rhs, fallback);
            if lhs == rhs {
                lhs
            } else {
                fallback
            }
        }
        _ => fallback,
    }
}
