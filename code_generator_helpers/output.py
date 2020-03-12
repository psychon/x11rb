class Output(object):
    """
    Delayed code generation output.

    This class manages things like indentation for you. Only after everything is
    done is the output actually written to a file.
    """

    indent_str = "    "

    def __init__(self):
        self._lines = []
        self._indent_level = 0

    def _out(self, extra, fmt, *args):
        indent = "".join([self.indent_str] * (self._indent_level + extra))
        self._lines.append(indent + (fmt % args))

    def __call__(self, fmt, *args):
        """
        Write the given output. The extra arguments are used for string
        interpolation with the format string.
        """
        self._out(0, fmt, *args)

    def with_prefix(self, prefix, text):
        """
        Write the given output. The text may contain line breaks. Every line is
        prefixed with the given prefix.
        """
        for line in text.splitlines():
            self("%s%s", prefix, line)

    def with_prefixes(self, prefix1, prefix2, text):
        """
        Write the given output. The text may contain line breaks. Every line is
        prefixed with a prefix. The first line gets the first prefix while
        following lines get the following prefixes.
        """
        lines = text.splitlines()
        if not lines:
            return
        self("%s%s", prefix1, lines[0])
        for line in lines[1:]:
            self("%s%s", prefix2, line)

    def indent(self, fmt, *args):
        """
        Write the given output with one extra level of indentation. The extra
        arguments are used for string interpolation with the format string.
        """
        self._out(1, fmt, *args)

    def indent_incr(self):
        """Increase indentation of the following output by one level."""
        self._indent_level += 1

    def indent_decr(self):
        """Decrease indentation of the following output by one level."""
        self._indent_level -= 1
        assert self._indent_level >= 0

    def copy_from(self, out):
        """Copy the output from the given Output instance to this one."""
        for line in out._lines:
            self("%s", line)

    def write_file(self, filename):
        with open(filename, 'w') as target:
            for line in self._lines:
                target.write(line.rstrip())
                target.write('\n')

    def __bool__(self):
        return bool(self._lines)

    __nonzero__ = __bool__


class Indent(object):
    """A context manager that increases indentation level in the output."""

    def __init__(self, output):
        self.output = output

    def __enter__(self):
        self.output.indent_incr()
        return self

    def __exit__(self, type, value, traceback):
        self.output.indent_decr()

def generated_code_header(output):
    """Add a Rust-header to the output saying that this file is generated."""
    output("// This file contains generated code. Do not edit directly.")
    output("// To regenerate this, run 'make'.")
    output("")
