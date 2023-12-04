use xcbgen::defs as xcbdefs;

use super::output::Output;

pub(super) fn handle_request(request_def: &xcbdefs::RequestDef, out: &mut Output) {
    let ns = request_def.namespace.upgrade().unwrap();
    if request_def.name == "GetProperty" && ns.header == "xproto" {
        let example_value = ["1, 2, 3, 4", "1, 1, 2, 2", "1, 2, 2, 1"];
        let example_expected: [&[u32]; 3] = [
            &[1, 2, 3, 4],
            &[(1 << 8) + 1, 2 * ((1 << 8) + 1)],
            &[(1 << 24) + (2 << 16) + (2 << 8) + 1],
        ];

        outln!(out, "impl GetPropertyReply {{");
        out.indented(|out| {
            for (i, &width) in [8, 16, 32].iter().enumerate() {
                outln!(
                    out,
                    r"/// Iterate over the contained value if its format is {width}.
///
/// This function checks if the `format` member of the reply
/// is {width}. If it it is not, `None` is returned. Otherwise
/// and iterator is returned that interprets the value in
/// this reply as type `u{width}`.
///
/// # Examples
///
/// Successfully iterate over the value:
/// ```
/// // First, we have to 'invent' a GetPropertyReply.
/// let reply = x11rb_protocol::protocol::xproto::GetPropertyReply {{
///     format: {width},
///     sequence: 0,
///     length: 0, // This value is incorrect
///     type_: 0, // This value is incorrect
///     bytes_after: 0,
///     value_len: 4,
///     value: vec![{example_value}],
/// }};
///
/// // This is the actual example: Iterate over the value.
/// let mut iter = reply.value{width}().unwrap();",
                    width = width,
                    example_value = example_value[i],
                );
                for expect in example_expected[i].iter() {
                    outln!(out, "/// assert_eq!(iter.next(), Some({}));", expect)
                }
                outln!(
                    out,
                    r"/// assert_eq!(iter.next(), None);
/// ```
///
/// An iterator is only returned when the `format` is correct.
/// The following example shows this.
/// ```
/// // First, we have to 'invent' a GetPropertyReply.
/// let reply = x11rb_protocol::protocol::xproto::GetPropertyReply {{
///     format: 42, // Not allowed in X11, but used for the example
///     sequence: 0,
///     length: 0, // This value is incorrect
///     type_: 0, // This value is incorrect
///     bytes_after: 0,
///     value_len: 4,
///     value: vec![1, 2, 3, 4],
/// }};
/// assert!(reply.value{width}().is_none());
/// ```
pub fn value{width}(&self) -> Option<impl Iterator<Item=u{width}> + '_> {{
    if self.format == {width} {{
        Some(crate::wrapper::PropertyIterator::new(&self.value))
    }} else {{
        None
    }}
}}",
                    width = width,
                );
            }
        });
        outln!(out, "}}");
        outln!(out, "");
    }
}

pub(super) fn handle_request_switch(
    request_def: &xcbdefs::RequestDef,
    switch_field: &xcbdefs::SwitchField,
    aux_name: &str,
    out: &mut Output,
) {
    let ns = request_def.namespace.upgrade().unwrap();
    if aux_name == "ConfigureWindowAux" && ns.header == "xproto" {
        outln!(out, "impl {} {{", aux_name);
        out.indented(|out| {
            outln!(
                out,
                r"/// Construct from a [`ConfigureRequestEvent`].
///
/// This function construct a new `ConfigureWindowAux` instance by accepting all requested
/// changes from a `ConfigureRequestEvent`. This function is useful for window managers that want
/// to handle `ConfigureRequestEvent`s.
pub fn from_configure_request(event: &ConfigureRequestEvent) -> Self {{
    let mut result = Self::new();
    let value_mask = u16::from(event.value_mask);"
            );
            out.indented(|out| {
                for case in switch_field.cases.iter() {
                    let fields = case.fields.borrow();
                    assert_eq!(1, fields.len());
                    let field = match &fields[0] {
                        xcbdefs::FieldDef::Normal(field) => field,
                        _ => unreachable!(),
                    };
                    let name = &field.name;
                    let flag = super::camel_case_to_upper_snake(name);
                    outln!(
                        out,
                        "if value_mask & u16::from(ConfigWindow::{}) != 0 {{",
                        flag,
                    );
                    if name == "stack_mode" || name == "sibling" {
                        // This already has the right type
                        outln!(out.indent(), "result = result.{}(event.{});", name, name);
                    } else {
                        let rust_type = match field.type_.type_.get_resolved() {
                            xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Int32) => "i32",
                            _ => "u32",
                        };
                        outln!(
                            out.indent(),
                            "result = result.{}({}::from(event.{}));",
                            name,
                            rust_type,
                            name,
                        );
                    }
                    outln!(out, "}}");
                }
                outln!(out, "result");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }
}

pub(super) fn handle_event(
    event_name: &str,
    event_full_def: &xcbdefs::EventFullDef,
    out: &mut Output,
) {
    let ns = event_full_def.namespace.upgrade().unwrap();
    if event_name == "ClientMessage" && ns.header == "xproto" {
        outln!(out, "impl ClientMessageEvent {{");
        out.indented(|out| {
            outln!(out, "/// Create a new `ClientMessageEvent`.");
            outln!(out, "///");
            outln!(out, "/// This function simplifies the creation of a `ClientMessageEvent` by applying");
            outln!(out, "/// some useful defaults:");
            outln!(out, "/// - `response_type = CLIENT_MESSAGE_EVENT`");
            outln!(out, "/// - `sequence = 0`");
            outln!(out, "///");
            outln!(out, "/// The other fields are set from the parameters given to this function.");
            outln!(out, "pub fn new(format: u8, window: Window, type_: impl Into<Atom>, data: impl Into<ClientMessageData>) -> Self {{");
            out.indented(|out| {
                outln!(out, "Self {{");
                out.indented(|out| {
                    outln!(out, "response_type: CLIENT_MESSAGE_EVENT,");
                    outln!(out, "format,");
                    outln!(out, "sequence: 0,");
                    outln!(out, "window,");
                    outln!(out, "type_: type_.into(),");
                    outln!(out, "data: data.into(),");
                });
                outln!(out, "}}");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }
}

pub(super) fn handle_struct(struct_def: &xcbdefs::StructDef, out: &mut Output) {
    let ns = struct_def.namespace.upgrade().unwrap();
    if struct_def.name == "FP3232" && ns.header == "xinput" {
        outln!(out, "impl Fp3232 {{");
        out.indented(|out| {
            outln!(out, "/// Convert to a floating point number.");
            outln!(out, "///");
            outln!(
                out,
                "/// A [Fp3232] contains a 32 bits integer part and another 32 bits for a"
            );
            outln!(
                out,
                "/// fractional component. This function converts this representation to a f64."
            );
            outln!(out, "pub fn as_f64(&self) -> f64 {{");
            out.indented(|out| {
                outln!(
                    out,
                    "(self.integral as f64) + (self.frac as f64) / ((1u64 << 32) as f64)"
                );
            });
            outln!(out, "}}");
            outln!(out, "/// Convert from a floating point number.");
            outln!(out, "///");
            outln!(
                out,
                "/// A [Fp3232] contains a 32 bits integer part and another 32 bits for a"
            );
            outln!(
                out,
                "/// fractional component. This function converts a f64 to this representation."
            );
            outln!(out, "///");
            outln!(
                out,
                "/// The behaviour for values greater or equal to `1^31` or less or equal to"
            );
            outln!(out, "/// `-1^31` is unspecified.");
            outln!(out, "pub fn from_f64(input: f64) -> Self {{");
            out.indented(|out| {
                outln!(out, "let scaled = input * ((1u64 << 32) as f64);");
                outln!(out, "let converted = scaled as i64;");
                outln!(out, "let integral = converted >> 32;");
                outln!(out, "let frac = converted - (integral << 32);");
                outln!(out, "let integral = if frac >= 0 {{");
                out.indented(|out| {
                    outln!(out, "integral");
                });
                outln!(out, "}} else {{");
                out.indented(|out| {
                    outln!(out, "integral - 1");
                });
                outln!(out, "}};");
                outln!(out, "let frac = converted - (integral << 32);");
                outln!(out, "let integral = i32::try_from(integral).unwrap();");
                outln!(out, "let frac = u32::try_from(frac).unwrap();");
                outln!(out, "Self {{ integral, frac }}");
            });
            outln!(out, "}}");
        });
        outln!(out, "}}");
    }
}

pub(super) fn handle_type_alias(type_alias: &xcbdefs::TypeAliasDef, out: &mut Output) {
    let ns = type_alias.namespace.upgrade().unwrap();
    if type_alias.new_name == "FP1616" && ns.header == "xinput" {
        // These are free function since Fp1616 is just a type alias for i32.

        outln!(out, "/// Convert a [Fp1616] to a floating point number.");
        outln!(out, "///");
        outln!(
            out,
            "/// A [Fp1616] is a 32 bit integer where the upper 16 bits represent an integer"
        );
        outln!(
            out,
            "/// component and the lower 16 bits are a fractional part. This function"
        );
        outln!(out, "/// converts this representation to a f32.");
        outln!(out, "pub fn fp1616_as_f32(input: Fp1616) -> f32 {{");
        out.indented(|out| {
            outln!(out, "(input as f32) / ((1u32 << 16) as f32)");
        });
        outln!(out, "}}");

        outln!(out, "/// Convert a floating point number to a [Fp1616].");
        outln!(out, "///");
        outln!(
            out,
            "/// A [Fp1616] is a 32 bit integer where the upper 16 bits represent an integer"
        );
        outln!(
            out,
            "/// component and the lower 16 bits are a fractional part. This function"
        );
        outln!(out, "/// converts this representation to a f32.");
        outln!(out, "pub fn f32_as_fp1616(input: f32) -> Fp1616 {{");
        out.indented(|out| {
            outln!(out, "(input * ((1u32 << 16) as f32)) as _");
        });
        outln!(out, "}}");
    }
}
