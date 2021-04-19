use span::Span;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub span: Span,
    pub left: Box<Expression>,
    pub operator: BinaryExpressionOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryExpressionOperator {
    /// +
    Addition,
    /// -
    Substitution,
    /// *
    Multiplication,
    /// /
    Division,
    /// %
    Modulus,
    /// **
    Exponentiation,
    /// <
    LessThan,
    /// <=
    LessThanEquals,
    /// >
    GreaterThan,
    /// >=
    GreaterThanEquals,
    /// in
    In,
    /// instanceof
    Instanceof,
    /// <<
    LeftShift,
    /// >>
    RightShift,
    /// >>>
    UnsignedRightShift,
    /// ==
    LooseEquals,
    /// !=
    LooseNotEquals,
    /// ===
    StrictEquals,
    /// !==
    StrictNotEquals,
    /// ??
    NullishCoalescing,
    /// |
    BitwiseOr,
    /// &
    BitwiseAnd,
    /// ^
    BitwiseXor,
}
