use std::rc::Rc;

use xcbgen::defs as xcbdefs;

use super::{
    gather_deducible_fields, to_rust_type_name, to_rust_variable_name, NamespaceGenerator, Output,
};
use super::super::{camel_case_to_lower_snake, CreateInfo, ResourceInfo};

pub(super) fn generate(
    generator: &NamespaceGenerator<'_, '_>,
    out: &mut Output,
    info: &ResourceInfo<'_>,
) {
    let lower_name = info.resource_name.to_ascii_lowercase();
    let free_function = camel_case_to_lower_snake(&info.free_request);
    let wrapper = format!("{}Wrapper", info.resource_name);
    outln!(out, "");
    outln!(out, "/// A RAII-like wrapper around a [{}].", info.resource_name);
    outln!(out, "///");
    outln!(out, "/// Instances of this struct represent a {} that is freed in `Drop`.", info.resource_name);
    outln!(out, "///");
    outln!(out, "/// Any errors during `Drop` are silently ignored. Most likely an error here means that your");
    outln!(out, "/// X11 connection is broken and later requests will also fail.");
    outln!(out, "#[derive(Debug)]");
    outln!(out, "pub struct {wrapper}<'c, C: RequestConnection>(&'c C, {name});", name = info.resource_name, wrapper = wrapper);
    outln!(out, "");
    outln!(out, "impl<'c, C: RequestConnection> {}<'c, C>", wrapper);
    outln!(out, "{{");
    out.indented(|out| {
        outln!(out, "/// Assume ownership of the given resource and destroy it in `Drop`.");
        outln!(out, "pub fn for_{}(conn: &'c C, id: {}) -> Self {{", lower_name, info.resource_name);
        outln!(out.indent(), "{}(conn, id)", wrapper);
        outln!(out, "}}");
        outln!(out, "");

        outln!(out, "/// Get the XID of the wrapped resource");
        outln!(out, "pub fn {}(&self) -> {} {{", lower_name, info.resource_name);
        outln!(out.indent(), "self.1");
        outln!(out, "}}");
        outln!(out, "");

        outln!(out, "/// Assume ownership of the XID of the wrapped resource");
        outln!(out, "///");
        outln!(out, "/// This function destroys this wrapper without freeing the underlying resource.");
        outln!(out, "pub fn into_{}(self) -> {} {{", lower_name, info.resource_name);
        outln!(out.indent(), "let id = self.1;");
        outln!(out.indent(), "std::mem::forget(self);");
        outln!(out.indent(), "id");
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");

    outln!(out, "impl<'c, C: Connection> {}<'c, C>", wrapper);
    outln!(out, "{{");
    out.indented(|out| {
        for create_request in info.create_requests.iter() {
            if let Some(request) = create_request {
                generate_creator(generator, out, request, info.resource_name, &wrapper, &lower_name);
            }
        }
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "impl<C: RequestConnection> From<&{wrapper}<'_, C>> for {name} {{", name = info.resource_name, wrapper = wrapper);
    out.indented(|out| {
        outln!(out, "fn from(from: &{}<'_, C>) -> Self {{", wrapper);
        outln!(out.indent(), "from.1");
        outln!(out, "}}");
    });
    outln!(out, "}}");
    outln!(out, "");
    outln!(out, "impl<C: RequestConnection> Drop for {}<'_, C> {{", wrapper);
    out.indented(|out| {
        outln!(out, "fn drop(&mut self) {{");
        outln!(out.indent(), "let _ = (self.0).{}(self.1);", free_function);
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
) {
    let request_name = request_info.request_name;
    let request_def = Rc::clone(generator
        .ns
        .request_defs
        .borrow()
        .get(request_name)
        .unwrap_or_else(|| {
            panic!("Did not find request {} in namespace {}", request_name, generator.ns.header);
        }));
    let request_fields = request_def.fields.borrow();
    let deducible_fields = gather_deducible_fields(&*request_fields);

    let function_name = camel_case_to_lower_snake(&request_def.name);
    let mut function_args = "conn: &'c C".to_string();
    let mut forward_args_with_resource = Vec::new();
    let mut forward_args_without_resource = Vec::new();

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
            xcbdefs::FieldDef::Fd(_) => unimplemented!(),
            xcbdefs::FieldDef::FdList(_) => unimplemented!(),
            xcbdefs::FieldDef::Normal(normal_field) => {
                (to_rust_variable_name(&normal_field.name), generator.field_value_type_to_rust_type(&normal_field.type_))
            }
            xcbdefs::FieldDef::List(list_field) => {
                let field_name = to_rust_variable_name(&list_field.name);
                assert!(generator.rust_value_type_is_u8(&list_field.element_type));
                let element_type = generator.field_value_type_to_rust_type(&list_field.element_type);
                let field_type = if let Some(list_len) = list_field.length() {
                    format!("&[{}; {}]", element_type, list_len)
                } else {
                    format!("&[{}]", element_type)
                };
                (field_name, field_type)
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                (to_rust_variable_name(&switch_field.name), format!("&{}Aux", to_rust_type_name(request_name)))
            }
        };
        if !request_info.created_argument.eq_ignore_ascii_case(&rust_field_name) {
            function_args.push_str(&format!(", {}: {}", rust_field_name, rust_field_type));

            forward_args_without_resource.push(rust_field_name.clone());
        }

        forward_args_with_resource.push(rust_field_name);
    }

    outln!(out, "");
    outln!(out, "/// Create a new {name} and return a {name} wrapper and a cookie.", name = resource_name);
    outln!(out, "///");
    outln!(out, "/// This is a thin wrapper around [{}] that allocates an id for the {}.", function_name, resource_name);
    outln!(out, "/// This function returns the resulting `{}` that owns the created {} and frees", wrapper_name, resource_name);
    outln!(out, "/// it in `Drop`. This also returns a `VoidCookie` that comes from the call to");
    outln!(out, "/// [{}].", function_name);
    outln!(out, "///");
    outln!(out, "/// Errors can come from the call to [Connection::generate_id] or [{}].", function_name);
    outln!(out, "pub fn {}_and_get_cookie({}) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError> {{", function_name, function_args);
    outln!(out.indent(), "let {} = conn.generate_id()?;", request_info.created_argument);
    outln!(out.indent(), "let cookie = conn.{}({})?;", function_name, forward_args_with_resource.join(", "));
    outln!(out.indent(), "Ok((Self::for_{}(conn, {}), cookie))", lower_name, request_info.created_argument);
    outln!(out, "}}");
    outln!(out, "");

    outln!(out, "/// Create a new {name} and return a {name} wrapper", name = resource_name);
    outln!(out, "///");
    outln!(out, "/// This is a thin wrapper around [{}] that allocates an id for the {}.", function_name, resource_name);
    outln!(out, "/// This function returns the resulting `{}` that owns the created {} and frees", wrapper_name, resource_name);
    outln!(out, "/// it in `Drop`.");
    outln!(out, "///");
    outln!(out, "/// Errors can come from the call to [Connection::generate_id] or [{}].", function_name);
    outln!(out, "pub fn {}({}) -> Result<Self, ReplyOrIdError> {{", function_name, function_args);
    outln!(out.indent(), "Ok(Self::{}_and_get_cookie(conn, {})?.0)", function_name, forward_args_without_resource.join(", "));
    outln!(out, "}}");
}
