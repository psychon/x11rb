"""
This module handles some special cases where we emit code that is not based on
the XML, but hand-written. Thus, this handles special cases.
"""

from .output import Indent


def skip_request(out, trait_out, obj, name, function_name):
    # We use this hook to also rewrite some parameter types.
    if name == ('xcb', 'InternAtom'):
        for field in obj.fields:
            if field.field_name == 'only_if_exists':
                from xcbgen.xtypes import SimpleType
                field.type = SimpleType(('uint8_t',), 1)
                field.type.xml_type = 'BOOL'

    if name == ('xcb', 'GetProperty'):
        extra_get_property_impl(out)

    # This request has a <switch> inside of another <switch>. Supporting
    # this requires moving <switch>-handling to complex_type(), I guess.
    if name == ('xcb', 'xkb', 'GetKbdByName'):
        trait_out("fn %s(&self) { unimplemented!(\"Not yet supported by the code generator\") }", function_name)
        out("pub fn %s<Conn>(_conn: &Conn) where Conn: RequestConnection + ?Sized {", function_name)
        out("unimplemented!(\"Not yet supported by the code generator\") }")
        return True
    return False


def extra_get_property_impl(out):
    """Emit extra code for xproto's GetProperty reply."""
    # The following values are used for the doc test below. They must work
    # on every possible endian. Right now this simply means: They are
    # symmetric.
    example_value = {
            8: "1, 2, 3, 4",
            16: "1, 1, 2, 2",
            32: "1, 2, 2, 1",
    }
    example_expected = {
            8: [1, 2, 3, 4],
            16: [(1 << 8) + 1, 2 * ((1 << 8) + 1)],
            32: [(1 << 24) + (2 << 16) + (2 << 8) + 1],
    }

    out("impl GetPropertyReply {")
    with Indent(out):
        for width in [8, 16, 32]:
            out("/// Iterate over the contained value if its format is %d.", width)
            out("///")
            out("/// This function checks if the `format` member of the reply")
            out("/// is %d. If it it is not, `None` is returned.  Otherwise", width)
            out("/// and iterator is returned that interprets the value in")
            out("/// this reply as type `u%d`.", width)
            out("///")
            out("/// # Examples")
            out("///")
            out("/// Successfully iterate over the value:")
            out("/// ```")
            out("/// // First, we have to 'invent' a GetPropertyReply.")
            out("/// let reply = x11rb::generated::xproto::GetPropertyReply {")
            out("///     response_type: 1,")
            out("///     format: %d,", width)
            out("///     sequence: 0,")
            out("///     length: 0, // This value is incorrect")
            out("///     type_: 0, // This value is incorrect")
            out("///     bytes_after: 0,")
            out("///     value_len: 4,")
            out("///     value: vec![%s],", example_value[width])
            out("/// };")
            out("///")
            out("/// // This is the actual example: Iterate over the value.")
            out("/// let mut iter = reply.value%d().unwrap();", width)
            for expect in example_expected[width]:
                out("/// assert_eq!(iter.next(), Some(%d));", expect)
            out("/// assert_eq!(iter.next(), None);")
            out("/// ```")
            out("///")
            out("/// An iterator is only returned when the `format` is correct.")
            out("/// The following example shows this.")
            out("/// ```")
            out("/// // First, we have to 'invent' a GetPropertyReply.")
            out("/// let reply = x11rb::generated::xproto::GetPropertyReply {")
            out("///     response_type: 1,")
            out("///     format: 42, // Not allowed in X11, but used for the example")
            out("///     sequence: 0,")
            out("///     length: 0, // This value is incorrect")
            out("///     type_: 0, // This value is incorrect")
            out("///     bytes_after: 0,")
            out("///     value_len: 4,")
            out("///     value: vec![1, 2, 3, 4],")
            out("/// };")
            out("/// assert!(reply.value%d().is_none());", width)
            out("/// ```")
            out("#[allow(single_use_lifetimes)] // Work around a rustc bug")
            out("pub fn value%d<'a>(&'a self) -> Option<impl Iterator<Item=u%d> + 'a> {", width, width)
            with Indent(out):
                out("if self.format == %d {", width)
                out.indent("Some(crate::wrapper::PropertyIterator::new(&self.value))")
                out("} else {")
                out.indent("None")
                out("}")
            out("}")
    out("}")
