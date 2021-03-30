/// This file specifies the precedence ordering used
/// by packet when parsing expressions.
///
/// The MDN specification for the precedence rules can be found here:
/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table

pub trait PrecedenceInfo {
    fn precedence(&self) -> Precedence;
    fn is_right_associative(&self) -> bool;
    fn is_left_associative(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Comma,
    Spread,
    Yield,
    Assign,
    Conditional,
    NullishCoalescing,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equals,
    Compare,
    Shift,
    Sum,
    Product,
    Exponentiation,
    Prefix,
    Postfix,
    New,
    Call,
    Member,
    Grouping,
}

impl Precedence {
    /// Lowers the precedence to the previous level.
    pub fn lower(&self) -> Precedence {
        match self {
            Precedence::Lowest => Precedence::Lowest,
            Precedence::Comma => Precedence::Lowest,
            Precedence::Spread => Precedence::Comma,
            Precedence::Yield => Precedence::Spread,
            Precedence::Assign => Precedence::Yield,
            Precedence::Conditional => Precedence::Assign,
            Precedence::NullishCoalescing => Precedence::Conditional,
            Precedence::LogicalOr => Precedence::NullishCoalescing,
            Precedence::LogicalAnd => Precedence::LogicalOr,
            Precedence::BitwiseOr => Precedence::LogicalAnd,
            Precedence::BitwiseXor => Precedence::BitwiseOr,
            Precedence::BitwiseAnd => Precedence::BitwiseXor,
            Precedence::Equals => Precedence::BitwiseAnd,
            Precedence::Compare => Precedence::Equals,
            Precedence::Shift => Precedence::Compare,
            Precedence::Sum => Precedence::Shift,
            Precedence::Product => Precedence::Sum,
            Precedence::Exponentiation => Precedence::Product,
            Precedence::Prefix => Precedence::Exponentiation,
            Precedence::Postfix => Precedence::Prefix,
            Precedence::New => Precedence::Postfix,
            Precedence::Call => Precedence::New,
            Precedence::Member => Precedence::Call,
            Precedence::Grouping => Precedence::Member,
        }
    }

    /// Raises the precedence to the next level.
    pub fn raise(&self) -> Precedence {
        match self {
            Precedence::Lowest => Precedence::Comma,
            Precedence::Comma => Precedence::Spread,
            Precedence::Spread => Precedence::Yield,
            Precedence::Yield => Precedence::Assign,
            Precedence::Assign => Precedence::Conditional,
            Precedence::Conditional => Precedence::NullishCoalescing,
            Precedence::NullishCoalescing => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::BitwiseOr,
            Precedence::BitwiseOr => Precedence::BitwiseXor,
            Precedence::BitwiseXor => Precedence::BitwiseAnd,
            Precedence::BitwiseAnd => Precedence::Equals,
            Precedence::Equals => Precedence::Compare,
            Precedence::Compare => Precedence::Shift,
            Precedence::Shift => Precedence::Sum,
            Precedence::Sum => Precedence::Product,
            Precedence::Product => Precedence::Exponentiation,
            Precedence::Exponentiation => Precedence::Prefix,
            Precedence::Prefix => Precedence::Postfix,
            Precedence::Postfix => Precedence::New,
            Precedence::New => Precedence::Call,
            Precedence::Call => Precedence::Member,
            Precedence::Member => Precedence::Grouping,
            Precedence::Grouping => Precedence::Grouping,
        }
    }
}
