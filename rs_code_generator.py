#!/usr/bin/env python

import getopt
import sys
import glob
import os
import code_generator_helpers.output as output_helper
import code_generator_helpers.module as module_helper
import code_generator_helpers.errors_events as errors_events

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
        output_dir = arg

if args:
    print('No further arguments expected')
    sys.exit(1)

main_output_file = output_helper.Output()
output_helper.generated_code_header(main_output_file)
main_output_file("use std::convert::TryInto;")
main_output_file("use crate::errors::ParseError;")
main_output_file("use crate::x11_utils::{Event as _, GenericError, GenericEvent};")
main_output_file("use xproto::QueryExtensionReply;")


# Now the real fun begins

current_module = None
all_modules = []


def add_main_output():
    main_output_file("")
    errors_events.generate(main_output_file, all_modules)


def rs_open(self):
    global current_module
    assert current_module is None
    current_module = module_helper.Module(self)
    all_modules.append(current_module)


def rs_close(self):
    global current_module
    current_module.close(self)
    output_file = os.path.join(output_dir, "%s.rs" % self.namespace.header)
    current_module.out.write_file(output_file)

    WITHOUT_FEATURE = ["bigreq", "ge", "xc_misc", "xproto"]
    current_module.has_feature = self.namespace.header not in WITHOUT_FEATURE
    if current_module.has_feature:
        main_output_file("#[cfg(feature = \"%s\")]", self.namespace.header)
    main_output_file("pub mod %s;", self.namespace.header)

    current_module = None


def rs_enum(self, name):
    current_module.enum(self, name)


def rs_simple(self, name):
    current_module.simple(self, name)


def rs_struct(self, name):
    current_module.struct(self, name)


def rs_union(self, name):
    current_module.union(self, name)


def rs_request(self, name):
    current_module.request(self, name)


def rs_eventstruct(self, name):
    current_module.eventstruct(self, name)


def rs_event(self, name):
    current_module.event(self, name)


def rs_error(self, name):
    current_module.error(self, name)


# We must create an "output" dictionary before any xcbgen imports
output = {'open':        rs_open,
          'close':       rs_close,
          'simple':      rs_simple,
          'enum':        rs_enum,
          'struct':      rs_struct,
          'union':       rs_union,
          'request':     rs_request,
          'eventstruct': rs_eventstruct,
          'event':       rs_event,
          'error':       rs_error,
          }

from xcbgen.state import Module  # noqa

names = glob.glob(input_dir + "/*.xml")
for name in sorted(names):
    module = Module(name, None)
    module.register()
    module.resolve()
    try:
        module.generate()
    except Exception:
        sys.stderr.write('Error occurred while generating: %s\n' % module.namespace.header)
        raise

add_main_output()

output_file = os.path.join(output_dir, "mod.rs")
main_output_file.write_file(output_file)
