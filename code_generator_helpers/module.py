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
        if expr.op in ['calculate_len', 'sumof']:
            return []
        if expr.op == '~':
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


def _emit_doc(out, doc):
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


class Module(object):
    def __init__(self, outer_module):
        self.out = Output()
        self.trait_out = Output()
        self.namespace = outer_module.namespace

        self.out("use std::convert::TryFrom;")
        self.out("#[allow(unused_imports)]")
        self.out("use std::convert::TryInto;")
        self.out("use std::io::IoSlice;")
        self.out("#[allow(unused_imports)]")
        self.out("use std::option::Option as RustOption;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::utils::{Buffer, RawFdContainer};")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::x11_utils::{GenericEvent as X11GenericEvent, GenericError as X11GenericError};")
        self.out("use crate::x11_utils::TryParse;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::connection::SequenceNumber;")
        self.out("#[allow(unused_imports)]")
        self.out("use crate::connection::{Cookie, CookieWithFds, Connection as X11Connection};")
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
        self.out("/// Extension trait defining the requests of this extension.")
        self.out("pub trait ConnectionExt: X11Connection {")
        with Indent(self.out):
            self.out.copy_from(self.trait_out)
        self.out("}")
        self.out("impl<C: X11Connection + ?Sized> ConnectionExt for C {}")

    def enum(self, enum, name):
        has_all_upper = any(ename.isupper() and len(ename) > 1 for (ename, value) in enum.values)

        rust_name = self._name(name)
        _emit_doc(self.out, enum.doc)
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

        self.out("impl Into<RustOption<%s>> for %s {", to_type, rust_name)
        with Indent(self.out):
            self.out("fn into(self) -> RustOption<%s> {", to_type)
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

            self.out("impl Into<RustOption<%s>> for %s {", larger_type, rust_name)
            with Indent(self.out):
                self.out("fn into(self) -> RustOption<%s> {", larger_type)
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
        assert not hasattr(simple, "doc")
        # FIXME: Figure out what to do with names. _to_rust_identifier() does the
        # right thing here, but then we get both 'pub type Window = u32;' and 'enum
        # Window', which the compiler does not like.
        name = self._name(name)
        if '_' in name:
            name = self._to_rust_identifier(name)
        self.out("pub type %s = %s;", name, self._to_rust_type(simple.name))
        self.out("")

    def struct(self, struct, name):
        assert not hasattr(struct, "doc")
        has_variable_size_list = any(field.type.is_list and field.type.nmemb is None for field in struct.fields)
        self.complex_type(struct, name, '', True, lambda name: self._to_rust_identifier(name))

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
        assert not hasattr(enum, "doc")

        rust_name = self._name(name)
        self.out("#[derive(Debug, Clone)]")
        self.out("pub struct %s(Vec<u8>);", rust_name)

        self.out("impl %s {", rust_name)
        with Indent(self.out):
            for field in enum.fields:
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

            self.out("fn to_ne_bytes(&self) -> &[u8] {")
            self.out.indent("&self.0")
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

        function_name = self._lower_snake_name(name)
        if function_name == "await":
            function_name = "await_"

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

            self._generate_aux(aux_name, obj, switch, mask_field, function_name)
        else:
            aux_name = None

        mark_length_fields(obj)

        letters = iter(string.ascii_uppercase)

        need_lifetime = any(field.visible and field.type.is_list for field in obj.fields)
        need_lifetime = need_lifetime and obj.reply
        if need_lifetime:
            generics = ["'c"]
            args = ["&'c self"]
        else:
            generics = []
            args = ["&self"]
        where = []

        fds, fds_is_list = [], False
        for field in obj.fields:
            if field.visible:
                rust_type = self._to_complex_rust_type(field, aux_name, '&')
                if (field.enum is not None and not field.type.is_list) or \
                        (field.isfd and not field.type.is_list) or \
                        (name == ('xcb', 'SendEvent') and field.field_name == 'event'):
                    if name == ('xcb', 'SendEvent') and field.field_name == 'event':
                        # Turn &[u8; 32] into [u8; 32]
                        assert rust_type[0] == '&'
                        rust_type = rust_type[1:]
                    letter = next(letters)
                    generics.append(letter)
                    where.append("%s: Into<%s>" % (letter, rust_type))
                    rust_type = letter
                rust_type = self._to_rust_identifier(rust_type)
                if name == ('xcb', 'InternAtom') and field.field_name == 'only_if_exists':
                    rust_type = 'bool'
                args.append("%s: %s" % (self._to_rust_variable(field.field_name), rust_type))
                if field.isfd:
                    fds.append("%s.into()" % self._to_rust_variable(field.field_name))
                    if field.type.is_list:
                        fds_is_list = True

        if is_list_fonts_with_info:
            assert need_lifetime
            result_type = "ListFontsWithInfoCookie<'c, Self>"
        elif obj.reply:
            if any(field.isfd for field in obj.reply.fields):
                cookie = "CookieWithFds"
            else:
                cookie = "Cookie"
            if need_lifetime:
                result_type = "%s<'c, Self, %sReply>" % (cookie, self._name(name))
            else:
                result_type = "%s<Self, %sReply>" % (cookie, self._name(name))
        else:
            result_type = "SequenceNumber"

        if generics:
            lifetime = "<%s>" % ", ".join(generics)
        else:
            lifetime = ""

        _emit_doc(self.trait_out, obj.doc)
        self.trait_out("fn %s%s(%s) -> Result<%s, ConnectionError>", function_name, lifetime, ", ".join(args), result_type)
        if where:
            self.trait_out("where %s", ", ".join(where))
        self.trait_out("{")
        with Indent(self.trait_out):

            if self.namespace.is_ext:
                self.trait_out('let extension_information = self.extension_information("%s")' % self.namespace.ext_xname)
                self.trait_out.indent(".ok_or(ConnectionError::UnsupportedExtension)?;")

            requests = []
            request = []

            def _emit_request():
                if not request:
                    return

                self.trait_out("let request%d = [", len(requests))
                requests.append("&request%d" % len(requests))
                for byte in request:
                    self.trait_out.indent("%s,", byte)
                self.trait_out("];")
                del request[:]

            def _emit_byte_conversion(field_name):
                if field.type.size is not None:
                    self.trait_out("let mut %s_bytes = Vec::with_capacity(%s * %s.len());", field.field_name, field.type.size, field.field_name)
                else:
                    self.trait_out("let mut %s_bytes = Vec::new();", field.field_name)
                self.trait_out("for value in %s {", field_name)
                self.trait_out.indent("%s_bytes.extend(value.to_ne_bytes().iter());", field_name)
                self.trait_out("}")

            fixed_request_length = sum((field.type.size * field.type.nmemb for field in obj.fields if field.type.nmemb is not None and field.type.size is not None and field.wire))
            request_length = [str(fixed_request_length)]
            for field in obj.fields:
                if not field.wire:
                    continue
                if field.type.nmemb is None:
                    size = field.type.size
                    if size is None:
                        _emit_byte_conversion(field.field_name)
                        request_length.append("%s_bytes.len()" % field.field_name)
                    else:
                        request_length.append("%s * %s.len()" % (size, self._to_rust_variable(field.field_name)))
                elif field.type.size is None:
                    assert field.type.nmemb is not None
                    self.trait_out("let %s_bytes = %s.to_ne_bytes();", field.field_name, field.field_name)
                    request_length.append("%s_bytes.len()" % field.field_name)
                if hasattr(field, 'lenfield_for_switch'):
                    self.trait_out("let %s = %s.value_mask();", field.field_name, field.lenfield_for_switch.field_name)
                    request_length.append("%s.wire_length()" % field.lenfield_for_switch.field_name)
            request_length = " + ".join(request_length)

            self.trait_out("let length: usize = (%s + 3) / 4;", request_length)
            for field in obj.fields:
                if not field.wire:
                    continue
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
                            self.trait_out("let %s: %s = %s.len().try_into()?;", other_field.field_name, self._to_rust_type(other_field.type.name), other_field.is_length_field_for.field_name)
                            return e.lenfield_name

                    self.trait_out("let %s: %s = (%s).try_into().unwrap();", field.field_name, self._to_rust_type(field.type.name), expr_to_str_and_emit(field.type.expr))
                    self.trait_out("let %s_bytes = %s.to_ne_bytes();", field.field_name, self._to_rust_variable(field.field_name))
                    for i in range(field.type.size):
                        request.append("%s_bytes[%d]" % (field.field_name, i))
                elif field.type.is_pad:
                    assert field.type.size == 1
                    for i in range(field.type.nmemb):
                        request.append("0")
                elif field.type.is_list:
                    if name == ('xcb', 'SendEvent') and field.field_name == 'event':
                        self.trait_out("let %s = %s.into();", field.field_name, field.field_name)
                        self.trait_out("let %s = &%s;", field.field_name, field.field_name)
                    if not (hasattr(field, "has_length_field") or field.type.fixed_size()):
                        self.trait_out("assert_eq!(%s.len(), %s, \"Argument %s has an incorrect length\");",
                                       self._to_rust_variable(field.field_name), self.expr_to_str(field.type.expr, "usize"), field.field_name)
                    if field.type.size == 1:
                        _emit_request()
                        requests.append(self._to_rust_variable(field.field_name))
                        if field == obj.fields[-1] and not field.type.fixed_size():
                            self.trait_out("let %s_bytes = %s;", self._to_rust_variable(field.field_name), self._to_rust_variable(field.field_name))
                    else:
                        if field.type.size is not None:
                            _emit_byte_conversion(field.field_name)

                        _emit_request()
                        requests.append("&%s_bytes" % field.field_name)
                elif field.wire:
                    if hasattr(field, "is_length_field_for"):
                        self.trait_out("let %s: %s = %s.len().try_into()?;",
                                       self._to_rust_variable(field.field_name),
                                       self._to_rust_type(field.type.name),
                                       self._to_rust_variable(field.is_length_field_for.field_name))
                    if field.enum is not None:
                        self.trait_out("let %s = %s.into();", field.field_name, field.field_name)
                    if field.type.name == ('float',):
                        # FIXME: Switch to a trait that we can implement on f32
                        self.trait_out("let %s = %s.to_bits().to_ne_bytes();", self._to_rust_variable(field.field_name + "_bytes"), self._to_rust_variable(field.field_name))
                    elif field.type.size is not None:  # Size None was already handled above
                        if (name == ('xcb', 'InternAtom') and field.field_name == 'only_if_exists'):
                            self.trait_out("let %s = %s as %s;", field.field_name, field.field_name, self._to_rust_type(field.type.name))
                        if field.field_name == "length":
                            source = "TryInto::<%s>::try_into(length).unwrap_or(0)" % self._to_rust_type(field.type.name)
                        else:
                            source = self._to_rust_variable(field.field_name)
                        self.trait_out("let %s = %s.to_ne_bytes();", self._to_rust_variable(field.field_name + "_bytes"), source)
                    if field.type.is_switch or field.type.size is None:
                        _emit_request()
                        requests.append("&%s_bytes" % field.field_name)
                    else:
                        for i in range(field.type.size):
                            request.append("%s[%d]" % (self._to_rust_variable(field.field_name + "_bytes"), i))

            _emit_request()

            last_field = obj.fields[-1]
            if last_field.isfd:
                last_field = obj.fields[-2]
            if last_field.type.is_list and not last_field.type.fixed_size():
                self.trait_out("let padding = &[0; 3][..(4 - (%s_bytes.len() %% 4)) %% 4];", last_field.field_name)
                requests.append("&padding")

            total_length = " + ".join(["(*%s).len()" % r for r in requests])
            self.trait_out("assert_eq!(%s, (%s + 3) / 4 * 4);", total_length, request_length)

            slices = ", ".join(["IoSlice::new(%s)" % r for r in requests])

            if fds:
                if not fds_is_list:
                    self.trait_out("let fds = vec!(%s);", ", ".join(fds))
                    fds = "fds"
                else:
                    # There may (currently) be only a single list of FDs
                    fds, = fds
            else:
                fds = "Vec::new()"

            if is_list_fonts_with_info:
                assert obj.reply
                self.trait_out("Ok(ListFontsWithInfoCookie::new(self.send_request_with_reply(&[%s], %s)?))", slices, fds)
            elif obj.reply:
                if any(field.isfd for field in obj.reply.fields):
                    self.trait_out("Ok(self.send_request_with_reply_with_fds(&[%s], %s)?)", slices, fds)
                else:
                    self.trait_out("Ok(self.send_request_with_reply(&[%s], %s)?)", slices, fds)
            else:
                self.trait_out("Ok(self.send_request_without_reply(&[%s], %s)?)", slices, fds)
        self.trait_out("}")
        self.trait_out("")

        if obj.reply:
            _emit_doc(self.out, obj.reply.doc)
            self.complex_type(obj.reply, name, 'Reply', False)

        self.out("")

    def eventstruct(self, eventstruct, name):
        assert False
        self.out("")

    def event(self, event, name):
        self.emit_opcode(name, 'Event', event.opcodes[name])
        _emit_doc(self.out, event.doc)
        if event.is_ge_event:
            self.out("#[derive(Debug, Clone, Copy)]")
            self.out("pub struct %sEvent {", self._name(name))
            self.out.indent("pub generic_events_are_currently_not_supported: bool")
            self.out("}")
        else:
            self.complex_type(event, name, 'Event', False)
            self._emit_from_generic(name, 'X11GenericEvent', 'Event')
            self._emit_serialise(event, name, 'Event')
        self.out("")

    def error(self, error, name):
        assert not hasattr(error, "doc")
        self.emit_opcode(name, 'Error', error.opcodes[name])
        self.complex_type(error, name, 'Error', False)
        self._emit_from_generic(name, 'X11GenericError', 'Error')
        self._emit_serialise(error, name, 'Error')
        self.out("")

    def _emit_from_generic(self, name, from_generic_type, extra_name):
        self.out("impl From<%s> for %s%s {", from_generic_type, self._name(name), extra_name)
        with Indent(self.out):
            self.out("fn from(value: %s) -> Self {", from_generic_type)
            self.out.indent("Self::try_from(Into::<Buffer>::into(value))" +
                            ".expect(\"Buffer should be large enough so that parsing cannot fail\")")
            self.out("}")
        self.out("}")

    def _emit_serialise(self, obj, name, extra_name):
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
                                    self.out("let %s_%d = self.%s[%d].to_ne_bytes();",
                                             field_name, i, field_name, i)
                                    for n in range(field.type.size):
                                        parts.append("%s_%d[%d]" % (field_name, i, n))
                        elif field.type.size == 1:
                            parts.append("self.%s" % field_name)
                        else:
                            self.out("let %s = self.%s.to_ne_bytes();", field_name, field_name)
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
        parts = []
        for field in fields:
            if not field.wire:
                if not field.isfd:
                    continue
                field_name = self._to_rust_variable(field.field_name)
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
            elif not field.type.is_pad:
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
                    try_parse_args = ["remaining"]
                    if hasattr(field.type, "member"):
                        if hasattr(field.type.member, "extra_try_parse_args"):
                            try_parse_args += field.type.member.extra_try_parse_args

                    field_name = self._to_rust_variable(field.field_name)
                    self.out("let list_length = %s;", self.expr_to_str(field.type.expr, 'usize'))
                    self.out("let mut %s = Vec::with_capacity(list_length);", field_name)
                    self.out("for _ in 0..list_length {")
                    with Indent(self.out):
                        self.out("let (v, new_remaining) = %s::try_parse(%s)?;", rust_type, ", ".join(try_parse_args))
                        self.out("%s.push(v);", field_name)
                        self.out("remaining = new_remaining;")
                    self.out("}")
                    parts.append(field_name)
                else:
                    self.out("let (%s, new_remaining) = %s::try_parse(remaining)?;", self._to_rust_variable(field.field_name), self._to_rust_identifier(rust_type))
                    self.out("remaining = new_remaining;")
                    if not hasattr(field, 'is_length_field_for'):
                        parts.append(self._to_rust_variable(field.field_name))
            else:
                assert field.type.is_pad
                if field.type.align != 1:
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

    def complex_type(self, complex, name, extra_name, impl_try_from, name_transform=lambda x: x):
        mark_length_fields(complex)

        fixed_size = all(field.type.fixed_size for field in complex.fields)
        no_variable_length_list = all(not field.type.is_list or field.type.nmemb is not None for field in complex.fields)
        no_union = all(not field.type.is_union for field in complex.fields)
        has_fds = any(field.isfd for field in complex.fields)
        assert not (impl_try_from and has_fds)
        if has_fds:
            self.out("#[derive(Debug)]")
        elif fixed_size and no_variable_length_list and no_union:
            self.out("#[derive(Debug, Clone, Copy)]")
        else:
            self.out("#[derive(Debug, Clone)]")
        self.out("pub struct %s%s {", name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            for field in complex.fields:
                if field.visible or (not field.type.is_pad and not hasattr(field, "is_length_field_for")):
                    field_name = self._to_rust_variable(field.field_name)
                    if field.isfd:
                        rust_type = "RawFdContainer"
                    else:
                        rust_type = self._to_rust_type(field.type.name)
                    if field.type.is_list:
                        if field.type.nmemb is None:
                            self.out("pub %s: Vec<%s>,", field_name, rust_type)
                        else:
                            self.out("pub %s: [%s; %s],", field_name, rust_type, field.type.nmemb)
                    else:
                        self.out("pub %s: %s,", field_name, self._to_rust_identifier(rust_type))
        self.out("}")

        # Collect all the fields that appear on the wire in the parent object
        wire_fields = [field.field_name for field in complex.fields if field.wire]
        # Collect all fields that are referenced
        referenced = []
        for field in complex.fields:
            if field.type.is_list:
                referenced.extend(get_references(field.type.expr))
        # Collect all the fields that appear "out of thin air"
        unresolved = [field for field in referenced if field not in wire_fields]

        if impl_try_from and not unresolved:
            method = "fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>"
            self.out("impl TryParse for %s%s {", name_transform(self._name(name)), extra_name)
        else:
            if has_fds:
                method = "fn try_parse_fd<'a>(value: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError>"
            elif impl_try_from:
                # Turn missing values into extra arguments
                assert unresolved
                unresolved_args, extra_args = [], []
                for field in complex.fields:
                    if field.field_name in unresolved:
                        field_type = self._to_complex_rust_type(field, None, '')
                        unresolved_args.append("%s: %s" % (field.field_name, field_type))
                        extra_args.append(field.field_name)
                method = "pub fn try_parse(value: &[u8], %s) -> Result<(Self, &[u8]), ParseError>" % ", ".join(unresolved_args)

                complex.extra_try_parse_args = extra_args
            else:
                method = "pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>"
            self.out("impl %s%s {", name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            self.out("%s {", method)
            with Indent(self.out):
                self.out("let mut remaining = value;")
                parts = self._emit_parsing_code(complex.fields)
                self.out("let result = %s%s { %s };", name_transform(self._name(name)), extra_name, ", ".join(parts))
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
            value = "Buffer"
        self.out("impl TryFrom<%s> for %s%s {", value, name_transform(self._name(name)), extra_name)
        with Indent(self.out):
            self.out("type Error = ParseError;")
            self.out("fn try_from(value: %s) -> Result<Self, Self::Error> {", value)
            if has_fds:
                self.out.indent("Self::try_from((&*value.0, value.1))")
            else:
                self.out.indent("Self::try_from(&*value)")
            self.out("}")
        self.out("}")

        if has_fds:
            value = "(&[u8], Vec<RawFdContainer>)"
        else:
            value = "&[u8]"
        self.out("impl TryFrom<%s> for %s%s {", value, name_transform(self._name(name)), extra_name)
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
            result = self._to_rust_type(field.type.name)
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

    def _generate_aux(self, name, request, switch, mask_field, request_function_name):
        # This used to assert that all fields have the same size, but sync's
        # CreateAlarm has both 32 bit and 64 bit numbers.
        min_field_size = min(field.type.size for field in switch.type.fields)
        assert all(field.type.size % min_field_size == 0 for field in switch.type.fields)

        self.out("/// Auxiliary and optional information for the %s function.", request_function_name)
        self.out("#[derive(Debug, Clone, Copy, Default)]")
        self.out("pub struct %s {", name)
        for field in switch.type.fields:
            self.out.indent("%s: RustOption<%s>,", self._aux_field_name(field), self._to_rust_type(field.type.name))
        self.out("}")

        self.out("impl %s {", name)
        with Indent(self.out):

            self.out("/// Create a new instance with all fields unset / not present.")
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

            self.out("fn to_ne_bytes(&self) -> Vec<u8> {")
            with Indent(self.out):
                self.out("let mut result = Vec::new();")
                for field in switch.type.fields:
                    self.out("if let Some(value) = self.%s {", self._aux_field_name(field))
                    self.out.indent("result.extend(value.to_ne_bytes().iter());")
                    self.out("}")
                self.out("result")
            self.out("}")

            self.out("fn value_mask(&self) -> %s {", self._to_rust_type(mask_field.type.name))
            with Indent(self.out):
                self.out("let mut mask = 0;")
                for field in switch.type.fields:
                    expr, = field.parent.expr
                    assert expr.op == "enumref"
                    enum_name = self._name(expr.lenfield_type.name)
                    self.out("if self.%s.is_some() {", self._aux_field_name(field))
                    self.out.indent("mask |= Into::<%s>::into(%s::%s);", self._to_rust_type(mask_field.type.name),
                                    enum_name, ename_to_rust(expr.lenfield_name))
                    self.out("}")
                self.out("mask")
            self.out("}")

            for field in switch.type.fields:
                aux_name = self._aux_field_name(field)
                self.out("/// Set the %s field of this structure.", field.field_name)
                self.out("pub fn %s<I>(mut self, value: I) -> Self where I: Into<RustOption<%s>> {", aux_name, self._to_rust_type(field.type.name))
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

        # Check for camel case names and deal with them
        if any(c.isupper() for c in name):
            assert name[0].islower()
            name = self._lower_snake_name((name,))

        return name

    def expr_to_str(self, e, type):
        if e.op is not None:
            if e.op == 'calculate_len':
                return e.op
            if e.op == 'sumof':
                assert e.rhs.op is None
                assert e.rhs.nmemb is None
                field_name = e.rhs.lenfield_name
                return "%s.iter().map(|x| x.%s as usize).sum()" % (e.lenfield_name, field_name)
            if e.op == '~':
                return "!(%s)" % self.expr_to_str(e.rhs, type)
            return "(%s) %s (%s)" % (self.expr_to_str(e.lhs, type), e.op, self.expr_to_str(e.rhs, type))
        elif e.nmemb is not None:
            return e.nmemb
        else:
            assert e.lenfield_name is not None
            return "%s as %s" % (self._to_rust_variable(e.lenfield_name), type)
