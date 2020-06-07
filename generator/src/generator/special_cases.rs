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
/// let reply = x11rb::protocol::xproto::GetPropertyReply {{
///     format: {},
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
/// let reply = x11rb::protocol::xproto::GetPropertyReply {{
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
pub fn value{width}<'a>(&'a self) -> Option<impl Iterator<Item=u{width}> + 'a> {{
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
