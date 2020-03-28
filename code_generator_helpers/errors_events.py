"""
This module generates the code for parsing a GenericEvent or GenericError into a
specific instance of the right event/error.
"""

from .output import Indent
from . import camel_case_to_upper_snake


def _get_errors_from_module(module):
    """Get errors from this module"""
    prefix = module.outer_module.namespace.prefix
    errors = module.outer_module.errors.values()
    errors = filter(lambda x: x[0][:len(prefix)] == prefix, errors)
    # The XML for GLX has a comment saying "fake number"
    errors = filter(lambda x: x[0] != ('xcb', 'Glx', 'Generic'), errors)
    return sorted(errors)


def _get_events_from_module(module):
    """Get events from this module"""
    prefix = module.outer_module.namespace.prefix
    events = module.outer_module.events.values()
    events = filter(lambda x: x[0][:len(prefix)] == prefix, events)
    return sorted(events)


def _errors(out, modules):
    errors = [_get_errors_from_module(module) for module in modules]
    xproto_index = next(iter(filter(lambda m: not m[1].namespace.is_ext, enumerate(modules))))[0]

    out("/// Enumeration of all possible X11 errors.")
    out("#[derive(Debug, Clone)]")
    out("pub enum Error<B: std::fmt::Debug + AsRef<[u8]>> {")
    out.indent("Unknown(GenericError<B>),")
    for module, mod_errors in zip(modules, errors):
        mod_name = module.namespace.header
        variant = mod_name[0].upper() + mod_name[1:]
        for name, error in mod_errors:
            err_name = name[-1] + "Error"
            if module.has_feature:
                out.indent("#[cfg(feature = \"%s\")]", module.namespace.header)
            out.indent("%s%s(%s::%s),", variant, err_name, mod_name, err_name)
    out("}")

    out("impl<B: std::fmt::Debug + AsRef<[u8]>> Error<B> {")
    with Indent(out):
        out("/// Parse a generic X11 error into a concrete error type.")
        out("pub fn parse(")
        out.indent("error: GenericError<B>,")
        out.indent("iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,")
        out(") -> Result<Self, ParseError> {")
        with Indent(out):
            out("let error_code = error.error_code();")

            out("// Check if this is a core protocol error")
            out("match error_code {")
            for name, err in errors[xproto_index]:
                opcode = camel_case_to_upper_snake(name[-1]) + "_ERROR"
                err_name = name[-1] + "Error"
                out.indent("xproto::%s => return Ok(Self::Xproto%s(error.into())),",
                           opcode, err_name)
            out.indent("_ => {}")
            out("}")

            out("// Find the extension that this error could belong to")
            out("let ext_info = iter")
            out.indent(".map(|(name, reply)| (name, reply.first_error))")
            out.indent(".filter(|&(_, first_error)| first_error <= error_code)")
            out.indent(".max_by_key(|&(_, first_error)| first_error);")
            out("match ext_info {")
            with Indent(out):
                for module, mod_errors in zip(modules, errors):
                    if not mod_errors or not module.namespace.is_ext:
                        continue
                    mod_name = module.namespace.header
                    ext_xname = module.namespace.ext_xname
                    variant = mod_name[0].upper() + mod_name[1:]
                    if module.has_feature:
                        out("#[cfg(feature = \"%s\")]", module.namespace.header)
                    out("Some((\"%s\", first_error)) => {", ext_xname)
                    with Indent(out):
                        out("match error_code - first_error {")
                        for name, error in mod_errors:
                            opcode = camel_case_to_upper_snake(name[-1]) + "_ERROR"
                            err_name = name[-1] + "Error"
                            out.indent("%s::%s => Ok(Self::%s%s(error.into())),",
                                       mod_name, opcode, variant, err_name)
                        out.indent("_ => Ok(Self::Unknown(error))")
                        out("}")
                    out("}")
                out("_ => Ok(Self::Unknown(error))")
            out("}")
        out("}")
    out("}")


def _events(out, modules):
    events = [_get_events_from_module(module) for module in modules]
    xproto_index = next(iter(filter(lambda m: not m[1].namespace.is_ext, enumerate(modules))))[0]

    out("/// Enumeration of all possible X11 events.")
    out("#[derive(Debug, Clone)]")
    out("pub enum Event<B: std::fmt::Debug + AsRef<[u8]>> {")
    out.indent("Unknown(GenericEvent<B>),")
    out.indent("Error(Error<B>),")
    for module, mod_events in zip(modules, events):
        mod_name = module.namespace.header
        variant = mod_name[0].upper() + mod_name[1:]
        for name, event in mod_events:
            err_name = name[-1] + "Event"
            if module.has_feature:
                out.indent("#[cfg(feature = \"%s\")]", module.namespace.header)
            out.indent("%s%s(%s::%s),", variant, err_name, mod_name, err_name)
    out("}")

    out("impl<B: std::fmt::Debug + AsRef<[u8]>> Event<B> {")
    with Indent(out):
        out("/// Parse a generic X11 event into a concrete event type.")
        out("pub fn parse(")
        out.indent("event: GenericEvent<B>,")
        out.indent("iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,")
        out(") -> Result<Self, ParseError> {")
        with Indent(out):
            out("let event_type = event.response_type();")
            out("// Check if this is a core protocol error or from the generic event extension")
            out("match event_type {")
            with Indent(out):
                out("0 => return Ok(Self::Error(Error::parse(event.try_into()?, iter)?)),")
                for name, event in events[xproto_index]:
                    if name == ('xcb', 'GeGeneric'):
                        # This does not really count and is parsed as an extension's event
                        continue
                    opcode = camel_case_to_upper_snake(name[-1]) + "_EVENT"
                    event_name = name[-1] + "Event"
                    out("xproto::%s => return Ok(Self::Xproto%s(event.into())),",
                        opcode, event_name)
                out("xproto::GE_GENERIC_EVENT => return Self::from_generic_event(event, iter),")
                out("_ => {}")
            out("}")

            out("// Find the extension that this event could belong to")
            out("let ext_info = iter")
            out.indent(".map(|(name, reply)| (name, reply.first_event))")
            out.indent(".filter(|&(_, first_event)| first_event <= event_type)")
            out.indent(".max_by_key(|&(_, first_event)| first_event);")
            out("match ext_info {")
            with Indent(out):
                for module, mod_events in zip(modules, events):
                    # XGE events are handled separately
                    has_normal_events = any(not e[1].is_ge_event for e in mod_events)
                    if not has_normal_events or not module.namespace.is_ext:
                        continue
                    mod_name = module.namespace.header
                    ext_xname = module.namespace.ext_xname
                    variant = mod_name[0].upper() + mod_name[1:]
                    if module.has_feature:
                        out("#[cfg(feature = \"%s\")]", module.namespace.header)
                    out("Some((\"%s\", first_event)) => {", ext_xname)
                    with Indent(out):
                        if module.namespace.header == 'xkb':
                            out("if event_type != first_event {")
                            out.indent("return Ok(Self::Unknown(event));")
                            out("}")
                            out("match event.raw_bytes()[1] {")
                        else:
                            out("match event_type - first_event {")
                        for name, event in mod_events:
                            if event.is_ge_event:
                                continue
                            opcode = camel_case_to_upper_snake(name[-1]) + "_EVENT"
                            event_name = name[-1] + "Event"
                            out.indent("%s::%s => Ok(Self::%s%s(event.into())),",
                                       mod_name, opcode, variant, event_name)
                        out.indent("_ => Ok(Self::Unknown(event))")
                        out("}")
                    out("}")
                out("_ => Ok(Self::Unknown(event))")
            out("}")
        out("}")
        out("")

        out("fn from_generic_event(")
        out.indent("event: GenericEvent<B>,")
        out.indent("iter: impl Iterator<Item=(&'static str, QueryExtensionReply)>,")
        out(") -> Result<Self, ParseError> {")
        with Indent(out):
            out("let bytes = event.raw_bytes();")
            out("let ge_event = xproto::GeGenericEvent::try_from(bytes)?;")
            out("#[allow(unused_variables)]")
            out("let (extension, event_type) = (ge_event.extension, ge_event.event_type);")
            out("let ext_name = iter")
            out.indent(".map(|(name, reply)| (name, reply.major_opcode))")
            out.indent(".find(|&(_, opcode)| extension == opcode)")
            out.indent(".map(|(name, _)| name);")
            out("match ext_name {")
            with Indent(out):
                for module, mod_events in zip(modules, events):
                    has_xge_events = any(e[1].is_ge_event for e in mod_events)
                    if not has_xge_events or not module.namespace.is_ext:
                        continue
                    mod_name = module.namespace.header
                    ext_xname = module.namespace.ext_xname
                    variant = mod_name[0].upper() + mod_name[1:]
                    if module.has_feature:
                        out("#[cfg(feature = \"%s\")]", module.namespace.header)
                    out("Some(\"%s\") => {", ext_xname)
                    with Indent(out):
                        out("match event_type {")
                        for name, event in mod_events:
                            if not event.is_ge_event:
                                continue
                            opcode = camel_case_to_upper_snake(name[-1]) + "_EVENT"
                            event_name = name[-1] + "Event"
                            out.indent("%s::%s => Ok(Self::%s%s(event.try_into()?)),",
                                       mod_name, opcode, variant, event_name)
                        out.indent("_ => Ok(Self::Unknown(event))")
                        out("}")
                    out("}")
                out("_ => Ok(Self::Unknown(event))")
            out("}")
        out("}")
    out("}")


def generate(out, modules):
    _errors(out, modules)
    _events(out, modules)
