use std::collections::hash_map::Entry as HashMapEntry;
use std::collections::HashMap;

use xcbgen::defs as xcbdefs;

#[derive(Copy, Clone, Debug)]
pub(crate) struct EnumInfo {
    // The size needed to hold all defined values for the enum
    pub(super) max_value_size: Option<u8>,
    // The range of wire sizes used for the enum
    // min, max
    pub(super) wire_size: Option<(u8, u8)>,
}

pub(super) fn default_debug_impl(name: &str, out: &mut crate::generator::Output) {
    outln!(
        out,
        "impl_debug_if_no_extra_traits!({}, \"{}\");",
        name,
        name
    );
}

/// Caches to avoid repeating some operations.
#[derive(Default)]
pub(crate) struct Caches {
    enum_infos: HashMap<usize, EnumInfo>,
    pub(super) derives: HashMap<usize, Derives>,
    pub(super) rust_type_names: HashMap<usize, String>,
}

impl Caches {
    #[inline]
    pub(crate) fn enum_info(&self, enum_def: &xcbdefs::EnumDef) -> EnumInfo {
        self.enum_infos[&(enum_def as *const xcbdefs::EnumDef as usize)]
    }

    fn put_enum_max_value_size(&mut self, enum_def: &xcbdefs::EnumDef, max_value_size: u8) {
        let id = enum_def as *const xcbdefs::EnumDef as usize;
        match self.enum_infos.entry(id) {
            HashMapEntry::Vacant(entry) => {
                entry.insert(EnumInfo {
                    max_value_size: Some(max_value_size),
                    wire_size: None,
                });
            }
            HashMapEntry::Occupied(mut entry) => {
                let entry_value = entry.get_mut();
                if entry_value.max_value_size.is_none() {
                    entry_value.max_value_size = Some(max_value_size);
                } else {
                    // Expected only once
                    unreachable!();
                }
            }
        }
    }

    fn put_enum_wire_size(&mut self, enum_def: &xcbdefs::EnumDef, wire_size: u8) {
        let id = enum_def as *const xcbdefs::EnumDef as usize;
        match self.enum_infos.entry(id) {
            HashMapEntry::Vacant(entry) => {
                entry.insert(EnumInfo {
                    max_value_size: None,
                    wire_size: Some((wire_size, wire_size)),
                });
            }
            HashMapEntry::Occupied(mut entry) => {
                let entry_value = entry.get_mut();
                match entry_value.wire_size {
                    None => entry_value.wire_size = Some((wire_size, wire_size)),
                    Some((ref mut min, ref mut max)) => {
                        *min = (*min).min(wire_size);
                        *max = (*max).max(wire_size)
                    }
                }
            }
        }
    }

    pub(crate) fn gather_enum_infos(&mut self, module: &xcbdefs::Module) {
        for ns in module.namespaces.borrow().values() {
            for request_def in ns.request_defs.borrow().values() {
                for field in request_def.fields.borrow().iter() {
                    self.gather_enum_infos_in_field(field);
                }
                if let Some(ref reply_def) = request_def.reply {
                    for field in reply_def.fields.borrow().iter() {
                        self.gather_enum_infos_in_field(field);
                    }
                }
            }

            for event_def in ns.event_defs.borrow().values() {
                match event_def {
                    xcbdefs::EventDef::Full(event_def) => {
                        for field in event_def.fields.borrow().iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::EventDef::Copy(_) => {}
                }
            }

            for error_def in ns.error_defs.borrow().values() {
                match error_def {
                    xcbdefs::ErrorDef::Full(error_def) => {
                        for field in error_def.fields.borrow().iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::ErrorDef::Copy(_) => {}
                }
            }

            for type_def in ns.type_defs.borrow().values() {
                match type_def {
                    xcbdefs::TypeDef::Struct(struct_def) => {
                        for field in struct_def.fields.borrow().iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::TypeDef::Union(union_def) => {
                        for field in union_def.fields.iter() {
                            self.gather_enum_infos_in_field(field);
                        }
                    }
                    xcbdefs::TypeDef::EventStruct(_) => {}
                    xcbdefs::TypeDef::Xid(_) => {}
                    xcbdefs::TypeDef::XidUnion(_) => {}
                    xcbdefs::TypeDef::Enum(enum_def) => {
                        self.gather_enum_infos_in_enum_def(enum_def);
                    }
                    xcbdefs::TypeDef::Alias(_) => {}
                }
            }
        }
    }

    fn gather_enum_infos_in_field(&mut self, field: &xcbdefs::FieldDef) {
        match field {
            xcbdefs::FieldDef::Pad(_) => {}
            xcbdefs::FieldDef::Normal(normal_field) => {
                self.gather_enum_infos_in_field_value_type(&normal_field.type_);
            }
            xcbdefs::FieldDef::List(list_field) => {
                self.gather_enum_infos_in_field_value_type(&list_field.element_type);
            }
            xcbdefs::FieldDef::Switch(switch_field) => {
                for case in switch_field.cases.iter() {
                    for field in case.fields.borrow().iter() {
                        self.gather_enum_infos_in_field(field);
                    }
                }
            }
            xcbdefs::FieldDef::Fd(_) => {}
            xcbdefs::FieldDef::FdList(_) => {}
            xcbdefs::FieldDef::Expr(expr_field) => {
                self.gather_enum_infos_in_field_value_type(&expr_field.type_);
            }
            xcbdefs::FieldDef::VirtualLen(_) => {}
        }
    }

    fn gather_enum_infos_in_field_value_type(&mut self, value_type: &xcbdefs::FieldValueType) {
        match value_type.value_set {
            xcbdefs::FieldValueSet::None => {}
            xcbdefs::FieldValueSet::Enum(ref enum_type)
            | xcbdefs::FieldValueSet::Mask(ref enum_type) => {
                let enum_def = match enum_type.get_resolved().get_original_type() {
                    xcbdefs::TypeRef::Enum(enum_type) => enum_type.upgrade().unwrap(),
                    _ => unreachable!(),
                };

                let size = match value_type.type_.get_resolved().get_original_type() {
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Bool) => 1,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card8) => 8,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card16) => 16,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Card32) => 32,
                    xcbdefs::TypeRef::BuiltIn(xcbdefs::BuiltInType::Byte) => 8,
                    _ => unreachable!(),
                };
                self.put_enum_wire_size(&enum_def, size);
            }
            xcbdefs::FieldValueSet::AltEnum(_) => {}
            xcbdefs::FieldValueSet::AltMask(_) => {}
        }
    }

    fn gather_enum_infos_in_enum_def(&mut self, enum_def: &xcbdefs::EnumDef) {
        // Get the maximum value of the defined variants for this enum
        let max_value = enum_def
            .items
            .iter()
            .map(|enum_item| match enum_item.value {
                xcbdefs::EnumValue::Value(value) => value,
                xcbdefs::EnumValue::Bit(bit) => 1 << bit,
            })
            .max()
            .unwrap();

        let size = if max_value == 1 && enum_def.items.len() == 2 {
            1
        } else if max_value <= 0xFF {
            8
        } else if max_value <= 0xFFFF {
            16
        } else {
            32
        };

        self.put_enum_max_value_size(enum_def, size);
    }
}

#[derive(Copy, Clone)]
pub(super) struct Derives {
    pub(super) debug: bool,
    pub(super) clone: bool,
    pub(super) copy: bool,
    /// "default" is a reserved keyword
    pub(super) default_: bool,
    pub(super) partial_eq: bool,
    pub(super) eq: bool,
    pub(super) partial_ord: bool,
    pub(super) ord: bool,
    pub(super) hash: bool,
}

impl Derives {
    #[inline]
    pub(super) fn all() -> Self {
        Self {
            debug: true,
            clone: true,
            copy: true,
            default_: true,
            partial_eq: true,
            eq: true,
            partial_ord: true,
            ord: true,
            hash: true,
        }
    }

    pub(super) fn intersect(&mut self, other: Self) {
        self.debug &= other.debug;
        self.clone &= other.clone;
        self.copy &= other.copy;
        self.default_ &= other.default_;
        self.partial_eq &= other.partial_eq;
        self.eq &= other.eq;
        self.partial_ord &= other.partial_ord;
        self.ord &= other.ord;
        self.hash &= other.hash;
    }

    pub(super) fn to_list(self) -> Vec<&'static str> {
        let mut list = Vec::new();
        if self.clone {
            list.push("Clone");
        }
        if self.copy {
            list.push("Copy");
        }
        if self.default_ {
            list.push("Default");
        }
        list
    }

    pub(super) fn extra_traits_list(self) -> Vec<&'static str> {
        let mut list = Vec::new();
        if self.debug {
            list.push("Debug");
        }
        if self.partial_eq {
            list.push("PartialEq");
        }
        if self.eq {
            list.push("Eq");
        }
        if self.partial_ord {
            list.push("PartialOrd")
        }
        if self.ord {
            list.push("Ord");
        }
        if self.hash {
            list.push("Hash");
        }
        list
    }
}

/// Constraints on the wire format of a struct.
#[derive(Debug)]
pub(super) enum StructSizeConstraint<'a> {
    /// No code constraining the size is emitted.
    None,
    /// This is a fixed size struct.
    Fixed(u8),
    /// This struct has a "length" field embedded that tells us how long it
    /// should be (in 4-byte units).
    EmbeddedLength {
        /// The minimum size.
        minimum: u8,
    },
    /// The size of the struct is calculated from a <length> expression
    LengthExpr(&'a xcbdefs::Expression),
}

/// Information about a switch case
pub(super) enum CaseInfo {
    /// The case contains a single visible field.
    ///
    /// The `usize` specifies the index of such field.
    SingleField(usize),
    /// The case contains many visible fields.
    ///
    /// The first `String` is the name of the field
    /// and the second `String` is the name of the
    /// created struct that contains all fields.
    MultiField(String, String),
}

/// Specifies how the value of a field can be calculated
/// from other fields.
#[derive(Debug)]
pub(super) enum DeducibleField {
    /// The value is the length of a list.
    ///
    /// `(list name, operation)`
    LengthOf(String, DeducibleLengthFieldOp),
    /// The value is the discriminant of a case switch
    ///
    /// `(switch name)`
    CaseSwitchExpr(String, DeducibleFieldOp),
    /// The value is the discriminant of a bitcase switch
    ///
    /// `(switch name)`
    BitCaseSwitchExpr(String, DeducibleFieldOp),
}

#[derive(Copy, Clone, Debug)]
pub(super) enum DeducibleLengthFieldOp {
    /// `deduced field = list length`
    None,
    /// `deduced field = list length * n`
    Mul(u32),
    /// `deduced field = list length / n`
    Div(u32),
}

#[derive(Debug, Clone)]
pub(super) enum DeducibleFieldOp {
    /// `deduced field = value`.
    None,
    /// `deduced field = value | expr`.
    Or(Box<xcbdefs::Expression>),
}

/// Gathers deducible fields (fields whose value can be calculated
/// from other fields) from a list of fields.
pub(super) fn gather_deducible_fields(
    fields: &[xcbdefs::FieldDef],
) -> HashMap<String, DeducibleField> {
    fn extract_length(expr: &xcbdefs::Expression) -> Option<(String, DeducibleLengthFieldOp)> {
        match expr {
            xcbdefs::Expression::FieldRef(field_ref_expr) => Some((
                field_ref_expr.field_name.clone(),
                DeducibleLengthFieldOp::None,
            )),
            xcbdefs::Expression::BinaryOp(bin_op_expr) => {
                if bin_op_expr.operator == xcbdefs::BinaryOperator::Mul {
                    match (&*bin_op_expr.lhs, &*bin_op_expr.rhs) {
                        (
                            xcbdefs::Expression::FieldRef(field_ref_expr),
                            xcbdefs::Expression::Value(value),
                        ) => Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleLengthFieldOp::Div(*value),
                        )),
                        (
                            xcbdefs::Expression::Value(value),
                            xcbdefs::Expression::FieldRef(field_ref_expr),
                        ) => Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleLengthFieldOp::Div(*value),
                        )),
                        _ => None,
                    }
                } else if bin_op_expr.operator == xcbdefs::BinaryOperator::Div {
                    match (&*bin_op_expr.lhs, &*bin_op_expr.rhs) {
                        (
                            xcbdefs::Expression::FieldRef(field_ref_expr),
                            xcbdefs::Expression::Value(value),
                        ) => Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleLengthFieldOp::Mul(*value),
                        )),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    let mut deducible_fields = HashMap::new();
    for field in fields.iter() {
        let deducible_field = match field {
            xcbdefs::FieldDef::List(list_field) => list_field
                .length_expr
                .as_ref()
                .and_then(extract_length)
                .map(|(len_field_name, op)| {
                    (
                        len_field_name,
                        DeducibleField::LengthOf(list_field.name.clone(), op),
                    )
                }),
            xcbdefs::FieldDef::FdList(fd_list_field) => extract_length(&fd_list_field.length_expr)
                .map(|(len_field_name, op)| {
                    (
                        len_field_name,
                        DeducibleField::LengthOf(fd_list_field.name.clone(), op),
                    )
                }),
            xcbdefs::FieldDef::Switch(switch_field) => {
                if switch_field.cases.iter().any(|case| case.exprs.len() != 1) {
                    None
                } else if switch_field.kind == xcbdefs::SwitchKind::Case {
                    if let xcbdefs::Expression::FieldRef(ref field_ref_expr) = switch_field.expr {
                        Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleField::CaseSwitchExpr(
                                switch_field.name.clone(),
                                DeducibleFieldOp::None,
                            ),
                        ))
                    } else {
                        unreachable!("Can't figure out deducible field of {:#?}", switch_field);
                    }
                } else if switch_field.kind == xcbdefs::SwitchKind::BitCase {
                    if let xcbdefs::Expression::FieldRef(ref field_ref_expr) = switch_field.expr {
                        Some((
                            field_ref_expr.field_name.clone(),
                            DeducibleField::BitCaseSwitchExpr(
                                switch_field.name.clone(),
                                DeducibleFieldOp::None,
                            ),
                        ))
                    } else if let xcbdefs::Expression::BinaryOp(ref binary_op_expr) =
                        switch_field.expr
                    {
                        if let xcbdefs::Expression::FieldRef(ref field_ref_expr) =
                            *binary_op_expr.lhs
                        {
                            match binary_op_expr.operator {
                                xcbdefs::BinaryOperator::And => {
                                    // This appears in XKB:SelectEvents.
                                    // There we're provided the additional constraint that
                                    // the right hand side of this operation is a strict superset of
                                    // the left hand side. Therefore, we can negate the right
                                    // hand side and OR it with the switch field to undo the
                                    // AND and deduce affectWhich.
                                    // Because this is not true in general, we assert this is
                                    // the field we expect.
                                    assert_eq!(field_ref_expr.field_name, "affectWhich");
                                    let rhs = binary_op_expr.rhs.clone();
                                    Some((
                                        field_ref_expr.field_name.clone(),
                                        DeducibleField::BitCaseSwitchExpr(
                                            switch_field.name.clone(),
                                            DeducibleFieldOp::Or(rhs.negate()),
                                        ),
                                    ))
                                }
                                // No other operators are actually used.
                                _ => unreachable!(),
                            }
                        } else {
                            unreachable!("Can't figure out deducible field of {:#?}", switch_field);
                        }
                    } else {
                        unreachable!("Can't figure out deducible field of {:#?}", switch_field);
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some((field_name, deducible_field)) = deducible_field {
            let is_not_ext_param = fields
                .iter()
                .any(|field| field.name() == Some(field_name.as_str()));

            if is_not_ext_param {
                // If the field is used more than once, deduce it from the first use
                // (do not replace entry).
                deducible_fields
                    .entry(field_name)
                    .or_insert(deducible_field);
            }
        }
    }

    deducible_fields
}

/// A helper to note what owns a field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FieldContainer {
    /// This field is part of a request.
    Request(String),
    /// The field belongs to something else.
    Other,
}

/// Converts a type name from the XML to a rust
/// type name (in CamelCase).
///
/// If the name is all uppercase, all but the first
/// letter are converter to lowercase.
pub(super) fn to_rust_type_name(name: &str) -> String {
    let mut name = String::from(name);
    if name.bytes().all(|c| !c.is_ascii_lowercase()) {
        name.make_ascii_lowercase();
    }

    // Convert to camel case
    let mut r = String::new();
    for chunk in name.split('_') {
        r.push_str(&chunk[..1]);
        let r_len = r.len();
        r[(r_len - 1)..].make_ascii_uppercase();
        r.push_str(&chunk[1..]);
    }
    r
}

/// Converts a type name from the XML to a rust
/// enum type name (in CamelCase).
///
/// If the name is not snake_case and is all uppercase,
/// it is left as is.
pub(super) fn to_rust_enum_type_name(name: &str) -> String {
    if name.contains('_') {
        to_rust_type_name(name)
    } else {
        name.into()
    }
}

/// Converts a name from the XML to a Rust variable
/// name (snake_case).
pub(super) fn to_rust_variable_name(name: &str) -> String {
    if name == "type" {
        "type_".into()
    } else if name == "match" {
        "match_".into()
    } else if name.bytes().any(|c| c.is_ascii_uppercase()) {
        // Deal with CamelCase
        super::super::camel_case_to_lower_snake(name)
    } else {
        name.into()
    }
}

/// Adds a prefix to `name`, spearated by an underscore.
///
/// If name ends with an underscore, it will be trimmed.
pub(super) fn prefix_var_name(name: &str, prefix: &str) -> String {
    let mut prefixed = format!("{prefix}_{name}");
    if prefixed.ends_with('_') {
        prefixed.truncate(prefixed.len() - 1);
    }
    prefixed
}

/// Adds a postfix to `name`, spearated by an underscore.
///
/// If `name` ends with an underscore, it will not insert an
/// extra one.
pub(super) fn postfix_var_name(name: &str, postfix: &str) -> String {
    if name.ends_with('_') {
        format!("{name}{postfix}")
    } else {
        format!("{name}_{postfix}")
    }
}

/// Converts the name of a enum value from the XML
/// to a Rust name.
pub(super) fn ename_to_rust(name: &str) -> String {
    if name == "DECnet" {
        // Special case
        "DEC_NET".into()
    } else {
        super::super::camel_case_to_upper_snake(&ename_to_camel_case(name))
    }
}

/// Converts the name of a enum value from the XML
/// to a camel case name.
pub(super) fn ename_to_camel_case(name: &str) -> String {
    // First convert to proper camel-case
    let mut name = String::from(name);
    if name.as_bytes()[0].is_ascii_digit() {
        name.insert(0, 'M');
    }
    if name.contains('_') && name.bytes().any(|c| c.is_ascii_lowercase()) {
        // xf86vidmode has a ModeFlag enum with items like
        // Positive_HSync. Turn this into PositiveHSync.
        name = name.replace('_', "");
    }
    name[..1].make_ascii_uppercase();
    name
}
