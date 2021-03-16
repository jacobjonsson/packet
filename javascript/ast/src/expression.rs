use crate::statement::BlockStatement;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    InfixExpression(InfixExpression),
    BooleanExpression(BooleanExpression),
    FunctionExpression(FunctionExpression),
    PrefixExpression(PrefixExpression),
    StringLiteral(StringLiteral),
    CallExpression(CallExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
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
pub struct FunctionExpression {
    pub parameters: Vec<Identifier>, // TODO: es6 and upwards supports more patterns, see here: https://github.com/estree/estree/blob/master/es5.md#patterns
    pub body: BlockStatement,
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

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub function: Identifier, // TODO: Should support function expressions as well.
    pub arguments: Vec<Box<Expression>>,
}
