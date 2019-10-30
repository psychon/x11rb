import re
import string

from .output import Output, Indent


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


def get_references(expr):
    if expr.op is not None:
        if expr.op == 'calculate_len':
            return []
        if expr.op == '~':
            return get_references(expr.rhs)
        return get_references(expr.lhs) + get_references(expr.rhs)
    elif expr.nmemb is None:
        return [expr.lenfield_name]
    return []


def mark_length_fields(self):
    lists = list(filter(lambda field: field.type.is_list, self.fields))

    # Partition the lists into "simple" (length depends on at most one length
    # field) and "complicated" (things like x * y / 8)
    simple_lists, complicated_lists = [], []

    for field in lists:
        if len(get_references(field.type.expr)) <= 1:
            simple_lists.append(field)
        else:
            complicated_lists.append(field)

    # Mark length fields for simple list as not visible and map them to their list
    for field in self.fields:
        related_lists = list(filter(lambda list_field: field.field_name == list_field.type.expr.lenfield_name, simple_lists))
        if len(related_lists) > 1:
            # Well, okay, multiple lists share the same length field. These are
            # complicated lists.
            complicated_lists.extend(related_lists)
        elif related_lists:
            related_list, = related_lists
            field.is_length_field_for = related_list
            related_list.has_length_field = field
            field.visible = False

    # Map length fields for complicated lists similarly, but keep them visible
    for list_field in complicated_lists:
        length_fields = list(filter(lambda field: field.field_name == list_field.type.expr.lenfield_name, self.fields))
        if length_fields:
            list_field.has_length_fields = length_fields


class Module(object):
    def __init__(self, outer_module):
        self.out = Output()
        self.namespace = outer_module.namespace

        self.out("use std::convert::TryFrom;")
        self.out("#[allow(unused_imports)]")
        self.out("use std::convert::TryInto;")
        self.out("use std::io::IoSlice;")
        self.out("use crate::utils::Buffer;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::x11_utils::{GenericEvent, GenericError as X11GenericError};")
        self.out("use crate::x11_utils::TryParse;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::connection::SequenceNumber;")
        self.out("use crate::connection::{Cookie, Connection as X11Connection};")
        if not self.namespace.is_ext:
            self.out("use crate::connection::ListFontsWithInfoCookie;")
        self.out("use crate::errors::{ParseError, ConnectionError};")

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
            self.out("pub const X11_EXTENSION_NAME: &'static str = \"%s\";", self.namespace.ext_xname)
        self.out("")

    def close(self, outer_module):
        pass

    def enum(self, enum, name):
        has_all_upper = any(ename.isupper() and len(ename) > 1 for (ename, value) in enum.values)

        def ename_to_rust(ename):
            if ename[0].isdigit():
                ename = 'M' + ename
            if "_" in ename and not ename.isupper():
                # xf86vidmode has a ModeFlag enum with items like
                # Positive_HSync. Turn this into PositiveHSync.
                ename = ename.replace('_', '')
            return ename[0].upper() + ename[1:]

        rust_name = self._name(name)
        self.out("#[derive(Debug, Clone, Copy)]")
        if has_all_upper:
            self.out("#[allow(non_camel_case_types)]")
        self.out("pub enum %s {", rust_name)
        for (ename, value) in enum.values:
            self.out.indent("%s,", ename_to_rust(ename))
        self.out("}")

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

        self.out("impl Into<Option<%s>> for %s {", to_type, rust_name)
        with Indent(self.out):
            self.out("fn into(self) -> Option<%s> {", to_type)
            self.out.indent("Some(self.into())")
            self.out("}")
        self.out("}")

        for larger_type in larger_types:
            self.out("impl Into<%s> for %s {", larger_type, rust_name)
            with Indent(self.out):
                self.out("fn into(self) -> %s {", larger_type)
                with Indent(self.out):
                    self.out("Into::<%s>::into(self) as _", to_type)
                self.out("}")
            self.out("}")

            self.out("impl Into<Option<%s>> for %s {", larger_type, rust_name)
            with Indent(self.out):
                self.out("fn into(self) -> Option<%s> {", larger_type)
                self.out.indent("Some(self.into())")
                self.out("}")
            self.out("}")

        def ok_for_bitmask(ename, value):
            return ename in bits or value == "0"

        looks_like_bitmask = all(ok_for_bitmask(ename, value) for (ename, value) in enum.values)
        if looks_like_bitmask and len(enum.values) > 1:
            self.out("bitmask_binop!(%s, %s);", rust_name, to_type)

        self.out("")

    def simple(self, simple, name):
        # FIXME: Figure out what to do with names. _to_rust_identifier() does the
        # right thing here, but then we get both 'pub type Window = u32;' and 'enum
        # Window', which the compiler does not like.
        name = self._name(name)
        if '_' in name:
            name = self._to_rust_identifier(name)
        self.out("pub type %s = %s;", name, self._to_rust_type(simple.name))
        self.out("")

    def struct(self, struct, name):
        has_variable_size_list = any(field.type.is_list and field.type.nmemb is None for field in struct.fields)
        self.complex_type(struct, name, False, '', lambda name: self._to_rust_identifier(name))

        if has_variable_size_list:
            length = None
            wire_type = "Vec<u8>"
        else:
            length = sum((field.type.size * field.type.nmemb for field in struct.fields))
            wire_type = "[u8; %s]" % length

        self.out("impl %s {", self._to_rust_identifier(self._name(name)))
        with Indent(self.out):
            self.out("pub fn to_ne_bytes(&self) -> %s {", wire_type)

            with Indent(self.out):
                if has_variable_size_list:
                    self.out("let mut result = Vec::new();")

                def _emit():
                    if not has_variable_size_list or not result_bytes:
                        return
                    self.out("result.extend([")
                    for result_value in result_bytes:
                        self.out.indent("%s,", result_value)
                    self.out("].iter());")
                    del result_bytes[:]

                result_bytes = []
                for field in struct.fields:
                    if field.type.is_pad:
                        if has_variable_size_list and field.type.align != 1:
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
                        _emit()
                        self.out("for obj in self.%s.iter() {", field.field_name)
                        self.out.indent("result.extend(obj.to_ne_bytes().iter());")
                        self.out("}")
                    elif field.type.is_list and field.type.nmemb is not None and field.type.size == 1:
                        for i in range(field.type.nmemb):
                            result_bytes.append("self.%s[%d]" % (self._to_rust_variable(field.field_name), i))
                    else:
                        if hasattr(field, "is_length_field_for"):
                            self.out("let %s = self.%s.len() as %s;", self._to_rust_variable(field.field_name), field.is_length_field_for.field_name, self._to_rust_type(field.type.name))
                            source = self._to_rust_variable(field.field_name)
                        else:
                            source = "self.%s" % self._to_rust_variable(field.field_name)
                        self.out("let %s = %s.to_ne_bytes();", self._to_rust_variable(field.field_name + "_bytes"), source)
                        for i in range(field.type.size):
                            result_bytes.append("%s[%d]" % (self._to_rust_variable(field.field_name + "_bytes"), i))
                _emit()

                if has_variable_size_list:
                    self.out("result")
                else:
                    self.out("[")
                    for result_value in result_bytes:
                        self.out.indent("%s,", result_value)
                    self.out("]")
            self.out("}")
        self.out("}")

        self.out("")

    def union(self, enum, name):
        rust_name = self._name(name)
        self.out("#[derive(Debug, Clone)]")
        self.out("pub struct %s(Vec<u8>);", rust_name)

        self.out("impl %s {", rust_name)
        with Indent(self.out):
            for field in enum.fields:
                result_type = self._to_complex_rust_type(field.type, None, '')
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

        fixed_length = max((field.type.size * field.type.nmemb for field in enum.fields))

        self.out("impl TryParse for %s {", rust_name)
        with Indent(self.out):
            self.out("fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {")
            with Indent(self.out):
                self.out("let inner = value[..%s].iter().copied().collect();", fixed_length)
                self.out("let result = %s(inner);", rust_name)
                self.out("Ok((result, &value[%s..]))", fixed_length)
            self.out("}")
        self.out("}")

        self.out("")

    def request(self, obj, name):
        self.emit_opcode(name, 'REQUEST', obj.opcode)

        has_fd = any(field.isfd for field in obj.fields)
        if has_fd:
            self.out("pub fn %s() {", self._lower_snake_name(name))
            self.out.indent("unimplemented!(\"FD passing is not yet implemented\");")
            self.out("}")
            return

        is_list_fonts_with_info = name == ('xcb', 'ListFontsWithInfo')

        switches = list(filter(lambda field: field.type.is_switch, obj.fields))
        assert len(switches) <= 1
        if switches:
            aux_name = "%sAux" % self._name(name)
            switch = switches[0]

            # Find the mask field for the switch
            lenfield_name = switch.type.expr.lenfield_name
            mask_field = list(filter(lambda field: field.field_name == lenfield_name, obj.fields))
            assert len(mask_field) == 1
            mask_field = mask_field[0]

            # Hide it from the API and "connect" it to the switch
            mask_field.visible = False
            mask_field.lenfield_for_switch = switch

            self._generate_aux(aux_name, obj, switch, mask_field)
        else:
            aux_name = None

        mark_length_fields(obj)

        letters = iter(string.ascii_uppercase)
        connection_type = next(letters)

        need_lifetime = any(field.visible and field.type.is_list for field in obj.fields)
        need_lifetime = need_lifetime and obj.reply
        if need_lifetime:
            generics = ["'c"]
            args = ["c: &'c %s" % connection_type]
        else:
            generics = []
            args = ["c: &%s" % connection_type]
        generics.append("%s: X11Connection" % connection_type)
        where = []

        for field in obj.fields:
            if field.visible:
                rust_type = self._to_complex_rust_type(field.type, aux_name, '&')
                if field.enum is not None and not field.type.is_list:
                    letter = next(letters)
                    generics.append(letter)
                    where.append("%s: Into<%s>" % (letter, rust_type))
                    rust_type = letter
                args.append("%s: %s" % (self._to_rust_variable(field.field_name), rust_type))

        if is_list_fonts_with_info:
            assert need_lifetime
            result_type = "ListFontsWithInfoCookie<'c, %s>" % connection_type
        elif obj.reply:
            if need_lifetime:
                result_type = "Cookie<'c, %s, %sReply>" % (connection_type, self._name(name))
            else:
                result_type = "Cookie<%s, %sReply>" % (connection_type, self._name(name))
        else:
            result_type = "SequenceNumber"

        if generics:
            lifetime = "<%s>" % ", ".join(generics)
        else:
            lifetime = ""

        function_name = self._lower_snake_name(name)
        if function_name == "await":
            function_name = "await_"
        self.out("pub fn %s%s(%s) -> Result<%s, ConnectionError>", function_name, lifetime, ", ".join(args), result_type)
        if where:
            self.out("where %s", ", ".join(where))
        self.out("{")
        with Indent(self.out):

            if self.namespace.is_ext:
                self.out('let extension_information = c.extension_information("%s")' % self.namespace.ext_xname)
                self.out.indent(".ok_or(ConnectionError::UnsupportedExtension)?;")

            requests = []
            request = []

            def _emit_request():
                if not request:
                    return

                self.out("let request%d = [", len(requests))
                requests.append("&request%d" % len(requests))
                for byte in request:
                    self.out.indent("%s,", byte)
                self.out("];")
                del request[:]

            def _emit_byte_conversion(field_name):
                if field.type.size is not None:
                    self.out("let mut %s_bytes = Vec::with_capacity(%s * %s.len());", field.field_name, field.type.size, field.field_name)
                else:
                    self.out("let mut %s_bytes = Vec::new();", field.field_name)
                self.out("for value in %s {", field_name)
                self.out.indent("%s_bytes.extend(value.to_ne_bytes().iter());", field_name)
                self.out("}")

            fixed_request_length = sum((field.type.size * field.type.nmemb for field in obj.fields if field.type.nmemb is not None and field.type.size is not None and field.wire))
            request_length = [str(fixed_request_length)]
            for field in obj.fields:
                if field.type.nmemb is None:
                    size = field.type.size
                    if size is None:
                        _emit_byte_conversion(field.field_name)
                        request_length.append("%s_bytes.len()" % field.field_name)
                    else:
                        request_length.append("%s * %s.len()" % (size, self._to_rust_variable(field.field_name)))
                elif field.type.size is None:
                    assert field.type.nmemb is not None
                    self.out("let %s_bytes = %s.to_ne_bytes();", field.field_name, field.field_name)
                    request_length.append("%s_bytes.len()" % field.field_name)
                if hasattr(field, 'lenfield_for_switch'):
                    self.out("let %s = %s.value_mask();", field.field_name, field.lenfield_for_switch.field_name)
                    request_length.append("%s.wire_length()" % field.lenfield_for_switch.field_name)
            request_length = " + ".join(request_length)

            self.out("let length: usize = (%s + 3) / 4;", request_length)
            for field in obj.fields:
                if field.field_name == "major_opcode" or field.field_name == "minor_opcode":
                    if self.namespace.is_ext and field.field_name == "major_opcode":
                        request.append('extension_information.major_opcode')
                    else:
                        request.append("%s_REQUEST" % self._upper_snake_name(name))
                elif field.type.is_expr:
                    # FIXME: Replace this with expr_to_str()
                    def expr_to_str_and_emit(e):
                        if e.op is not None:
                            return "%s %s %s" % (expr_to_str_and_emit(e.lhs), e.op, expr_to_str_and_emit(e.rhs))
                        elif e.nmemb is not None:
                            return e.nmemb
                        else:
                            assert e.lenfield_name is not None
                            other_field = [field for field in obj.fields if e.lenfield_name == field.field_name]
                            assert len(other_field) == 1
                            other_field = other_field[0]
                            self.out("let %s: %s = %s.len().try_into()?;", other_field.field_name, self._to_rust_type(other_field.type.name), other_field.is_length_field_for.field_name)
                            return e.lenfield_name

                    self.out("let %s: %s = (%s).try_into().unwrap();", field.field_name, self._to_rust_type(field.type.name), expr_to_str_and_emit(field.type.expr))
                    self.out("let %s_bytes = %s.to_ne_bytes();", field.field_name, self._to_rust_variable(field.field_name))
                    for i in range(field.type.size):
                        request.append("%s_bytes[%d]" % (field.field_name, i))
                elif field.type.is_pad:
                    assert field.type.size == 1
                    for i in range(field.type.nmemb):
                        request.append("0")
                elif field.type.is_list:
                    if field.type.size == 1:
                        _emit_request()
                        requests.append(self._to_rust_variable(field.field_name))
                        if field == obj.fields[-1] and not field.type.fixed_size():
                            self.out("let %s_bytes = %s;", self._to_rust_variable(field.field_name), self._to_rust_variable(field.field_name))
                    else:
                        if field.type.size is not None:
                            _emit_byte_conversion(field.field_name)

                        _emit_request()
                        requests.append("&%s_bytes" % field.field_name)
                elif field.wire:
                    if hasattr(field, "is_length_field_for"):
                        self.out("let %s: %s = %s.len().try_into()?;", self._to_rust_variable(field.field_name), self._to_rust_type(field.type.name), self._to_rust_variable(field.is_length_field_for.field_name))
                    if field.enum is not None:
                        self.out("let %s = %s.into();", field.field_name, field.field_name)
                    if field.type.name == ('float',):
                        # FIXME: Switch to a trait that we can implement on f32
                        self.out("let %s = %s.to_bits().to_ne_bytes();", self._to_rust_variable(field.field_name + "_bytes"), self._to_rust_variable(field.field_name))
                    elif field.type.size is not None:  # Size None was already handled above
                        if field.field_name == "length":
                            source = "TryInto::<%s>::try_into(length)?" % self._to_rust_type(field.type.name)
                        else:
                            source = self._to_rust_variable(field.field_name)
                        self.out("let %s = %s.to_ne_bytes();", self._to_rust_variable(field.field_name + "_bytes"), source)
                    if field.type.is_switch or field.type.size is None:
                        _emit_request()
                        requests.append("&%s_bytes" % field.field_name)
                    else:
                        for i in range(field.type.size):
                            request.append("%s[%d]" % (self._to_rust_variable(field.field_name + "_bytes"), i))

            _emit_request()

            last_field = obj.fields[-1]
            if last_field.type.is_list and not last_field.type.fixed_size():
                self.out("let padding = &[0; 3][..(4 - (%s_bytes.len() %% 4)) %% 4];", last_field.field_name)
                requests.append("&padding")

            total_length = " + ".join(["(*%s).len()" % r for r in requests])
            self.out("assert_eq!(%s, (%s + 3) / 4 * 4);", total_length, request_length)

            slices = ", ".join(["IoSlice::new(%s)" % r for r in requests])

            if is_list_fonts_with_info:
                assert obj.reply
                self.out("Ok(ListFontsWithInfoCookie::new(c.send_request_with_reply(&[%s])))", slices)
            elif obj.reply:
                self.out("Ok(c.send_request_with_reply(&[%s]))", slices)
            else:
                self.out("Ok(c.send_request_without_reply(&[%s]))", slices)
        self.out("}")

        if obj.reply:
            has_fd = any(field.isfd for field in obj.reply.fields)
            if has_fd:
                self.out("#[derive(Debug, Clone, Copy)]")
                self.out("pub struct %sReply {}", self._name(name))
                self.out("impl TryFrom<Buffer> for %sReply {", self._name(name))
                with Indent(self.out):
                    self.out("type Error = ParseError;")
                    self.out("fn try_from(_value: Buffer) -> Result<Self, Self::Error> {")
                    self.out.indent("unimplemented!(\"Replies with FDs are not yet supported\");")
                    self.out("}")
                self.out("}")
            else:
                self.complex_type(obj.reply, name, False, 'Reply')

        self.out("")

    def eventstruct(self, eventstruct, name):
        self.out("")

    def event(self, event, name):
        self.emit_opcode(name, 'Event', event.opcodes[name])
        if event.is_ge_event:
            self.out("#[derive(Debug, Clone, Copy)]")
            self.out("pub struct %sEvent {", self._name(name))
            self.out.indent("pub generic_events_are_currently_not_supported: bool")
            self.out("}")
        else:
            self.complex_type(event, name, 'GenericEvent', 'Event')
        self.out("")

    def error(self, error, name):
        self.emit_opcode(name, 'Error', error.opcodes[name])
        self.complex_type(error, name, 'X11GenericError', 'Error')
        self.out("")

    def emit_opcode(self, name, extra_name, opcode):
        if opcode == "-1" and name == ('xcb', 'Glx', 'Generic'):
            return  # The XML has a comment saying "fake number"
        self.out("pub const %s_%s: u8 = %s;", self._upper_snake_name(name), extra_name.upper(), opcode)

    def _emit_parsing_code(self, fields):
        parts = []
        for field in fields:
            assert field.wire  # I *guess* that non-wire fields just have to be skipped
            if field.visible or hasattr(field, 'is_length_field_for'):
                rust_type = self._to_rust_type(field.type.name)
                if field.type.is_list and field.type.nmemb is not None:
                    for i in range(field.type.nmemb):
                        self.out("let (%s_%s, new_remaining) = %s::try_parse(remaining)?;", field.field_name, i, self._to_rust_type(field.type.name))
                        self.out("remaining = new_remaining;")
                    self.out("let %s = [", field.field_name)
                    for i in range(field.type.nmemb):
                        self.out.indent("%s_%s,", field.field_name, i)
                    self.out("];")
                    parts.append(field.field_name)
                elif field.type.is_list:
                    field_name = self._to_rust_variable(field.field_name)
                    self.out("let list_length = %s;", self.expr_to_str(field.type.expr, 'usize'))
                    self.out("let mut %s = Vec::with_capacity(list_length);", field_name)
                    self.out("for _ in 0..list_length {")
                    with Indent(self.out):
                        self.out("let (v, new_remaining) = %s::try_parse(remaining)?;", rust_type)
                        self.out("%s.push(v);", field_name)
                        self.out("remaining = new_remaining;")
                    self.out("}")
                    parts.append(field_name)
                else:
                    self.out("let (%s, new_remaining) = %s::try_parse(remaining)?;", self._to_rust_variable(field.field_name), rust_type)
                    self.out("remaining = new_remaining;")
                    if field.visible:
                        parts.append(self._to_rust_variable(field.field_name))
            else:
                if field.type.is_pad and field.type.align != 1:
                    assert field.type.size * field.type.nmemb == 1
                    align = field.type.align
                    self.out("// Align offset to multiple of %s", align)
                    self.out("let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;")
                    self.out("let misalignment = (%s - (offset %% %s)) %% %s;", align, align, align)
                    length = "misalignment"
                else:
                    length = field.type.size * field.type.nmemb
                self.out("remaining = &remaining.get(%s..).ok_or(ParseError::ParseError)?;", length)

        return parts

    def complex_type(self, complex, name, from_generic_type, extra_name, name_transform=lambda x: x):
        mark_length_fields(complex)

        if all(field.type.fixed_size and (not field.type.is_list or field.type.nmemb is not None) and not field.type.is_union for field in complex.fields):
            self.out("#[derive(Debug, Clone, Copy)]")
        else:
            self.out("#[derive(Debug, Clone)]")
        self.out("pub struct %s%s {", name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            for field in complex.fields:
                if field.visible:
                    field_name = self._to_rust_variable(field.field_name)
                    if field.type.is_list:
                        if field.type.nmemb is None:
                            self.out("pub %s: Vec<%s>,", field_name, self._to_rust_type(field.type.name))
                        else:
                            self.out("pub %s: [%s; %s],", field_name, self._to_rust_type(field.type.name), field.type.nmemb)
                    else:
                        self.out("pub %s: %s,", field_name, self._to_rust_type(field.type.name))
        self.out("}")

        self.out("impl TryParse for %s%s {", name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            self.out("fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {")
            with Indent(self.out):
                self.out("let mut remaining = value;")
                parts = self._emit_parsing_code(complex.fields)
                self.out("let result = %s%s { %s };", name_transform(self._name(name)), extra_name, ", ".join(parts))
                self.out("Ok((result, remaining))")
            self.out("}")
        self.out("}")

        self.out("impl TryFrom<Buffer> for %s%s {", name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: Buffer) -> Result<Self, Self::Error> {")
            self.out.indent("Self::try_from(&*value)")
            self.out("}")
        self.out("}")

        if from_generic_type:
            self.out("impl TryFrom<%s> for %s%s {", from_generic_type, name_transform(self._name(name)), extra_name)
            with Indent(self.out):
                self.out("type Error = ParseError;")
                self.out("fn try_from(value: %s) -> Result<Self, Self::Error> {", from_generic_type)
                self.out.indent("Self::try_from(Into::<Buffer>::into(value))")
                self.out("}")
            self.out("}")

        self.out("impl TryFrom<&[u8]> for %s%s {", name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: &[u8]) -> Result<Self, Self::Error> {")
            self.out.indent("Ok(Self::try_parse(value)?.0)")
            self.out("}")
        self.out("}")

    def _to_complex_rust_type(self, field_type, aux_name, modifier):
        if field_type.is_switch:
            return modifier + aux_name
        result = self._to_rust_type(field_type.name)
        if field_type.is_list:
            if field_type.nmemb is None:
                result = "%s[%s]" % (modifier, result)
            else:
                result = "%s[%s; %s]" % (modifier, result, field_type.nmemb)
        return result

    def _generate_aux(self, name, request, switch, mask_field):
        # This used to assert that all fields have the same size, but sync's
        # CreateAlarm has both 32 bit and 64 bit numbers.
        min_field_size = min(field.type.size for field in switch.type.fields)
        assert all(field.type.size % min_field_size == 0 for field in switch.type.fields)

        self.out("#[derive(Debug, Clone, Copy, Default)]")
        self.out("pub struct %s {", name)
        for field in switch.type.fields:
            self.out.indent("pub %s: Option<%s>,", self._aux_field_name(field), self._to_rust_type(field.type.name))
        self.out("}")

        self.out("impl %s {", name)
        with Indent(self.out):

            self.out("pub fn new() -> Self {")
            self.out.indent("Default::default()")
            self.out("}")

            self.out("fn wire_length(&self) -> usize {")
            with Indent(self.out):
                self.out("let mut result = 0;")
                for field in switch.type.fields:
                    self.out("if self.%s.is_some() {", self._aux_field_name(field))
                    self.out.indent("result += %s;", field.type.size)
                    self.out("}")
                self.out("result")
            self.out("}")

            self.out("pub fn to_ne_bytes(&self) -> Vec<u8> {")
            with Indent(self.out):
                self.out("let mut result = Vec::new();")
                for field in switch.type.fields:
                    self.out("if let Some(value) = self.%s {", self._aux_field_name(field))
                    self.out.indent("result.extend(value.to_ne_bytes().iter());")
                    self.out("}")
                self.out("result")
            self.out("}")

            self.out("pub fn value_mask(&self) -> %s {", self._to_rust_type(mask_field.type.name))
            with Indent(self.out):
                self.out("let mut mask = 0;")
                for field in switch.type.fields:
                    expr, = field.parent.expr
                    assert expr.op == "enumref"
                    enum_name = self._name(expr.lenfield_type.name)
                    self.out("if self.%s.is_some() {", self._aux_field_name(field))
                    self.out.indent("mask |= Into::<%s>::into(%s::%s);", self._to_rust_type(mask_field.type.name), enum_name, expr.lenfield_name)
                    self.out("}")
                self.out("mask")
            self.out("}")

            for field in switch.type.fields:
                aux_name = self._aux_field_name(field)
                self.out("pub fn %s<I>(mut self, value: I) -> Self where I: Into<Option<%s>> {", aux_name, self._to_rust_type(field.type.name))
                with Indent(self.out):
                    self.out("self.%s = value.into();", aux_name)
                    self.out("self")
                self.out("}")

        self.out("}")

    def _name(self, name):
        orig_name = name
        if name[0] == 'xcb':
            name = name[1:]
        if self.namespace.is_ext and name[0] == self.namespace.ext_name:
            name = name[1:]
        assert len(name) == 1, orig_name
        return name[0]

    def _lower_snake_name(self, name):
        name = self._name(name)
        name = re.sub('([a-z0-9])([A-Z])', '\\1_\\2', name)
        return name.lower()

    def _upper_snake_name(self, name):
        return self._lower_snake_name(name).upper()

    def _aux_field_name(self, field):
        return self._lower_snake_name((field.field_name,))

    def _to_rust_type(self, name):
        orig_name = name
        if type(name) == tuple:
            if name[0] == 'xcb':
                name = name[1:]
            if self.namespace.is_ext and name[0] == self.namespace.ext_name:
                name = name[1:]
            if len(name) == 2:
                # If this is still a tuple, it should be a type from another module
                # and we should have imported that module.
                ext = name[0].lower()  # FIXME: How to get the name of the ext?
                name = ext + "::" + name[1],
            assert len(name) == 1, orig_name
            name = name[0]

        if name in rust_type_mapping:
            return rust_type_mapping[name]
        elif name.isupper():
            return self._to_rust_identifier(name)
        else:
            return name

    def _to_rust_identifier(self, name):
        # If the NAME is all uppercase, turn it into Name (the upper() is done below)
        if name.isupper():
            name = name.lower()
        # If the name contains "_", turn it into camel case
        if "_" in name:
            name = re.sub('_(.)', lambda pat: pat.group(1).upper(), name.lower())
        # The first letter should always be upper case
        return name[0].upper() + name[1:]

    def _to_rust_variable(self, name):
        if name == "type":
            name = "type_"

        # Check for camel case names and deal with them
        if any(c.isupper() for c in name):
            assert name[0].islower()
            name = self._lower_snake_name((name,))

        return name

    def expr_to_str(self, e, type):
        if e.op is not None:
            if e.op == 'calculate_len':
                return e.op
            if e.op == '~':
                return "!(%s)" % self.expr_to_str(e.rhs, type)
            return "(%s) %s (%s)" % (self.expr_to_str(e.lhs, type), e.op, self.expr_to_str(e.rhs, type))
        elif e.nmemb is not None:
            return e.nmemb
        else:
            assert e.lenfield_name is not None
            return "%s as %s" % (self._to_rust_variable(e.lenfield_name), type)
