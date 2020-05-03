//! Definitions describing the structure of the XML protocol and its extension.
//!
//! This module contains the [`Module`] type. A [`Module`] contains some [`Namespace`]s, where each
//! [`Namespace`] corresponds to one XML file from the xcb-proto project. Each XML file describes
//! one X11 extension or the core protocol.

use std::cell::RefCell;
use std::collections::hash_map::Entry as HashMapEntry;
use std::convert::TryFrom;
use std::rc::{Rc, Weak};

use fxhash::FxHashMap;
use once_cell::unsync::OnceCell;

/// A `Module` contains a collection of namespaces.
#[derive(Debug)]
pub struct Module {
    /// All namespaces in this module
    pub namespaces: RefCell<FxHashMap<String, Rc<Namespace>>>,
}

impl Module {
    /// Create a new, empty module
    pub fn new() -> Rc<Self> {
        Rc::new(Module {
            namespaces: RefCell::new(FxHashMap::default()),
        })
    }

    /// Returns `false` if the namespace name was already used.
    pub fn insert_namespace(&self, ns: Rc<Namespace>) -> bool {
        match self.namespaces.borrow_mut().entry(ns.header.clone()) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                entry.insert(ns);
                true
            }
        }
    }

    /// Find a namespace by `header` name.
    pub fn namespace(&self, header: &str) -> Option<Rc<Namespace>> {
        self.namespaces.borrow().get(header).cloned()
    }

    /// Find a namespace by `extension-name`.
    pub fn get_namespace_by_ext_name(&self, name: &str) -> Option<Rc<Namespace>> {
        self.namespaces
            .borrow()
            .values()
            .find(|ns| {
                if let Some(ref ext_info) = ns.ext_info {
                    ext_info.name == name
                } else {
                    false
                }
            })
            .cloned()
    }

    /// Get a sorted list of all loaded namespaces.
    ///
    /// The namespaces are sorted by name with the exception of `xproto` being before everything
    /// else.
    pub fn sorted_namespaces(&self) -> Vec<Rc<Namespace>> {
        let mut namespaces: Vec<_> = self.namespaces.borrow().values().cloned().collect();
        namespaces.sort_by(|a, b| {
            // Always put xproto at the beginning
            match (a.header.as_str(), b.header.as_str()) {
                ("xproto", "xproto") => std::cmp::Ordering::Equal,
                ("xproto", _) => std::cmp::Ordering::Less,
                (_, "xproto") => std::cmp::Ordering::Greater,
                (_, _) => a.header.cmp(&b.header),
            }
        });
        namespaces
    }
}

/// Information about an X11 extension.
#[derive(Clone, Debug)]
pub struct ExtInfo {
    /// The name of the extension as used in `QueryExtension`.
    pub xname: String,

    /// The name of the extension as meant for humans.
    pub name: String,

    /// A special property used by libxcb.
    ///
    /// See xcb-proto's `NEWS` file for more information.
    pub multiword: bool,

    /// The major version number of the extension.
    pub major_version: u16,

    /// The minor version number of the extension.
    pub minor_version: u16,
}

/// A namespace of X11 definitions.
///
/// Every XML file corresponds to one namespace.
#[derive(Debug)]
pub struct Namespace {
    /// The module that owns this namespace.
    pub module: Weak<Module>,

    /// The `header` name for this namespace.
    ///
    /// This is a good choice for naming files.
    pub header: String,

    /// Information about the extension that is described.
    ///
    /// This should only be `None` for the core X11 protocol `xproto.
    pub ext_info: Option<ExtInfo>,

    /// Other namespaces that are imported into this namespace.
    pub imports: RefCell<FxHashMap<String, Import>>,

    /// The requests that are defined in this module.
    pub request_defs: RefCell<FxHashMap<String, Rc<RequestDef>>>,

    /// The events that are defined in this module.
    pub event_defs: RefCell<FxHashMap<String, EventDef>>,

    /// The errors that are defined in this module.
    pub error_defs: RefCell<FxHashMap<String, ErrorDef>>,

    /// The types that are defined in this module.
    pub type_defs: RefCell<FxHashMap<String, TypeDef>>,

    /// All definitions in this module in the order they appear in the XML description.
    pub src_order_defs: RefCell<Vec<Def>>,
}

impl Namespace {
    /// Create a new namespace.
    ///
    /// This function creates a new namespace in the given `module`.
    ///
    /// The namespace is not added to the `module` yet. See [`Module::insert_namespace`] for that.
    pub fn new(module: &Rc<Module>, header: String, ext_info: Option<ExtInfo>) -> Rc<Self> {
        Rc::new(Self {
            module: Rc::downgrade(module),
            header,
            ext_info,
            imports: RefCell::new(FxHashMap::default()),
            request_defs: RefCell::new(FxHashMap::default()),
            event_defs: RefCell::new(FxHashMap::default()),
            error_defs: RefCell::new(FxHashMap::default()),
            type_defs: RefCell::new(FxHashMap::default()),
            src_order_defs: RefCell::new(Vec::new()),
        })
    }

    /// Record a new import in this namespace.
    ///
    /// The import is not yet resolved, but only its existence recorded.
    pub fn add_import(&self, import_name: String) {
        self.imports.borrow_mut().insert(
            import_name.clone(),
            Import {
                name: import_name,
                ns: OnceCell::new(),
            },
        );
    }

    /// Insert new request definitions into this namespace.
    ///
    /// Returns `false` if the name is already in use.
    #[must_use]
    pub fn insert_request_def(&self, name: String, request_def: Rc<RequestDef>) -> bool {
        match self.request_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                entry.insert(Rc::clone(&request_def));
                self.src_order_defs
                    .borrow_mut()
                    .push(Def::Request(request_def));
                true
            }
        }
    }

    /// Insert a new event definition into this namespace.
    ///
    /// Returns `false` if the name is already in use.
    #[must_use]
    pub fn insert_event_def(&self, name: String, event_def: EventDef) -> bool {
        match self.event_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                let clone = match event_def {
                    EventDef::Full(ref def) => EventDef::Full(Rc::clone(def)),
                    EventDef::Copy(ref def) => EventDef::Copy(Rc::clone(def)),
                };
                entry.insert(event_def);
                self.src_order_defs.borrow_mut().push(Def::Event(clone));
                true
            }
        }
    }

    /// Insert a new error definition into this namespace.
    ///
    /// Returns `false` if the name is already in use.
    #[must_use]
    pub fn insert_error_def(&self, name: String, error_def: ErrorDef) -> bool {
        match self.error_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                let clone = match error_def {
                    ErrorDef::Full(ref def) => ErrorDef::Full(Rc::clone(def)),
                    ErrorDef::Copy(ref def) => ErrorDef::Copy(Rc::clone(def)),
                };
                entry.insert(error_def);
                self.src_order_defs.borrow_mut().push(Def::Error(clone));
                true
            }
        }
    }

    /// Insert a new type definition into this namespace.
    ///
    /// Returns `false` if the name is already in use.
    #[must_use]
    pub fn insert_type_def(&self, name: String, type_def: TypeDef) -> bool {
        match self.type_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                let clone = match type_def {
                    TypeDef::Struct(ref def) => TypeDef::Struct(Rc::clone(def)),
                    TypeDef::Union(ref def) => TypeDef::Union(Rc::clone(def)),
                    TypeDef::EventStruct(ref def) => TypeDef::EventStruct(Rc::clone(def)),
                    TypeDef::Xid(ref def) => TypeDef::Xid(Rc::clone(def)),
                    TypeDef::XidUnion(ref def) => TypeDef::XidUnion(Rc::clone(def)),
                    TypeDef::Enum(ref def) => TypeDef::Enum(Rc::clone(def)),
                    TypeDef::Alias(ref def) => TypeDef::Alias(Rc::clone(def)),
                };
                entry.insert(type_def);
                self.src_order_defs.borrow_mut().push(Def::Type(clone));
                true
            }
        }
    }

    /// Get an imported namespace by name.
    ///
    /// This function returns `None` if the namespace is not found.
    ///
    /// # Panics
    ///
    /// Panics if the namespace exists, but was not yet resolved.
    pub fn get_import(&self, name: &str) -> Option<Rc<Namespace>> {
        self.imports.borrow().get(name).map(|import| import.ns())
    }

    /// Get an event reference by name.
    ///
    /// Only the current namespace is searched and not its imports.
    ///
    /// This function returns `None` if the event is not found.
    pub fn get_event(&self, name: &str) -> Option<EventRef> {
        self.event_defs
            .borrow()
            .get(name)
            .map(|event_def| match event_def {
                EventDef::Full(def) => EventRef::Full(Rc::downgrade(def)),
                EventDef::Copy(def) => EventRef::Copy(Rc::downgrade(def)),
            })
    }

    /// Get an error reference by name.
    ///
    /// Only the current namespace is searched and not its imports.
    ///
    /// This function returns `None` if the event is not found.
    pub fn get_error(&self, name: &str) -> Option<ErrorRef> {
        self.error_defs
            .borrow()
            .get(name)
            .map(|error_def| match error_def {
                ErrorDef::Full(def) => ErrorRef::Full(Rc::downgrade(def)),
                ErrorDef::Copy(def) => ErrorRef::Copy(Rc::downgrade(def)),
            })
    }

    /// Get a type reference by name.
    ///
    /// Only the current namespace is searched and not its imports.
    ///
    /// This function returns `None` if the event is not found.
    pub fn get_type(&self, name: &str) -> Option<TypeRef> {
        self.type_defs
            .borrow()
            .get(name)
            .map(|type_def| match type_def {
                TypeDef::Struct(def) => TypeRef::Struct(Rc::downgrade(def)),
                TypeDef::Union(def) => TypeRef::Union(Rc::downgrade(def)),
                TypeDef::EventStruct(def) => TypeRef::EventStruct(Rc::downgrade(def)),
                TypeDef::Xid(def) => TypeRef::Xid(Rc::downgrade(def)),
                TypeDef::XidUnion(def) => TypeRef::XidUnion(Rc::downgrade(def)),
                TypeDef::Enum(def) => TypeRef::Enum(Rc::downgrade(def)),
                TypeDef::Alias(def) => TypeRef::Alias(Rc::downgrade(def)),
            })
    }

    /// Get an event definition by event `number` and whether it is XGE.
    pub fn get_event_by_number(&self, number: u16, is_xge: bool) -> Option<EventDef> {
        self.event_defs
            .borrow()
            .values()
            .find(|event_def| match event_def {
                EventDef::Full(event_full_def) => {
                    event_full_def.number == number && event_full_def.xge == is_xge
                }
                EventDef::Copy(event_copy_def) => {
                    event_copy_def.number == number && event_def.is_xge() == is_xge
                }
            })
            .cloned()
    }
}

/// Representation of an `<import>`.
#[derive(Debug)]
pub struct Import {
    /// The name that was `<import>`ed.
    pub name: String,

    /// After resolving, this is a reference to the namespace that was imported.
    pub ns: OnceCell<Weak<Namespace>>,
}

impl Import {
    /// Get the namespace that is imported.
    ///
    /// # Panics
    ///
    /// Panics if this import was not yet resolved or if the target namespace was already dropped.
    pub fn ns(&self) -> Rc<Namespace> {
        self.ns.get().unwrap().upgrade().unwrap()
    }
}

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
    Struct(Rc<StructDef>),
    Union(Rc<UnionDef>),
    EventStruct(Rc<EventStructDef>),
    Xid(Rc<XidTypeDef>),
    XidUnion(Rc<XidUnionDef>),
    Enum(Rc<EnumDef>),
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

/// Information about a `<paramref>` that is contained in some expression.
#[derive(Clone, Debug)]
pub struct ExternalParam {
    /// The name of the parameter that is referenced.
    pub name: String,

    /// The type of the parameter that is referenced.
    pub type_: TypeRef,
}

/// A reference to an event by name.
#[derive(Debug)]
pub struct NamedEventRef {
    /// The name of the event that is referenced.
    pub name: String,

    /// The definition of the event that is referenced.
    ///
    /// This field is only set up during resolving.
    // FIXME Can this be changed to a Weak<EventFullDef>?
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

/// An alignment specification.
///
/// This structure represents a requirement that some byte position `pos` satisfies
/// `pos % align == offset`.
///
/// `align` must be a power of 2 and `offset` must be less than `align`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    align: u32,
    offset: u32,
}

impl Alignment {
    /// Creates a new `Alignment` with `align` and `offset`.
    ///
    /// # Panics
    ///
    /// Panics if `align` is not a power of two or if `offset` is
    /// equal or greater than `align`.
    pub fn new(align: u32, offset: u32) -> Self {
        assert!(align.is_power_of_two() && offset < align);
        Self { align, offset }
    }

    /// Returns the value of `align`.
    #[inline]
    pub fn align(self) -> u32 {
        self.align
    }

    /// Returns the value of `offset`.
    #[inline]
    pub fn offset(self) -> u32 {
        self.offset
    }

    /// Advance this alignment specification by some variably sized object.
    ///
    /// The resulting value describes the alignment at the end of the variably sized object if it
    /// is aligned by `self` at its beginning.
    pub fn advance_variable_size(self, size: VariableSize) -> Self {
        let align = if size.incr == 0 {
            self.align
        } else {
            self.align.min(size.incr)
        };
        let offset = self.offset.wrapping_add(size.base) % align;
        Self { align, offset }
    }

    /// Returns an alignment that meets `self` and `other`.
    pub fn union(self, other: Self) -> Option<Self> {
        match self.align.cmp(&other.align) {
            std::cmp::Ordering::Less => {
                if (other.offset % self.align) != self.offset {
                    None
                } else {
                    Some(other)
                }
            }
            std::cmp::Ordering::Equal => {
                if self.offset != other.offset {
                    None
                } else {
                    Some(self)
                }
            }
            std::cmp::Ordering::Greater => {
                if (self.offset % other.align) != other.align {
                    None
                } else {
                    Some(self)
                }
            }
        }
    }

    /// Returns an alignment that is met by `self` and `other`.
    // FIXME: `align = self.align.min(other.align)` strikes my as fishy. See my comment on
    // advance_variable_size().
    pub fn intersection(self, other: Self) -> Self {
        let align = self.align.min(other.align);
        let offset1 = self.offset % align;
        let offset2 = other.offset % align;

        if offset1 != offset2 {
            let align1 = (1u32 << offset1.trailing_zeros().min(31)).min(align);
            let align2 = (1u32 << offset2.trailing_zeros().min(31)).min(align);
            Self {
                align: align1.min(align2),
                offset: 0,
            }
        } else {
            Self {
                align,
                offset: offset1,
            }
        }
    }

    /// Returns whether `self` meets the alignment requirements
    /// of `required`.
    pub fn meets(self, required: Self) -> bool {
        // `self.align >= required.align` is equivalent to
        // `self.align % required.align == 0` because `align`
        // is always a power of 2.
        self.align >= required.align && (self.offset % required.align) == required.offset
    }
}

/// Represents the size of an object that has a variable size.
///
/// An object has a minimum size that is described by `base`. It can then have a variable number of
/// increments of size `incr`. That is, the object can have a size of `base + incr * n` for some
/// non-negative value of `n`.
///
/// `incr` must be zero or a power of 2
// FIXME: Make the members non-pub and make the constructor enforce the power-of-2 requirement.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableSize {
    base: u32,
    incr: u32,
}

impl VariableSize {
    /// Creates a new `VariableSize` with `base` and `incr`.
    ///
    /// # Panics
    ///
    /// Panics if `incr` is not zero or a power of 2.
    pub fn new(base: u32, incr: u32) -> Self {
        assert!(incr == 0 || incr.is_power_of_two());
        Self { base, incr }
    }

    /// Returns the value of `base`.
    #[inline]
    pub fn base(self) -> u32 {
        self.base
    }

    /// Returns the value of `incr`.
    #[inline]
    pub fn incr(self) -> u32 {
        self.incr
    }

    /// Get the minimum of two values, but not zero (unless both are zero).
    fn incr_union(incr1: u32, incr2: u32) -> u32 {
        if incr1 == 0 {
            incr2
        } else if incr2 == 0 {
            incr1
        } else {
            incr1.min(incr2)
        }
    }

    /// Reduce the base by the given increment.
    fn reduce_base(base: u32, incr: u32) -> u32 {
        if incr == 0 {
            base
        } else {
            base % incr
        }
    }

    /// Create an instance that describes a zero-sized type.
    #[inline]
    pub fn zero() -> Self {
        Self { base: 0, incr: 0 }
    }

    /// Return a description of the size of things when `self` is appended to `other`.
    // FIXME: This code makes no sense to me. When I have something of size `1+4*n` and something
    // of size `2+8*m`, the result has sizes `3+4*n+8*m` and not `3+4*l`. There is a discrepancy
    // between what the docs for `VariableSize` say what it represents and what this method is
    // supposed to do.
    pub fn append(self, other: Self) -> Self {
        Self {
            // FIXME: check overflow
            base: self.base + other.base,
            incr: Self::incr_union(self.incr, other.incr),
        }
    }

    /// Returns an `AlignSize` that can represent all the sizes
    /// represented by `self` and `other`.
    pub fn union(self, other: Self) -> Self {
        let incr_union = Self::incr_union(self.incr, other.incr);
        if self.base == other.base {
            Self {
                base: self.base,
                incr: incr_union,
            }
        } else {
            let base1 = Self::reduce_base(self.base, incr_union);
            let base2 = Self::reduce_base(other.base, incr_union);
            if base1 == base2 {
                Self {
                    base: base1,
                    incr: incr_union,
                }
            } else if base1 == 0 || base2 == 0 {
                // FIXME: I am quite confused by this code. Some quick println!-debugging says that
                // this is only called with self == Self::zero(). In this case it returns base:0,
                // incr: other.base.
                Self {
                    base: 0,
                    incr: 1u32 << (base1 | base2).trailing_zeros().min(31),
                }
            } else {
                // FIXME: I am quite confused by this code.
                let min_base = base1.min(base2);
                let max_base = base1.max(base2);
                let incr1 = 1u32 << min_base.trailing_zeros().min(31);
                let incr2 = 1u32 << (max_base - min_base).trailing_zeros().min(31);
                if incr1 > incr2 {
                    // FIXME: How can `1u32 << (1u32 << [something])` not overflow? that requires
                    // [something] to be at most 4.
                    // A quick unreachable!() says that this is dead code
                    Self {
                        base: max_base - min_base,
                        incr: 1u32 << incr1,
                    }
                } else {
                    Self {
                        base: min_base,
                        incr: incr2,
                    }
                }
                /*Self {
                    base: 0,
                    incr: 1u32 << (base1 | base2).trailing_zeros().min(31),
                }*/
            }
        }
    }

    /// Describe the size of an arbitrary number of elements.
    pub fn zero_one_or_many(self) -> Self {
        let base = Self::reduce_base(self.base, self.incr);
        if base == 0 {
            Self {
                base: 0,
                incr: self.incr,
            }
        } else {
            // FIXME: Why is it correct to just ignore `incr` in this case?
            Self {
                base: 0,
                incr: 1u32 << base.trailing_zeros().min(31),
            }
        }
    }

    /// Describe the size of `n` elements of this type.
    // FIXME: I am not quite sure what is going on here, but a quick println!()-debugging says this
    // is only called with incr:0 and n > 0 and in that case it returns base:base*n and incr:0.
    pub fn repeat_n(self, n: u32) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            let (base, base_overflow) = self
                .base
                .checked_mul(n)
                .map(|base| (base, false))
                .unwrap_or_else(|| Self::reduce_base(self.base, self.incr).overflowing_mul(n));

            let (mut incr, incr_overflow) = self.incr.overflowing_mul(n);
            if incr_overflow {
                incr = 1 << 31;
            }

            if base_overflow {
                incr = 1 << (incr | base | 1 << 31).trailing_zeros().min(31);
                Self { base: 0, incr }
            } else {
                if incr != 0 {
                    incr = 1 << incr.trailing_zeros().min(31);
                }
                Self { base, incr }
            }
        }
    }
}

/// Some computed alignment information.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ComplexAlignment {
    /// Alignment at the beginning of the structure.
    pub begin: Alignment,
    /// Alignment at the end of the structure.
    pub body: AlignBody,
    /// Internal alignments made inside the object using
    /// align pads.
    // FIXME: What does that mean?
    pub internal_align: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlignBody {
    Size(VariableSize),
    EndAlign(Alignment),
}

impl ComplexAlignment {
    #[inline]
    pub fn fixed_size(size: u32, align: u32) -> Self {
        Self {
            begin: Alignment { align, offset: 0 },
            body: AlignBody::Size(VariableSize {
                base: size,
                incr: 0,
            }),
            internal_align: 1,
        }
    }

    #[inline]
    pub fn zero_sized() -> Self {
        Self::fixed_size(0, 1)
    }

    pub fn end_align(self) -> Alignment {
        match self.body {
            AlignBody::Size(size) => self.begin.advance_variable_size(size),
            AlignBody::EndAlign(end_align) => end_align,
        }
    }

    pub fn zero_one_or_many(self) -> Option<Self> {
        if !self.end_align().meets(self.begin) {
            None
        } else {
            match self.body {
                AlignBody::Size(size) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::Size(size.zero_one_or_many()),
                    internal_align: self.internal_align,
                }),
                AlignBody::EndAlign(end_align) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::EndAlign(self.begin.union(end_align)?),
                    internal_align: self.internal_align,
                }),
            }
        }
    }

    pub fn repeat_n(self, n: u32) -> Option<Self> {
        if n == 0 {
            Some(Self {
                begin: self.begin,
                body: AlignBody::Size(VariableSize::zero()),
                internal_align: self.internal_align,
            })
        } else if n > 1 && !self.end_align().meets(self.begin) {
            None
        } else {
            match self.body {
                AlignBody::Size(size) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::Size(size.repeat_n(n)),
                    internal_align: self.internal_align,
                }),
                AlignBody::EndAlign(end_align) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::EndAlign(end_align),
                    internal_align: self.internal_align,
                }),
            }
        }
    }

    pub fn append(self, next: Self) -> Option<Self> {
        if !self.end_align().meets(next.begin) {
            None
        } else {
            let new_body = match (self.body, next.body) {
                (AlignBody::Size(curr_size), AlignBody::Size(next_size)) => {
                    AlignBody::Size(curr_size.append(next_size))
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::Size(next_size)) => {
                    AlignBody::EndAlign(curr_end_align.advance_variable_size(next_size))
                }
                (_, AlignBody::EndAlign(next_end_align)) => AlignBody::EndAlign(next_end_align),
            };
            Some(Self {
                begin: self.begin,
                body: new_body,
                internal_align: self.internal_align.max(next.internal_align),
            })
        }
    }

    pub fn union_append(self, other: Self) -> Option<Self> {
        if !self.begin.meets(other.begin) {
            None
        } else {
            let new_body = match (self.body, other.body) {
                (AlignBody::Size(curr_size), AlignBody::Size(next_size)) => {
                    AlignBody::Size(curr_size.union(next_size))
                }
                (AlignBody::Size(curr_size), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(
                        self.begin
                            .advance_variable_size(curr_size)
                            .intersection(next_end_align),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::Size(next_size)) => {
                    AlignBody::EndAlign(
                        curr_end_align.intersection(self.begin.advance_variable_size(next_size)),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(curr_end_align.intersection(next_end_align))
                }
            };
            Some(Self {
                begin: self.begin,
                body: new_body,
                internal_align: self.internal_align.max(other.internal_align),
            })
        }
    }

    pub fn bitcase_append(self, next: Self) -> Option<Self> {
        if !self.end_align().meets(next.begin) {
            None
        } else {
            let new_body = match (self.body, next.body) {
                (AlignBody::Size(curr_size), AlignBody::Size(next_size)) => {
                    AlignBody::Size(curr_size.union(curr_size.append(next_size)))
                }
                (AlignBody::Size(curr_size), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(
                        self.begin
                            .advance_variable_size(curr_size)
                            .intersection(next_end_align),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::Size(next_size)) => {
                    AlignBody::EndAlign(
                        curr_end_align
                            .intersection(curr_end_align.advance_variable_size(next_size)),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(curr_end_align.intersection(next_end_align))
                }
            };
            Some(Self {
                begin: self.begin,
                body: new_body,
                internal_align: self.internal_align.max(next.internal_align),
            })
        }
    }
}

/// Some field of a complex type.
#[derive(Debug)]
pub enum FieldDef {
    /// A `<pad>`.
    Pad(PadField),

    /// A `<field>`.
    Normal(NormalField),

    /// A `<list>` that is not a list of file descripts.
    List(ListField),

    /// A `<switch>`.
    Switch(SwitchField),

    /// A `<fd>`.
    Fd(FdField),

    /// A `<list>` containing file descriptors.
    FdList(FdListField),

    /// A `<exprfield>`.
    Expr(ExprField),

    /// An invented length field for a list that has no length.
    ///
    /// Fields of this kind are synthesised while resolving.
    VirtualLen(VirtualLenField),
}

impl FieldDef {
    /// Get the name of the field, if it has a name.
    ///
    /// This returns the `name` attribute of the original field. `<pad>`s do not have a name. For a
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Pad(_) => None,
            Self::Normal(normal_field) => Some(&normal_field.name),
            Self::List(list_field) => Some(&list_field.name),
            Self::Switch(switch_field) => Some(&switch_field.name),
            Self::Fd(fd_field) => Some(&fd_field.name),
            Self::FdList(fd_list_field) => Some(&fd_list_field.name),
            Self::Expr(expr_field) => Some(&expr_field.name),
            Self::VirtualLen(virtual_len_field) => Some(&virtual_len_field.name),
        }
    }

    /// Get the type of the field.
    ///
    /// * For `<field>`s and `<exprfield>`s, this returns the type of the field.
    /// * For `<list>`s, this returns the element type of the list.
    /// * For virtual length fields, this returns the type of the length, which is a `CARD32`.
    pub fn value_type(&self) -> Option<&FieldValueType> {
        match self {
            Self::Pad(_) => None,
            Self::Normal(normal_field) => Some(&normal_field.type_),
            Self::List(list_field) => Some(&list_field.element_type),
            Self::Switch(_) => None,
            Self::Fd(_) => None,
            Self::FdList(_) => None,
            Self::Expr(expr_field) => Some(&expr_field.type_),
            Self::VirtualLen(virtual_len_field) => Some(&virtual_len_field.type_),
        }
    }

    pub fn size(&self) -> Option<u32> {
        match self {
            Self::Pad(pad_field) => match pad_field.kind {
                PadKind::Align(_) => None,
                PadKind::Bytes(pad_size) => Some(u32::from(pad_size)),
            },
            Self::Normal(normal_field) => normal_field.type_.size(),
            Self::List(list_field) => {
                let element_size = list_field.element_type.size()?;
                let list_len = list_field.length()?;
                Some(element_size * list_len)
            }
            Self::Switch(switch_field) => switch_field.size(),
            // fds are not serialized, they are sent with fd passing
            Self::Fd(_) => Some(0),
            Self::FdList(_) => Some(0),
            Self::Expr(expr_field) => expr_field.type_.size(),
            // not serialized
            Self::VirtualLen(_) => Some(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PadField {
    pub kind: PadKind,
    pub serialize: bool,
}

#[derive(Debug)]
pub struct NormalField {
    pub name: String,
    pub type_: FieldValueType,
}

#[derive(Debug)]
pub struct ListField {
    pub name: String,
    pub element_type: FieldValueType,
    pub length_expr: Option<Expression>,
}

impl ListField {
    pub fn has_fixed_length(&self) -> bool {
        self.length().is_some()
    }

    pub fn length(&self) -> Option<u32> {
        if let Some(ref length_expr) = self.length_expr {
            match length_expr {
                Expression::Value(v) => Some(u32::try_from(*v).unwrap()),
                Expression::Bit(bit) => Some(1 << *bit),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct SwitchField {
    pub name: String,
    pub expr: Expression,
    pub required_start_align: Option<Alignment>,
    pub alignment: OnceCell<ComplexAlignment>,
    pub kind: SwitchKind,
    pub cases: Vec<SwitchCase>,
    pub external_params: RefCell<Vec<ExternalParam>>,
}

impl SwitchField {
    pub fn size(&self) -> Option<u32> {
        if self.kind != SwitchKind::Case {
            return None;
        }

        let mut size = None;
        for case in self.cases.iter() {
            let case_size = case.size()?;
            if let Some(size) = size {
                if size != case_size {
                    return None;
                }
            } else {
                size = Some(case_size);
            }
        }
        size
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwitchKind {
    BitCase,
    Case,
}

#[derive(Debug)]
pub struct SwitchCase {
    pub name: Option<String>,
    pub exprs: Vec<Expression>,
    pub required_start_align: Option<Alignment>,
    pub alignment: OnceCell<ComplexAlignment>,
    pub fields: RefCell<Vec<FieldDef>>,
    pub external_params: RefCell<Vec<ExternalParam>>,
}

impl SwitchCase {
    fn size(&self) -> Option<u32> {
        self.fields
            .borrow()
            .iter()
            .try_fold(0, |sz, field| Some(sz + field.size()?))
    }
}

#[derive(Debug)]
pub struct FdField {
    pub name: String,
}

#[derive(Debug)]
pub struct FdListField {
    pub name: String,
    pub length_expr: Expression,
}

impl FdListField {
    pub fn length(&self) -> Option<u32> {
        match self.length_expr {
            Expression::Value(v) => Some(u32::try_from(v).unwrap()),
            Expression::Bit(bit) => Some(1 << bit),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ExprField {
    pub name: String,
    pub type_: FieldValueType,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct VirtualLenField {
    pub name: String,
    // FIXME: This is always BuiltInType::Card32, so this field can be removed
    pub type_: FieldValueType,
    pub list_name: String,
}

#[derive(Debug)]
pub struct FieldValueType {
    pub type_: NamedTypeRef,
    pub value_set: FieldValueSet,
}

impl FieldValueType {
    pub fn size(&self) -> Option<u32> {
        self.type_.def.get().unwrap().size()
    }
}

#[derive(Debug)]
pub enum FieldValueSet {
    None,
    Enum(NamedTypeRef),
    AltEnum(NamedTypeRef),
    Mask(NamedTypeRef),
    AltMask(NamedTypeRef),
}

#[derive(Clone, Debug)]
pub enum PadKind {
    Bytes(u16),
    Align(u16),
}

#[derive(Clone, Debug)]
pub enum TypeRef {
    BuiltIn(BuiltInType),
    Struct(Weak<StructDef>),
    Union(Weak<UnionDef>),
    EventStruct(Weak<EventStructDef>),
    Xid(Weak<XidTypeDef>),
    XidUnion(Weak<XidUnionDef>),
    Enum(Weak<EnumDef>),
    Alias(Weak<TypeAliasDef>),
}

impl TypeRef {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInType {
    Card8,
    Card16,
    Card32,
    Card64,
    Int8,
    Int16,
    Int32,
    Int64,
    Byte,
    Bool,
    Char,
    Float,
    Double,
    Void,
}

impl BuiltInType {
    pub fn size(self) -> u32 {
        match self {
            Self::Card8 => 1,
            Self::Card16 => 2,
            Self::Card32 => 4,
            Self::Card64 => 8,
            Self::Int8 => 1,
            Self::Int16 => 2,
            Self::Int32 => 4,
            Self::Int64 => 8,
            Self::Byte => 1,
            Self::Bool => 1,
            Self::Char => 1,
            Self::Float => 4,
            Self::Double => 8,
            Self::Void => 1,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    BinaryOp(BinaryOpExpr),
    UnaryOp(UnaryOpExpr),
    FieldRef(FieldRefExpr),
    ParamRef(ParamRefExpr),
    EnumRef(EnumRefExpr),
    PopCount(Box<Expression>),
    SumOf(SumOfExpr),
    ListElementRef,
    Value(u32),
    Bit(u8),
}

#[derive(Debug)]
pub struct BinaryOpExpr {
    pub operator: BinaryOperator,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Shl,
}

#[derive(Debug)]
pub struct UnaryOpExpr {
    pub operator: UnaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
}

#[derive(Debug)]
pub struct FieldRefExpr {
    pub field_name: String,
    pub resolved: OnceCell<ResolvedFieldRef>,
}

#[derive(Debug)]
pub struct ResolvedFieldRef {
    pub ref_kind: FieldRefKind,
    pub field_type: TypeRef,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FieldRefKind {
    LocalField,
    ExtParam,
    SumOfRef,
}

#[derive(Debug)]
pub struct ParamRefExpr {
    pub field_name: String,
    pub type_: NamedTypeRef,
}

#[derive(Debug)]
pub struct EnumRefExpr {
    pub enum_: NamedTypeRef,
    pub variant: String,
}

#[derive(Debug)]
pub struct SumOfExpr {
    pub field_name: String,
    pub resolved_field: OnceCell<ResolvedFieldRef>,
    pub operand: Option<Box<Expression>>,
}

#[derive(Clone, Debug)]
pub struct Doc {
    pub brief: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
    pub fields: Vec<FieldDoc>,
    pub errors: Vec<ErrorDoc>,
    pub sees: Vec<SeeDoc>,
}

#[derive(Clone, Debug)]
pub struct FieldDoc {
    pub name: String,
    pub doc: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ErrorDoc {
    pub type_: String,
    pub doc: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SeeDoc {
    pub type_: String,
    pub name: String,
}

// Replacement for `Weak::ptr_eq`, which requires
// Rust 1.39.
fn weak_ptr_eq<T>(weak1: &Weak<T>, weak2: &Weak<T>) -> bool {
    Rc::ptr_eq(&weak1.upgrade().unwrap(), &weak2.upgrade().unwrap())
}
