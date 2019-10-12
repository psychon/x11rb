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
