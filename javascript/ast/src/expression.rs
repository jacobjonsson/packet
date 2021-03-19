use crate::statement::BlockStatement;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    AssignmentExpression(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    LogicalExpression(LogicalExpression),
    BooleanExpression(BooleanExpression),
    FunctionExpression(FunctionExpression),
    PrefixExpression(PrefixExpression),
    StringLiteral(StringLiteral),
    CallExpression(CallExpression),
    ConditionalExpression(ConditionalExpression),
    UpdateExpression(UpdateExpression),
    ThisExpression(ThisExpression),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpression {}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpression {
    pub elements: Vec<Option<Box<Expression>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyKey {
    StringLiteral(StringLiteral),
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub key: PropertyKey,
    pub value: Expression,
    pub kind: PropertyKind,
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
pub struct PrefixExpression {
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub function: Identifier, // TODO: Should support function expressions as well.
    pub arguments: Vec<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternate: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    EqualsEquals,
    EqualsEqualsEquals,
    ExclamationEquals,
    ExclamationEqualsEquals,
    LessThan,
    LessThanLessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Bar,
    Caret,
    Ampersand,
    In,
    Instanceof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateExpression {
    pub prefix: bool,
    pub argument: Box<Expression>,
    pub operator: UpdateOperator,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentOperator {
    Equals,
    PlusEquals,
    MinusEquals,
    AsteriskEquals,
    SlashEquals,
    PercentEquals,
    LessThanLessThanEquals,
    GreaterThanGreaterThanEquals,
    GreaterThanGreaterThanGreaterThanEquals,
    BarEquals,
    CaretEquals,
    AmpersandEquals,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub operator: AssignmentOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOperator {
    BarBar,
    AmpersandAmpersand,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpression {
    pub left: Box<Expression>,
    pub operator: LogicalOperator,
    pub right: Box<Expression>,
}
