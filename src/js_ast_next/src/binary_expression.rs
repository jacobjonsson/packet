use span::Span;

use crate::{precedence::Precedence, Expression};

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

impl BinaryExpressionOperator {
    pub fn precedence(&self) -> Precedence {
        match self {
            BinaryExpressionOperator::Addition => Precedence::Sum,
            BinaryExpressionOperator::Substitution => Precedence::Sum,
            BinaryExpressionOperator::Multiplication => Precedence::Product,
            BinaryExpressionOperator::Division => Precedence::Product,
            BinaryExpressionOperator::Modulus => Precedence::Product,
            BinaryExpressionOperator::Exponentiation => Precedence::Exponentiation,
            BinaryExpressionOperator::LessThan => Precedence::Comparison,
            BinaryExpressionOperator::LessThanEquals => Precedence::Comparison,
            BinaryExpressionOperator::GreaterThan => Precedence::Comparison,
            BinaryExpressionOperator::GreaterThanEquals => Precedence::Comparison,
            BinaryExpressionOperator::In => Precedence::Comparison,
            BinaryExpressionOperator::Instanceof => Precedence::Comparison,
            BinaryExpressionOperator::LeftShift => Precedence::Shift,
            BinaryExpressionOperator::RightShift => Precedence::Shift,
            BinaryExpressionOperator::UnsignedRightShift => Precedence::Shift,
            BinaryExpressionOperator::LooseEquals => Precedence::Equality,
            BinaryExpressionOperator::LooseNotEquals => Precedence::Equality,
            BinaryExpressionOperator::StrictEquals => Precedence::Equality,
            BinaryExpressionOperator::StrictNotEquals => Precedence::Equality,
            BinaryExpressionOperator::NullishCoalescing => Precedence::NullishCoalescing,
            BinaryExpressionOperator::BitwiseOr => Precedence::BitwiseOr,
            BinaryExpressionOperator::BitwiseAnd => Precedence::BitwiseAnd,
            BinaryExpressionOperator::BitwiseXor => Precedence::BitwiseXor,
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            BinaryExpressionOperator::Addition => "+",
            BinaryExpressionOperator::Substitution => "-",
            BinaryExpressionOperator::Multiplication => "*",
            BinaryExpressionOperator::Division => "/",
            BinaryExpressionOperator::Modulus => "%",
            BinaryExpressionOperator::Exponentiation => "**",
            BinaryExpressionOperator::LessThan => "<",
            BinaryExpressionOperator::LessThanEquals => "<=",
            BinaryExpressionOperator::GreaterThan => ">",
            BinaryExpressionOperator::GreaterThanEquals => ">=",
            BinaryExpressionOperator::In => "in",
            BinaryExpressionOperator::Instanceof => "instanceof",
            BinaryExpressionOperator::LeftShift => "<<",
            BinaryExpressionOperator::RightShift => ">>",
            BinaryExpressionOperator::UnsignedRightShift => ">>>",
            BinaryExpressionOperator::LooseEquals => "==",
            BinaryExpressionOperator::LooseNotEquals => "!=",
            BinaryExpressionOperator::StrictEquals => "===",
            BinaryExpressionOperator::StrictNotEquals => "!==",
            BinaryExpressionOperator::NullishCoalescing => "??",
            BinaryExpressionOperator::BitwiseOr => "|",
            BinaryExpressionOperator::BitwiseAnd => "&",
            BinaryExpressionOperator::BitwiseXor => "^",
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            BinaryExpressionOperator::In | BinaryExpressionOperator::Instanceof => true,
            _ => false,
        }
    }
}
