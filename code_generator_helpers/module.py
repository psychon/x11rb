import re

from .output import Output, Indent
import code_generator_helpers.special_cases as special_cases
from .request import generate_request_code as generate_request_code


rust_type_mapping = {
        'uint64_t': 'u64',
        'uint32_t': 'u32',
        'uint16_t': 'u16',
        'uint8_t':  'u8',
        'int64_t':  'i64',
        'int32_t':  'i32',
        'int16_t':  'i16',
        'int8_t':   'i8',
        'CARD64':   'u64',
        'CARD32':   'u32',
        'CARD16':   'u16',
        'CARD8':    'u8',
        'INT64':    'i64',
        'INT32':    'i32',
        'INT16':    'i16',
        'INT8':     'i8',
        'char':     'u8',
        'BYTE':     'u8',
        'void':     'u8',
        'float':    'f32',
        'double':   'f64',
        'BOOL':     'bool',
}


def get_references(expr):
    if expr.op is not None:
        if expr.op in ['calculate_len', 'sumof']:
            return []
        if expr.op in ['~', 'popcount']:
            return get_references(expr.rhs)
        return get_references(expr.lhs) + get_references(expr.rhs)
    elif expr.nmemb is None:
        return [expr.lenfield_name]
    return []


def mark_length_fields(self):
    lists = list(filter(lambda field: field.type.is_list, self.fields))

    # Collect a all "simple" lists (length depends on at most one length field)
    simple_lists = []

    for field in lists:
        if len(get_references(field.type.expr)) <= 1:
            simple_lists.append(field)

    # Mark length fields for simple list as not visible and map them to their list
    for field in self.fields:
        if not field.type.is_simple:
            # Skip things that cannot possibly be length fields. This is
            # necessary so that xinput's ListInputDevices "devices" field is not
            # marked as a length field and hidden.
            continue

        related_lists = list(filter(lambda list_field: field.field_name == list_field.type.expr.lenfield_name, simple_lists))
        if len(related_lists) == 1:
            related_list, = related_lists
            field.is_length_field_for = related_list
            related_list.has_length_field = field
            field.visible = False


def get_derives(obj_type):
    if hasattr(obj_type, "computed_derives"):
        possible_derives = ["Debug", "Clone", "Copy", "PartialEq", "Eq"]
        return [item for item in possible_derives if item not in obj_type.underivable]
    obj_type.computed_derives = True

    underivable = set()
    if obj_type.is_container:
        if obj_type.is_union:
            # These are represented as a type containing Vec
            underivable.add("Copy")
            # And comparing unions makes no sense (in the current implementation)
            underivable.update(["PartialEq", "Eq"])

        for field in obj_type.fields:
            get_derives(field.type)
            underivable.update(field.type.underivable)

            # Sigh, FDs are weird and need special care
            if hasattr(field, "isfd") and field.isfd:
                # RawFdContainer cannot be cloned
                underivable.update(["Copy", "Clone"])
    elif obj_type.is_list:
        get_derives(obj_type.member)
        underivable.update(obj_type.member.underivable)

        if obj_type.nmemb is None:
            # Variable length list, represented as Vec
            underivable.add("Copy")
    elif obj_type.is_simple:
        if obj_type.name == ('float',) or obj_type.name == ('double',):
            # f32/f64 do not implement Eq
            underivable.add("Eq")
    else:
        assert obj_type.is_pad, obj_type

    obj_type.underivable = underivable
    return get_derives(obj_type)


def emit_doc(out, doc):
    if doc is None:
        return
    out("/// %s.", doc.brief)
    out("///")
    if doc.description:
        out.with_prefix("/// ", doc.description)

    if doc.fields:
        out("///")
        out("/// # Fields")
        out("///")
        for (field, text) in sorted(doc.fields.items()):
            # Prevent rustdoc interpreting many leading spaces as code examples
            text = re.sub(r"\n *", "\n", text)
            out.with_prefixes("/// * `%s` - " % field, "/// ", text)

    if doc.errors:
        out("///")
        out("/// # Errors")
        out("///")
        for (error, text) in sorted(doc.errors.items()):
            out.with_prefixes("/// * `%s` - " % error, "///", text)

    if doc.see:
        out("///")
        out("/// # See")
        out("///")
        for (see, text) in sorted(doc.see.items()):
            out("/// * %s: %s", see, text)

    if doc.example:
        out("///")
        out("/// # Example")
        out("///")
        out("/// ```text")
        out.with_prefix("/// ", doc.example)
        out("/// ```")


def ename_to_rust(ename):
    if ename[0].isdigit():
        ename = 'M' + ename
    if "_" in ename and not ename.isupper():
        # xf86vidmode has a ModeFlag enum with items like
        # Positive_HSync. Turn this into PositiveHSync.
        ename = ename.replace('_', '')
    return ename[0].upper() + ename[1:]


def is_bool(type):
    return hasattr(type, 'xml_type') and type.xml_type == 'BOOL'


def find_field(fields, name):
    result, = list(filter(lambda f: f.field_name == name, fields))
    return result


class Module(object):
    def __init__(self, outer_module):
        self.out = Output()
        self.trait_out = Output()
        self.namespace = outer_module.namespace
        self.outer_module = outer_module

        self.out("#![allow(clippy::unreadable_literal)]")
        self.out("#![allow(clippy::too_many_arguments)]")
        self.out("#![allow(clippy::identity_op)]")
        self.out("#![allow(clippy::trivially_copy_pass_by_ref)]")
        self.out("#![allow(clippy::eq_op)]")
        self.out("use std::convert::TryFrom;")
        self.out("#[allow(unused_imports)]")
        self.out("use std::convert::TryInto;")
        self.out("use std::io::IoSlice;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::utils::{Buffer, RawFdContainer};")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::x11_utils::Event as _;")
        self.out("use crate::x11_utils::{TryParse, Serialize};")
        self.out("use crate::connection::RequestConnection;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::cookie::{Cookie, CookieWithFds, VoidCookie};")
        if not self.namespace.is_ext:
            self.out("use crate::cookie::ListFontsWithInfoCookie;")
        self.out("use crate::errors::{ParseError, ConnectionError};")

        self.generic_event_name = "GenericEvent"
        self.option_name = "Option"
        if self.namespace.is_ext and self.namespace.header == "present":
            self.generic_event_name = "crate::x11_utils::GenericEvent"
            self.option_name = "std::option::Option"
        else:
            self.out("#[allow(unused_imports)]")
            self.out("use crate::x11_utils::GenericEvent;")

        self.generic_error_name = "GenericError"
        if self.namespace.is_ext and self.namespace.header == "glx":
            self.generic_error_name = "crate::x11_utils::GenericError"
        else:
            self.out("#[allow(unused_imports)]")
            self.out("use crate::x11_utils::GenericError;")

        for (name, header) in outer_module.imports:
            assert name == header, (name, header)  # I don't know what is going on here...
            self.out("#[allow(unused_imports)]")
            if header == "xproto":
                self.out("use super::%s::*;", header)
            else:
                self.out("use super::%s;", header)

        if self.namespace.is_ext:
            self.out("")
            self.out("/// The X11 name of the extension for QueryExtension")
            self.out("pub const X11_EXTENSION_NAME: &str = \"%s\";", self.namespace.ext_xname)

            self.out("")
            self.out("/// The version number of this extension that this client library supports.")
            self.out("///")
            self.out("/// This constant contains the version number of this extension that is supported")
            self.out("/// by this build of x11rb. For most things, it does not make sense to use this")
            self.out("/// information. If you need to send a `QueryVersion`, it is recommended to instead")
            self.out("/// send the maximum version of the extension that you need.")
            self.out("pub const X11_XML_VERSION: (u32, u32) = (%s, %s);",
                     self.namespace.major_version, self.namespace.minor_version)
        self.out("")

    def close(self, outer_module):
        self.out("/// Extension trait defining the requests of this extension.")
        self.out("pub trait ConnectionExt: RequestConnection {")
        with Indent(self.out):
            self.out.copy_from(self.trait_out)
        self.out("}")
        self.out("impl<C: RequestConnection + ?Sized> ConnectionExt for C {}")

    def enum(self, enum, name):
        rust_name = self._name(name)
        emit_doc(self.out, enum.doc)
        self.out("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        # Is any of the variants all upper-case?
        if any(ename.isupper() and len(ename) > 1 for (ename, value) in enum.values):
            self.out("#[allow(non_camel_case_types)]")
        self.out("pub enum %s {", rust_name)
        for (ename, value) in enum.values:
            self.out.indent("%s,", ename_to_rust(ename))
        self.out("}")

        # Guess which types this enum can be represented in. We do this based on the
        # highest value that appears in any of the variants.
        highest_value = max((int(value) for (ename, value) in enum.values))
        if highest_value < 1 << 8:
            to_type = "u8"
            larger_types = ["u16", "u32"]
        elif highest_value < 1 << 16:
            to_type = "u16"
            larger_types = ["u32"]
        else:
            assert highest_value < 1 << 32
            to_type = "u32"
            larger_types = []

        self.out("impl Into<%s> for %s {", to_type, rust_name)
        with Indent(self.out):
            self.out("fn into(self) -> %s {", to_type)
            with Indent(self.out):
                self.out("match self {")
                with Indent(self.out):
                    bits = [ename for (ename, bit) in enum.bits]
                    for (ename, value) in enum.values:
                        if ename not in bits:
                            self.out("%s::%s => %s,", rust_name, ename_to_rust(ename), value)
                    for (ename, bit) in enum.bits:
                        self.out("%s::%s => 1 << %s,", rust_name, ename_to_rust(ename), bit)
                self.out("}")
            self.out("}")
        self.out("}")

        self.out("impl Into<%s<%s>> for %s {", self.option_name, to_type, rust_name)
        with Indent(self.out):
            self.out("fn into(self) -> %s<%s> {", self.option_name, to_type)
            self.out.indent("Some(self.into())")
            self.out("}")
        self.out("}")

        for larger_type in larger_types:
            self.out("impl Into<%s> for %s {", larger_type, rust_name)
            with Indent(self.out):
                self.out("fn into(self) -> %s {", larger_type)
                with Indent(self.out):
                    self.out("Into::<%s>::into(self).into()", to_type)
                self.out("}")
            self.out("}")

            self.out("impl Into<%s<%s>> for %s {", self.option_name, larger_type, rust_name)
            with Indent(self.out):
                self.out("fn into(self) -> %s<%s> {", self.option_name, larger_type)
                self.out.indent("Some(self.into())")
                self.out("}")
            self.out("}")

        # Is this enum a bitmask? It is all values are bits or the special value zero...
        def ok_for_bitmask(ename, value):
            return ename in bits or value == "0"

        looks_like_bitmask = all(ok_for_bitmask(ename, value) for (ename, value) in enum.values)
        # ...but not if all values are the special value zero
        if looks_like_bitmask and any(value != "0" for (ename, value) in enum.values):
            self.out("bitmask_binop!(%s, %s);", rust_name, to_type)

        self.out("")

    def simple(self, simple, name):
        assert not hasattr(simple, "doc")
        # FIXME: Figure out what to do with names. _to_rust_identifier() does the
        # right thing here, but then we get both 'pub type Window = u32;' and 'enum
        # Window', which the compiler does not like.
        name = self._name(name)
        self.out("pub type %s = %s;", name, self._name(simple.name))
        self.out("")

    def struct(self, struct, name):
        assert not hasattr(struct, "doc")

        # Emit the struct definition itself
        self.complex_type(struct, self._name(name), True, [])

        # And now emit some functions for the struct.
        self._generate_serialize(self._name(name), struct)

        self.out("")

    def _generate_serialize(self, type_name, complex):
        self.out("impl Serialize for %s {", type_name)
        with Indent(self.out):
            if complex.fixed_size():
                # Everything is fixed-size so we can return an array.
                length = sum((field.type.get_total_size() for field in complex.fields))
                wire_type = "[u8; %s]" % length
            else:
                # For a variable size list, we do not know beforehand the size of the
                # serialized data. Thus, return a Vec.
                length = None
                wire_type = "Vec<u8>"

            self.out("type Bytes = %s;", wire_type)
            self.out("fn serialize(&self) -> Self::Bytes {")

            with Indent(self.out):
                if complex.fixed_size():
                    def _emit():
                        assert False, "We do not have a variable size list, but we do?"

                    def _final_emit():
                        pass
                else:
                    self.out("let mut result = Vec::new();")

                    def _emit():
                        if complex.fixed_size() or not result_bytes:
                            return
                        self.out("result.extend([")
                        for result_value in result_bytes:
                            self.out.indent("%s,", result_value)
                        self.out("].iter());")
                        del result_bytes[:]
                    _final_emit = _emit

                # This gathers the bytes of the result; its content is copied to
                # result:Vec<u8> by _emit(). This happens in front of variable sized lists
                result_bytes = []

                for field in complex.fields:
                    field_name = self._to_rust_variable(field.field_name)
                    if field.type.is_pad:
                        if not complex.fixed_size() and field.type.align != 1:
                            # Align the output buffer to a multiple of field.type.align
                            assert field.type.size == 1
                            assert field.type.nmemb == 1
                            self.out("while result.len() %% %s != 0 {", field.type.align)
                            self.out.indent("result.push(0);")
                            self.out("}")
                        else:
                            assert field.type.align == 1
                            assert field.type.size == 1
                            for i in range(field.type.nmemb):
                                result_bytes.append("0")
                    elif field.type.is_list and field.type.nmemb is None:
                        # This is a variable sized list, so emit bytes to 'result' and
                        # then add this list directly to 'result'
                        _emit()
                        self.out("for obj in self.%s.iter() {", field_name)
                        self.out.indent("result.extend(obj.serialize().iter());")
                        self.out("}")
                    elif field.type.is_list and field.type.nmemb is not None and field.type.size == 1:
                        # Fixed-sized list with byte-sized members
                        for i in range(field.type.nmemb):
                            result_bytes.append("self.%s[%d]" % (field_name, i))
                    else:
                        # Fixed-sized list with "large" members. We have first serialize
                        # the members individually and then assemble that into the output.
                        field_name_bytes = self._to_rust_variable(field.field_name + "_bytes")
                        if hasattr(field, "is_length_field_for"):
                            # This field is a length field for some list. We get the value
                            # for this field as the length of the list.
                            self.out("let %s = self.%s.len() as %s;", field_name,
                                     field.is_length_field_for.field_name, self._field_type(field))
                            source = field_name
                        else:
                            # Get the value of this field from "self".
                            source = "self.%s" % field_name
                            if is_bool(field.type):
                                source = "(%s as u8)" % source
                        # First serialize the value itself...
                        self.out("let %s = %s.serialize();", field_name_bytes, source)
                        # ...then copy to the output.
                        for i in range(field.type.size):
                            result_bytes.append("%s[%d]" % (field_name_bytes, i))
                _final_emit()

                if complex.fixed_size():
                    self.out("[")
                    for result_value in result_bytes:
                        self.out.indent("%s,", result_value)
                    self.out("]")
                else:
                    self.out("result")
            self.out("}")
        self.out("}")

    def union(self, union, name):
        assert not hasattr(union, "doc")

        rust_name = self._name(name)
        self.out("#[derive(Debug, Clone)]")
        self.out("pub struct %s(Vec<u8>);", rust_name)

        self.out("impl %s {", rust_name)
        with Indent(self.out):
            for field in union.fields:
                assert not field.isfd
                result_type = self._to_complex_rust_type(field, None, '')
                self.out("pub fn as_%s(&self) -> %s {", self._lower_snake_name(('xcb', field.field_name)), result_type)
                with Indent(self.out):
                    self.out("fn do_the_parse(value: &[u8]) -> Result<%s, ParseError> {", result_type)
                    with Indent(self.out):
                        self.out("let mut remaining = value;")
                        parts = self._emit_parsing_code([field])
                        self.out("let _ = remaining;")
                        assert len(parts) == 1
                        self.out("Ok(%s)", parts[0])
                    self.out("}")
                    self.out("do_the_parse(&self.0[..]).unwrap()")
                self.out("}")
        self.out("}")

        self.out("impl Serialize for %s {", rust_name)
        with Indent(self.out):
            self.out("type Bytes = Vec<u8>;")
            self.out("fn serialize(&self) -> Self::Bytes {")
            self.out.indent("self.0.clone()")
            self.out("}")
        self.out("}")

        self.out("impl TryParse for %s {", rust_name)
        with Indent(self.out):
            self.out("fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {")
            with Indent(self.out):
                union_size = union.get_total_size()
                self.out("let inner = value.get(..%s)", union_size)
                self.out.indent(".ok_or(ParseError::ParseError)?")
                self.out.indent(".iter().copied().collect();")
                self.out("let result = %s(inner);", rust_name)
                self.out("Ok((result, &value[%s..]))", union_size)
            self.out("}")
        self.out("}")

        # Get unique field types
        seen = set()
        field_types = [field for field in union.fields
                       if not (field.type in seen or seen.add(field.type))]

        for field in field_types:
            assert not field.isfd
            rust_type = self._to_complex_rust_type(field, None, '')
            self.out("impl From<%s> for %s {", rust_type, rust_name)
            with Indent(self.out):
                self.out("fn from(value: %s) -> Self {", rust_type)
                if field.type.is_list:
                    if field.type.name != ('uint8_t',):
                        assert field.type.fixed_size()
                        with Indent(self.out):
                            for i in range(field.type.nmemb):
                                self.out("let value%d = value[%d].serialize();",
                                         i, i)
                            self.out("let value = [")
                            for i in range(field.type.nmemb):
                                for n in range(field.type.size):
                                    self.out.indent("value%d[%d],", i, n)
                            self.out("];")
                    self.out.indent("Self(value.to_vec())")
                else:
                    self.out.indent("Self(value.serialize().to_vec())")
                self.out("}")
            self.out("}")

        self.out("")

    def request(self, obj, name):
        mark_length_fields(obj)
        self.emit_opcode(name, 'REQUEST', obj.opcode)

        function_name = self._lower_snake_name(name)
        if function_name == "await":
            function_name = "await_"

        if special_cases.skip_request(self.out, self.trait_out, obj, name, function_name):
            return
        generate_request_code(self, obj, name, function_name)

    def eventstruct(self, eventstruct, name):
        assert False
        self.out("")

    def event(self, event, name):
        self.emit_opcode(name, 'Event', event.opcodes[name])
        emit_doc(self.out, event.doc)
        self.complex_type(event, self._name(name) + 'Event', False, [])

        if not event.is_ge_event:
            self._emit_from_generic(name, self.generic_event_name, 'Event')
            self._emit_serialize(event, name, 'Event')
        else:
            self._emit_tryfrom_generic(name, self.generic_event_name, 'Event')
        self.out("")

    def error(self, error, name):
        assert not hasattr(error, "doc")
        self.emit_opcode(name, 'Error', error.opcodes[name])
        self.complex_type(error, self._name(name) + 'Error', False, [])
        self._emit_from_generic(name, self.generic_error_name, 'Error')
        self._emit_serialize(error, name, 'Error')
        self.out("")

    def _emit_from_generic(self, name, from_generic_type, extra_name):
        self.out("impl From<%s> for %s%s {", from_generic_type, self._name(name), extra_name)
        with Indent(self.out):
            self.out("fn from(value: %s) -> Self {", from_generic_type)
            self.out.indent("Self::try_from(Into::<Buffer>::into(value))" +
                            ".expect(\"Buffer should be large enough so that parsing cannot fail\")")
            self.out("}")
        self.out("}")
        self.out("impl From<&%s> for %s%s {", from_generic_type, self._name(name), extra_name)
        with Indent(self.out):
            self.out("fn from(value: &%s) -> Self {", from_generic_type)
            self.out.indent("Self::try_from(value.raw_bytes())" +
                            ".expect(\"Buffer should be large enough so that parsing cannot fail\")")
            self.out("}")
        self.out("}")

    def _emit_tryfrom_generic(self, name, from_generic_type, extra_name):
        self.out("impl TryFrom<%s> for %s%s {", from_generic_type, self._name(name), extra_name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: %s) -> Result<Self, Self::Error> {", from_generic_type)
            self.out.indent("Self::try_from(Into::<Buffer>::into(value))")
            self.out("}")
        self.out("}")
        self.out("impl TryFrom<&%s> for %s%s {", from_generic_type, self._name(name), extra_name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: &%s) -> Result<Self, Self::Error> {", from_generic_type)
            self.out.indent("Self::try_from(value.raw_bytes())")
            self.out("}")
        self.out("}")

    def _emit_serialize(self, obj, name, extra_name):
        # Emit code for serialising an event or an error into an [u8; 32]
        self.out("impl Into<[u8; 32]> for &%s%s {", self._name(name), extra_name)
        with Indent(self.out):
            self.out("fn into(self) -> [u8; 32] {")
            with Indent(self.out):
                parts = []
                for field in obj.fields:
                    field_name = self._to_rust_variable(field.field_name)
                    if field.wire:
                        if field.type.is_pad:
                            assert field.type.align == 1
                            assert field.type.size == 1
                            for i in range(field.type.nmemb):
                                parts.append("0")
                        elif field.type.is_list:
                            assert field.type.nmemb is not None
                            assert field.type.size is not None
                            for i in range(field.type.nmemb):
                                if field.type.size == 1:
                                    parts.append("self.%s[%d]" % (field_name, i))
                                else:
                                    self.out("let %s_%d = self.%s[%d].serialize();",
                                             field_name, i, field_name, i)
                                    for n in range(field.type.size):
                                        parts.append("%s_%d[%d]" % (field_name, i, n))
                        elif field.type.size == 1:
                            if is_bool(field.type):
                                parts.append("(self.%s as u8)" % field_name)
                            else:
                                parts.append("self.%s" % field_name)
                        else:
                            self.out("let %s = self.%s.serialize();", field_name, field_name)
                            for i in range(field.type.size):
                                parts.append("%s[%d]" % (field_name, i))

                assert len(parts) <= 32
                if len(parts) < 32:
                    parts.append("/* trailing padding */ 0")
                    while len(parts) < 32:
                        parts.append("0")

                self.out("[")
                with Indent(self.out):
                    while len(parts) > 8:
                        self.out("%s,", ", ".join(parts[:8]))
                        parts = parts[8:]
                    self.out("%s", ", ".join(parts))
                self.out("]")
            self.out("}")
        self.out("}")

        self.out("impl Into<[u8; 32]> for %s%s {", self._name(name), extra_name)
        with Indent(self.out):
            self.out("fn into(self) -> [u8; 32] {")
            self.out.indent("(&self).into()")
            self.out("}")
        self.out("}")

    def emit_opcode(self, name, extra_name, opcode):
        if opcode == "-1" and name == ('xcb', 'Glx', 'Generic'):
            return  # The XML has a comment saying "fake number"
        self.out("/// Opcode for the %s %s", self._name(name), extra_name.lower())
        self.out("pub const %s_%s: u8 = %s;", self._upper_snake_name(name), extra_name.upper(), opcode)

    def _emit_parsing_code(self, fields):
        """Emit the code for parsing the given fields. After this code runs, the
        fields will be available as rust variables. The list of variable names
        is returned."""
        parts = []
        for field in fields:
            field_name = self._to_rust_variable(field.field_name)
            if not field.isfd:
                rust_type = self._field_type(field)
            try_parse_args = ["remaining"]
            if hasattr(field.type, "extra_try_parse_args"):
                try_parse_args += field.type.extra_try_parse_args
            if field.type.is_list and hasattr(field.type.member, "extra_try_parse_args"):
                try_parse_args += field.type.member.extra_try_parse_args
            if not field.wire:
                if not field.isfd:
                    continue
                if field.type.is_simple:
                    self.out("if fds.is_empty() { return Err(ParseError::ParseError) }")
                    self.out("let %s = fds.remove(0);", field_name)
                else:
                    assert field.type.is_list
                    self.out("let fds_len = %s;", self.expr_to_str(field.type.expr, 'usize'))
                    self.out("if fds.len() < fds_len { return Err(ParseError::ParseError) }")
                    self.out("let mut %s = fds.split_off(fds_len);", field_name)
                    self.out("std::mem::swap(fds, &mut %s);", field_name)
                parts.append(field_name)
            elif field.type.is_pad:
                if field.type.align != 1:
                    assert field.type.size * field.type.nmemb == 1
                    align = field.type.align
                    self.out("// Align offset to multiple of %s", align)
                    self.out("let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;")
                    self.out("let misalignment = (%s - (offset %% %s)) %% %s;", align, align, align)
                    length = "misalignment"
                else:
                    length = field.type.get_total_size()
                self.out("remaining = &remaining.get(%s..).ok_or(ParseError::ParseError)?;", length)
            elif field.type.is_list and field.type.nmemb is not None:
                for i in range(field.type.nmemb):
                    self.out("let (%s_%s, new_remaining) = %s::try_parse(%s)?;",
                             field_name, i, self._field_type(field),
                             ", ".join(try_parse_args))
                    self.out("remaining = new_remaining;")
                self.out("let %s = [", field_name)
                for i in range(field.type.nmemb):
                    self.out.indent("%s_%s,", field_name, i)
                self.out("];")
                parts.append(field_name)
            elif field.type.is_list:
                if field.type.expr.op != 'calculate_len' and len(try_parse_args) == 1:
                    assert try_parse_args == ["remaining"], try_parse_args
                    self.out("let (%s, new_remaining) = crate::x11_utils::parse_list(remaining, %s)?;",
                             field_name, self.expr_to_str(field.type.expr, 'usize'))
                    self.out("remaining = new_remaining;")
                else:
                    if field.type.expr.op != 'calculate_len':
                        self.out("let list_length = %s;", self.expr_to_str(field.type.expr, 'usize'))
                        self.out("let mut %s = Vec::with_capacity(list_length);", field_name)
                        self.out("for _ in 0..list_length {")
                    else:
                        self.out("// Length is 'everything left in the input'")
                        self.out("let mut %s = Vec::new();", field_name)
                        self.out("while !remaining.is_empty() {")

                    with Indent(self.out):
                        self.out("let (v, new_remaining) = %s::try_parse(%s)?;", rust_type, ", ".join(try_parse_args))
                        self.out("%s.push(v);", field_name)
                        self.out("remaining = new_remaining;")
                    self.out("}")

                parts.append(field_name)
            else:
                self.out("let (%s, new_remaining) = %s::try_parse(%s)?;",
                         field_name, rust_type, ", ".join(try_parse_args))
                self.out("remaining = new_remaining;")
                if not hasattr(field, 'is_length_field_for'):
                    parts.append(self._to_rust_variable(field.field_name))

        return parts

    def complex_type_struct(self, complex, name, parent_fields):
        """Emit a complex type as a struct. """

        mark_length_fields(complex)

        for field in complex.fields:
            if field.type.is_switch:
                self._emit_switch(field.type, self._name(field.field_type), complex.fields)

        self.out("#[derive(%s)]", ", ".join(get_derives(complex)))

        self.out("pub struct %s {", name)
        with Indent(self.out):
            for field in complex.fields:
                if field.visible or (not field.type.is_pad and not hasattr(field, "is_length_field_for")):
                    field_name = self._to_rust_variable(field.field_name)
                    self.out("pub %s: %s,", field_name, self._to_complex_owned_rust_type(field))
        self.out("}")

    def complex_type(self, complex, name, impl_try_parse, parent_fields):
        """Emit a complex type as a struct. This also adds some parsing code and
        a Serialize implementation."""

        self.complex_type_struct(complex, name, parent_fields)

        has_fds = any(field.isfd for field in complex.fields)
        assert not (impl_try_parse and has_fds)

        # Collect all the fields that appear on the wire in the parent object
        wire_fields = [field.field_name for field in complex.fields if field.wire]
        # Collect all fields that are referenced
        referenced = []
        for field in complex.fields:
            if field.type.is_list:
                referenced.extend(get_references(field.type.expr))
        # Collect all the fields that appear "out of thin air": They are
        # referenced, but not on the wire.
        unresolved = [field for field in referenced if field not in wire_fields]

        if impl_try_parse and not unresolved:
            method = "fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>"
            self.out("impl TryParse for %s {", name)
        else:
            if has_fds:
                method = "fn try_parse_fd<'a>(value: &'a [u8], fds: &mut Vec<RawFdContainer>)"
                method += " -> Result<(Self, &'a [u8]), ParseError>"
            elif impl_try_parse:
                # Turn missing values into extra arguments
                assert unresolved
                unresolved_args, extra_args = [], []
                for field in parent_fields:
                    if field.field_name in unresolved:
                        field_name = self._to_rust_variable(field.field_name)
                        field_type = self._to_complex_rust_type(field, None, '')
                        unresolved_args.append("%s: %s" % (field_name, field_type))
                        extra_args.append(field_name)
                method = "pub fn try_parse(value: &[u8], %s) -> Result<(Self, &[u8]), ParseError>"
                method = method % ", ".join(unresolved_args)

                complex.extra_try_parse_args = extra_args
            else:
                method = "pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>"
            self.out("impl %s {", name)
        with Indent(self.out):
            self.out("%s {", method)
            with Indent(self.out):
                self.out("let mut remaining = value;")
                parts = self._emit_parsing_code(complex.fields)
                self.out("let result = %s { %s };", name, ", ".join(parts))
                self.out("Ok((result, remaining))")
            self.out("}")
        self.out("}")

        if unresolved:
            # The remaining traits cannot be implemented
            self.out("// Skipping TryFrom implementations because of unresolved members")
            return

        if has_fds:
            value = "(Buffer, Vec<RawFdContainer>)"
        else:
            value = "&Buffer"
        self.out("impl TryFrom<%s> for %s {", value, name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: %s) -> Result<Self, Self::Error> {", value)
            if has_fds:
                self.out.indent("Self::try_from((&*value.0, value.1))")
            else:
                self.out.indent("Self::try_from(&**value)")
            self.out("}")
        self.out("}")

        if not has_fds:
            self.out("impl TryFrom<Buffer> for %s {", name)
            with Indent(self.out):
                self.out("type Error = ParseError;")
                self.out("fn try_from(value: Buffer) -> Result<Self, Self::Error> {")
                self.out.indent("Self::try_from(&*value)")
                self.out("}")
            self.out("}")

        if has_fds:
            value = "(&[u8], Vec<RawFdContainer>)"
        else:
            value = "&[u8]"
        self.out("impl TryFrom<%s> for %s {", value, name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: %s) -> Result<Self, Self::Error> {", value)
            if has_fds:
                self.out.indent("let (value, mut fds) = value;")
                self.out.indent("Ok(Self::try_parse_fd(value, &mut fds)?.0)")
            else:
                self.out.indent("Ok(Self::try_parse(value)?.0)")
            self.out("}")
        self.out("}")

    def _to_complex_rust_type(self, field, aux_name, modifier):
        if field.type.is_switch:
            return modifier + aux_name
        if not field.isfd:
            result = self._field_type(field)
            if field.type.is_list:
                if field.type.nmemb is None:
                    result = "%s[%s]" % (modifier, result)
                else:
                    result = "%s[%s; %s]" % (modifier, result, field.type.nmemb)
        else:
            result = "RawFdContainer"
            if field.type.is_list:
                assert field.type.nmemb is None
                result = "Vec<RawFdContainer>"
        return result

    def _to_complex_owned_rust_type(self, field):
        if field.isfd:
            result = "RawFdContainer"
        else:
            result = self._field_type(field)
        if field.type.is_list:
            if field.type.nmemb is None:
                result = "Vec<%s>" % (result,)
            else:
                result = "[%s; %s]" % (result, field.type.nmemb)
        return result

    def _emit_switch_type(self, switch_type, name, parent_fields, generate_serialize, generate_try_parse, doc=None):
        for case in switch_type.bitcases:
            # Is there more than one visible field?
            visible_fields = list(filter(lambda x: x.visible, case.type.fields))
            if len(visible_fields) > 1:
                # Yes, then we need to generate a helper struct to ensure that
                # these fields are only specified together.
                case.rust_name = name + self._to_rust_identifier(case.type.name[-1])
                if generate_try_parse:
                    # This also generates a TryParse implementation
                    self.complex_type(case.type, case.rust_name, True, parent_fields)
                else:
                    self.complex_type_struct(case.type, case.rust_name, parent_fields)

                if generate_serialize:
                    self._generate_serialize(case.rust_name, case.type)
            else:
                field, = visible_fields
                case.only_field = field

        # Either everything should be a case or a bitcase
        assert all(case.type.is_case == switch_type.bitcases[0].type.is_case for case in switch_type.bitcases)
        assert all(case.type.is_bitcase == switch_type.bitcases[0].type.is_bitcase for case in switch_type.bitcases)

        if doc:
            self.out("/// %s", doc)
        derives = get_derives(switch_type)
        if switch_type.bitcases[0].type.is_bitcase:
            derives += ["Default"]
        self.out("#[derive(%s)]", ", ".join(derives))
        if switch_type.bitcases[0].type.is_bitcase:
            self.out("pub struct %s {", name)
            for case in switch_type.bitcases:
                if hasattr(case, "rust_name"):
                    self.out.indent("%s: %s<%s>,", case.type.name[-1], self.option_name, case.rust_name)
                else:
                    field = case.only_field
                    field_type = self._to_complex_owned_rust_type(field)
                    self.out.indent("%s: %s<%s>,", self._aux_field_name(field), self.option_name, field_type)
        else:
            self.out("pub enum %s {", name)
            for case in switch_type.bitcases:
                if hasattr(case, "rust_name"):
                    self.out.indent("%s(%s),", self._to_rust_identifier(case.type.name[-1]), case.rust_name)
                else:
                    field = case.only_field
                    field_type = self._to_complex_owned_rust_type(field)
                    self.out.indent("%s(%s),", self._to_rust_identifier(field.field_name), field_type)
        self.out("}")

    def _emit_switch(self, switch_type, name, parent_fields):
        # Either everything should be a case or a bitcase
        assert all(case.type.is_case == switch_type.bitcases[0].type.is_case for case in switch_type.bitcases)
        assert all(case.type.is_bitcase == switch_type.bitcases[0].type.is_bitcase for case in switch_type.bitcases)

        self._emit_switch_type(switch_type, name, parent_fields, False, True)
        switch_field_type = find_field(switch_type.parents[-1].fields,
                                       switch_type.expr.lenfield_name)
        switch_field_type_string = self._to_complex_rust_type(switch_field_type, None, '')
        switch_field_name = self._to_rust_variable(switch_field_type.field_name)

        # Collect missing values that are needed for parsing
        unresolved_with_type, unresolved_without_type = [], []
        unresolved_without_type.append(switch_field_name)
        unresolved_with_type.append("%s: %s" % (switch_field_name,
                                                switch_field_type_string))
        for case in switch_type.bitcases:
            for field in case.type.fields:
                if hasattr(field.type, "expr") and field.type.expr.nmemb is None:
                    lenfield_name = field.type.expr.lenfield_name
                    field_name = self._to_rust_variable(lenfield_name)
                    if field_name in unresolved_without_type:
                        # We already generated this argument in a previous iteration
                        continue
                    if any(field.field_name == lenfield_name for field in case.type.fields):
                        # This references a field that is part of the same bitcase,
                        # so this does not have to become a function argument.
                        continue
                    referenced_field = find_field(switch_type.parents[-1].fields,
                                                  lenfield_name)
                    field_type = self._to_complex_rust_type(referenced_field, None, '')
                    unresolved_without_type.append(field_name)
                    unresolved_with_type.append("%s: %s" % (field_name, field_type))

        switch_type.extra_try_parse_args = unresolved_without_type
        self.out("impl %s {", name)
        with Indent(self.out):
            args = ["value: &[u8]"] + unresolved_with_type
            self.out("fn try_parse(%s) -> Result<(Self, &[u8]), ParseError> {",
                     ", ".join(args))
            with Indent(self.out):
                self.out("let mut remaining = value;")
                if switch_type.bitcases[0].type.is_bitcase:
                    all_parts = []
                    for case in switch_type.bitcases:
                        expr, = case.type.expr
                        assert expr.op == 'enumref'
                        if hasattr(case, "rust_name"):
                            field_name = case.type.name[-1]
                            field_type = case.rust_name
                        else:
                            field_name = self._to_rust_variable(case.only_field.field_name)
                            field_type = self._to_complex_owned_rust_type(case.only_field)
                        self.out("let %s = if %s & Into::<%s>::into(%s::%s) != 0 {",
                                 field_name, switch_field_name,
                                 self._name(switch_field_type.field_type),
                                 self._name(expr.lenfield_type.name),
                                 ename_to_rust(expr.lenfield_name))
                        with Indent(self.out):
                            if hasattr(case, "rust_name"):
                                args = ["remaining"]
                                if hasattr(case.type, "extra_try_parse_args"):
                                    args += case.type.extra_try_parse_args
                                self.out("let (%s, new_remaining) = %s::try_parse(%s)?;",
                                         field_name, field_type, ", ".join(args))
                                self.out("remaining = new_remaining;")
                            else:
                                self._emit_parsing_code(case.type.fields)
                            self.out("Some(%s)", field_name)
                        self.out("} else {")
                        self.out.indent("None")
                        self.out("};")
                        all_parts.append(field_name)
                    self.out("let result = %s { %s };", name, ", ".join(all_parts))
                    self.out("Ok((result, remaining))")
                else:
                    self.out("let mut parse_result = None;")
                    for case in switch_type.bitcases:
                        expr, = case.type.expr
                        assert expr.op == 'enumref'
                        if hasattr(case, "rust_name"):
                            field_name = case.type.name[-1]
                            field_type = case.rust_name
                        else:
                            field_name = self._to_rust_variable(case.only_field.field_name)
                            field_type = self._to_complex_owned_rust_type(case.only_field)
                        self.out("if %s == Into::<%s>::into(%s::%s) {", switch_field_name,
                                 self._name(switch_field_type.field_type),
                                 self._name(expr.lenfield_type.name),
                                 ename_to_rust(expr.lenfield_name))
                        with Indent(self.out):
                            if hasattr(case, "rust_name"):
                                args = ["remaining"]
                                if hasattr(case.type, "extra_try_parse_args"):
                                    args += case.type.extra_try_parse_args
                                self.out("let (%s, new_remaining) = %s::try_parse(%s)?;",
                                         field_name, field_type, ", ".join(args))
                                self.out("remaining = new_remaining;")
                                variant_name = self._to_rust_identifier(case.type.name[-1])
                            else:
                                self._emit_parsing_code(case.type.fields)
                                variant_name = self._to_rust_identifier(case.only_field.field_name)
                            msg = "The XML should prevent more than one 'if' from matching"
                            self.out("assert!(parse_result.is_none(), \"%s\");", msg)
                            self.out("parse_result = Some(%s::%s(%s));", name, variant_name, field_name)
                        self.out("}")
                    self.out("match parse_result {")
                    self.out.indent("None => Err(ParseError::ParseError),")
                    self.out.indent("Some(result) => Ok((result, remaining))")
                    self.out("}")
            self.out("}")
        self.out("}")

    def _generate_aux(self, name, request, switch, mask_field, request_function_name):
        # This used to assert that all fields have the same size, but sync's
        # CreateAlarm has both 32 bit and 64 bit numbers. Now we assert that all
        # sizes are a multiple of the smallest size.
        # Thanks to XKB having variable sized lists in SetMap, this assert is
        # restricted to fixed size fields.
        fixed_size_fields = list(filter(lambda field: field.type.fixed_size(), switch.type.fields))
        if fixed_size_fields:
            min_field_size = min(field.type.size for field in fixed_size_fields)
            assert all(field.type.size % min_field_size == 0 for field in fixed_size_fields)

        self._emit_switch_type(switch.type, name, request.fields, True, False, "Auxiliary and optional information"
                               + " for the %s function." % (request_function_name,))

        self.out("impl %s {", name)
        with Indent(self.out):

            self.out("/// Create a new instance with all fields unset / not present.")
            self.out("pub fn new() -> Self {")
            self.out.indent("Default::default()")
            self.out("}")

            self.out("fn value_mask(&self) -> %s {", self._field_type(mask_field))
            with Indent(self.out):
                self.out("let mut mask = 0;")
                for case in switch.type.bitcases:
                    expr, = case.type.expr
                    assert expr.op == "enumref"
                    enum_name = self._name(expr.lenfield_type.name)
                    if hasattr(case, "rust_name"):
                        field_name = case.type.name[-1]
                    else:
                        field = case.only_field
                        field_name = self._aux_field_name(field)
                    self.out("if self.%s.is_some() {", field_name)
                    self.out.indent("mask |= Into::<%s>::into(%s::%s);", self._field_type(mask_field),
                                    enum_name, ename_to_rust(expr.lenfield_name))
                    self.out("}")
                self.out("mask")
            self.out("}")

            for case in switch.type.bitcases:
                if hasattr(case, "rust_name"):
                    aux_name = case.type.name[-1]
                    field_name = case.type.name[-1]
                    field_type = case.rust_name
                else:
                    field = case.only_field
                    aux_name = self._aux_field_name(field)
                    field_name = field.field_name
                    field_type = self._to_complex_owned_rust_type(field)
                self.out("/// Set the %s field of this structure.", field_name)
                self.out("pub fn %s<I>(mut self, value: I) -> Self where I: Into<Option<%s>> {",
                         aux_name, field_type)
                with Indent(self.out):
                    self.out("self.%s = value.into();", aux_name)
                    self.out("self")
                self.out("}")
        self.out("}")

        self.out("impl Serialize for %s {", name)
        with Indent(self.out):
            self.out("type Bytes = Vec<u8>;")
            self.out("fn serialize(&self) -> Vec<u8> {")
            with Indent(self.out):
                self.out("let mut result = Vec::new();")
                for case in switch.type.bitcases:
                    if hasattr(case, "rust_name"):
                        self.out("if let Some(value) = &self.%s {", case.type.name[-1])
                        with Indent(self.out):
                            self.out("result.extend(value.serialize().iter());")
                        self.out("}")
                    else:
                        field = case.only_field
                        self.out("if let Some(value) = &self.%s {", self._aux_field_name(field))
                        with Indent(self.out):
                            self.out("result.extend(value.serialize().iter());")
                        self.out("}")
                self.out("result")
            self.out("}")
        self.out("}")

    def _find_type_for_name(self, name_to_find):
        for (name, type) in self.outer_module.types.values():
            if name == name_to_find:
                return type

    def _field_type(self, field):
        if is_bool(field.type):
            return 'bool'
        return self._name(field.field_type)

    def _name(self, name):
        """Get the name as a string. xcbgen represents names as tuples like
        ('xcb', 'RandR', 'MonitorInfo'). This function produces MonitorInfo."""
        orig_name = name

        if name[0] == 'xcb':
            name = name[1:]
        else:
            assert len(name) == 1, name
            name = (rust_type_mapping.get(name[0], name[0]),)
        if self.namespace.is_ext and name[0] == self.namespace.ext_name:
            name = name[1:]
        if len(name) == 2:
            # Could be a type from another module which we imported
            try:
                ext = self.outer_module.get_namespace(name[0]).header
            except KeyError:
                ext = None

            if ext:
                # Yup, it's a type from another module.
                name = (ext + "::" + name[1],)
            else:
                name = (name[0] + self._to_rust_identifier(name[1]),)
        assert len(name) == 1, orig_name
        name = name[0]

        type_for_name = self._find_type_for_name(orig_name)
        if type_for_name:
            if type_for_name.is_simple:
                if '_' in name:
                    name = self._to_rust_identifier(name)
            if type_for_name.is_container:
                name = self._to_rust_identifier(name)

        return name

    def _lower_snake_name(self, name):
        """Convert a name tuple to a lowercase snake name. MonitorInfo is turned
        into monitor_info."""
        name = self._name(name)
        name = re.sub('([a-z0-9])([A-Z])', '\\1_\\2', name)
        return name.lower()

    def _upper_snake_name(self, name):
        """Convert a name tuple to a uppercase snake name. MonitorInfo is turned
        into MONITOR_INFO."""
        return self._lower_snake_name(name).upper()

    def _aux_field_name(self, field):
        return self._lower_snake_name((field.field_name,))

    def _to_rust_identifier(self, name):
        if name in rust_type_mapping.values():
            return name

        prefix_end = name.rfind('::')
        if prefix_end != -1:
            prefix_end += 2
            prefix = name[:prefix_end]
            name = name[prefix_end:]
        else:
            prefix = ""

        # If the NAME is all uppercase, turn it into Name (the upper() is done below)
        if name.isupper():
            name = name.lower()
        # If the name contains "_", turn it into camel case
        if "_" in name:
            name = re.sub('_(.)', lambda pat: pat.group(1).upper(), name.lower())
        # The first letter should always be upper case
        return prefix + name[0].upper() + name[1:]

    def _to_rust_variable(self, name):
        if name == "type":
            name = "type_"
        if name == "match":
            name = "match_"

        # Check for camel case names and deal with them
        if any(c.isupper() for c in name):
            assert name[0].islower()
            name = self._lower_snake_name((name,))

        return name

    def expr_to_str(self, e, type=None):
        if e.op is not None:
            if e.op == 'calculate_len':
                return e.op
            if e.op == 'sumof':
                field_name = e.lenfield_name
                return "%s.iter().map(|x| Into::<%s>::into(*x)).sum()" % (self._lower_snake_name((e.lenfield_name,)), type)
            if e.op == 'popcount':
                assert e.rhs.op is None
                assert e.rhs.nmemb is None
                field_name = e.rhs.lenfield_name
                return "TryInto::<%s>::try_into(%s.count_ones()).unwrap()" % (type, self._lower_snake_name((field_name,)),)
            if e.op == '~':
                return "!(%s)" % self.expr_to_str(e.rhs, type)
            return "(%s) %s (%s)" % (self.expr_to_str(e.lhs, type), e.op, self.expr_to_str(e.rhs, type))
        elif e.nmemb is not None:
            return e.nmemb
        else:
            assert e.lenfield_name is not None
            if type is not None:
                return "%s as %s" % (self._to_rust_variable(e.lenfield_name), type)
            else:
                return self._to_rust_variable(e.lenfield_name)
