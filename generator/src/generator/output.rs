pub(super) struct Output {
    data: String,
    indent: usize,
}

impl Output {
    #[inline]
    pub(super) fn new() -> Self {
        Self {
            data: String::new(),
            indent: 0,
        }
    }

    #[inline]
    pub(super) fn into_data(self) -> String {
        assert_eq!(self.indent, 0);
        self.data
    }

    #[inline]
    pub(super) fn incr_indent(&mut self) {
        self.indent += 1;
    }

    #[inline]
    pub(super) fn decr_indent(&mut self) {
        self.indent -= 1;
    }

    #[inline]
    pub(super) fn indented(&mut self, f: impl FnOnce(&mut Self)) {
        self.incr_indent();
        f(self);
        self.decr_indent();
    }

    #[inline]
    pub(super) fn indent(&mut self) -> Indented<'_> {
        Indented { out: self }
    }
}

impl std::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if i != 0 {
                self.data.push('\n');
            }
            if !line.is_empty() {
                if self.data.ends_with('\n') {
                    for _ in 0..self.indent {
                        self.data.push_str("    ");
                    }
                }
                self.data.push_str(line);
            }
        }
        Ok(())
    }
}

pub(super) struct Indented<'a> {
    out: &'a mut Output,
}

impl std::fmt::Write for Indented<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.out.incr_indent();
        let r = std::fmt::Write::write_str(&mut self.out, s);
        self.out.decr_indent();
        r
    }
}

macro_rules! out {
    ($out:expr, $($args:tt)+) => {
        {
            use std::fmt::Write as _;
            write!($out, $($args)+).unwrap();
        }
    };
}

macro_rules! outln {
    ($out:expr, $($args:tt)+) => {
        {
            use std::fmt::Write as _;
            writeln!($out, $($args)+).unwrap();
        }
    };
}
