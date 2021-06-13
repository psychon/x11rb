use std::convert::TryFrom;

use xcbgen::defs as xcbdefs;

use super::{
    expr_to_str, to_rust_type_name, to_rust_variable_name, FieldContainer, NamespaceGenerator,
    Output,
};

pub(super) fn emit_field_parse(
    generator: &NamespaceGenerator<'_, '_>,
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
                emit_value_parse(generator, &normal_field.type_, from),
            );
        }
        xcbdefs::FieldDef::List(list_field) => {
            let rust_field_name = to_rust_variable_name(&list_field.name);

            if generator.rust_value_type_is_u8(&list_field.element_type) {
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
                         {}.try_to_usize()?)?;",
                        rust_field_name,
                        from,
                        expr_to_str(
                            generator,
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
            } else if can_use_simple_list_parsing(generator, &list_field.element_type)
                && list_field.length_expr.is_some()
                && list_field.length().is_none()
            {
                let rust_element_type =
                    generator.type_to_rust_type(list_field.element_type.type_.get_resolved());
                outln!(
                    out,
                    "let ({}, remaining) = crate::x11_utils::parse_list::<{}>(remaining, \
                     {}.try_to_usize()?)?;",
                    rust_field_name,
                    rust_element_type,
                    expr_to_str(
                        generator,
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
                        emit_value_parse(generator, &list_field.element_type, from),
                    );
                    emit_value_post_parse(&list_field.element_type, &tmp_name, out);
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
                        "let list_length = {}.try_to_usize()?;",
                        expr_to_str(
                            generator,
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
                        emit_value_parse(generator, &list_field.element_type, from),
                    );
                    emit_value_post_parse(&list_field.element_type, "v", out);
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
            let mut parse_params = vec![String::from("remaining")];
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
                "let fds_len = {}.try_to_usize()?;",
                expr_to_str(
                    generator,
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
                        emit_value_parse(generator, &expr_ref.type_, from),
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

pub(super) fn emit_field_post_parse(field: &xcbdefs::FieldDef, out: &mut Output) {
    match field {
        xcbdefs::FieldDef::Normal(normal_field) => {
            let rust_field_name = to_rust_variable_name(&normal_field.name);
            emit_value_post_parse(&normal_field.type_, &rust_field_name, out);
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

fn emit_value_parse(
    generator: &NamespaceGenerator<'_, '_>,
    type_: &xcbdefs::FieldValueType,
    from: &str,
) -> String {
    let type_type = type_.type_.get_resolved();
    let rust_type = generator.type_to_rust_type(type_type);
    let params = generator.get_type_parse_params(type_type, from);
    format!("{}::try_parse({})?", rust_type, params.join(", "))
}

fn emit_value_post_parse(type_: &xcbdefs::FieldValueType, var_name: &str, out: &mut Output) {
    if let xcbdefs::FieldValueSet::Enum(_) = type_.value_set {
        // Handle turning things into enum instances.
        outln!(out, "let {var} = {var}.into();", var = var_name);
    }
}

fn needs_post_parse(type_: &xcbdefs::FieldValueType) -> bool {
    if let xcbdefs::FieldValueSet::Enum(_) = type_.value_set {
        true
    } else {
        false
    }
}

pub(super) fn can_use_simple_list_parsing(
    generator: &NamespaceGenerator<'_, '_>,
    type_: &xcbdefs::FieldValueType,
) -> bool {
    generator
        .get_type_parse_params(&type_.type_.get_resolved(), "")
        .len()
        == 1
        && !needs_post_parse(type_)
}
