use std::cell::RefCell;
use std::collections::hash_map::Entry as HashMapEntry;
use std::convert::TryFrom;
use std::rc::{Rc, Weak};

use fxhash::FxHashMap;
use once_cell::unsync::OnceCell;

#[derive(Debug)]
pub struct Module {
    pub namespaces: RefCell<FxHashMap<String, Rc<Namespace>>>,
}

impl Module {
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

    pub fn namespace(&self, header: &str) -> Option<Rc<Namespace>> {
        self.namespaces.borrow().get(header).cloned()
    }

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
}

#[derive(Clone, Debug)]
pub struct ExtInfo {
    pub xname: String,
    pub name: String,
    pub multiword: bool,
    pub major_version: u16,
    pub minor_version: u16,
}

#[derive(Debug)]
pub struct Namespace {
    pub module: Weak<Module>,
    pub header: String,
    pub ext_info: Option<ExtInfo>,
    pub imports: RefCell<FxHashMap<String, Import>>,
    pub request_defs: RefCell<FxHashMap<String, Rc<RequestDef>>>,
    pub event_defs: RefCell<FxHashMap<String, EventDef>>,
    pub error_defs: RefCell<FxHashMap<String, ErrorDef>>,
    pub type_defs: RefCell<FxHashMap<String, TypeDef>>,
    pub src_order_defs: RefCell<Vec<Def>>,
}

impl Namespace {
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

    pub fn add_import(&self, import_name: String) {
        self.imports.borrow_mut().insert(
            import_name.clone(),
            Import {
                name: import_name,
                ns: OnceCell::new(),
            },
        );
    }

    /// Returns `false` if the name is already in use.
    pub fn insert_request_def(&self, name: String, request_def: Rc<RequestDef>) -> bool {
        match self.request_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                entry.insert(request_def);
                true
            }
        }
    }

    /// Returns `false` if the name is already in use.
    pub fn insert_event_def(&self, name: String, event_def: EventDef) -> bool {
        match self.event_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                entry.insert(event_def);
                true
            }
        }
    }

    /// Returns `false` if the name is already in use.
    pub fn insert_error_def(&self, name: String, error_def: ErrorDef) -> bool {
        match self.error_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                entry.insert(error_def);
                true
            }
        }
    }

    /// Returns `false` if the name is already in use.
    pub fn insert_type_def(&self, name: String, type_def: TypeDef) -> bool {
        match self.type_defs.borrow_mut().entry(name) {
            HashMapEntry::Occupied(_) => false,
            HashMapEntry::Vacant(entry) => {
                entry.insert(type_def);
                true
            }
        }
    }

    pub fn get_import(&self, name: &str) -> Option<Rc<Namespace>> {
        self.imports.borrow().get(name).map(|import| import.ns())
    }

    pub fn get_event(&self, name: &str) -> Option<EventRef> {
        self.event_defs
            .borrow()
            .get(name)
            .map(|event_def| match event_def {
                EventDef::Full(def) => EventRef::Full(Rc::downgrade(def)),
                EventDef::Copy(def) => EventRef::Copy(Rc::downgrade(def)),
            })
    }

    pub fn get_error(&self, name: &str) -> Option<ErrorRef> {
        self.error_defs
            .borrow()
            .get(name)
            .map(|error_def| match error_def {
                ErrorDef::Full(def) => ErrorRef::Full(Rc::downgrade(def)),
                ErrorDef::Copy(def) => ErrorRef::Copy(Rc::downgrade(def)),
            })
    }

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

#[derive(Debug)]
pub struct Import {
    pub name: String,
    pub ns: OnceCell<Weak<Namespace>>,
}

impl Import {
    pub fn ns(&self) -> Rc<Namespace> {
        self.ns.get().unwrap().upgrade().unwrap()
    }
}

#[derive(Debug)]
pub enum Def {
    Request(Rc<RequestDef>),
    Event(EventDef),
    Error(ErrorDef),
    Type(TypeDef),
}

#[derive(Debug)]
pub struct RequestDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub opcode: u8,
    pub combine_adjacent: bool,
    pub required_start_align: Option<Alignment>,
    pub fields: RefCell<Vec<FieldDef>>,
    pub reply: Option<Rc<ReplyDef>>,
    pub doc: Option<Doc>,
}

#[derive(Debug)]
pub struct ReplyDef {
    pub request: OnceCell<Weak<RequestDef>>,
    pub required_start_align: Option<Alignment>,
    pub fields: RefCell<Vec<FieldDef>>,
    pub doc: Option<Doc>,
}

#[derive(Clone, Debug)]
pub enum EventDef {
    Full(Rc<EventFullDef>),
    Copy(Rc<EventCopyDef>),
}

impl EventDef {
    pub fn name(&self) -> &str {
        match self {
            Self::Full(event_full_def) => &event_full_def.name,
            Self::Copy(event_copy_def) => &event_copy_def.name,
        }
    }

    pub fn namespace(&self) -> Rc<Namespace> {
        match self {
            Self::Full(event_full_def) => event_full_def.namespace.upgrade().unwrap(),
            Self::Copy(event_copy_def) => event_copy_def.namespace.upgrade().unwrap(),
        }
    }

    pub fn get_original_full_def(&self) -> Rc<EventFullDef> {
        match self {
            Self::Full(event_full_def) => event_full_def.clone(),
            Self::Copy(event_copy_def) => {
                let mut event_copy_def = event_copy_def.clone();
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

    pub fn is_xge(&self) -> bool {
        self.get_original_full_def().xge
    }

    pub fn as_event_ref(&self) -> EventRef {
        match self {
            Self::Full(event_full_def) => EventRef::Full(Rc::downgrade(event_full_def)),
            Self::Copy(event_copy_def) => EventRef::Copy(Rc::downgrade(event_copy_def)),
        }
    }
}

#[derive(Debug)]
pub struct EventFullDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub number: u16,
    pub no_sequence_number: bool,
    pub xge: bool,
    pub required_start_align: Option<Alignment>,
    pub fields: RefCell<Vec<FieldDef>>,
    pub doc: Option<Doc>,
}

#[derive(Debug)]
pub struct EventCopyDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub number: u16,
    pub ref_: NamedEventRef,
}

impl EventCopyDef {
    pub fn get_original_full_def(&self) -> Rc<EventFullDef> {
        self.ref_.def.get().unwrap().get_original_full_def()
    }
}

#[derive(Clone, Debug)]
pub enum ErrorDef {
    Full(Rc<ErrorFullDef>),
    Copy(Rc<ErrorCopyDef>),
}

impl ErrorDef {
    pub fn name(&self) -> &str {
        match self {
            Self::Full(error_full_def) => &error_full_def.name,
            Self::Copy(error_copy_def) => &error_copy_def.name,
        }
    }

    pub fn namespace(&self) -> Rc<Namespace> {
        match self {
            Self::Full(error_full_def) => error_full_def.namespace.upgrade().unwrap(),
            Self::Copy(error_copy_def) => error_copy_def.namespace.upgrade().unwrap(),
        }
    }

    pub fn get_original_full_def(&self) -> Rc<ErrorFullDef> {
        match self {
            Self::Full(error_full_def) => error_full_def.clone(),
            Self::Copy(error_copy_def) => {
                let mut error_copy_def = error_copy_def.clone();
                loop {
                    match error_copy_def.ref_.def.get().unwrap() {
                        ErrorRef::Full(error_full_def) => return error_full_def.upgrade().unwrap(),
                        ErrorRef::Copy(event_copy_ref) => {
                            error_copy_def = event_copy_ref.upgrade().unwrap()
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ErrorFullDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    // Signed because there are some `-1` somewhere.
    pub number: i16,
    pub required_start_align: Option<Alignment>,
    pub fields: RefCell<Vec<FieldDef>>,
}

#[derive(Debug)]
pub struct ErrorCopyDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub number: i16,
    pub ref_: NamedErrorRef,
}

impl ErrorCopyDef {
    pub fn get_original_full_def(&self) -> Rc<ErrorFullDef> {
        self.ref_.def.get().unwrap().get_original_full_def()
    }
}

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

#[derive(Debug)]
pub struct StructDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub alignment: OnceCell<ComplexAlignment>,
    pub fields: RefCell<Vec<FieldDef>>,
    pub external_params: RefCell<Vec<ExternalParam>>,
}

impl StructDef {
    pub fn size(&self) -> Option<u32> {
        self.fields
            .borrow()
            .iter()
            .try_fold(0, |sz, field| Some(sz + field.size()?))
    }
}

#[derive(Debug)]
pub struct UnionDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub alignment: OnceCell<ComplexAlignment>,
    pub fields: Vec<FieldDef>,
}

impl UnionDef {
    pub fn size(&self) -> u32 {
        self.fields
            .iter()
            .map(|field| field.size().unwrap())
            .max()
            .unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct EventStructDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub alloweds: Vec<EventStructAllowed>,
}

#[derive(Clone, Debug)]
pub struct EventStructAllowed {
    pub extension: String,
    pub xge: bool,
    pub opcode_min: u16,
    pub opcode_max: u16,
    pub resolved: RefCell<Vec<EventRef>>,
}

#[derive(Debug)]
pub struct XidTypeDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
}

#[derive(Debug)]
pub struct XidUnionDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub types: Vec<NamedTypeRef>,
}

#[derive(Debug)]
pub struct EnumDef {
    pub namespace: Weak<Namespace>,
    pub name: String,
    pub items: Vec<EnumItem>,
    pub doc: Option<Doc>,
}

#[derive(Debug)]
pub struct EnumItem {
    pub name: String,
    pub value: EnumValue,
}

#[derive(Debug)]
pub enum EnumValue {
    Value(u32),
    Bit(u8),
}

#[derive(Debug)]
pub struct TypeAliasDef {
    pub namespace: Weak<Namespace>,
    pub new_name: String,
    pub old_name: NamedTypeRef,
}

impl TypeAliasDef {
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

#[derive(Clone, Debug)]
pub struct ExternalParam {
    pub name: String,
    pub type_: TypeRef,
}

#[derive(Debug)]
pub struct NamedEventRef {
    pub name: String,
    pub def: OnceCell<EventRef>,
}

impl NamedEventRef {
    pub fn unresolved(name: String) -> Self {
        Self {
            name,
            def: OnceCell::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EventRef {
    Full(Weak<EventFullDef>),
    Copy(Weak<EventCopyDef>),
}

impl EventRef {
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

    pub fn is_xge(&self) -> bool {
        self.get_original_full_def().xge
    }

    pub fn as_event_def(&self) -> EventDef {
        match self {
            Self::Full(event_full_def) => EventDef::Full(event_full_def.upgrade().unwrap()),
            Self::Copy(event_copy_def) => EventDef::Copy(event_copy_def.upgrade().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct NamedErrorRef {
    pub name: String,
    pub def: OnceCell<ErrorRef>,
}

impl NamedErrorRef {
    pub fn unresolved(name: String) -> Self {
        Self {
            name,
            def: OnceCell::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ErrorRef {
    Full(Weak<ErrorFullDef>),
    Copy(Weak<ErrorCopyDef>),
}

impl ErrorRef {
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

#[derive(Debug)]
pub struct NamedTypeRef {
    pub name: String,
    pub def: OnceCell<TypeRef>,
}

impl NamedTypeRef {
    pub fn unresolved(name: String) -> Self {
        Self {
            name,
            def: OnceCell::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    pub align: u32,
    pub offset: u32,
}

impl Alignment {
    #[inline]
    pub fn min() -> Self {
        Self {
            align: 1,
            offset: 0,
        }
    }

    #[inline]
    pub fn max() -> Self {
        Self {
            align: Self::max_align(),
            offset: 0,
        }
    }

    #[inline]
    pub const fn max_align() -> u32 {
        1 << 31
    }

    pub fn advance_fixed_size(self, size: u32) -> Self {
        Self {
            align: self.align,
            offset: self.offset.wrapping_add(size) % self.align,
        }
    }

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
                if (self.offset & other.align) != other.align {
                    None
                } else {
                    Some(self)
                }
            }
        }
    }

    /// Returns an alignment that is met by `self` and `other`.
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
        self.align >= required.align && (self.offset % required.align) == required.offset
    }
}

/// Represents the size of an object.
///
/// `size = base + incr * n`
///
/// `incr` must be zero or a power of 2
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableSize {
    pub base: u32,
    pub incr: u32,
}

impl VariableSize {
    fn incr_union(incr1: u32, incr2: u32) -> u32 {
        if incr1 == 0 {
            incr2
        } else if incr2 == 0 {
            incr1
        } else {
            incr1.min(incr2)
        }
    }

    fn reduce_base(base: u32, incr: u32) -> u32 {
        if incr == 0 {
            base
        } else {
            base % incr
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self { base: 0, incr: 0 }
    }

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
                Self {
                    base: 0,
                    incr: 1u32 << (base1 | base2).trailing_zeros().min(31),
                }
            } else {
                let min_base = base1.min(base2);
                let max_base = base1.max(base2);
                let incr1 = 1u32 << min_base.trailing_zeros().min(31);
                let incr2 = 1u32 << (max_base - min_base).trailing_zeros().min(31);
                if incr1 > incr2 {
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

    pub fn zero_or_one(self) -> Self {
        let base = Self::reduce_base(self.base, self.incr);
        if base == 0 {
            Self {
                base: 0,
                incr: self.incr,
            }
        } else {
            Self {
                base: 0,
                incr: 1u32 << base.trailing_zeros().min(31),
            }
        }
    }

    pub fn zero_one_or_many(self) -> Self {
        let base = Self::reduce_base(self.base, self.incr);
        if base == 0 {
            Self {
                base: 0,
                incr: self.incr,
            }
        } else {
            Self {
                base: 0,
                incr: 1u32 << base.trailing_zeros().min(31),
            }
        }
    }

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ComplexAlignment {
    /// Alignment at the beginning of the structure.
    pub begin: Alignment,
    /// Alignment at the end of the structure.
    pub body: AlignBody,
    /// Internal alignments made inside the object using
    /// align pads.
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

    pub fn zero_or_one(self) -> Self {
        match self.body {
            AlignBody::Size(size) => Self {
                begin: self.begin,
                body: AlignBody::Size(size.zero_or_one()),
                internal_align: self.internal_align,
            },
            AlignBody::EndAlign(end_align) => Self {
                begin: self.begin,
                body: AlignBody::EndAlign(self.begin.intersection(end_align)),
                internal_align: self.internal_align,
            },
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

#[derive(Debug)]
pub enum FieldDef {
    Pad(PadField),
    Normal(NormalField),
    List(ListField),
    Switch(SwitchField),
    Fd(FdField),
    FdList(FdListField),
    Expr(ExprField),
    VirtualLen(VirtualLenField),
}

impl FieldDef {
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
        if let Some(ref length_expr) = self.length_expr {
            match length_expr {
                Expression::Value(_) | Expression::Bit(_) => true,
                _ => false,
            }
        } else {
            false
        }
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
