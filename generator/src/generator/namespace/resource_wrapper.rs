use std::collections::BTreeSet;
use std::fmt::Write;
use std::rc::Rc;

use xcbgen::defs as xcbdefs;

use super::super::{camel_case_to_lower_snake, ext_has_feature, CreateInfo, ResourceInfo};
use super::{
    gather_deducible_fields, to_rust_type_name, to_rust_variable_name, NamespaceGenerator, Output,
};

pub(super) fn generate(
    generator: &NamespaceGenerator<'_, '_>,
    out: &mut Output,
    info: &ResourceInfo<'_>,
) {
    let lower_name = info.resource_name.to_ascii_lowercase();
    let free_function = camel_case_to_lower_snake(info.free_request);
    let wrapper = format!("{}Wrapper", info.resource_name);
    outln!(out, "");
    outln!(
        out,
        "/// A RAII-like wrapper around a [{}].",
        info.resource_name,
    );
    outln!(out, "///");
    outln!(
        out,
        "/// Instances of this struct represent a {} that is freed in `Drop`.",
        info.resource_name,
    );
    outln!(out, "///");
    outln!(out, "/// Any errors during `Drop` are silently ignored. Most likely an error here means that your");
    outln!(
        out,
        "/// X11 connection is broken and later requests will also fail.",
    );
    outln!(out, "#[derive(Debug)]");
    outln!(
        out,
        "pub struct {wrapper}<'c, C: RequestConnection>(&'c C, {name});",
        name = info.resource_name,
        wrapper = wrapper,
    );
    outln!(out, "");
    outln!(out, "impl<'c, C: RequestConnection> {}<'c, C>", wrapper);
    outln!(out, "{{");
    out.indented(|out| {
        outln!(
            out,
            "/// Assume ownership of the given resource and destroy it in `Drop`.",
        );
        outln!(
            out,
            "pub fn for_{}(conn: &'c C, id: {}) -> Self {{",
            lower_name,
            info.resource_name,
        );
        outln!(out.indent(), "{}(conn, id)", wrapper);
        outln!(out, "}}");
        outln!(out, "");

        outln!(out, "/// Get the XID of the wrapped resource");
        outln!(
            out,
            "pub fn {}(&self) -> {} {{",
            lower_name,
            info.resource_name,
        );
        outln!(out.indent(), "self.1");
        outln!(out, "}}");
        outln!(out, "");

        outln!(
            out,
            "/// Assume ownership of the XID of the wrapped resource",
        );
        outln!(out, "///");
        outln!(
            out,
            "/// This function destroys this wrapper without freeing the underlying resource.",
        );
        outln!(
            out,
            "pub fn into_{}(self) -> {} {{",
            lower_name,
            info.resource_name,
        );
        outln!(out.indent(), "let id = self.1;");
        outln!(out.indent(), "std::mem::forget(self);");
        outln!(out.indent(), "id");
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");

    let mut uses = BTreeSet::new();

    outln!(out, "impl<'c, C: X11Connection> {}<'c, C>", wrapper);
    outln!(out, "{{");
    out.indented(|out| {
        for create_request in info.create_requests.iter() {
            generate_creator(
                generator,
                out,
                create_request,
                info.resource_name,
                &wrapper,
                &lower_name,
                &mut uses,
            );
        }
    });
    outln!(out, "}}");
    // Add "use"s for other extensions that appear in the wrapper type
    for header in uses.iter() {
        outln!(out, "#[cfg(feature = \"{}\")]", header);
        outln!(out, "#[allow(unused_imports)]");
        outln!(out, "use super::{};", header);
    }
    outln!(out, "");
    outln!(
        out,
        "impl<C: RequestConnection> From<&{wrapper}<'_, C>> for {name} {{",
        name = info.resource_name,
        wrapper = wrapper,
    );
    out.indented(|out| {
        outln!(out, "fn from(from: &{}<'_, C>) -> Self {{", wrapper);
        outln!(out.indent(), "from.1");
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(
        out,
        "impl<C: RequestConnection> Drop for {}<'_, C> {{",
        wrapper,
    );
    out.indented(|out| {
        outln!(out, "fn drop(&mut self) {{");
        outln!(out.indent(), "let _ = {}(self.0, self.1);", free_function);
        outln!(out, "}}");
    });
    outln!(out, "}}");
}

fn generate_creator(
    generator: &NamespaceGenerator<'_, '_>,
    out: &mut Output,
    request_info: &CreateInfo<'_>,
    resource_name: &str,
    wrapper_name: &str,
    lower_name: &str,
    uses: &mut BTreeSet<String>,
) {
    let (request_ext, request_name) = match request_info.request_name.split_once(':') {
        None => (None, request_info.request_name),
        Some((ext_name, request_name)) => (Some(ext_name), request_name),
    };
    let request_ns = request_ext.map(|ext_name| {
        generator.module.namespace(ext_name).unwrap_or_else(|| {
            panic!(
                "Could not find namespace {} for resolving request name {}",
                ext_name, request_info.request_name,
            );
        })
    });
    let has_feature = request_ns
        .as_ref()
        .map(|ns| ext_has_feature(&ns.header))
        .unwrap_or(false);
    let request_ns = request_ns.as_deref().unwrap_or(generator.ns);
    let request_def = Rc::clone(
        request_ns
            .request_defs
            .borrow()
            .get(request_name)
            .unwrap_or_else(|| {
                panic!(
                    "Did not find request {} in namespace {}",
                    request_name, generator.ns.header,
                );
            }),
    );
    let request_fields = request_def.fields.borrow();
    let deducible_fields = gather_deducible_fields(&request_fields);

    if request_ext.is_some() {
        uses.insert(request_ns.header.clone());
    }

    let mut letter_iter = b'A'..=b'Z';

    let function_raw_name = camel_case_to_lower_snake(&request_def.name);
    let (function_raw_name, function_name) = match request_ext {
        Some(_) => (
            format!("{}_{}", request_ns.header, function_raw_name),
            format!("super::{}::{}", request_ns.header, function_raw_name),
        ),
        None => (function_raw_name.clone(), function_raw_name),
    };
    let mut function_args = "conn: &'c C".to_string();
    let mut forward_args_with_resource = Vec::new();
    let mut forward_args_without_resource = Vec::new();
    let mut generics = Vec::new();
    let mut wheres = Vec::new();

    for field in request_fields.iter() {
        if !generator.field_is_visible(field, &deducible_fields) {
            continue;
        }
        match field.name() {
            Some("major_opcode") | Some("minor_opcode") | Some("length") => continue,
            _ => {}
        }

        let (rust_field_name, rust_field_type) = match field {
            xcbdefs::FieldDef::Pad(_) => unimplemented!(),
            xcbdefs::FieldDef::Expr(_) => unimplemented!(),
            xcbdefs::FieldDef::VirtualLen(_) => unimplemented!(),
            xcbdefs::FieldDef::Fd(fd_field) => {
                let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                let where_ = format!("{}: Into<RawFdContainer>", generic_param);
                generics.push(generic_param.clone());
                wheres.push(where_);
                (fd_field.name.clone(), generic_param)
            }
            xcbdefs::FieldDef::FdList(_) => unimplemented!(),
            xcbdefs::FieldDef::Normal(normal_field) => {
                let rust_field_name = to_rust_variable_name(&normal_field.name);
                let rust_field_type = generator.field_value_type_to_rust_type(&normal_field.type_);

                let use_into = if generator
                    .use_enum_type_in_field(&normal_field.type_)
                    .is_none()
                {
                    !matches!(normal_field.type_.value_set, xcbdefs::FieldValueSet::None)
                } else {
                    false
                };

                if use_into {
                    let generic_param = format!("{}", char::from(letter_iter.next().unwrap()));
                    let where_ = format!("{}: Into<{}>", generic_param, rust_field_type);
                    generics.push(generic_param.clone());
                    wheres.push(where_);
                    (rust_field_name, generic_param)
                } else {
                    (rust_field_name, rust_field_type)
                }
            }
            xcbdefs::FieldDef::List(list_field) => {
                let field_name = to_rust_variable_name(&list_field.name);
                let element_type =
                    generator.field_value_type_to_rust_type(&list_field.element_type);
                let field_type = if let Some(list_len) = list_field.length() {
                    format!("&[{}; {}]", element_type, list_len)
                } else {
                    format!("&[{}]", element_type)
                };
                (field_name, field_type)
            }
            xcbdefs::FieldDef::Switch(switch_field) => (
                to_rust_variable_name(&switch_field.name),
                format!("&{}Aux", to_rust_type_name(request_name)),
            ),
        };
        if !request_info
            .created_argument
            .eq_ignore_ascii_case(&rust_field_name)
        {
            write!(function_args, ", {}: {}", rust_field_name, rust_field_type).ok();

            forward_args_without_resource.push(rust_field_name.clone());
        }

        forward_args_with_resource.push(rust_field_name);
    }

    let generics_decl = if generics.is_empty() {
        "".to_string()
    } else {
        format!("<{}>", generics.join(", "))
    };

    outln!(out, "");
    outln!(
        out,
        "/// Create a new {name} and return a {name} wrapper and a cookie.",
        name = resource_name,
    );
    outln!(out, "///");
    outln!(
        out,
        "/// This is a thin wrapper around [{}] that allocates an id for the {}.",
        function_name,
        resource_name,
    );
    outln!(
        out,
        "/// This function returns the resulting `{}` that owns the created {} and frees",
        wrapper_name,
        resource_name,
    );
    outln!(
        out,
        "/// it in `Drop`. This also returns a `VoidCookie` that comes from the call to",
    );
    outln!(out, "/// [{}].", function_name);
    outln!(out, "///");
    outln!(
        out,
        "/// Errors can come from the call to [X11Connection::generate_id] or [{}].",
        function_name,
    );
    if has_feature {
        outln!(out, "#[cfg(feature = \"{}\")]", request_ns.header);
    }
    outln!(
        out,
        "pub fn {}_and_get_cookie{}({}) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError>",
        function_raw_name,
        generics_decl,
        function_args,
    );
    emit_where(out, &wheres);
    outln!(out, "{{");
    outln!(
        out.indent(),
        "let {} = conn.generate_id()?;",
        request_info.created_argument,
    );
    outln!(
        out.indent(),
        "let cookie = {}(conn, {})?;",
        function_name,
        forward_args_with_resource.join(", "),
    );
    outln!(
        out.indent(),
        "Ok((Self::for_{}(conn, {}), cookie))",
        lower_name,
        request_info.created_argument,
    );
    outln!(out, "}}");
    outln!(out, "");

    outln!(
        out,
        "/// Create a new {name} and return a {name} wrapper",
        name = resource_name,
    );
    outln!(out, "///");
    outln!(
        out,
        "/// This is a thin wrapper around [{}] that allocates an id for the {}.",
        function_name,
        resource_name,
    );
    outln!(
        out,
        "/// This function returns the resulting `{}` that owns the created {} and frees",
        wrapper_name,
        resource_name,
    );
    outln!(out, "/// it in `Drop`.");
    outln!(out, "///");
    outln!(
        out,
        "/// Errors can come from the call to [X11Connection::generate_id] or [{}].",
        function_name,
    );
    if has_feature {
        outln!(out, "#[cfg(feature = \"{}\")]", request_ns.header);
    }
    outln!(
        out,
        "pub fn {}{}({}) -> Result<Self, ReplyOrIdError>",
        function_raw_name,
        generics_decl,
        function_args,
    );
    emit_where(out, &wheres);
    outln!(out, "{{");
    outln!(
        out.indent(),
        "Ok(Self::{}_and_get_cookie(conn, {})?.0)",
        function_raw_name,
        forward_args_without_resource.join(", "),
    );
    outln!(out, "}}");
}

fn emit_where(out: &mut Output, wheres: &[String]) {
    if !wheres.is_empty() {
        outln!(out, "where");
        for where_ in wheres.iter() {
            outln!(out.indent(), "{},", where_);
        }
    }
}
