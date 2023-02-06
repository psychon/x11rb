use std::fmt;

/// Emit sync or async code.
#[derive(Debug, PartialEq)]
pub(super) enum ImplMode {
    Sync,
    Async,
}

impl ImplMode {
    pub(super) fn fn_async(&self) -> impl fmt::Display {
        match self {
            ImplMode::Sync => "",
            ImplMode::Async => "async ",
        }
    }

    pub(super) fn dot_await(&self) -> impl fmt::Display {
        match self {
            ImplMode::Sync => "",
            ImplMode::Async => ".await",
        }
    }

    pub(super) fn ret_ty(&self, inner: impl fmt::Display, named: bool) -> impl fmt::Display {
        let (begin, end) = match self {
            ImplMode::Sync => ("", "".to_string()),
            ImplMode::Async => (
                "Pin<Box<dyn Future<Output = ",
                format!("> + Send + '{}>>", if named { "future" } else { "_" },),
            ),
        };

        format!("{}{}{}", begin, inner, end)
    }
}
