#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    InfixExpression(InfixExpression),
    BooleanExpression(BooleanExpression),
    PrefixExpression(PrefixExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub value: i64,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BooleanExpression {
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}
