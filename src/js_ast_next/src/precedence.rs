/// See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence
/// Make sure to keep the values in sync with the raise and lower methods below.
#[derive(Debug, Clone, PartialEq)]
pub enum Precedence {
    Comma = 1,
    Yield = 2,
    Assignment = 3,
    Conditional = 4,
    NullishCoalescing = 5,
    LogicalOr = 6,
    LogicalAnd = 7,
    BitwiseOr = 8,
    BitwiseXor = 9,
    BitwiseAnd = 10,
    Equality = 11,
    Comparison = 12,
    Shift = 13,
    Sum = 14,
    Product = 15,
    Exponentiation = 16,
    Prefix = 17,
    Postfix = 18,
    New = 19,
    Call = 20,
    Grouping = 21,
}

impl Precedence {
    /// Raises the current precedence level by one.
    pub fn raise(&self) -> Precedence {
        match self {
            Precedence::Comma => Precedence::Yield,
            Precedence::Yield => Precedence::Assignment,
            Precedence::Assignment => Precedence::Conditional,
            Precedence::Conditional => Precedence::NullishCoalescing,
            Precedence::NullishCoalescing => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::BitwiseOr,
            Precedence::BitwiseOr => Precedence::BitwiseXor,
            Precedence::BitwiseXor => Precedence::BitwiseAnd,
            Precedence::BitwiseAnd => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Shift,
            Precedence::Shift => Precedence::Sum,
            Precedence::Sum => Precedence::Product,
            Precedence::Product => Precedence::Exponentiation,
            Precedence::Exponentiation => Precedence::Prefix,
            Precedence::Prefix => Precedence::Postfix,
            Precedence::Postfix => Precedence::New,
            Precedence::New => Precedence::Call,
            Precedence::Call => Precedence::Grouping,
            Precedence::Grouping => Precedence::Grouping,
        }
    }

    /// Lowers the current precedence level by one.
    pub fn lower(&self) -> Precedence {
        match self {
            Precedence::Comma => Precedence::Comma,
            Precedence::Yield => Precedence::Comma,
            Precedence::Assignment => Precedence::Yield,
            Precedence::Conditional => Precedence::Assignment,
            Precedence::NullishCoalescing => Precedence::Conditional,
            Precedence::LogicalOr => Precedence::NullishCoalescing,
            Precedence::LogicalAnd => Precedence::LogicalOr,
            Precedence::BitwiseOr => Precedence::LogicalAnd,
            Precedence::BitwiseXor => Precedence::BitwiseOr,
            Precedence::BitwiseAnd => Precedence::BitwiseXor,
            Precedence::Equality => Precedence::BitwiseAnd,
            Precedence::Comparison => Precedence::Equality,
            Precedence::Shift => Precedence::Comparison,
            Precedence::Sum => Precedence::Shift,
            Precedence::Product => Precedence::Sum,
            Precedence::Exponentiation => Precedence::Product,
            Precedence::Prefix => Precedence::Exponentiation,
            Precedence::Postfix => Precedence::Prefix,
            Precedence::New => Precedence::Postfix,
            Precedence::Call => Precedence::New,
            Precedence::Grouping => Precedence::Call,
        }
    }
}
