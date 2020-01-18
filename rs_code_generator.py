#!/usr/bin/env python

import getopt
import sys
import glob
import os
import code_generator_helpers.output as output_helper
import code_generator_helpers.module as module_helper

try:
    opts, args = getopt.getopt(sys.argv[1:], "p:o:i:")
except getopt.GetoptError as err:
    print(err)
    print('Usage: %s [-p path] -i input -o output main_module' % (sys.argv[0]))
    sys.exit(1)

for (opt, arg) in opts:
    if opt == '-p':
        sys.path.insert(1, arg)
    if opt == '-i':
        input_dir = arg
    if opt == '-o':
        output_dir = arg

if not args:
    print("Missing name for main module")
    sys.exit()
main_module = args.pop()
if args:
    print('No further arguments expected')
    sys.exit(1)

main_output_file = output_helper.Output()


# Now the real fun begins

current_module = None


def rs_open(self):
    global current_module
    assert current_module is None
    current_module = module_helper.Module(self)


def rs_close(self):
    global current_module
    current_module.close(self)
    output_file = os.path.join(output_dir, "%s.rs" % self.namespace.header)
    current_module.out.write_file(output_file)
    current_module = None

    main_output_file("pub mod %s;", self.namespace.header)


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
unsupported = [
        "xinput.xml",    # InputInfo has a <switch name="info"> that causes problems
        ]
names = [name for name in names if os.path.basename(name) not in unsupported]

for name in names:
    module = Module(name, None)
    module.register()
    module.resolve()
    try:
        module.generate()
    except Exception:
        sys.stderr.write('Error occurred while generating: %s\n' % module.namespace.header)
        raise

output_file = os.path.join(output_dir, "%s.rs" % main_module)
main_output_file.write_file(output_file)
