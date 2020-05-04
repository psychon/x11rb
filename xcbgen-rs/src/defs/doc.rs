/// Documentation of some type.
#[derive(Clone, Debug)]
pub struct Doc {
    /// A brief, one-line description.
    pub brief: Option<String>,

    /// A verbose description.
    pub description: Option<String>,

    /// A usage example. This will most likely assume libxcb's C API.
    pub example: Option<String>,

    /// Documentation of the individual fields.
    pub fields: Vec<FieldDoc>,

    /// The possible X11 errors that can occur.
    pub errors: Vec<ErrorDoc>,

    /// A reference to a related type.
    pub sees: Vec<SeeDoc>,
}

/// Documentation for a single type.
#[derive(Clone, Debug)]
pub struct FieldDoc {
    /// The name of the field.
    pub name: String,

    /// A description of this field.
    pub doc: Option<String>,
}

/// Documentation of a X11 error that can occur.
#[derive(Clone, Debug)]
pub struct ErrorDoc {
    /// The name of the error.
    pub type_: String,

    /// A description of when this error occurs.
    pub doc: Option<String>,
}

/// A reference to another type for further information.
#[derive(Clone, Debug)]
pub struct SeeDoc {
    /// The kind of type that is referenced.
    // TODO: Turn this into an enum?
    pub type_: String,

    /// The name of the thing that is referenced, for example the name of a request.
    pub name: String,
}
