"""
This module handles some special cases where we emit code that is not based on
the XML, but hand-written. Thus, this handles special cases.
"""

def skip_request(out, trait_out, obj, name, function_name):
    # We use this hook to also rewrite some parameter types.
    if name == ('xcb', 'InternAtom'):
        for field in obj.fields:
            if field.field_name == 'only_if_exists':
                from xcbgen.xtypes import SimpleType
                field.type = SimpleType(('uint8_t',), 1)
                field.type.xml_type = 'BOOL'

    # This request has a <switch> inside of another <switch>. Supporting
    # this requires moving <switch>-handling to complex_type(), I guess.
    if name == ('xcb', 'xkb', 'GetKbdByName'):
        trait_out("fn %s(&self) { unimplemented!(\"Not yet supported by the code generator\") }", function_name)
        out("pub fn %s<Conn>(_conn: &Conn) where Conn: RequestConnection + ?Sized {", function_name)
        out("unimplemented!(\"Not yet supported by the code generator\") }")
        return True
    return False
