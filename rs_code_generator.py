#!/usr/bin/env python

import getopt
import sys
import glob

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
    assert len(name) == 2
    return name[1]

rust_type_mapping = {
        'uint32_t': 'u32',
        'uint16_t': 'u16',
        'uint8_t':  'u8',
        'int32_t':  'i32',
        'int16_t':  'i16',
        'int8_t':   'i8',
}

def _to_rust_type(name):
    if type(name) == tuple and len(name) == 1:
        name = name[0]

    assert name in rust_type_mapping, name
    return rust_type_mapping[name]

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
    _out("struct MyTryError();");

def rs_close(self):
    _out_indent_decr()
    _out("}")

def rs_enum(self, name):
    def name_to_identifier(ename):
        if ename[0].isdigit():
            return 'M' + ename
        return ename

    rust_name = _name(name)
    _out("enum %s {", rust_name)
    _out_indent_incr()
    for (ename, value) in self.values:
        _out("%s,", name_to_identifier(ename))
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
            _out("%s::%s => %s,", rust_name, name_to_identifier(ename), value)
    for (ename, bit) in self.bits:
        _out("%s::%s => 1 << %s,", rust_name, name_to_identifier(ename), bit)
    _out_indent_decr()
    _out("}")
    _out_indent_decr()
    _out("}")
    _out_indent_decr()
    _out("}")

def rs_simple(self, name):
    _out("type %s = %s;", _name(name), _to_rust_type(self.name))

def complex_type(self, name, extra_name):
    opcode = self.opcodes[name]
    _out("const %s_%s: u8 = %s;", _name(name).upper(), extra_name.upper(), opcode)

    is_simple = all(field.type.is_simple or field.type.is_pad for field in self.fields)
    is_fixed_size = all((field.type.fixed_size() and field.type.nmemb == 1) or field.type.is_pad for field in self.fields)
    if (not is_simple) or (not is_fixed_size):
        print("skipping complicated event", self, name, opcode)
        return

    _out("struct %s%s {", _name(name), extra_name)
    _out_indent_incr()
    for field in self.fields:
        if not field.type.is_pad:
            _out("pub %s: %s,", field.field_name, _to_rust_type(field.type.name))
    _out_indent_decr()
    _out("}")

    _out("impl TryFrom<&[u8]> for %s%s {", _name(name), extra_name)
    _out_indent_incr()
    _out("type Error = MyTryError;")
    _out("fn try_from(value: &[u8]) -> Result<Self, Self::Error> {")
    _out_indent_incr()
    offset = 0
    for field in self.fields:
        rust_type = _to_rust_type(field.type.name)
        next_offset = offset + field.type.size
        if not field.type.is_pad:
            _out("let %s = %s::from_ne_bytes(value.get(%s..%s).ok_or(MyTryError())?.try_into().unwrap());",
                    field.field_name, rust_type, offset, next_offset)
        offset = next_offset

    _out("Ok(%s%s {", _name(name), extra_name)
    _out_indent_incr()
    for field in self.fields:
        if not field.type.is_pad:
            _out("%s,", field.field_name)
    _out_indent_decr()
    _out("})")
    _out_indent_decr()
    _out("}")
    _out_indent_decr()
    _out("}")

def rs_struct(self, name):
    print("struct", self, name)

def rs_union(self, name):
    print("union", self, name)

def rs_request(self, name):
    opcode = self.opcode
    _out("const %s_REQUEST: u8 = %s;", _name(name).upper(), opcode)

    is_simple = all(field.type.is_simple or field.type.is_pad for field in self.fields)
    is_fixed_size = all((field.type.fixed_size() and field.type.nmemb == 1) or field.type.is_pad for field in self.fields)
    if (not is_simple) or (not is_fixed_size):
        print("skipping complicated request", self, name, opcode)
        return
    if self.reply:
        print("skipping request with reply", self, name, opcode)
        return

    args = ["c: &Connection"]
    for field in self.fields:
        if field.visible:
            args.append("%s: %s" % (field.field_name, _to_rust_type(field.type.name)))

    _out("fn %s(%s) -> Result<SequenceNumber, Box<dyn Error>> {", _name(name), ", ".join(args))
    _out_indent_incr()

    request = []
    for field in self.fields:
        if field.field_name == "major_opcode":
            request.append("%s_REQUEST" % _name(name).upper())
        elif field.type.is_pad:
            for i in range(field.type.size):
                request.append("0")
        else:
            if field.field_name == "length":
                value = sum((field.type.size for field in self.fields))
                value = "(%susize / 4)" % value
            else:
                value = field.field_name
            _out("let %s_bytes = %s.to_ne_bytes();", field.field_name, value)
            for i in range(field.type.size):
                request.append("%s_bytes[%d]" % (field.field_name, i))

    _out("let request = [");
    _out_indent_incr()
    for byte in request:
        _out("%s,", byte)
    _out_indent_decr()
    _out("];")
    _out("let bufs = [IoSlice::new(&request)];")
    _out("c.send_request_without_reply(&bufs)")
    _out_indent_decr()
    _out("}")

def rs_eventstruct(self, name):
    print("eventstruct", self, name)

def rs_event(self, name):
    complex_type(self, name, 'Event')

def rs_error(self, name):
    complex_type(self, name, 'Error')

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
        target.write(line)
        target.write('\n')
