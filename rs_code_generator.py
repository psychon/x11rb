#!/usr/bin/env python

import getopt
import sys
import glob
import re

try:
    opts, args = getopt.getopt(sys.argv[1:], "p:o:i:")
except getopt.GetoptError as err:
    print(err)
    print('Usage: %s [-p path] -i input -o output' % (sys.argv[0]))
    sys.exit(1)

for (opt, arg) in opts:
    if opt == '-p':
        sys.path.insert(1, arg)
    if opt == '-i':
        input_dir = arg
    if opt == '-o':
        output_file = arg
if args:
    print('No further arguments expected')
    sys.exit(1)

_lines = []
_indent_level = 0
def _out(fmt, *args):
    indent = "".join(["    "] * _indent_level)
    _lines.append(indent + (fmt % args))

def _out_indent_incr():
    global _indent_level
    _indent_level += 1

def _out_indent_decr():
    global _indent_level
    _indent_level -= 1

def _name(name):
    assert len(name) == 2, name
    return name[1]

def _lower_snake_name(name):
    name = _name(name)
    name = re.sub('([a-z0-9])([A-Z])', '\\1_\\2', name)
    return name.lower()

def _upper_snake_name(name):
    return _lower_snake_name(name).upper()

rust_type_mapping = {
        'uint32_t': 'u32',
        'uint16_t': 'u16',
        'uint8_t':  'u8',
        'int32_t':  'i32',
        'int16_t':  'i16',
        'int8_t':   'i8',
        'char':     'u8',
}

def _to_rust_type(name):
    if type(name) == tuple and len(name) == 1:
        name = name[0]

    assert name in rust_type_mapping, name
    return rust_type_mapping[name]

def _to_rust_identifier(name):
    if name[0].isdigit():
        name = 'M' + name
    name = re.sub('_(.)', lambda pat: pat.group(1).upper(), name.lower())
    return name[0].upper() + name[1:]

def _to_rust_variable(name):
    if name == "type":
        name = "type_"
    return name

# Now the real fun begins

def rs_open(self):
    _out("pub mod %s {", self.namespace.header)
    _out_indent_incr()
    _out("use std::convert::TryFrom;")
    _out("use std::convert::TryInto;")
    _out("use std::io::IoSlice;")
    _out("use std::error::Error;")
    _out("use crate::xcb_ffi::Connection;")
    _out("use crate::xcb_ffi::SequenceNumber;")
    _out("use crate::xcb_ffi::Cookie;")
    _out("pub struct MyTryError();");
    _out("")

def rs_close(self):
    _out_indent_decr()
    _out("}")

def rs_enum(self, name):
    def ename_to_rust(ename):
        # if all upercase or number -> to_rust_identifier, else keep as is
        if ename.isupper() or ename.isdigit():
            return _to_rust_identifier(ename)
        else:
            return ename[0].upper() + ename[1:]

    rust_name = _name(name)
    _out("pub enum %s {", rust_name)
    _out_indent_incr()
    for (ename, value) in self.values:
        _out("%s,", ename_to_rust(ename))
    _out_indent_decr()
    _out("}")

    highest_value = max((int(value) for (ename, value) in self.values))
    if highest_value < 1 << 8:
        to_type = "u8"
    elif highest_value < 1 << 16:
        to_type = "u16"
    else:
        assert highest_value < 1 << 32
        to_type = "u32"
    _out("impl Into<%s> for %s {", to_type, rust_name)
    _out_indent_incr()
    _out("fn into(self) -> %s {", to_type)
    _out_indent_incr()
    _out("match self {")
    _out_indent_incr()
    bits = [ename for (ename, bit) in self.bits]
    for (ename, value) in self.values:
        if ename not in bits:
            _out("%s::%s => %s,", rust_name, ename_to_rust(ename), value)
    for (ename, bit) in self.bits:
        _out("%s::%s => 1 << %s,", rust_name, ename_to_rust(ename), bit)
    _out_indent_decr()
    _out("}")
    _out_indent_decr()
    _out("}")
    _out_indent_decr()
    _out("}")
    _out("")

def rs_simple(self, name):
    _out("pub type %s = %s;", _name(name), _to_rust_type(self.name))
    _out("")

def emit_opcode(name, extra_name, opcode):
    _out("pub const %s_%s: u8 = %s;", _upper_snake_name(name), extra_name.upper(), opcode)

def complex_type(self, name, extra_name, name_transform=lambda x: x):
    is_simple = all(field.type.is_simple or field.type.is_pad for field in self.fields)
    is_fixed_size = all((field.type.fixed_size() and field.type.nmemb == 1) or field.type.is_pad for field in self.fields)
    if (not is_simple) or (not is_fixed_size):
        print("skipping complicated complex type", extra_name, self, name)
        return

    _out("pub struct %s%s {", name_transform(_name(name)), extra_name)
    _out_indent_incr()
    for field in self.fields:
        if field.visible:
            _out("pub %s: %s,", field.field_name, _to_rust_type(field.type.name))
    _out_indent_decr()
    _out("}")

    _out("impl TryFrom<&[u8]> for %s%s {", name_transform(_name(name)), extra_name)
    _out_indent_incr()
    _out("type Error = MyTryError;")
    _out("fn try_from(value: &[u8]) -> Result<Self, Self::Error> {")
    _out_indent_incr()
    offset = 0
    for field in self.fields:
        rust_type = _to_rust_type(field.type.name)
        assert field.wire  # I *guess* that non-wire fields just have to be skipped
        next_offset = offset + field.type.size
        if field.visible:
            _out("let %s = %s::from_ne_bytes(value.get(%s..%s).ok_or(MyTryError())?.try_into().unwrap());",
                    field.field_name, rust_type, offset, next_offset)
        offset = next_offset

    _out("Ok(%s%s {", name_transform(_name(name)), extra_name)
    _out_indent_incr()
    for field in self.fields:
        if field.visible:
            _out("%s,", field.field_name)
    _out_indent_decr()
    _out("})")
    _out_indent_decr()
    _out("}")
    _out_indent_decr()
    _out("}")

def rs_struct(self, name):
    complex_type(self, name, '', lambda name: _to_rust_identifier(name))
    _out("")

def rs_union(self, name):
    rust_name = _name(name)
    _out("pub enum %s {", rust_name)
    _out_indent_incr()
    for field in self.fields:
        _out("%s([%s; %s]),", _to_rust_identifier(field.field_name), _to_rust_type(field.type.member.name), field.type.expr.nmemb)
    _out_indent_decr()
    _out("}")
    _out("")

def rs_request(self, name):
    emit_opcode(name, 'REQUEST', self.opcode)

    is_simple = all(field.type.is_simple or field.type.is_pad for field in self.fields)
    is_fixed_size = all((field.type.fixed_size() and field.type.nmemb == 1) or field.type.is_pad for field in self.fields)
    if (not is_simple) or (not is_fixed_size):
        print("skipping complicated request", self, name)
        return

    def _to_complex_rust_type(field_type):
        result = _to_rust_type(field_type.name)
        if field_type.is_list:
            if field_type.nmemb is None:
                result = "&[%s]" % result
            else:
                result = "&[%s; %s]" % (result, field_type.nmemb)
        return result

    lifetime = False
    args = ["c: &Connection"]
    for field in self.fields:
        if field.visible:
            if field.type.is_list:
                lifetime = True
            args.append("%s: %s" % (_to_rust_variable(field.field_name), _to_complex_rust_type(field.type)))

    if self.reply:
        if lifetime:
            lifetime = "<'c>"
            result_type = "Cookie<'c, %sReply>" % _name(name)
            args[0] = "c: &'c Connection"
        else:
            lifetime = ""
            result_type = "Cookie<%sReply>" % _name(name)
    else:
        result_type = "SequenceNumber"
        lifetime = ""

    _out("pub fn %s%s(%s) -> Result<%s, Box<dyn Error>> {", _lower_snake_name(name), lifetime, ", ".join(args), result_type)
    _out_indent_incr()

    requests = []
    request = []

    def _emit_request():
        if not request:
            return

        _out("let request%d = [", len(requests));
        requests.append("&request%d" % len(requests))
        _out_indent_incr()
        for byte in request:
            _out("%s,", byte)
        _out_indent_decr()
        _out("];")
        del request[:]

    fixed_request_length = sum((field.type.size * field.type.nmemb for field in self.fields if field.type.nmemb is not None))
    request_length = [ str(fixed_request_length) ]
    for field in self.fields:
        if field.type.nmemb is None:
            request_length.append("%s * %s.len()" % (field.type.size, field.field_name))
    request_length = " + ".join(request_length)

    _out("let length: usize = (%s) / 4;", request_length)
    for field in self.fields:
        if field.field_name == "major_opcode":
            request.append("%s_REQUEST" % _upper_snake_name(name))
        elif field.type.is_pad:
            for i in range(field.type.size):
                request.append("0")
        else:
            if field.type.is_list:
                if field.type.size == 1:
                    _emit_request()
                    requests.append(field.field_name)
                else:
                    _out("let mut %s_bytes = Vec::with_capacity(%s * %s.len());", field.field_name, field.type.size, field.field_name);
                    _out("for value in %s {", field.field_name);
                    _out_indent_incr()
                    _out("%s_bytes.extend(value.to_ne_bytes().iter());", field.field_name)
                    _out_indent_decr()
                    _out("}")

                    _emit_request()
                    requests.append("&%s_bytes" % field.field_name)
            else:
                _out("let %s_bytes = %s.to_ne_bytes();", field.field_name, _to_rust_variable(field.field_name))
                for i in range(field.type.size):
                    request.append("%s_bytes[%d]" % (field.field_name, i))

    _emit_request()

    total_length = " + ".join(["(*%s).len()" % r for r in requests])
    _out("assert_eq!(%s, %s);", total_length, request_length);

    slices = ", ".join(["IoSlice::new(%s)" % r for r in requests])

    if self.reply:
        _out("c.send_request_with_reply(&[%s])", slices)
    else:
        _out("c.send_request_without_reply(&[%s])", slices)
    _out_indent_decr()
    _out("}")

    if self.reply:
        complex_type(self.reply, name, 'Reply')
        # Work-around for some types not being supported currently and thus not
        # getting emitted
        is_simple = all(field.type.is_simple or field.type.is_pad for field in self.reply.fields)
        is_fixed_size = all((field.type.fixed_size() and field.type.nmemb == 1) or field.type.is_pad for field in self.reply.fields)
        if not (is_simple and is_fixed_size):
            _out("pub struct %s%s {}", _name(name), 'Reply')
    _out("")

def rs_eventstruct(self, name):
    print("eventstruct", self, name)
    _out("")

def rs_event(self, name):
    if self.is_ge_event:
        print("skipping GE event", self, name)
        return
    emit_opcode(name, 'Event', self.opcodes[name])
    complex_type(self, name, 'Event')
    _out("")

def rs_error(self, name):
    emit_opcode(name, 'Event', self.opcodes[name])
    complex_type(self, name, 'Error')
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
names = [input_dir + "/xproto.xml"]
for name in names:
    module = Module(name, None)
    module.register()
    module.resolve()
    module.generate()

with open(output_file, 'w') as target:
    for line in _lines:
        target.write(line.rstrip())
        target.write('\n')
