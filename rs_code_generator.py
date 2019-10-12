#!/usr/bin/env python

import getopt
import sys
import glob
import re
import string
import os

try:
    opts, args = getopt.getopt(sys.argv[1:], "p:o:i:")
except getopt.GetoptError as err:
    print(err)
    print('Usage: %s [-p path] -i input -o output main_module' % (sys.argv[0]))
    sys.exit(1)

for (opt, arg) in opts:
    if opt == '-p':
        sys.path.insert(1, arg)
    if opt == '-i':
        input_dir = arg
    if opt == '-o':
        output_dir = arg

if not args:
    print("Missing name for main module")
    sys.exit()
main_module = args.pop()
if args:
    print('No further arguments expected')
    sys.exit(1)

exts = []

_lines = []
_indent_level = 0
def _out(fmt, *args):
    indent = "".join(["    "] * _indent_level)
    _lines.append(indent + (fmt % args))

def _out_indent(fmt, *args):
    indent = "".join(["    "] * (_indent_level + 1))
    _lines.append(indent + (fmt % args))

def _out_indent_incr():
    global _indent_level
    _indent_level += 1

def _out_indent_decr():
    global _indent_level
    _indent_level -= 1

class Indent(object):
    """A context manager that increases indentation level in the output."""

    def __enter__(self):
        _out_indent_incr()
        return self

    def __exit__(self, type, value, traceback):
        _out_indent_decr()

def _name(name):
    orig_name = name
    if name[0] == 'xcb':
        name = name[1:]
    if current_namespace.is_ext and name[0] == current_namespace.ext_name:
        name = name[1:]
    assert len(name) == 1, orig_name
    return name[0]

def _lower_snake_name(name):
    name = _name(name)
    name = re.sub('([a-z0-9])([A-Z])', '\\1_\\2', name)
    return name.lower()

def _upper_snake_name(name):
    return _lower_snake_name(name).upper()

rust_type_mapping = {
        'uint64_t': 'u64',
        'uint32_t': 'u32',
        'uint16_t': 'u16',
        'uint8_t':  'u8',
        'int64_t':  'i64',
        'int32_t':  'i32',
        'int16_t':  'i16',
        'int8_t':   'i8',
        'char':     'u8',
        'float':    'f32',
        'double':   'f64',
}

def _to_rust_type(name):
    orig_name = name
    if type(name) == tuple:
        if name[0] == 'xcb':
            name = name[1:]
        if current_namespace.is_ext and name[0] == current_namespace.ext_name:
            name = name[1:]
        assert len(name) == 1, orig_name
        name = name[0]

    if name in rust_type_mapping:
        return rust_type_mapping[name]
    elif name.isupper():
        return _to_rust_identifier(name)
    else:
        return name

def _to_rust_identifier(name):
    # If the NAME is all uppercase, turn it into Name (the upper() is done below)
    if name.isupper():
        name = name.lower()
    # If the name contains "_", turn it into camel case
    if "_" in name:
        name = re.sub('_(.)', lambda pat: pat.group(1).upper(), name.lower())
    # The first letter should always be upper case
    return name[0].upper() + name[1:]

def _to_rust_variable(name):
    if name == "type":
        name = "type_"

    # Check for camel case names and deal with them
    if any(c.isupper() for c in name):
        assert name[0].islower()
        name = _lower_snake_name((name,))

    return name

# Now the real fun begins

def _write_output(filename):
    output_file = os.path.join(output_dir, filename)
    with open(output_file, 'w') as target:
        for line in _lines:
            target.write(line.rstrip())
            target.write('\n')
    del _lines[:]

def rs_open(self):
    global current_namespace

    assert not _lines

    current_namespace = self.namespace

    _out("use std::convert::TryFrom;")
    _out("#[allow(unused_imports)]")
    _out("use std::convert::TryInto;")
    _out("use std::io::IoSlice;")
    _out("use crate::utils::Buffer;")
    _out("#[allow(unused_imports)]")
    _out("use crate::x11_utils::{GenericEvent, GenericError as X11GenericError};")
    _out("use crate::x11_utils::TryParse;")
    _out("#[allow(unused_imports)]")
    _out("use crate::connection::SequenceNumber;")
    _out("use crate::connection::{Cookie, Connection};")
    if not current_namespace.is_ext:
        _out("use crate::connection::ListFontsWithInfoCookie;")
    _out("use crate::errors::{ParseError, ConnectionError};")

    for (name, header) in self.imports:
        assert name == header, (name, header) # I don't know what is going on here...
        _out("#[allow(unused_imports)]")
        _out("use super::%s::*;", header)

    _out("")

def rs_close(self):
    global exts

    _write_output("%s.rs" % self.namespace.header)
    exts.append(self.namespace.header)

def rs_enum(self, name):
    has_all_upper = any(ename.isupper() and len(ename) > 1 for (ename, value) in self.values)

    def ename_to_rust(ename):
        if ename[0].isdigit():
            ename = 'M' + ename
        return ename[0].upper() + ename[1:]

    rust_name = _name(name)
    _out("#[derive(Debug, Clone, Copy)]")
    if has_all_upper:
        _out("#[allow(non_camel_case_types)]")
    _out("pub enum %s {", rust_name)
    for (ename, value) in self.values:
        _out_indent("%s,", ename_to_rust(ename))
    _out("}")

    highest_value = max((int(value) for (ename, value) in self.values))
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

    _out("impl Into<%s> for %s {", to_type, rust_name)
    with Indent():
        _out("fn into(self) -> %s {", to_type)
        with Indent():
            _out("match self {")
            with Indent():
                bits = [ename for (ename, bit) in self.bits]
                for (ename, value) in self.values:
                    if ename not in bits:
                        _out("%s::%s => %s,", rust_name, ename_to_rust(ename), value)
                for (ename, bit) in self.bits:
                    _out("%s::%s => 1 << %s,", rust_name, ename_to_rust(ename), bit)
            _out("}")
        _out("}")
    _out("}")

    _out("impl Into<Option<%s>> for %s {", to_type, rust_name)
    with Indent():
        _out("fn into(self) -> Option<%s> {", to_type)
        _out_indent("Some(self.into())")
        _out("}")
    _out("}")

    for larger_type in larger_types:
        _out("impl Into<%s> for %s {", larger_type, rust_name)
        with Indent():
            _out("fn into(self) -> %s {", larger_type)
            with Indent():
                _out("Into::<%s>::into(self) as _", to_type)
            _out("}")
        _out("}")

        _out("impl Into<Option<%s>> for %s {", larger_type, rust_name)
        with Indent():
            _out("fn into(self) -> Option<%s> {", larger_type)
            _out_indent("Some(self.into())")
            _out("}")
        _out("}")

    def ok_for_bitmask(ename, value):
        return ename in bits or value == "0"

    looks_like_bitmask = all(ok_for_bitmask(ename, value) for (ename, value) in self.values)
    if looks_like_bitmask and len(self.values) > 1:
        _out("bitmask_binop!(%s, %s);", rust_name, to_type)

    _out("")

def rs_simple(self, name):
    # FIXME: Figure out what to do with names. _to_rust_identifier() does the
    # right thing here, but then we get both 'pub type Window = u32;' and 'enum
    # Window', which the compiler does not like.
    name = _name(name)
    if '_' in name:
        name = _to_rust_identifier(name)
    _out("pub type %s = %s;", name, _to_rust_type(self.name))
    _out("")

def emit_opcode(name, extra_name, opcode):
    if opcode == "-1" and name == ('xcb', 'Glx', 'Generic'):
        return # The XML has a comment saying "fake number"
    _out("pub const %s_%s: u8 = %s;", _upper_snake_name(name), extra_name.upper(), opcode)

def mark_length_fields(self):
    # Find length fields
    length_fields = {}
    for field in self.fields:
        if field.type.is_list:
            assert field.type.expr.lenfield_name not in length_fields, \
                (self.name, field.field_name, length_fields[field.type.expr.lenfield_name].field_name)
            length_fields[field.type.expr.lenfield_name] = field

    # Mark length fields as not visible and map them to their list
    for field in self.fields:
        if field.field_name in length_fields:
            field.is_length_field_for = length_fields[field.field_name]
            length_fields[field.field_name].has_length_field = field
            field.visible = False

def _emit_parsing_code(fields):
    parts = []
    for field in fields:
        assert field.wire  # I *guess* that non-wire fields just have to be skipped
        if field.visible or hasattr(field, 'is_length_field_for'):
            rust_type = _to_rust_type(field.type.name)
            if field.type.is_list and field.type.nmemb is not None:
                for i in range(field.type.nmemb):
                    _out("let (%s_%s, new_remaining) = %s::try_parse(remaining)?;", field.field_name, i, _to_rust_type(field.type.name));
                    _out("remaining = new_remaining;")
                _out("let %s = [", field.field_name)
                for i in range(field.type.nmemb):
                    _out_indent("%s_%s,", field.field_name, i)
                _out("];")
                parts.append(field.field_name)
            elif field.type.is_list:
                _out("let mut %s = Vec::with_capacity(%s.try_into()?);",
                        field.field_name, _to_rust_variable(field.has_length_field.field_name))
                _out("for _ in 0..%s {", _to_rust_variable(field.has_length_field.field_name))
                with Indent():
                    _out("let (v, new_remaining) = %s::try_parse(remaining)?;", rust_type)
                    _out("%s.push(v);", field.field_name)
                    _out("remaining = new_remaining;")
                _out("}")
                parts.append(field.field_name)
            else:
                _out("let (%s, new_remaining) = %s::try_parse(remaining)?;", _to_rust_variable(field.field_name), rust_type)
                _out("remaining = new_remaining;")
                if field.visible:
                    parts.append(_to_rust_variable(field.field_name))
        else:
            if field.type.is_pad and field.type.align != 1:
                assert field.type.size * field.type.nmemb == 1
                align = field.type.align
                _out("// Align offset to multiple of %s", align)
                _out("let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;")
                _out("let misalignment = (%s - (offset %% %s)) %% %s;", align, align, align)
                length = "misalignment"
            else:
                length = field.type.size * field.type.nmemb
            _out("remaining = &remaining.get(%s..).ok_or(ParseError::ParseError)?;", length)

    return parts

def complex_type(self, name, from_generic_type, extra_name, name_transform=lambda x: x):
    mark_length_fields(self)

    if all(field.type.fixed_size and (not field.type.is_list or field.type.nmemb is not None) and not field.type.is_union for field in self.fields):
        _out("#[derive(Debug, Clone, Copy)]")
    else:
        _out("#[derive(Debug, Clone)]")
    _out("pub struct %s%s {", name_transform(_name(name)), extra_name)
    with Indent():
        for field in self.fields:
            if field.visible:
                field_name = _to_rust_variable(field.field_name)
                if field.type.is_list:
                    if field.type.nmemb is None:
                        _out("pub %s: Vec<%s>,", field_name, _to_rust_type(field.type.name))
                    else:
                        _out("pub %s: [%s; %s],", field_name, _to_rust_type(field.type.name), field.type.nmemb)
                else:
                    _out("pub %s: %s,", field_name, _to_rust_type(field.type.name))
    _out("}")

    _out("impl TryParse for %s%s {", name_transform(_name(name)), extra_name)
    with Indent():
        _out("fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {")
        with Indent():
            _out("let mut remaining = value;")
            parts = _emit_parsing_code(self.fields)
            _out("let result = %s%s { %s };", name_transform(_name(name)), extra_name, ", ".join(parts))
            _out("Ok((result, remaining))")
        _out("}")
    _out("}")

    _out("impl TryFrom<Buffer> for %s%s {", name_transform(_name(name)), extra_name)
    with Indent():
        _out("type Error = ParseError;")
        _out("fn try_from(value: Buffer) -> Result<Self, Self::Error> {")
        _out_indent("Self::try_from(&*value)")
        _out("}")
    _out("}")

    if from_generic_type:
        _out("impl TryFrom<%s> for %s%s {", from_generic_type, name_transform(_name(name)), extra_name)
        with Indent():
            _out("type Error = ParseError;")
            _out("fn try_from(value: %s) -> Result<Self, Self::Error> {", from_generic_type)
            _out_indent("Self::try_from(Into::<Buffer>::into(value))")
            _out("}")
        _out("}")

    _out("impl TryFrom<&[u8]> for %s%s {", name_transform(_name(name)), extra_name)
    with Indent():
        _out("type Error = ParseError;")
        _out("fn try_from(value: &[u8]) -> Result<Self, Self::Error> {")
        _out_indent("Ok(Self::try_parse(value)?.0)")
        _out("}")
    _out("}")

def rs_struct(self, name):
    has_variable_size_list = any(field.type.is_list and field.type.nmemb is None for field in self.fields)
    complex_type(self, name, False, '', lambda name: _to_rust_identifier(name))

    if has_variable_size_list:
        length = None
        wire_type = "Vec<u8>"
    else:
        length = sum((field.type.size * field.type.nmemb for field in self.fields))
        wire_type = "[u8; %s]" % length

    _out("impl %s {", _to_rust_identifier(_name(name)))
    with Indent():
        _out("pub fn to_ne_bytes(&self) -> %s {", wire_type)

        with Indent():
            if has_variable_size_list:
                _out("let mut result = Vec::new();")

            def _emit():
                if not has_variable_size_list or not result_bytes:
                    return
                _out("result.extend([")
                for result_value in result_bytes:
                    _out_indent("%s,", result_value)
                _out("].iter());")
                del result_bytes[:]

            result_bytes = []
            for field in self.fields:
                if field.type.is_pad:
                    if has_variable_size_list and field.type.align != 1:
                        assert field.type.size == 1
                        assert field.type.nmemb == 1
                        _out("while result.len() %% %s != 0 {", field.type.align)
                        _out_indent("result.push(0);")
                        _out("}")
                    else:
                        assert field.type.align == 1
                        assert field.type.size == 1
                        for i in range(field.type.nmemb):
                            result_bytes.append("0")
                elif field.type.is_list and field.type.nmemb is None:
                    _emit()
                    _out("for obj in self.%s.iter() {", field.field_name)
                    _out_indent("result.extend(obj.to_ne_bytes().iter());")
                    _out("}")
                else:
                    if hasattr(field, "is_length_field_for"):
                        _out("let %s = self.%s.len() as %s;", _to_rust_variable(field.field_name), field.is_length_field_for.field_name, _to_rust_type(field.type.name))
                        source = _to_rust_variable(field.field_name)
                    else:
                        source = "self.%s" % _to_rust_variable(field.field_name)
                    _out("let %s = %s.to_ne_bytes();", _to_rust_variable(field.field_name + "_bytes"), source)
                    for i in range(field.type.size):
                        result_bytes.append("%s[%d]" % (_to_rust_variable(field.field_name + "_bytes"), i))
            _emit()

            if has_variable_size_list:
                _out("result")
            else:
                _out("[")
                for result_value in result_bytes:
                    _out_indent("%s,", result_value)
                _out("]")
        _out("}")
    _out("}")

    _out("")

def _to_complex_rust_type(field_type, aux_name, modifier):
    if field_type.is_switch:
        return modifier + aux_name
    result = _to_rust_type(field_type.name)
    if field_type.is_list:
        if field_type.nmemb is None:
            result = "%s[%s]" % (modifier, result)
        else:
            result = "%s[%s; %s]" % (modifier, result, field_type.nmemb)
    return result

def rs_union(self, name):
    rust_name = _name(name)
    _out("#[derive(Debug, Clone)]")
    _out("pub struct %s(Vec<u8>);", rust_name)

    _out("impl %s {", rust_name)
    with Indent():
        for field in self.fields:
            result_type = _to_complex_rust_type(field.type, None, '')
            _out("pub fn as_%s(&self) -> %s {", _lower_snake_name(('xcb', field.field_name)), result_type)
            with Indent():
                _out("fn do_the_parse(value: &[u8]) -> Result<%s, ParseError> {", result_type)
                with Indent():
                    _out("let mut remaining = value;")
                    parts = _emit_parsing_code([field])
                    _out("let _ = remaining;")
                    assert len(parts) == 1
                    _out("Ok(%s)", parts[0])
                _out("}")
                _out("do_the_parse(&self.0[..]).unwrap()")
            _out("}")
    _out("}")

    fixed_length = max((field.type.size * field.type.nmemb for field in self.fields))

    _out("impl TryParse for %s {", rust_name)
    with Indent():
        _out("fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {")
        with Indent():
            _out("let inner = value[..%s].iter().copied().collect();", fixed_length)
            _out("let result = %s(inner);", rust_name)
            _out("Ok((result, &value[%s..]))", fixed_length)
        _out("}")
    _out("}")

    _out("")

def _generate_aux(name, request, switch, mask_field):
    field_size = switch.type.fields[0].type.size
    assert all(field.type.size == field_size for field in switch.type.fields)
    mask_field.individual_field_size = field_size

    _out("#[derive(Debug, Clone, Copy, Default)]")
    _out("pub struct %s {", name)
    for field in switch.type.fields:
        _out_indent("pub %s: Option<%s>,", field.field_name, _to_rust_type(field.type.name))
    _out("}")

    _out("impl %s {", name)
    with Indent():

        _out("pub fn new() -> Self {")
        _out_indent("Default::default()")
        _out("}")

        _out("pub fn to_ne_bytes(&self) -> Vec<u8> {")
        with Indent():
            _out("let mut result = Vec::new();")
            for field in switch.type.fields:
                _out("if let Some(value) = self.%s {", field.field_name)
                _out_indent("result.extend(value.to_ne_bytes().iter());")
                _out("}")
            _out("result")
        _out("}")

        _out("pub fn value_mask(&self) -> %s {", _to_rust_type(mask_field.type.name))
        with Indent():
            _out("let mut mask = 0;")
            for field in switch.type.fields:
                expr, = field.parent.expr
                assert expr.op == "enumref"
                enum_name = _name(expr.lenfield_type.name)
                _out("if self.%s.is_some() {", field.field_name)
                _out_indent("mask |= Into::<%s>::into(%s::%s);", _to_rust_type(mask_field.type.name), enum_name, expr.lenfield_name)
                _out("}")
            _out("mask")
        _out("}")

        for field in switch.type.fields:
            _out("pub fn %s<I>(mut self, value: I) -> Self where I: Into<Option<%s>> {", field.field_name, _to_rust_type(field.type.name))
            with Indent():
                _out("self.%s = value.into();", field.field_name)
                _out("self")
            _out("}")

    _out("}")

def rs_request(self, name):
    emit_opcode(name, 'REQUEST', self.opcode)

    has_fd = any(field.isfd for field in self.fields)
    if has_fd:
        _out("pub fn %s() {", _lower_snake_name(name))
        _out_indent("unimplemented!(\"FD passing is not yet implemented\");")
        _out("}")
        return

    is_list_fonts_with_info = name == ('xcb', 'ListFontsWithInfo')

    switches = list(filter(lambda field: field.type.is_switch, self.fields))
    assert len(switches) <= 1
    if switches:
        aux_name = "%sAux" % _name(name)
        switch = switches[0]

        # Find the mask field for the switch
        lenfield_name = switch.type.expr.lenfield_name
        mask_field = list(filter(lambda field: field.field_name == lenfield_name, self.fields))
        assert len(mask_field) == 1
        mask_field = mask_field[0]

        # Hide it from the API and "connect" it to the switch
        mask_field.visible = False
        mask_field.lenfield_for_switch = switch

        _generate_aux(aux_name, self, switch, mask_field)
    else:
        aux_name = None

    mark_length_fields(self)

    letters = iter(string.ascii_uppercase)
    connection_type = next(letters)

    need_lifetime = any(field.visible and field.type.is_list for field in self.fields)
    need_lifetime = need_lifetime and self.reply
    if need_lifetime:
        generics = ["'c"]
        args = ["c: &'c %s" % connection_type]
    else:
        generics = []
        args = ["c: &%s" % connection_type]
    generics.append("%s: Connection" % connection_type)
    where = []

    for field in self.fields:
        if field.visible:
            rust_type = _to_complex_rust_type(field.type, aux_name, '&')
            if field.enum is not None and not field.type.is_list:
                letter = next(letters)
                generics.append(letter)
                where.append("%s: Into<%s>" % (letter, rust_type))
                rust_type = letter
            args.append("%s: %s" % (_to_rust_variable(field.field_name), rust_type))

    if is_list_fonts_with_info:
        assert need_lifetime
        result_type = "ListFontsWithInfoCookie<'c, %s>" % connection_type
    elif self.reply:
        if need_lifetime:
            result_type = "Cookie<'c, %s, %sReply>" % (connection_type, _name(name))
        else:
            result_type = "Cookie<%s, %sReply>" % (connection_type, _name(name))
    else:
        result_type = "SequenceNumber"

    if generics:
        lifetime = "<%s>" % ", ".join(generics)
    else:
        lifetime = ""

    _out("pub fn %s%s(%s) -> Result<%s, ConnectionError>", _lower_snake_name(name), lifetime, ", ".join(args), result_type)
    if where:
        _out("where %s", ", ".join(where))
    _out("{")
    with Indent():

        if current_namespace.is_ext:
            _out('let extension_information = c.extension_information("%s")' % current_namespace.ext_xname)
            _out_indent(".ok_or(ConnectionError::UnsupportedExtension)?;")

        requests = []
        request = []

        def _emit_request():
            if not request:
                return

            _out("let request%d = [", len(requests));
            requests.append("&request%d" % len(requests))
            for byte in request:
                _out_indent("%s,", byte)
            _out("];")
            del request[:]

        def _emit_byte_conversion(field_name):
            if field.type.size is not None:
                _out("let mut %s_bytes = Vec::with_capacity(%s * %s.len());", field.field_name, field.type.size, field.field_name);
            else:
                _out("let mut %s_bytes = Vec::new();", field.field_name);
            _out("for value in %s {", field_name);
            _out_indent("%s_bytes.extend(value.to_ne_bytes().iter());", field_name)
            _out("}")

        fixed_request_length = sum((field.type.size * field.type.nmemb for field in self.fields if field.type.nmemb is not None and field.wire))
        request_length = [ str(fixed_request_length) ]
        for field in self.fields:
            if field.type.nmemb is None:
                size = field.type.size
                if size is None:
                    _emit_byte_conversion(field.field_name)
                    request_length.append("%s_bytes.len()" % field.field_name)
                else:
                    request_length.append("%s * %s.len()" % (size, _to_rust_variable(field.field_name)))
            if hasattr(field, 'lenfield_for_switch'):
                _out("let %s = %s.value_mask();", field.field_name, field.lenfield_for_switch.field_name)
                request_length.append("(%s * %s.count_ones()) as usize" % (field.individual_field_size, field.field_name))
        request_length = " + ".join(request_length)

        _out("let length: usize = (%s + 3) / 4;", request_length)
        for field in self.fields:
            if field.field_name == "major_opcode" or field.field_name == "minor_opcode":
                if current_namespace.is_ext and field.field_name == "major_opcode":
                    request.append('extension_information.major_opcode')
                else:
                    request.append("%s_REQUEST" % _upper_snake_name(name))
            elif field.type.is_expr:
                def expr_to_str(e):
                    if e.op is not None:
                        return "%s %s %s" % (expr_to_str(e.lhs), e.op, expr_to_str(e.rhs))
                    elif e.nmemb is not None:
                        return e.nmemb
                    else:
                        assert e.lenfield_name is not None
                        other_field = [field for field in self.fields if e.lenfield_name == field.field_name]
                        assert len(other_field) == 1
                        other_field = other_field[0]
                        _out("let %s: %s = %s.len().try_into()?;", other_field.field_name, _to_rust_type(other_field.type.name), other_field.is_length_field_for.field_name)
                        return e.lenfield_name

                _out("let %s: %s = (%s).try_into().unwrap();", field.field_name, _to_rust_type(field.type.name), expr_to_str(field.type.expr))
                _out("let %s_bytes = %s.to_ne_bytes();", field.field_name, _to_rust_variable(field.field_name))
                for i in range(field.type.size):
                    request.append("%s_bytes[%d]" % (field.field_name, i))
            elif field.type.is_pad:
                assert field.type.size == 1
                for i in range(field.type.nmemb):
                    request.append("0")
            elif field.type.is_list:
                if field.type.size == 1:
                    _emit_request()
                    requests.append(_to_rust_variable(field.field_name))
                    if field == self.fields[-1] and not field.type.fixed_size():
                        _out("let %s_bytes = %s;", _to_rust_variable(field.field_name), _to_rust_variable(field.field_name))
                else:
                    if field.type.size is not None:
                        _emit_byte_conversion(field.field_name)

                    _emit_request()
                    requests.append("&%s_bytes" % field.field_name)
            elif field.wire:
                if hasattr(field, "is_length_field_for"):
                    _out("let %s: %s = %s.len().try_into()?;", _to_rust_variable(field.field_name), _to_rust_type(field.type.name), _to_rust_variable(field.is_length_field_for.field_name))
                if field.enum is not None:
                    _out("let %s = %s.into();", field.field_name, field.field_name);
                if field.type.name == ('float',):
                    # FIXME: Switch to a trait that we can implement on f32
                    _out("let %s = %s.to_bits().to_ne_bytes();", _to_rust_variable(field.field_name + "_bytes"), _to_rust_variable(field.field_name))
                else:
                    if field.field_name == "length":
                        source = "TryInto::<%s>::try_into(length)?" % _to_rust_type(field.type.name)
                    else:
                        source = _to_rust_variable(field.field_name)
                    _out("let %s = %s.to_ne_bytes();", _to_rust_variable(field.field_name + "_bytes"), source)
                if field.type.is_switch:
                    _emit_request()
                    requests.append("&%s_bytes" % field.field_name)
                else:
                    for i in range(field.type.size):
                        request.append("%s[%d]" % (_to_rust_variable(field.field_name + "_bytes"), i))

        _emit_request()

        last_field = self.fields[-1]
        if last_field.type.is_list and not last_field.type.fixed_size():
            _out("let padding = &[0; 3][..(4 - (%s_bytes.len() %% 4)) %% 4];", last_field.field_name)
            requests.append("&padding")

        total_length = " + ".join(["(*%s).len()" % r for r in requests])
        _out("assert_eq!(%s, (%s + 3) / 4 * 4);", total_length, request_length);

        slices = ", ".join(["IoSlice::new(%s)" % r for r in requests])

        if is_list_fonts_with_info:
            assert self.reply
            _out("Ok(ListFontsWithInfoCookie::new(c.send_request_with_reply(&[%s])))", slices)
        elif self.reply:
            _out("Ok(c.send_request_with_reply(&[%s]))", slices)
        else:
            _out("Ok(c.send_request_without_reply(&[%s]))", slices)
    _out("}")

    if self.reply:
        has_fd = any(field.isfd for field in self.reply.fields)
        if has_fd:
            _out("#[derive(Debug, Clone, Copy)]")
            _out("pub struct %sReply {}", _name(name))
            _out("impl TryFrom<Buffer> for %sReply {", _name(name))
            with Indent():
                _out("type Error = ParseError;")
                _out("fn try_from(_value: Buffer) -> Result<Self, Self::Error> {")
                _out_indent("unimplemented!(\"Replies with FDs are not yet supported\");")
                _out("}")
            _out("}")
        else:
            complex_type(self.reply, name, False, 'Reply')

    _out("")

def rs_eventstruct(self, name):
    print("eventstruct", self, name)
    _out("")
    assert False

def rs_event(self, name):
    emit_opcode(name, 'Event', self.opcodes[name])
    complex_type(self, name, 'GenericEvent', 'Event')
    _out("")

def rs_error(self, name):
    emit_opcode(name, 'Error', self.opcodes[name])
    complex_type(self, name, 'X11GenericError', 'Error')
    _out("")

# We must create an "output" dictionary before any xcbgen imports
output = {'open'       : rs_open,
          'close'      : rs_close,
          'simple'     : rs_simple,
          'enum'       : rs_enum,
          'struct'     : rs_struct,
          'union'      : rs_union,
          'request'    : rs_request,
          'eventstruct': rs_eventstruct,
          'event'      : rs_event,
          'error'      : rs_error,
          }

from xcbgen.state import Module
import xcbgen.xtypes as xtypes

names = glob.glob(input_dir + "/*.xml")
unsupported = [
        "render.xml", # New assert uncovered problems
        "damage.xml", "xfixes.xml", "composite.xml", # depend on render.xml
        "dri2.xml",      # Causes an error in python around has_length_field: There is a weird alignment construction that causes two fields to reference the same length field (in 'Connect' request, second use is <op>)
        "present.xml",   # depends on sync.xml
        "randr.xml",     # Causes an error in python around has_length_field; also lots of <op>s that reference a length field
        "sync.xml",      # <switch> with different sized fields: CreateAlarm has both CARD32 and INT64
        "xf86vidmode.xml", # Causes an error in python around has_length_field
        "xinput.xml",    # Problem in _to_rust_name()
        "xkb.xml",       # Causes an error in python around has_length_field
        "xv.xml",        # Causes an error in python around has_length_field
        "xvmc.xml",      # Problem in _to_rust_type()
        ]
names = [name for name in names if os.path.basename(name) not in unsupported]

for name in names:
    module = Module(name, None)
    module.register()
    module.resolve()
    try:
        module.generate()
    except:
        sys.stderr.write('Error occurred while generating: %s\n' % module.namespace.header)
        raise

output_file = os.path.join(output_dir, "%s.rs" % main_module)
with open(output_file, 'w') as target:
    for ext in exts:
        target.write("pub mod ")
        target.write(ext)
        target.write(";")
        target.write("\n")
