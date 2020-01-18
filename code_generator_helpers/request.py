"""
This module contains the code for generating the function for sending a request.
"""

import string
import code_generator_helpers.module
from .output import Indent


def handle_switches(module, obj, name, function_name):
    """Handle <switch> tags by generating the *Aux structure"""
    switches = list(filter(lambda field: field.type.is_switch, obj.fields))
    assert len(switches) <= 1
    if not switches:
        return None

    aux_name = "%sAux" % module._name(name)
    switch = switches[0]

    # Find the mask field for the switch
    lenfield_name = switch.type.expr.lenfield_name
    mask_field = list(filter(lambda field: field.field_name == lenfield_name, obj.fields))
    assert len(mask_field) == 1
    mask_field = mask_field[0]

    # Hide it from the API and "connect" it to the switch
    mask_field.visible = False
    mask_field.lenfield_for_switch = switch

    module._generate_aux(aux_name, obj, switch, mask_field, function_name)
    return aux_name


def collect_function_arguments(module, obj, name, aux_name):
    # Iterator of letters, used for naming generic parameters
    letters = iter(string.ascii_uppercase)

    generics = []
    args = ["overwritten-later"]
    arg_names = ["self"]
    where = []
    fds, fds_is_list = [], False
    for field in obj.fields:
        if field.visible:
            rust_type = module._to_complex_rust_type(field, aux_name, '&')

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

            args.append("%s: %s" % (module._to_rust_variable(field.field_name), rust_type))
            arg_names.append(module._to_rust_variable(field.field_name))
            if field.isfd:
                if field.type.is_list:
                    fds_is_list = True
                    fds.append(module._to_rust_variable(field.field_name))
                else:
                    fds.append("%s.into()" % module._to_rust_variable(field.field_name))

    return args, arg_names, generics, where, fds, fds_is_list


def pick_return_type(module, obj, name, need_lifetime):
    if name == ('xcb', 'ListFontsWithInfo'):
        assert need_lifetime
        result_type_trait = "ListFontsWithInfoCookie<'c, Self>"
        result_type_func = "ListFontsWithInfoCookie<'c, Conn>"
    elif obj.reply:
        if any(field.isfd for field in obj.reply.fields):
            cookie = "CookieWithFds"
        else:
            cookie = "Cookie"
        if need_lifetime:
            result_type_trait = "%s<'c, Self, %sReply>" % (cookie, module._name(name))
            result_type_func = "%s<'c, Conn, %sReply>" % (cookie, module._name(name))
        else:
            result_type_trait = "%s<Self, %sReply>" % (cookie, module._name(name))
            result_type_func = "%s<Conn, %sReply>" % (cookie, module._name(name))
    else:
        if need_lifetime:
            result_type_trait = "VoidCookie<'c, Self>"
            result_type_func = "VoidCookie<'c, Conn>"
        else:
            result_type_trait = "VoidCookie<Self>"
            result_type_func = "VoidCookie<Conn>"
    return result_type_trait, result_type_func


def request_implementation(module, obj, name, fds, fds_is_list):
    """Generate the code that actually serialises all the arguments."""
    if module.namespace.is_ext:
        module.out('let extension_information = conn.extension_information("%s")' % module.namespace.ext_xname)
        module.out.indent(".ok_or(ConnectionError::UnsupportedExtension)?;")

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
        module.out("let length_so_far = %s(%s).len();",
                   add, part)
        requests.append(part)

    # This function adds the current 'request' to the list of 'requests'
    def _emit_request():
        if not request:
            return

        module.out("let request%d = [", len(requests))
        for byte in request:
            module.out.indent("%s,", byte)
        module.out("];")
        _emit_add_to_requests("&request%d" % len(requests))
        del request[:]

    pad_count = []

    # Emit padding to align the request to the given alignment
    def _emit_padding_for_alignment(align):
        _emit_request()

        pad_count.append('')
        module.out("let padding%s = &[0; %d][..(%d - (length_so_far %% %d)) %% %d];",
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
        rust_variable = module._to_rust_variable(field.field_name)
        if field.type.is_switch:
            field_bytes = module._to_rust_variable(field.field_name + "_bytes")
            module.out("let %s = %s.serialize();", field_bytes, rust_variable)
            request_length.append("%s.len()" % field_bytes)
        elif field.type.nmemb is None:
            size = field.type.size
            if size is None:
                # Variable length list with variable sized items:
                # xproto::SetFontPath seems to be the only example
                module.out("let %s_bytes = %s.serialize();",
                           rust_variable, rust_variable)
                request_length.append("%s_bytes.len()" % rust_variable)
            else:
                # Variable length list with fixed sized items
                request_length.append("%s * %s.len()" % (size, rust_variable))
        elif field.type.size is None:
            # Variable sized element. Only example seems to be RandR's
            # SetMonitor request. MonitorInfo contains a list.
            assert field.type.nmemb is not None
            module.out("let %s_bytes = %s.serialize();", field.field_name, field.field_name)
            request_length.append("%s_bytes.len()" % field.field_name)
        if hasattr(field, 'lenfield_for_switch'):
            # This our special Aux-argument that represents a <switch>
            if field.lenfield_for_switch.type.bitcases[0].type.is_bitcase:
                module.out("let %s = %s.value_mask();", rust_variable,
                           field.lenfield_for_switch.field_name)
            else:
                module.out("let %s = %s.%s();", rust_variable,
                           field.lenfield_for_switch.field_name, field.field_name)
    request_length = " + ".join(request_length)

    if has_variable_length:
        # This will cause the division by 4 to "round up". Code below
        # will actually add padding bytes.
        request_length += " + 3"

    # The "length" variable contains the request length in four byte
    # quantities. This rounds up to a multiple of four, because we
    module.out("let length: usize = (%s) / 4;", request_length)

    # Now construct the actual request bytes
    for field in obj.fields:
        if not field.wire:
            continue
        field_name = field.field_name
        rust_variable = module._to_rust_variable(field_name)
        if field_name == "major_opcode" or field_name == "minor_opcode":
            # This is a special case for "magic" fields that we compute
            if module.namespace.is_ext and field_name == "major_opcode":
                request.append('extension_information.major_opcode')
            else:
                request.append("%s_REQUEST" % module._upper_snake_name(name))
        elif field.type.is_expr:
            assert name == ('xcb', 'QueryTextExtents')
            # There is an expression referring to the string_len argument in the
            # XML. However, this argument is just the length of the string
            # argument and thus not actually visible in our code. Thus, get the
            # variable that is needed for the expression.
            module.out("let string_len: u32 = string.len().try_into()?;")

            # First compute the expression in its actual type, then
            # convert it to bytes, then append the bytes to request
            module.out("// The following unwrap cannot fail since the value")
            module.out("// is either 0 or 1. Both fit into an u8.")
            module.out("let %s: %s = (%s).try_into().unwrap();", field_name,
                       module._field_type(field), module.expr_to_str(field.type.expr, None))
            module.out("let %s_bytes = %s.serialize();", field_name, rust_variable)
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
                module.out("let %s = %s.into();", field_name, field_name)
                module.out("let %s = &%s;", field_name, field_name)

            # Lists without a "simple" length field or with a fixed size
            # could be passed incorrectly be the caller; check this.
            if not (hasattr(field, "has_length_field") or field.type.fixed_size()):
                module.out("assert_eq!(%s.len(), %s, \"Argument %s has an incorrect length\");",
                           rust_variable, module.expr_to_str(field.type.expr, "usize"), field_name)
            if field.type.size == 1:
                # This list has u8 entries, we can avoid a copy
                _emit_request()
                _emit_add_to_requests(rust_variable)
            else:
                if field.type.size is not None:
                    if field.type.size <= 32:
                        module.out("let %s_bytes = %s.serialize();",
                                   rust_variable, rust_variable)
                    else:
                        # I am not happy about the trait std::array::LengthAtMost32
                        module.out("let mut %s_bytes = Vec::new();",
                                   rust_variable)
                        module.out("for value in %s {", rust_variable)
                        module.out.indent("%s_bytes.extend(value.serialize().iter());",
                                          rust_variable)
                        module.out("}")
                # else: Already called .serialize() on the list above

                _emit_request()
                _emit_add_to_requests("&%s_bytes" % rust_variable)
        elif field.wire:
            if hasattr(field, "is_length_field_for"):
                # This is a length field for some list, get the length.
                # (Needed because this is not a function argument)
                module.out("let %s: %s = %s.len().try_into()?;", rust_variable,
                           module._field_type(field),
                           module._to_rust_variable(field.is_length_field_for.field_name))
            if field.enum is not None:
                # This is a generic parameter, call Into::into
                module.out("let %s = %s.into();", rust_variable, rust_variable)
            field_bytes = module._to_rust_variable(field_name + "_bytes")
            if field.type.is_switch:
                # The previous loop for calculating the request length
                # already called serialize() to get the bytes.
                pass
            elif field.type.size is not None:  # Size None was already handled above
                if field_name == "length":
                    # This is the request length which we explicitly
                    # calculated above. If it does not fit into u16,
                    # Connection::compute_length_field() fixes this.
                    source = "TryInto::<%s>::try_into(length).unwrap_or(0)" % module._field_type(field)
                else:
                    source = rust_variable
                if code_generator_helpers.module.is_bool(field.type) or \
                        (name == ('xcb', 'InternAtom') and field_name == 'only_if_exists'):
                    module.out("let %s = (%s as u8).serialize();", field_bytes, source)
                else:
                    module.out("let %s = %s.serialize();", field_bytes, source)
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
    module.out("assert_eq!(length_so_far, length * 4);")

    # Now we actually send the request

    slices = ", ".join(["IoSlice::new(%s)" % r for r in requests])

    if fds:
        if not fds_is_list:
            module.out("let fds = vec!(%s);", ", ".join(fds))
            fds = "fds"
        else:
            # There may (currently) be only a single list of FDs
            fds, = fds
    else:
        fds = "Vec::new()"

    if name == ('xcb', 'ListFontsWithInfo'):
        assert obj.reply
        module.out("Ok(ListFontsWithInfoCookie::new(conn.send_request_with_reply(&[%s], %s)?))", slices, fds)
    elif obj.reply:
        if any(field.isfd for field in obj.reply.fields):
            module.out("Ok(conn.send_request_with_reply_with_fds(&[%s], %s)?)", slices, fds)
        else:
            module.out("Ok(conn.send_request_with_reply(&[%s], %s)?)", slices, fds)
    else:
        module.out("Ok(conn.send_request_without_reply(&[%s], %s)?)", slices, fds)


def generate_request_code(module, obj, name, function_name):
    # Does this have a <switch>? If so, we generate an *Aux structure
    aux_name = handle_switches(module, obj, name, function_name)

    # Now start constructing the function prototype of the function for
    # sending requests.

    # We need a lifetime if there are more than one references in the
    # arguments. Right now that means: Any lists? An aux argument?
    need_lifetime = aux_name or any(field.type.is_list for field in obj.fields)
    if name == ('xcb', 'SendEvent'):
        # We do "magic" below to replace the &[u8; 32] argument with Into<[u8;32]>
        need_lifetime = False

    # Now collect the arguments of the function.
    # 'args' contains the actual parameters (name and type)
    # 'arg_names' contains the names of the parameters
    # 'generics' contains the names of the generic arguments that are needed
    # 'where' collects "where" conditions that need to be added to the prototype
    # 'fds' will collect all file # descriptors in the request.
    # 'fds_is_list' indicates a list of FDs.
    args, arg_names, generics, where, fds, fds_is_list = collect_function_arguments(module, obj, name, aux_name)

    # Figure out the return type of the request function
    result_type_trait, result_type_func = pick_return_type(module, obj, name, need_lifetime)

    # Finally: Actually emit the function prototype.

    # First for the trait, this just calls the global function
    if need_lifetime:
        args[0] = "&'c self"
        prefix_generics = ["'c"]
    else:
        args[0] = "&self"
        prefix_generics = []
    if prefix_generics or generics:
        generics_str = "<%s>" % ", ".join(prefix_generics + generics)
    else:
        generics_str = ""

    code_generator_helpers.module.emit_doc(module.trait_out, obj.doc)
    module.trait_out("fn %s%s(%s) -> Result<%s, ConnectionError>", function_name,
                     generics_str, ", ".join(args), result_type_trait)
    if where:
        module.trait_out("where %s", ", ".join(where))
    module.trait_out("{")
    if function_name in arg_names:
        module.trait_out.indent("self::%s(%s)", function_name, ", ".join(arg_names))
    else:
        module.trait_out.indent("%s(%s)", function_name, ", ".join(arg_names))
    module.trait_out("}")
    module.trait_out("")

    # Then emit the global function that actually does all the work
    if need_lifetime:
        args[0] = "conn: &'c Conn"
        prefix_generics = ["'c", "Conn"]
    else:
        args[0] = "conn: &Conn"
        prefix_generics = ["Conn"]
    generics_str = "<%s>" % ", ".join(prefix_generics + generics)
    code_generator_helpers.module.emit_doc(module.out, obj.doc)
    module.out("pub fn %s%s(%s) -> Result<%s, ConnectionError>", function_name, generics_str, ", ".join(args), result_type_func)
    module.out("where %s", ", ".join(['Conn: RequestConnection + ?Sized'] + where))
    module.out("{")
    with Indent(module.out):
        request_implementation(module, obj, name, fds, fds_is_list)
    module.out("}")

    if obj.reply:
        code_generator_helpers.module.emit_doc(module.out, obj.reply.doc)
        module.complex_type(obj.reply, module._name(name) + 'Reply', False, [])

    module.out("")
