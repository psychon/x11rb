"""
This module generates the code for parsing a GenericEvent or GenericError into a
specific instance of the right event/error.
"""

from .output import Indent


def _get_errors_from_module(module):
    """Get errors from this module"""
    prefix = module.outer_module.namespace.prefix
    errors = module.outer_module.errors.values()
    errors = filter(lambda x: x[0][:len(prefix)] == prefix, errors)
    # The XML for GLX has a comment saying "fake number"
    errors = filter(lambda x: x[0] != ('xcb', 'Glx', 'Generic'), errors)
    return sorted(errors)


def _errors(out, modules):
    errors = [_get_errors_from_module(module) for module in modules]
    opcodes = [[int(error.opcodes[name]) for name, error in mod_errors] for mod_errors in errors]
    xproto_index = next(filter(lambda m: not m[1].namespace.is_ext, enumerate(modules)))[0]

    out("/// Enumeration of all possible X11 errors.")
    out("#[derive(Debug, Clone)]")
    out("pub enum Error<B: std::fmt::Debug + AsRef<[u8]>> {")
    out.indent("Unknown(GenericError<B>),")
    for module, mod_errors in zip(modules, errors):
        mod_name = module.namespace.header
        variant = mod_name[0].upper() + mod_name[1:]
        for name, error in mod_errors:
            err_name = name[-1] + "Error"
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
            out("// Find the extension that this error could belong to")
            out("let error_code = error.error_code();")
            out("let bytes = error.raw_bytes();")

            out("// Check if this is a core protocol error")
            out("match error_code {")
            for (name, err), opcode in zip(errors[xproto_index], opcodes[xproto_index]):
                err_name = name[-1] + "Error"
                out.indent("%s => return Ok(Self::Xproto%s(xproto::%s::try_parse(bytes)?.0)),",
                           opcode, err_name, err_name)
            out.indent("_ => {}")
            out("}")

            out("let ext_info = iter")
            out.indent(".map(|(name, reply)| (name, reply.first_error))")
            out.indent(".filter(|&(_, first_error)| first_error <= error_code)")
            out.indent(".max_by_key(|&(_, first_error)| first_error);")
            out("match ext_info {")
            with Indent(out):
                for module, mod_errors, mod_opcodes in zip(modules, errors, opcodes):
                    if not mod_errors or not module.namespace.is_ext:
                        continue
                    mod_name = module.namespace.header
                    ext_xname = module.namespace.ext_xname
                    variant = mod_name[0].upper() + mod_name[1:]
                    out("Some((\"%s\", first_error)) => {", ext_xname)
                    with Indent(out):
                        out("match error_code - first_error {")
                        for (name, error), opcode in zip(mod_errors, mod_opcodes):
                            err_name = name[-1] + "Error"
                            out.indent("%s => Ok(Self::%s%s(%s::%s::try_parse(bytes)?.0)),",
                                       opcode, variant, err_name, mod_name, err_name)
                        out.indent("_ => Ok(Self::Unknown(error))")
                        out("}")
                    out("}")
                out("_ => Ok(Self::Unknown(error))")
            out("}")
        out("}")
    out("}")


def generate(out, modules):
    _errors(out, modules)
