use std::cell::RefCell;

use once_cell::unsync::OnceCell;

use super::{Alignment, ComplexAlignment, Expression, ExternalParam, NamedTypeRef};

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
    pub(crate) fn value_type(&self) -> Option<&FieldValueType> {
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

    /// Get the size in bytes of this field on the wire.
    ///
    /// This returns `None` for fields that have a variable size
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

/// A `<pad>` field.
#[derive(Clone, Copy, Debug)]
pub struct PadField {
    /// The kind and size of padding
    pub kind: PadKind,

    /// The value of the `serialize` attribute.
    ///
    /// If no attribute was present, the value is `false`.
    pub serialize: bool,
}

/// A `<field>` field.
#[derive(Debug)]
pub struct NormalField {
    /// The name of the field.
    pub name: String,

    /// The kind of values that the field can take.
    pub type_: FieldValueType,
}

/// A `<list>` field.
#[derive(Debug)]
pub struct ListField {
    /// The name of the field.
    pub name: String,

    /// The kind of values that the field can take.
    pub element_type: FieldValueType,

    /// The expression describing the length of the list, if present.
    pub length_expr: Option<Expression>,
}

impl ListField {
    /// Does the list have a fixed/constant length.
    ///
    /// This function is true if there is only one possible length for the list.
    pub fn has_fixed_length(&self) -> bool {
        self.length().is_some()
    }

    /// Get the length of the list if it is statically known.
    pub fn length(&self) -> Option<u32> {
        if let Some(ref length_expr) = self.length_expr {
            match length_expr {
                Expression::Value(v) => Some(*v),
                Expression::Bit(bit) => Some(1 << *bit),
                _ => None,
            }
        } else {
            None
        }
    }
}

/// A `<switch>` field.
#[derive(Debug)]
pub struct SwitchField {
    /// The name of the field.
    pub name: String,

    /// The expression on which the `<switch>`... well, switches.
    pub expr: Expression,

    /// The value of this switch's `<required_start_align>` child, if any.
    pub required_start_align: Option<Alignment>,

    /// The computed alignment for this switch.
    ///
    /// This field is only set after resolving.
    pub alignment: OnceCell<ComplexAlignment>,

    /// The kind of switch.
    ///
    /// All `cases` of this switch have to conform to this kind: Either they are all `<case>`s or
    /// all `<bitcase>`s.
    pub kind: SwitchKind,

    /// The `cases` that the switch can have.
    pub cases: Vec<SwitchCase>,

    /// The list of `<paramref>`s that appear in descendants of this switch.
    ///
    /// This list is empty before the switch was resolved.
    pub external_params: RefCell<Vec<ExternalParam>>,
}

impl SwitchField {
    /// Get the size in bytes of this switch on the wire.
    ///
    /// This returns `None` if the switch does not have a fixed size. Only switches of kind
    /// `<bitcase>` can have a fixed size.
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

/// The possible kinds that a `<switch>` can have.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwitchKind {
    /// All children of the `<switch>` are `<bitcase>`s.
    BitCase,

    /// All children of the `<switch>` are `<case>`s.
    Case,
}

/// A single `<case>` or `<bitcase>` of a switch.
#[derive(Debug)]
pub struct SwitchCase {
    /// The value of the `name` attribute, if present.
    pub name: Option<String>,

    /// All the expressions that this case contains.
    pub exprs: Vec<Expression>,

    /// The value of the `<required_start_align>` child, if any.
    pub(crate) required_start_align: Option<Alignment>,

    /// Alignment information about this case.
    ///
    /// This information is not available before the case was resolved.
    pub alignment: OnceCell<ComplexAlignment>,

    /// The list of fields of this case.
    ///
    /// The fields appear in the order in which they are defined.
    pub fields: RefCell<Vec<FieldDef>>,

    /// The list of `<paramref>`s that appear in descendants of this case.
    ///
    /// This list is empty before the case was resolved.
    pub external_params: RefCell<Vec<ExternalParam>>,
}

impl SwitchCase {
    /// Get the size in bytes of this case on the wire.
    ///
    /// This returns `None` if this case does not have a constant size.
    fn size(&self) -> Option<u32> {
        self.fields
            .borrow()
            .iter()
            .try_fold(0, |sz, field| Some(sz + field.size()?))
    }
}

/// A `<fd>` field.
#[derive(Debug)]
pub struct FdField {
    /// The value of the `name` attribute.
    pub name: String,
}

/// A `<list>` field with `type="fd"`.
#[derive(Debug)]
pub struct FdListField {
    /// The value of the `name` attribute.
    pub name: String,

    /// The length of the list
    pub length_expr: Expression,
}

impl FdListField {
    /// Get the length of the list if it is statically known.
    pub fn length(&self) -> Option<u32> {
        match self.length_expr {
            Expression::Value(v) => Some(v),
            Expression::Bit(bit) => Some(1 << bit),
            _ => None,
        }
    }
}

/// A `<exprfield>` field.
#[derive(Debug)]
pub struct ExprField {
    /// The value of the `name` attribute.
    pub name: String,

    /// The type of the field.
    pub type_: FieldValueType,

    /// The expression describing how the field is computed.
    pub expr: Expression,
}

/// An invented length field for a list that has no length.
///
/// There is currently exactly one `<exprfield>` tag in xcb-proto:
/// ```text
/// <request name="QueryTextExtents" opcode="48">
///   <exprfield type="BOOL" name="odd_length">
///     <op op="&amp;"><fieldref>string_len</fieldref><value>1</value></op>
///   </exprfield>
///   <field type="FONTABLE" name="font" />
///   <list type="CHAR2B" name="string" />
///   <reply>
/// [...]
/// ```
/// The `<exprfield>` above refers to a field `string_len` that does not exist. This refers to the
/// implicit length of the `string` list.
///
/// To make this work, a `VirtualLenField` is invented during resolving and inserted in the
/// representation of the request above.
#[derive(Debug)]
pub struct VirtualLenField {
    /// The name of the virtual length field.
    pub name: String,

    /// The type of the field.
    pub(crate) type_: FieldValueType,

    /// The name of the referenced list.
    pub list_name: String,
}

/// The type of a field's value.
#[derive(Debug)]
pub struct FieldValueType {
    /// The actual type of the field's value.
    pub type_: NamedTypeRef,

    /// Constraints describing the possible values that the field can have.
    ///
    /// This represents the `enum`, `altenum`, `mask`, and `altmask` attributes of the XML.
    pub value_set: FieldValueSet,
}

impl FieldValueType {
    /// Get the size in bytes of this field on the wire.
    ///
    /// This returns `None` if the struct does not have a fixed size.
    ///
    /// # Panics
    ///
    /// Panics if this value type was not yet resolved.
    pub fn size(&self) -> Option<u32> {
        self.type_.get_resolved().size()
    }
}

/// A possible value set constraining a field's value.
#[derive(Debug)]
pub enum FieldValueSet {
    /// The field can take any possible value of its type.
    None,

    /// The field can only takes values from the referenced value.
    Enum(NamedTypeRef),

    /// The field can take any possible value of its type, but values of the referenced enum have a
    /// special meaning.
    AltEnum(NamedTypeRef),

    /// The field is a bitmask of the values of the referenced enum.
    Mask(NamedTypeRef),

    /// The field can take any possible value of its type, but the referenced enum gives special
    /// meaning to some bits.
    AltMask(NamedTypeRef),
}

/// The kind of padding a `<pad>` can represent.
#[derive(Clone, Debug, Copy)]
pub enum PadKind {
    /// A fixed size padding of the given size (`bytes` attribute).
    Bytes(u16),

    /// An alignment padding aligning to the given alignment ( `align` attribute).
    Align(u16),
}

/// A built-in type that is not explicitly defined.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BuiltInType {
    /// An unsigned integer with 8 bits.
    Card8,

    /// An unsigned integer with 16 bits.
    Card16,

    /// An unsigned integer with 32 bits.
    Card32,

    /// An unsigned integer with 64 bits.
    Card64,

    /// A signed integer with 8 bits.
    Int8,

    /// A signed integer with 16 bits.
    Int16,

    /// A signed integer with 32 bits.
    Int32,

    /// A signed integer with 64 bits.
    Int64,

    /// A single byte.
    Byte,

    /// A boolean value
    Bool,

    /// A single character.
    ///
    /// Characters are represented as bytes. It best not to think about non-ASCII characters.
    Char,

    /// A single precision floating point value with 32 bits as defined by IEEE 754.
    Float,

    /// A double precision floating point value with 64 bits as defined by IEEE 754.
    Double,

    /// A value with an unspecified encoding.
    ///
    /// One example for this is the argument to xproto's `ChangeProperty` request. Depending on
    /// other values in the request, the arguments have type `Card8`, `Card16`, or `Card32`.
    ///
    /// Values of this type can be treated as bytes. In particular, the size of a single value is
    /// one and larger values (like in the `ChangeProperty` example above) are composed of multiple
    /// instances of the `Void` type.
    Void,
}

impl BuiltInType {
    /// Get the size in bytes of this type on the wire.
    pub(crate) fn size(self) -> u32 {
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
