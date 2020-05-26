use once_cell::unsync::OnceCell;

use super::{NamedTypeRef, TypeRef};

/// An expression for computing some value.
///
/// This is a recursive type describing a tree of the expression.
#[derive(Debug, Clone)]
pub enum Expression {
    /// An operation with two sub-expressions.
    BinaryOp(BinaryOpExpr),

    /// An operation with one sub-expression.
    UnaryOp(UnaryOpExpr),

    /// A reference to a field of the type in which this expression is evaluated.
    FieldRef(FieldRefExpr),

    /// A reference to an outer parameter.
    ///
    /// Currently, there is only a single `<paramref>` in xcb-proto:
    /// The reply to xinput's `GetDeviceMotionEvents` request has a `num_axes` field and an
    /// `events` list. This list contains structures of type `DeviceTimeCoord`. This structure in
    /// turn has a list. The length of this list is a `<paramref>` to `num_axes`.
    ///
    /// Thus, a `<paramref>` is a reference to a field that is not even present in the type where
    /// the expression is originally, but in some context that is even farther away.
    ParamRef(ParamRefExpr),

    /// A reference to a value of an enum.
    EnumRef(EnumRefExpr),

    /// The value of this expression is the number of bits that are set in the value of the inner
    /// expression.
    PopCount(Box<Expression>),

    /// The value of this expression is the sum of some other expression evaluated for each element
    /// of some list.
    SumOf(SumOfExpr),

    /// A reference to the current list element inside of a `SumOf` expression.
    ListElementRef,

    /// A constant and fixed value.
    Value(u32),

    /// A constant and fixed value specified in the form `1 << bit`.
    Bit(u8),
}

impl Expression {
    /// Negate an expression, attempting to be efficient where possible.
    pub fn negate(self: Box<Expression>) -> Box<Expression> {
        match *self {
            Expression::UnaryOp(UnaryOpExpr {
                operator: UnaryOperator::Not,
                rhs,
            }) => rhs,
            Expression::BinaryOp(BinaryOpExpr {
                operator: BinaryOperator::And,
                lhs,
                rhs,
            }) => {
                // Apply De Morgan's law, because in the one case it appears
                // in xcb it is advantageous to do so.
                Box::new(Expression::BinaryOp(BinaryOpExpr {
                    operator: BinaryOperator::Or,
                    lhs: lhs.negate(),
                    rhs: rhs.negate(),
                }))
            }
            _ => Box::new(Expression::UnaryOp(UnaryOpExpr {
                operator: UnaryOperator::Not,
                rhs: self,
            })),
        }
    }
}

/// An expression with two sub-expressions.
#[derive(Debug, Clone)]
pub struct BinaryOpExpr {
    /// The operator that is applied to the two sub-expressions.
    pub operator: BinaryOperator,

    /// The left hand side of the expression.
    pub lhs: Box<Expression>,

    /// The right hand side of the expression.
    pub rhs: Box<Expression>,
}

/// The possible binary operators
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    /// Addition
    Add,

    /// Subtraction
    Sub,

    /// Multiplication
    Mul,

    /// (Integer) division
    Div,

    /// Logical and
    And,

    /// Logical or,
    Or,

    /// A logical left shift
    Shl,
}

/// An expression with one sub-expression
#[derive(Debug, Clone)]
pub struct UnaryOpExpr {
    /// The operator that is applied to the sub-expression.
    pub operator: UnaryOperator,

    /// The sub-expression.
    pub rhs: Box<Expression>,
}

/// The possible unary operators
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Bitwise negation
    Not,
}

/// An expression referencing a field from the surrounding context.
#[derive(Debug, Clone)]
pub struct FieldRefExpr {
    /// The name of the field that is referenced.
    pub field_name: String,

    /// After resolution, this contains information about the referenced field.
    pub resolved: OnceCell<ResolvedFieldRef>,
}

/// Information about a field reference.
#[derive(Debug, Clone)]
pub struct ResolvedFieldRef {
    /// The kind of reference to another field.
    pub ref_kind: FieldRefKind,

    /// The type of the referenced field.
    pub field_type: TypeRef,
}

/// Possible kinds of references to fields inside of an expression.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FieldRefKind {
    /// The reference is to a field of the expression where the field is evaluated.
    LocalField,

    /// The reference is to an external parameter from a surrounding context.
    ExtParam,

    /// The reference is used inside of a [`SumOfExpr`].
    SumOfRef,
}

/// An expression referencing a parameter from a surrounding context.
#[derive(Debug, Clone)]
pub struct ParamRefExpr {
    /// The name of the referenced field.
    pub field_name: String,

    /// The type of the referenced field.
    pub type_: NamedTypeRef,
}

/// An expression referencing a value from an enumeration.
#[derive(Debug, Clone)]
pub struct EnumRefExpr {
    /// The enum that is referenced.
    pub enum_: NamedTypeRef,

    /// The name of the enum's variant that is used.
    pub variant: String,
}

/// An expression representing the sum over some referenced list.
#[derive(Debug, Clone)]
pub struct SumOfExpr {
    /// The name of the list that is used.
    pub field_name: String,

    /// After resolving, a reference to the list that is used.
    pub resolved_field: OnceCell<ResolvedFieldRef>,

    /// The expression that is applied to each element of the list.
    ///
    /// If no expression is provided, the elements are summed as-is.
    pub operand: Box<Expression>,
}
