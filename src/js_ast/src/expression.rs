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
    StringLiteral(StringLiteral),
    RegexpLiteral(RegexpLiteral),
    NullLiteral(NullLiteral),
    CallExpression(CallExpression),
    ConditionalExpression(ConditionalExpression),
    UpdateExpression(UpdateExpression),
    UnaryExpression(UnaryExpression),
    ThisExpression(ThisExpression),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
    NewExpression(NewExpression),
    MemberExpression(MemberExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
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
    pub computed: bool,
    pub key: PropertyKey,
    pub value: Expression,
    pub kind: PropertyKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegexpLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NullLiteral {}

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
    pub id: Option<Identifier>,
    pub parameters: Vec<Pattern>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternate: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Exclamation,
    Tilde,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expression>,
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
pub enum AssignmentExpressionLeft {
    Expression(Expression),
    Pattern(Pattern),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub operator: AssignmentOperator,
    pub left: Box<AssignmentExpressionLeft>,
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

/* -------------------------------------------------------------------------- */
/*                                  Patterns                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Identifier(Identifier),
    ObjectPattern(ObjectPattern),
    ArrayPattern(ArrayPattern),
    RestElement(RestElement),
    AssignmentPattern(AssignmentPattern),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentProperty {
    pub key: PropertyKey,
    pub value: Box<Pattern>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectPatternProperty {
    AssignmentProperty(AssignmentProperty),
    RestElement(RestElement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectPattern {
    pub properties: Vec<ObjectPatternProperty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayPattern {
    pub properties: Vec<Option<Pattern>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RestElement {
    pub argument: Box<Pattern>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentPattern {
    pub right: Box<Expression>,
}
