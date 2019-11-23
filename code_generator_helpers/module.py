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


def compute_copy_clone(obj_type):
    if hasattr(obj_type, "computed_rust_copy_clone"):
        return
    obj_type.computed_rust_copy_clone = True

    copy, clone = True, True
    if obj_type.is_container:
        if obj_type.is_union:
            # These are represented as a type containing Vec
            copy = False

        for field in obj_type.fields:
            compute_copy_clone(field.type)
            copy &= field.type.is_rust_copy
            clone &= field.type.is_rust_clone

            # Sigh, FDs are weird and need special care
            if hasattr(field, "isfd") and field.isfd:
                # RawFdContainer cannot be cloned
                copy, clone = False, False
    elif obj_type.is_list:
        if obj_type.nmemb is None:
            # Variable length list, represented as Vec
            copy = False
    else:
        assert obj_type.is_simple or obj_type.is_pad, obj_type

    obj_type.is_rust_copy = copy
    obj_type.is_rust_clone = clone


def is_copy(obj):
    compute_copy_clone(obj)
    return obj.is_rust_copy


def is_clone(obj):
    compute_copy_clone(obj)
    return obj.is_rust_clone


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
        rust_name = self._name(name)
        _emit_doc(self.out, enum.doc)
        self.out("#[derive(Debug, Clone, Copy)]")
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
        if '_' in name:
            name = self._to_rust_identifier(name)
        self.out("pub type %s = %s;", name, self._to_rust_type(simple.name))
        self.out("")

    def struct(self, struct, name):
        assert not hasattr(struct, "doc")

        # Emit the struct definition itself
        self.complex_type(struct, name, '', True, lambda name: self._to_rust_identifier(name))

        # And now emit some functions for the struct.

        has_variable_size_list = any(field.type.is_list and field.type.nmemb is None for field in struct.fields)
        if has_variable_size_list:
            # For a variable size list, we do not know beforehand the size of the
            # serialised data. Thus, return a Vec.
            length = None
            wire_type = "Vec<u8>"
        else:
            # Everything is fixed-size so we can return an array.
            length = sum((field.type.get_total_size() for field in struct.fields))
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
                    _final_emit = _emit
                else:
                    def _emit():
                        assert False, "We do not have a variable size list, but we do?"

                    def _final_emit():
                        pass

                # This gathers the bytes of the result; its content is copied to
                # result:Vec<u8> by _emit(). This happens in front of variable sized lists
                result_bytes = []

                for field in struct.fields:
                    if field.type.is_pad:
                        if has_variable_size_list and field.type.align != 1:
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
                        self.out("for obj in self.%s.iter() {", field.field_name)
                        self.out.indent("result.extend(obj.to_ne_bytes().iter());")
                        self.out("}")
                    elif field.type.is_list and field.type.nmemb is not None and field.type.size == 1:
                        # Fixed-sized list with byte-sized members
                        field_name = self._to_rust_variable(field.field_name)
                        for i in range(field.type.nmemb):
                            result_bytes.append("self.%s[%d]" % (field_name, i))
                    else:
                        # Fixed-sized list with "large" members. We have first serialise
                        # the members individually and then assemble that into the output.
                        field_name = self._to_rust_variable(field.field_name)
                        field_name_bytes = self._to_rust_variable(field.field_name + "_bytes")
                        if hasattr(field, "is_length_field_for"):
                            # This field is a length field for some list. We get the value
                            # for this field as the length of the list.
                            self.out("let %s = self.%s.len() as %s;", field_name,
                                     field.is_length_field_for.field_name, self._to_rust_type(field.type.name))
                            source = field_name
                        else:
                            # Get the value of this field from "self".
                            source = "self.%s" % field_name
                        # First serialise the value itself...
                        self.out("let %s = %s.to_ne_bytes();", field_name_bytes, source)
                        # ...then copy to the output.
                        for i in range(field.type.size):
                            result_bytes.append("%s[%d]" % (field_name_bytes, i))
                _final_emit()

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

            self.out("fn to_ne_bytes(&self) -> &[u8] {")
            self.out.indent("&self.0")
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

        self.out("")

    def request(self, obj, name):
        self.emit_opcode(name, 'REQUEST', obj.opcode)

        function_name = self._lower_snake_name(name)
        if function_name == "await":
            function_name = "await_"

        is_list_fonts_with_info = name == ('xcb', 'ListFontsWithInfo')

        # Does this have a <switch>? If so, we generate an *Aux structure
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

        # Now start constructing the function prototype of the function for
        # sending requests.

        # Iterator of letters, used for naming generic parameters
        letters = iter(string.ascii_uppercase)

        # We need a lifetime if there are more than one references in the
        # arguments. Right now that means: Any lists?
        need_lifetime = any(field.type.is_list for field in obj.fields)
        need_lifetime = need_lifetime and obj.reply
        if need_lifetime:
            generics = ["'c"]
            args = ["&'c self"]
        else:
            generics = []
            args = ["&self"]
        # list of "where" conditions to add to the prototype
        where = []

        # Now collect the arguments of the function. 'fds' will collect all file
        # descriptors in the request. fds_is_list indicates a list of FDs.
        fds, fds_is_list = [], False
        for field in obj.fields:
            if field.visible:
                rust_type = self._to_complex_rust_type(field, aux_name, '&')

                # Does this argument have a generic type? This is yes if
                # - it comes from an enum, or
                # - it is a FD (but not a list of FDs)
                # - It is the "event" argument of SendEvent
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

        # Figure out the return type of the request function
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
            generics = "<%s>" % ", ".join(generics)
        else:
            generics = ""

        # Finally: Actually emit the function prototype
        _emit_doc(self.trait_out, obj.doc)
        self.trait_out("fn %s%s(%s) -> Result<%s, ConnectionError>", function_name, generics, ", ".join(args), result_type)
        if where:
            self.trait_out("where %s", ", ".join(where))
        self.trait_out("{")
        with Indent(self.trait_out):

            if self.namespace.is_ext:
                self.trait_out('let extension_information = self.extension_information("%s")' % self.namespace.ext_xname)
                self.trait_out.indent(".ok_or(ConnectionError::UnsupportedExtension)?;")

            # Now generate code for serialising the request. The request bytes
            # will be collected in a number of arrays. Some of them are
            # constructed in this function and others are passed in.

            # List of arrays to send
            requests = []
            # The latest array of byte that is being constructed
            request = []

            # Add the given byte array to requests
            def _emit_add_to_requests(part):
                if requests:
                    add = "length_so_far + "
                else:
                    add = ""
                self.trait_out("let length_so_far = %s(%s).len();",
                               add, part)
                requests.append(part)

            # This function adds the current 'request' to the list of 'requests'
            def _emit_request():
                if not request:
                    return

                self.trait_out("let request%d = [", len(requests))
                for byte in request:
                    self.trait_out.indent("%s,", byte)
                self.trait_out("];")
                _emit_add_to_requests("&request%d" % len(requests))
                del request[:]

            # Emit the code for converting a list to bytes.
            def _emit_byte_conversion(field):
                field_name = field.field_name
                if field.type.size is not None:
                    self.trait_out("let mut %s_bytes = Vec::with_capacity(%s * %s.len());",
                                   field_name, field.type.size, field_name)
                else:
                    self.trait_out("let mut %s_bytes = Vec::new();", field_name)
                self.trait_out("for value in %s {", field_name)
                self.trait_out.indent("%s_bytes.extend(value.to_ne_bytes().iter());", field_name)
                self.trait_out("}")

            pad_count = []

            # Emit padding to align the request to the given alignment
            def _emit_padding_for_alignment(align):
                _emit_request()

                pad_count.append('')
                self.trait_out("let padding%s = &[0; %d][..(%d - (length_so_far %% %d)) %% %d];",
                               len(pad_count), align - 1, align, align, align)
                _emit_add_to_requests("&padding%s" % len(pad_count))

            # Get the length of all fixed-length parts of the request
            fixed_request_length, has_variable_length = 0, False
            for field in obj.fields:
                if field.wire:
                    if field.type.fixed_size():
                        fixed_request_length += field.type.get_total_size()
                    else:
                        has_variable_length = True

            # The XML does not describe trailing padding in requests. Everything
            # is implicitly padded to a four byte boundary. Variably sized
            # requests are handled below.
            if has_variable_length:
                trailing_padding = 0
            else:
                padded_length = (fixed_request_length + 3) // 4
                trailing_padding = padded_length * 4 - fixed_request_length
                fixed_request_length += trailing_padding

            # This list collects expression that describe the wire length of the
            # request when summed.
            request_length = [str(fixed_request_length)]

            # Collect the request length and emit some byte conversions
            for field in obj.fields:
                if not field.wire:
                    continue
                if field.type.nmemb is None:
                    size = field.type.size
                    if size is None:
                        # Variable length list with variable sized items:
                        # xproto::SetFontPath seems to be the only example
                        _emit_byte_conversion(field)
                        request_length.append("%s_bytes.len()" % field.field_name)
                    else:
                        # Variable length list with fixed sized items
                        request_length.append("%s * %s.len()" % (size, self._to_rust_variable(field.field_name)))
                elif field.type.size is None:
                    # Variable sized element. Only example seems to be RandR's
                    # SetMonitor request. MonitorInfo contains a list.
                    assert field.type.nmemb is not None
                    self.trait_out("let %s_bytes = %s.to_ne_bytes();", field.field_name, field.field_name)
                    request_length.append("%s_bytes.len()" % field.field_name)
                if hasattr(field, 'lenfield_for_switch'):
                    # This our special Aux-argument that represents a <switch>
                    self.trait_out("let %s = %s.value_mask();", field.field_name, field.lenfield_for_switch.field_name)
                    request_length.append("%s.wire_length()" % field.lenfield_for_switch.field_name)
            request_length = " + ".join(request_length)

            if has_variable_length:
                # This will cause the division by 4 to "round up". Code below
                # will actually add padding bytes.
                request_length += " + 3"

            # The "length" variable contains the request length in four byte
            # quantities. This rounds up to a multiple of four, because we
            self.trait_out("let length: usize = (%s) / 4;", request_length)

            # Now construct the actual request bytes
            for field in obj.fields:
                if not field.wire:
                    continue
                field_name = field.field_name
                rust_variable = self._to_rust_variable(field_name)
                if field_name == "major_opcode" or field_name == "minor_opcode":
                    # This is a special case for "magic" fields that we compute
                    if self.namespace.is_ext and field_name == "major_opcode":
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
                            other_field = [f for f in obj.fields if e.lenfield_name == f.field_name]
                            assert len(other_field) == 1
                            other_field = other_field[0]
                            self.trait_out("let %s: %s = %s.len().try_into()?;",
                                           other_field.field_name, self._to_rust_type(other_field.type.name),
                                           other_field.is_length_field_for.field_name)
                            return e.lenfield_name

                    # First compute the expression in its actual type, then
                    # convert it to bytes, then append the bytes to request
                    self.trait_out("let %s: %s = (%s).try_into().unwrap();", field_name,
                                   self._to_rust_type(field.type.name), expr_to_str_and_emit(field.type.expr))
                    self.trait_out("let %s_bytes = %s.to_ne_bytes();", field_name,
                                   rust_variable)
                    for i in range(field.type.size):
                        request.append("%s_bytes[%d]" % (field_name, i))
                elif field.type.is_pad:
                    if field.type.fixed_size():
                        # Padding of a fixed size, we simply emit zero bytes
                        assert field.type.size == 1
                        for i in range(field.type.nmemb):
                            request.append("0")
                    else:
                        # Padding of a variable length, we have to get the
                        # length so far and add padding
                        assert field.type.size == 1 and field.type.nmemb == 1
                        _emit_padding_for_alignment(4)
                elif field.type.is_list:
                    # We faked the type of SendEvent's event argument before,
                    # now we have to convert it for the following code
                    if name == ('xcb', 'SendEvent') and field_name == 'event':
                        self.trait_out("let %s = %s.into();", field_name, field_name)
                        self.trait_out("let %s = &%s;", field_name, field_name)

                    # Lists without a "simple" length field or with a fixed size
                    # could be passed incorrectly be the caller; check this.
                    if not (hasattr(field, "has_length_field") or field.type.fixed_size()):
                        self.trait_out("assert_eq!(%s.len(), %s, \"Argument %s has an incorrect length\");",
                                       rust_variable, self.expr_to_str(field.type.expr, "usize"), field_name)
                    if field.type.size == 1:
                        # This list has u8 entries, we can avoid a copy
                        _emit_request()
                        _emit_add_to_requests(rust_variable)
                    else:
                        if field.type.size is not None:
                            _emit_byte_conversion(field)
                        # else: Already called _emit_byte_conversion() above

                        _emit_request()
                        _emit_add_to_requests("&%s_bytes" % field_name)
                elif field.wire:
                    if hasattr(field, "is_length_field_for"):
                        # This is a length field for some list, get the length.
                        # (Needed because this is not a function argument)
                        self.trait_out("let %s: %s = %s.len().try_into()?;", rust_variable,
                                       self._to_rust_type(field.type.name),
                                       self._to_rust_variable(field.is_length_field_for.field_name))
                    if field.enum is not None:
                        # This is a generic parameter, call Into::into
                        self.trait_out("let %s = %s.into();", field_name, field_name)
                    field_bytes = self._to_rust_variable(field_name + "_bytes")
                    if field.type.name == ('float',):
                        # FIXME: Switch to a trait that we can implement on f32
                        self.trait_out("let %s = %s.to_bits().to_ne_bytes();", field_bytes, rust_variable)
                    elif field.type.size is not None:  # Size None was already handled above
                        if (name == ('xcb', 'InternAtom') and field_name == 'only_if_exists'):
                            # We faked a bool argument, convert to u8
                            self.trait_out("let %s = %s as %s;", field_name, field_name, self._to_rust_type(field.type.name))
                        if field_name == "length":
                            # This is the request length which we explicitly
                            # calculated above. If it does not fit into u16,
                            # Connection::compute_length_field() fixes this.
                            source = "TryInto::<%s>::try_into(length).unwrap_or(0)" % self._to_rust_type(field.type.name)
                        else:
                            source = rust_variable
                        self.trait_out("let %s = %s.to_ne_bytes();", field_bytes, source)
                    if field.type.is_switch or field.type.size is None:
                        # We have a byte array that we can directly send
                        _emit_request()
                        _emit_add_to_requests("&" + field_bytes)
                    else:
                        # Copy the bytes to the request array
                        for i in range(field.type.size):
                            request.append("%s[%d]" % (field_bytes, i))

            if trailing_padding:
                request.append("0 /* trailing padding */")
                request.extend(["0"] * (trailing_padding - 1))

            # Emit everything that is still in 'request'
            _emit_request()
            assert not request

            if has_variable_length:
                # Add zero bytes so that the total length is a multiple of 4
                _emit_padding_for_alignment(4)

            # Emit an assert that checks that the sum of all the byte arrays in
            # 'requests' is the same as our computed length field
            self.trait_out("assert_eq!(length_so_far, length * 4);")

            # Now we actually send the request

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
        """Emit the code for parsing the given fields. After this code runs, the
        fields will be available as rust variables. The list of variable names
        is returned."""
        parts = []
        for field in fields:
            field_name = self._to_rust_variable(field.field_name)
            rust_type = self._to_rust_type(field.type.name)
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
                    self.out("let (%s_%s, new_remaining) = %s::try_parse(remaining)?;",
                             field.field_name, i, self._to_rust_type(field.type.name))
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
                self.out("let (%s, new_remaining) = %s::try_parse(remaining)?;",
                         field_name, self._to_rust_identifier(rust_type))
                self.out("remaining = new_remaining;")
                if not hasattr(field, 'is_length_field_for'):
                    parts.append(self._to_rust_variable(field.field_name))

        return parts

    def complex_type(self, complex, name, extra_name, impl_try_parse, name_transform=lambda x: x):
        """Emit a complex type as a struct. This also adds some parsing code and
        a to_ne_bytes() implementation."""

        mark_length_fields(complex)

        has_fds = any(field.isfd for field in complex.fields)
        assert not (impl_try_parse and has_fds)
        if is_clone(complex):
            if is_copy(complex):
                self.out("#[derive(Debug, Clone, Copy)]")
            else:
                self.out("#[derive(Debug, Clone)]")
        else:
            self.out("#[derive(Debug)]")

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
        # Collect all the fields that appear "out of thin air": They are
        # referenced, but not on the wire.
        unresolved = [field for field in referenced if field not in wire_fields]

        if impl_try_parse and not unresolved:
            method = "fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>"
            self.out("impl TryParse for %s%s {", name_transform(self._name(name)), extra_name)
        else:
            if has_fds:
                method = "fn try_parse_fd<'a>(value: &'a [u8], fds: &mut Vec<RawFdContainer>)"
                method += " -> Result<(Self, &'a [u8]), ParseError>"
            elif impl_try_parse:
                # Turn missing values into extra arguments
                assert unresolved
                unresolved_args, extra_args = [], []
                for field in complex.fields:
                    if field.field_name in unresolved:
                        field_type = self._to_complex_rust_type(field, None, '')
                        unresolved_args.append("%s: %s" % (field.field_name, field_type))
                        extra_args.append(field.field_name)
                method = "pub fn try_parse(value: &[u8], %s) -> Result<(Self, &[u8]), ParseError>"
                method = method % ", ".join(unresolved_args)

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
        # CreateAlarm has both 32 bit and 64 bit numbers. Now we assert that all
        # sizes are a multiple of the smallest size.
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
                self.out("pub fn %s<I>(mut self, value: I) -> Self where I: Into<RustOption<%s>> {",
                         aux_name, self._to_rust_type(field.type.name))
                with Indent(self.out):
                    self.out("self.%s = value.into();", aux_name)
                    self.out("self")
                self.out("}")

        self.out("}")

    def _name(self, name):
        """Get the name as a string. xcbgen represents names as tuples like
        ('xcb', 'RandR', 'MonitorInfo'). This function produces MonitorInfo."""
        orig_name = name
        if name[0] == 'xcb':
            name = name[1:]
        if self.namespace.is_ext and name[0] == self.namespace.ext_name:
            name = name[1:]
        assert len(name) == 1, orig_name
        return name[0]

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

    def _to_rust_type(self, name):
        """Convert an xcbgen name tuple to a rust type."""
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
