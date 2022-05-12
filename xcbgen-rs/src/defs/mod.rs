//! Definitions describing the structure of the XML protocol and its extension.
//!
//! This module contains the [`Module`] type. A [`Module`] contains some [`Namespace`]s, where each
//! [`Namespace`] corresponds to one XML file from the xcb-proto project. Each XML file describes
//! one X11 extension or the core protocol.

use std::cell::RefCell;
use std::collections::hash_map::Entry as HashMapEntry;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use once_cell::unsync::OnceCell;

mod alignment;
mod doc;
mod expression;
mod fields;
mod top_level;

pub use alignment::*;
pub use doc::*;
pub use expression::*;
pub use fields::*;
pub use top_level::*;

/// A `Module` contains a collection of namespaces.
#[derive(Debug)]
pub struct Module {
    /// All namespaces in this module
    pub namespaces: RefCell<HashMap<String, Rc<Namespace>>>,
}

impl Module {
    /// Create a new, empty module
    pub fn new() -> Rc<Self> {
        Rc::new(Module {
            namespaces: RefCell::new(HashMap::new()),
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
                ns.ext_info
                    .as_ref()
                    .map(|ext_info| ext_info.name == name)
                    .unwrap_or(false)
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
    /// This should only be `None` for the core X11 protocol `xproto`.
    pub ext_info: Option<ExtInfo>,

    /// Other namespaces that are imported into this namespace.
    pub imports: RefCell<HashMap<String, Import>>,

    /// The requests that are defined in this module.
    pub request_defs: RefCell<HashMap<String, Rc<RequestDef>>>,

    /// The events that are defined in this module.
    pub event_defs: RefCell<HashMap<String, EventDef>>,

    /// The errors that are defined in this module.
    pub error_defs: RefCell<HashMap<String, ErrorDef>>,

    /// The types that are defined in this module.
    pub type_defs: RefCell<HashMap<String, TypeDef>>,

    /// All definitions in this module in the order they appear in the XML description.
    pub src_order_defs: RefCell<Vec<Def>>,

    /// Does this module contain any file descriptors, or depend on any
    /// that do?
    contains_fds: OnceCell<bool>,
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
            imports: RefCell::new(HashMap::new()),
            request_defs: RefCell::new(HashMap::new()),
            event_defs: RefCell::new(HashMap::new()),
            error_defs: RefCell::new(HashMap::new()),
            type_defs: RefCell::new(HashMap::new()),
            src_order_defs: RefCell::new(Vec::new()),
            contains_fds: OnceCell::new(),
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

    /// Tell if the namespace contains file descriptors, or depend on any
    /// modules containing file descriptors.
    pub fn contains_fds(&self) -> bool {
        *self.contains_fds.get_or_init(|| {
            // iterate over the types we have to see if any of them
            // use fd fields
            // only requests and replies will have fds
            let request_defs = self.request_defs.borrow();

            let has_fd = request_defs.values().any(|request| {
                // combine request/reply fields into a single iterator
                let request_fields = request.fields.borrow();
                let reply_fields = request.reply.as_ref().map(|reply| reply.fields.borrow());

                request_fields
                    .iter()
                    .chain(reply_fields.iter().flat_map(|i| i.iter()))
                    .any(|field| matches!(field, FieldDef::Fd(_) | FieldDef::FdList(_)))
            });

            // see if any of our dependent modules contain fds
            let imports = self.imports.borrow();
            let dependencies_have_fds = imports
                .values()
                .filter_map(|import| import.ns.get())
                .filter_map(|ns| ns.upgrade())
                .any(|ns| ns.contains_fds());

            has_fd || dependencies_have_fds
        })
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

/// Information about a `<paramref>` that is contained in some expression.
#[derive(Clone, Debug)]
pub struct ExternalParam {
    /// The name of the parameter that is referenced.
    pub name: String,

    /// The type of the parameter that is referenced.
    pub type_: TypeRef,
}
