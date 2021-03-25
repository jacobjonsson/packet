#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    StringLiteral(StringLiteral),
    NumberLiteral(NumericLiteral),
    BooleanLiteral(BooleanLiteral),
    RegexpLiteral(RegexpLiteral),
    NullLiteral(NullLiteral),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumericLiteral {
    pub value: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegexpLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NullLiteral {}