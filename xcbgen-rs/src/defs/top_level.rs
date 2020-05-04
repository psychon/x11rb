use std::cell::RefCell;
use std::rc::{Rc, Weak};

use once_cell::unsync::OnceCell;

use super::{Alignment, BuiltInType, ComplexAlignment, Doc, ExternalParam, FieldDef, Namespace};

/// Any kind of definition in a namespace.
#[derive(Debug)]
pub enum Def {
    /// A request definition.
    Request(Rc<RequestDef>),

    /// An event definition.
    Event(EventDef),

    /// An error definition.
    Error(ErrorDef),

    /// A type definition.
    Type(TypeDef),
}

/// A `<request>` definition.
#[derive(Debug)]
pub struct RequestDef {
    /// Reference to the namespace that this request is defined in.
    pub namespace: Weak<Namespace>,

    /// The name of the request.
    pub name: String,

    /// The opcode of the request.
    pub opcode: u8,

    /// Whether adjacent requests of this kind can be combined.
    ///
    /// For example, xproto's `PolyRectangle` request contains a list of rectangles to draw. If two
    /// such requests are sent one after the other with the same fields, their list of rectangles
    /// can be combined into a single `PolyRectangle` request.
    pub combine_adjacent: bool,

    /// The value of this request's `<required_start_align>` child, if any.
    pub required_start_align: Option<Alignment>,

    /// The list of fields of this request.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: RefCell<Vec<FieldDef>>,

    /// The definition of the reply to this request, if any.
    pub reply: Option<Rc<ReplyDef>>,

    /// The documentation of this request.
    pub doc: Option<Doc>,
}

/// A `<reply>` definition.
#[derive(Debug)]
pub struct ReplyDef {
    /// The request that causes this reply.
    ///
    /// This reference should always be set after parsing so that `.get()` should not return
    /// `None`.
    pub request: OnceCell<Weak<RequestDef>>,

    /// The value of this request's `<required_start_align>` child, if any.
    pub required_start_align: Option<Alignment>,

    /// The list of fields of this reply.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: RefCell<Vec<FieldDef>>,

    /// The documentation of this reply.
    pub doc: Option<Doc>,
}

/// An `<event>` or `<eventcopy>` definition.
#[derive(Clone, Debug)]
pub enum EventDef {
    /// This is an `<event>`.
    Full(Rc<EventFullDef>),

    /// This is an `<eventcopy>`.
    Copy(Rc<EventCopyDef>),
}

impl EventDef {
    /// The value of the `name` attribute.
    pub fn name(&self) -> &str {
        match self {
            Self::Full(event_full_def) => &event_full_def.name,
            Self::Copy(event_copy_def) => &event_copy_def.name,
        }
    }

    /// The namespace that this event belongs to.
    pub fn namespace(&self) -> Rc<Namespace> {
        match self {
            Self::Full(event_full_def) => event_full_def.namespace.upgrade().unwrap(),
            Self::Copy(event_copy_def) => event_copy_def.namespace.upgrade().unwrap(),
        }
    }

    /// Get the full definition of this event.
    ///
    /// For an `<event>`, this simply returns the contained event. For an `<eventcopy>`, this
    /// returns the definition of the original `<event>` that was copied.
    ///
    /// # Panics
    ///
    /// Panics if an `<eventcopy>` was not yet resolved.
    pub fn get_original_full_def(&self) -> Rc<EventFullDef> {
        match self {
            Self::Full(event_full_def) => event_full_def.clone(),
            Self::Copy(event_copy_def) => event_copy_def.get_original_full_def(),
        }
    }

    /// Is this event XGE?
    pub fn is_xge(&self) -> bool {
        self.get_original_full_def().xge
    }

    /// Downgrade this definition to a weak reference.
    ///
    /// See [`Rc::downgrade`].
    pub fn as_event_ref(&self) -> EventRef {
        match self {
            Self::Full(event_full_def) => EventRef::Full(Rc::downgrade(event_full_def)),
            Self::Copy(event_copy_def) => EventRef::Copy(Rc::downgrade(event_copy_def)),
        }
    }
}

/// An `<event>` definition.
#[derive(Debug)]
pub struct EventFullDef {
    /// The namespace that this event belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the event.
    pub name: String,

    /// The `number` of the event.
    pub number: u16,

    /// The `no-sequence-number` attribute of the event.
    ///
    /// The only known event without a sequence number is xproto's `KeymapNotify`.
    pub no_sequence_number: bool,

    /// The `xge` attribute of the `<event>`.
    pub xge: bool,

    /// The value of this event's `<required_start_align>` child, if any.
    pub required_start_align: Option<Alignment>,

    /// The list of fields of this event.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: RefCell<Vec<FieldDef>>,

    /// The documentation of this event.
    pub doc: Option<Doc>,
}

/// An `<eventcopy>` definition.
#[derive(Debug)]
pub struct EventCopyDef {
    /// The namespace that this event belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the new event.
    pub name: String,

    /// The `number` of the new event.
    pub number: u16,

    /// A reference to the event that is copied.
    pub ref_: NamedEventRef,
}

impl EventCopyDef {
    /// Get the event that was copied.
    ///
    /// # Panics
    ///
    /// Panics if the reference was not resolved yet.
    pub fn get_original_full_def(&self) -> Rc<EventFullDef> {
        self.ref_.def.get().unwrap().get_original_full_def()
    }
}

/// An `<error>` or `<errorcopy>` definition.
#[derive(Clone, Debug)]
pub enum ErrorDef {
    /// This is an `<error>`.
    Full(Rc<ErrorFullDef>),

    /// This is an `<errorcopy>`.
    Copy(Rc<ErrorCopyDef>),
}

impl ErrorDef {
    /// The value of the `name` attribute.
    pub fn name(&self) -> &str {
        match self {
            Self::Full(error_full_def) => &error_full_def.name,
            Self::Copy(error_copy_def) => &error_copy_def.name,
        }
    }

    /// The namespace that this error belongs to.
    pub fn namespace(&self) -> Rc<Namespace> {
        match self {
            Self::Full(error_full_def) => error_full_def.namespace.upgrade().unwrap(),
            Self::Copy(error_copy_def) => error_copy_def.namespace.upgrade().unwrap(),
        }
    }

    /// Get the full definition of this error.
    ///
    /// For an `<error>`, this simply returns the contained error. For an `<errorcopy>`, this
    /// returns the definition of the original `<error>` that was copied.
    ///
    /// # Panics
    ///
    /// Panics if an `<errorcopy>` was not yet resolved.
    pub fn get_original_full_def(&self) -> Rc<ErrorFullDef> {
        match self {
            Self::Full(error_full_def) => error_full_def.clone(),
            Self::Copy(error_copy_def) => error_copy_def.get_original_full_def(),
        }
    }
}

/// An `<error>` definition.
#[derive(Debug)]
pub struct ErrorFullDef {
    /// The namespace that this error belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the error.
    pub name: String,

    /// The `number` of the error.
    // Signed because there are some `-1` somewhere.
    pub number: i16,

    /// The value of this error's `<required_start_align>` child, if any.
    pub required_start_align: Option<Alignment>,

    /// The list of fields of this error.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: RefCell<Vec<FieldDef>>,
}

/// An `<errorcopy>` definition.
#[derive(Debug)]
pub struct ErrorCopyDef {
    /// The namespace that this error belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the new error.
    pub name: String,

    /// The `number` of the new error.
    pub number: i16,

    /// A reference to the error that is copied.
    pub ref_: NamedErrorRef,
}

impl ErrorCopyDef {
    /// Get the error that was copied.
    ///
    /// # Panics
    ///
    /// Panics if the reference was not resolved yet.
    pub fn get_original_full_def(&self) -> Rc<ErrorFullDef> {
        self.ref_.def.get().unwrap().get_original_full_def()
    }
}

/// An enumeration of the possible type definitions that a module can contain.
///
/// This enumeration represents any kind of child of the `<xcb>` tag with the exception of
/// `<request>`, `<event>`, `<eventcopy>`, `<error>`, and `<errorcopy>` because these types have
/// some special meaning in the protocol.
#[derive(Debug)]
pub enum TypeDef {
    /// A `<struct>` definition.
    Struct(Rc<StructDef>),

    /// A `<union>` definition.
    Union(Rc<UnionDef>),

    /// A `<eventstruct>` definition.
    EventStruct(Rc<EventStructDef>),

    /// A `<xidtype>` definition.
    Xid(Rc<XidTypeDef>),

    /// A `<xidunion>` definition.
    XidUnion(Rc<XidUnionDef>),

    /// A `<enum>` definition.
    Enum(Rc<EnumDef>),

    /// A `<typedef>` definition.
    Alias(Rc<TypeAliasDef>),
}

/// A `<struct>` definition.
#[derive(Debug)]
pub struct StructDef {
    /// The namespace that this struct belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the struct.
    pub name: String,

    /// Alignment information about this struct.
    ///
    /// This information is not available before the struct was resolved.
    pub alignment: OnceCell<ComplexAlignment>,

    /// The list of fields of this struct.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: RefCell<Vec<FieldDef>>,

    /// The list of `<paramref>`s that appear in descendants of this struct.
    ///
    /// This list is empty before the struct was resolved.
    pub external_params: RefCell<Vec<ExternalParam>>,
}

impl StructDef {
    /// Get the size in bytes of this struct on the wire.
    ///
    /// This returns `None` if the struct does not have a fixed size.
    pub fn size(&self) -> Option<u32> {
        self.fields
            .borrow()
            .iter()
            .try_fold(0, |sz, field| Some(sz + field.size()?))
    }
}

/// A `<union>` definition.
#[derive(Debug)]
pub struct UnionDef {
    /// The namespace that this union belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the union.
    pub name: String,

    /// Alignment information about this union.
    ///
    /// This information is not available before the union was resolved.
    pub alignment: OnceCell<ComplexAlignment>,

    /// The list of fields of this union.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: Vec<FieldDef>,
}

impl UnionDef {
    /// Get the size of the union.
    ///
    /// This function returns the size of the largest child of the union.
    pub fn size(&self) -> u32 {
        self.fields
            .iter()
            .map(|field| field.size().unwrap())
            .max()
            .unwrap()
    }
}

/// A `<eventstruct>` definition.
#[derive(Clone, Debug)]
pub struct EventStructDef {
    /// The namespace that this eventstruct belongs to.
    pub namespace: Weak<Namespace>,

    /// The `name` of the eventstruct.
    pub name: String,

    /// The list of `<allowed>` children.
    pub alloweds: Vec<EventStructAllowed>,
}

/// A `<allowed>` definition inside of an `<eventstruct>`.
#[derive(Clone, Debug)]
pub struct EventStructAllowed {
    /// The extension that events come from.
    pub extension: String,

    /// Whether XGEs are allowed.
    pub xge: bool,

    /// The minimum opcode of the events.
    pub opcode_min: u16,

    /// The maximum opcode of the events.
    pub opcode_max: u16,

    /// The list of events that this `<allowed>` references.
    ///
    /// This member is set up during resolution
    pub resolved: RefCell<Vec<EventRef>>,
}

/// An `<xidtype>` definition.
#[derive(Debug)]
pub struct XidTypeDef {
    /// The namespace that this XID type belongs to.
    pub namespace: Weak<Namespace>,

    /// The name of the new XID type.
    pub name: String,
}

/// An `<xidunion>` definition.
#[derive(Debug)]
pub struct XidUnionDef {
    /// The namespace that this XID union belongs to.
    pub namespace: Weak<Namespace>,

    /// The name of the new XID type.
    pub name: String,

    /// The list of types that are contained in the union.
    pub types: Vec<NamedTypeRef>,
}

/// An `<enum>` definition.
#[derive(Debug)]
pub struct EnumDef {
    /// The namespace that this XID union belongs to.
    pub namespace: Weak<Namespace>,

    /// The name of the new XID type.
    pub name: String,

    /// The list of items in this enum.
    pub items: Vec<EnumItem>,

    /// The documentation of this enum.
    pub doc: Option<Doc>,
}

/// An `<item>` inside of an `<enum>`.
#[derive(Debug)]
pub struct EnumItem {
    /// The name of this item.
    pub name: String,

    /// The value of this item.
    pub value: EnumValue,
}

/// The value of an enum item
#[derive(Debug)]
pub enum EnumValue {
    /// A `<value>`.
    Value(u32),

    /// A `<bit>`.
    ///
    /// A bit encodes a value of `1 << bit`.
    Bit(u8),
}

/// A `<typedef>` definition.
#[derive(Debug)]
pub struct TypeAliasDef {
    /// The namespace that this typedef belongs to.
    pub namespace: Weak<Namespace>,

    /// The name of the new type.
    pub new_name: String,

    /// The name of the old type that is given a new name.
    pub old_name: NamedTypeRef,
}

impl TypeAliasDef {
    /// Get the original type that is copied in this type def.
    ///
    /// # Panics
    ///
    /// Panics if the type definition was not yet resolved.
    pub fn get_original_type(&self) -> TypeRef {
        let mut type_ref = self.old_name.def.get().unwrap().clone();
        loop {
            match type_ref {
                TypeRef::Alias(alias) => {
                    type_ref = alias.upgrade().unwrap().old_name.def.get().unwrap().clone();
                }
                _ => return type_ref,
            }
        }
    }
}

/// A reference to an event by name.
#[derive(Debug)]
pub struct NamedEventRef {
    /// The name of the event that is referenced.
    pub name: String,

    /// The definition of the event that is referenced.
    ///
    /// This field is only set up during resolving.
    pub def: OnceCell<EventRef>,
}

impl NamedEventRef {
    /// Create a new unresolved instance.
    pub fn unresolved(name: String) -> Self {
        Self {
            name,
            def: OnceCell::new(),
        }
    }
}

/// A reference to an event.
#[derive(Clone, Debug)]
pub enum EventRef {
    /// A reference to an `<event>`.
    Full(Weak<EventFullDef>),

    /// A reference to an `<eventcopy>`.
    Copy(Weak<EventCopyDef>),
}

impl EventRef {
    /// Get the full definition of this event.
    ///
    /// For an `<event>`, this simply returns the contained event. For an `<eventcopy>`, this
    /// returns the definition of the original `<event>` that was copied.
    ///
    /// # Panics
    ///
    /// Panics if an `<eventcopy>` was not yet resolved.
    pub fn get_original_full_def(&self) -> Rc<EventFullDef> {
        match self {
            Self::Full(event_full_def) => event_full_def.upgrade().unwrap(),
            Self::Copy(event_copy_def) => {
                let mut event_copy_def = event_copy_def.upgrade().unwrap();
                loop {
                    match event_copy_def.ref_.def.get().unwrap() {
                        EventRef::Full(event_full_def) => return event_full_def.upgrade().unwrap(),
                        EventRef::Copy(event_copy_ref) => {
                            event_copy_def = event_copy_ref.upgrade().unwrap()
                        }
                    }
                }
            }
        }
    }

    /// Is this an XGE?
    pub fn is_xge(&self) -> bool {
        self.get_original_full_def().xge
    }

    /// Upgrade this event reference to an event definition.
    ///
    /// See [`Weak::Upgrade`] for more about what this really does.
    pub fn as_event_def(&self) -> EventDef {
        match self {
            Self::Full(event_full_def) => EventDef::Full(event_full_def.upgrade().unwrap()),
            Self::Copy(event_copy_def) => EventDef::Copy(event_copy_def.upgrade().unwrap()),
        }
    }
}

/// A reference to an error by name.
#[derive(Debug)]
pub struct NamedErrorRef {
    /// The name of the error that is referenced.
    pub name: String,

    /// The definition of the event that is referenced.
    ///
    /// This field is only set up during resolving.
    pub def: OnceCell<ErrorRef>,
}

impl NamedErrorRef {
    /// Create a new unresolved instance.
    pub fn unresolved(name: String) -> Self {
        Self {
            name,
            def: OnceCell::new(),
        }
    }
}

/// A reference to an error.
#[derive(Clone, Debug)]
pub enum ErrorRef {
    /// A reference to an `<error>`.
    Full(Weak<ErrorFullDef>),

    /// A reference to an `<errorcopy>`.
    Copy(Weak<ErrorCopyDef>),
}

impl ErrorRef {
    /// Get the full definition of this error.
    ///
    /// For an `<error>`, this simply returns the contained error. For an `<errorcopy>`, this
    /// returns the definition of the original `<error>` that was copied.
    ///
    /// # Panics
    ///
    /// Panics if an `<errorcopy>` was not yet resolved.
    pub fn get_original_full_def(&self) -> Rc<ErrorFullDef> {
        match self {
            Self::Full(error_full_def) => error_full_def.upgrade().unwrap(),
            Self::Copy(error_copy_def) => {
                let mut error_copy_def = error_copy_def.upgrade().unwrap();
                loop {
                    match error_copy_def.ref_.def.get().unwrap() {
                        ErrorRef::Full(error_full_def) => return error_full_def.upgrade().unwrap(),
                        ErrorRef::Copy(error_copy_ref) => {
                            error_copy_def = error_copy_ref.upgrade().unwrap()
                        }
                    }
                }
            }
        }
    }

    /// Upgrade this error reference to an error definition.
    ///
    /// See [`Weak::Upgrade`] for more about what this really does.
    pub fn as_error_def(&self) -> ErrorDef {
        match self {
            Self::Full(error_full_def) => ErrorDef::Full(error_full_def.upgrade().unwrap()),
            Self::Copy(error_copy_def) => ErrorDef::Copy(error_copy_def.upgrade().unwrap()),
        }
    }
}

/// A reference to a type by name.
#[derive(Debug)]
pub struct NamedTypeRef {
    /// The name of the type that is referenced
    pub name: String,

    /// The definition of the type that is referenced.
    ///
    /// This field is only set up during resolving.
    pub def: OnceCell<TypeRef>,
}

impl NamedTypeRef {
    /// Create a new unresolved instance.
    pub fn unresolved(name: String) -> Self {
        Self {
            name,
            def: OnceCell::new(),
        }
    }
}

/// A reference to some type.
#[derive(Clone, Debug)]
pub enum TypeRef {
    /// A built-in type is referenced.
    BuiltIn(BuiltInType),

    /// A struct is referenced.
    Struct(Weak<StructDef>),

    /// A union is referenced.
    Union(Weak<UnionDef>),

    /// An event structure is referenced.
    EventStruct(Weak<EventStructDef>),

    /// A X11 ID is referenced.
    Xid(Weak<XidTypeDef>),

    /// A union of X11 IDs is referenced.
    XidUnion(Weak<XidUnionDef>),

    /// An enumeration is referenced.
    Enum(Weak<EnumDef>),

    /// A type alias is referenced.
    Alias(Weak<TypeAliasDef>),
}

impl TypeRef {
    /// Check if two types are structurally identical.
    ///
    /// This does e.g. not resolve type aliases, but instead checks that both sides reference the
    /// same type alias.
    pub fn same_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BuiltIn(builtin1), Self::BuiltIn(builtin2)) => builtin1 == builtin2,
            (Self::Struct(def1), Self::Struct(def2)) => weak_ptr_eq(def1, def2),
            (Self::Union(def1), Self::Union(def2)) => weak_ptr_eq(def1, def2),
            (Self::EventStruct(def1), Self::EventStruct(def2)) => weak_ptr_eq(def1, def2),
            (Self::Xid(def1), Self::Xid(def2)) => weak_ptr_eq(def1, def2),
            (Self::XidUnion(def1), Self::XidUnion(def2)) => weak_ptr_eq(def1, def2),
            (Self::Enum(def1), Self::Enum(def2)) => weak_ptr_eq(def1, def2),
            (Self::Alias(def1), Self::Alias(def2)) => weak_ptr_eq(def1, def2),
            _ => false,
        }
    }

    /// Get the size in bytes of this case on the wire.
    ///
    /// This returns `None` if the type does not have a constant size.
    pub fn size(&self) -> Option<u32> {
        match self {
            Self::BuiltIn(builtin_type) => Some(builtin_type.size()),
            Self::Struct(struct_def) => {
                let struct_def = struct_def.upgrade().unwrap();
                struct_def.size()
            }
            Self::Union(union_def) => {
                let union_def = union_def.upgrade().unwrap();
                Some(union_def.size())
            }
            Self::EventStruct(event_struct_def) => {
                let event_struct_def = event_struct_def.upgrade().unwrap();
                for allowed in event_struct_def.alloweds.iter() {
                    for event in allowed.resolved.borrow().iter() {
                        let event_def = event.get_original_full_def();
                        if event_def.xge {
                            return None;
                        }
                    }
                }

                Some(32)
            }
            Self::Xid(_) => Some(4),
            Self::XidUnion(_) => Some(4),
            // never used directly
            Self::Enum(_) => unreachable!(),
            Self::Alias(type_alias_def) => {
                let type_alias_def = type_alias_def.upgrade().unwrap();
                type_alias_def.old_name.def.get().unwrap().size()
            }
        }
    }

    /// Get the original type.
    ///
    /// This function resolves type aliases by calling [`TypeAliasDef::get_original_type`].
    pub fn get_original_type(&self) -> Self {
        match self {
            Self::Alias(type_alias_def) => {
                let type_alias_def = type_alias_def.upgrade().unwrap();
                type_alias_def.get_original_type()
            }
            _ => self.clone(),
        }
    }
}

// Replacement for `Weak::ptr_eq`, which requires
// Rust 1.39.
fn weak_ptr_eq<T>(weak1: &Weak<T>, weak2: &Weak<T>) -> bool {
    Rc::ptr_eq(&weak1.upgrade().unwrap(), &weak2.upgrade().unwrap())
}
