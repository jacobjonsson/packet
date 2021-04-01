/// This file defines the JavaScript abstract syntax tree (AST)
/// that packet uses during parsing, bundling and printing.
///
/// This file is organized is very simple way, it first specifies the
/// top level AST node, then all of the enums, followed by all of the
/// AST-nodes organized by alphabetic order.
///
/// All of the nodes are stored in the same file and are not organized by
/// type (expression, statement, literal, etc) sine many of the nodes reference
/// each other and keeping them all in one file makes it easier to make
/// sweeping changes.
///
/// The AST largely conforms to the ESTree specification but does make
/// some changes in certain places. The ESTree specification is not as
/// strict as the AST defined in this file.
///
/// For example, the ESTree specifies an object property like this:
/// ```ts
/// Property {
///   key: Expression,
///   value: Expression,
///   method: bool,
///   shorthand: bool,
///   computed: bool,
/// }
/// ```
/// This is a very concise representation of an object property,
/// but it it also one that is problematic since it makes it possible
/// to represent illegal states. For example, the **computed** field
/// specifies if the key was surrounded by brackets `({ [a]: b })` and the **shorthand**
/// specifies if the shorthand syntax was used (`({ a })`).
///
/// However, an object property may not be both computed and shorthand at
/// the same time. Parsers that implement this specification will
/// therefore need to rely on runtime logic to prevent illegal states.
///
/// Packet takes a different route and is much stricter in it's representation of the AST.
/// We try to avoid making illegal states completely un-representable, taking full use of
/// rusts powerful type-system. This does lead to a more verbose AST but reduces the
/// need for runtime logic.
use precedence::{Precedence, PrecedenceInfo};

pub mod precedence;

/// The AST is the top level node that contains all of the statements
/// and expression the program contains.
///
/// It will in the future also contain various metadata that might be relevant
/// to other parts of packet such as names and imports.
#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    pub statements: Vec<Statement>,
}

// ----- Enums -----

/// A binding represents the binding between a variable and a value.
///
/// The basic binding is an Identifier: a = 2;
///
/// Bindings can also be created using destructuring assignment.
///
/// Object: ({ a } = b);
///
/// Array: [a] = b;
///
/// Note that most other AST specs includes the rest element as a binding
/// but this isn't always accurate since the rest element can't be used in
/// all places where the other bindings can be.
///
/// This is valid: [...a] = b
///
/// But this is not valid: ...a = b
#[derive(Debug, PartialEq, Clone)]
pub enum Binding {
    Identifier(Identifier),
    Object(ObjectBinding),
    Array(ArrayBinding),
}

/// An expression is any valid unit of code that resolves to a value.
///
/// The expression enum contains all of the expression available in the AST.
/// Note that not all of the expressions are allowed to exit in a place where
/// another expression is allowed.
///
/// For example, the spread element is an expression but is only allowed to
/// be used in call, array and object expressions.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Assignment(AssignmentExpression),
    ArrowFunction(ArrowFunctionExpression),
    Array(ArrayExpression),
    BigIntLiteral(BigIntLiteral),
    Binary(BinaryExpression),
    BooleanLiteral(BooleanLiteral),
    Call(CallExpression),
    Class(ClassExpression),
    Conditional(ConditionalExpression),
    Function(FunctionExpression),
    Identifier(Identifier),
    Logical(LogicalExpression),
    Member(MemberExpression),
    New(NewExpression),
    NullLiteral(NullLiteral),
    NumericLiteral(NumericLiteral),
    Object(ObjectExpression),
    RegexpLiteral(RegexpLiteral),
    Sequence(SequenceExpression),
    StringLiteral(StringLiteral),
    Super(SuperExpression),
    TemplateLiteral(TemplateLiteral),
    This(ThisExpression),
    Unary(UnaryExpression),
    Update(UpdateExpression),
}

/// This is the top level literal enum,
/// it contains all of the literals packet support.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(StringLiteral),
    Number(NumericLiteral),
    Boolean(BooleanLiteral),
    Regexp(RegexpLiteral),
    Null(NullLiteral),
}

/// LiteralPropertyName is used by classes and objects.
#[derive(Debug, PartialEq, Clone)]
pub enum LiteralPropertyName {
    Identifier(Identifier),
    String(StringLiteral),
    Numeric(NumericLiteral),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    BlockStatement(BlockStatement),
    BreakStatement(BreakStatement),
    ClassDeclaration(ClassDeclaration),
    ContinueStatement(ContinueStatement),
    DebuggerStatement(DebuggerStatement),
    DoWhileStatement(DoWhileStatement),
    EmptyStatement(EmptyStatement),
    ExportAllDeclaration(ExportAllDeclaration),
    ExportDefaultDeclaration(ExportDefaultDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
    ExportNamedSpecifiers(ExportNamedSpecifiers),
    Expression(ExpressionStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    ForStatement(ForStatement),
    FunctionDeclaration(FunctionDeclaration),
    IfStatement(IfStatement),
    ImportDeclaration(ImportDeclaration),
    LabeledStatement(LabeledStatement),
    ReturnStatement(ReturnStatement),
    SwitchStatement(SwitchStatement),
    ThrowStatement(ThrowStatement),
    TryStatement(TryStatement),
    VariableDeclaration(VariableDeclaration),
    WhileStatement(WhileStatement),
    WithStatement(WithStatement),
}

// ----- Nodes -----

/// This is a special class node and it is only allowed
/// to be used in export statements. This the the only
/// occurrence where class declaration does not have
/// an identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct AnonymousDefaultExportedClassDeclaration {
    pub extends: Option<Expression>,
    pub body: Vec<ClassPropertyKind>,
}

/// This is a special function node and it is only allowed
/// to be used in export statements. This the the only
/// occurrence where function declaration does not have
/// an identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct AnonymousDefaultExportedFunctionDeclaration {
    pub generator: bool,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentKind {
    Expression(Expression),
    Spread(SpreadElement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub left: AssignmentExpressionLeft,
    pub operator: AssignmentExpressionOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentExpressionLeft {
    Binding(Binding),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentExpressionOperator {
    Assign,
    AdditionAssign,
    SubstitutionAssign,
    MultiplicationAssign,
    DivisionAssign,
    ModulusAssign,
    ExponentiationAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseOrAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    NullishCoalescingAssign,
    LogicalOrAssign,
    LogicalAndAssign,
}

/// [1,2,3]
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpression {
    pub items: Vec<Option<ArrayExpressionItem>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayExpressionItem {
    Spread(SpreadElement),
    Expression(Expression),
}

/// [a] = b
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayBinding {
    pub items: Vec<Option<ArrayBindingItemKind>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrayBindingItemKind {
    Item(ArrayBindingItem),
    Rest(RestElement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayBindingItem {
    pub binding: Binding,
    pub initializer: Option<Expression>,
}

/// () => {}
/// () => 3 * 3
#[derive(Debug, PartialEq, Clone)]
pub struct ArrowFunctionExpression {
    pub parameters: Vec<ParameterKind>,
    pub body: ArrowFunctionExpressionBody,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArrowFunctionExpressionBody {
    BlockStatement(BlockStatement),
    Expression(Box<Expression>),
}

/// 1n
///
/// The value is stored as a string to avoid precision loss.
///
/// The string does not include the n suffix.
#[derive(Debug, PartialEq, Clone)]
pub struct BigIntLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
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

impl PrecedenceInfo for BinaryExpressionOperator {
    fn precedence(&self) -> Precedence {
        match self {
            BinaryExpressionOperator::Addition => Precedence::Sum,
            BinaryExpressionOperator::Substitution => Precedence::Sum,
            BinaryExpressionOperator::Multiplication => Precedence::Product,
            BinaryExpressionOperator::Division => Precedence::Product,
            BinaryExpressionOperator::Modulus => Precedence::Product,
            BinaryExpressionOperator::Exponentiation => Precedence::Exponentiation,
            BinaryExpressionOperator::LessThan => Precedence::Compare,
            BinaryExpressionOperator::LessThanEquals => Precedence::Compare,
            BinaryExpressionOperator::GreaterThan => Precedence::Compare,
            BinaryExpressionOperator::GreaterThanEquals => Precedence::Compare,
            BinaryExpressionOperator::In => Precedence::Compare,
            BinaryExpressionOperator::Instanceof => Precedence::Compare,
            BinaryExpressionOperator::LeftShift => Precedence::Shift,
            BinaryExpressionOperator::RightShift => Precedence::Shift,
            BinaryExpressionOperator::UnsignedRightShift => Precedence::Shift,
            BinaryExpressionOperator::LooseEquals => Precedence::Equals,
            BinaryExpressionOperator::LooseNotEquals => Precedence::Equals,
            BinaryExpressionOperator::StrictEquals => Precedence::Equals,
            BinaryExpressionOperator::StrictNotEquals => Precedence::Equals,
            BinaryExpressionOperator::NullishCoalescing => Precedence::NullishCoalescing,
            BinaryExpressionOperator::BitwiseOr => Precedence::BitwiseOr,
            BinaryExpressionOperator::BitwiseAnd => Precedence::BitwiseAnd,
            BinaryExpressionOperator::BitwiseXor => Precedence::BitwiseXor,
        }
    }

    fn is_right_associative(&self) -> bool {
        match self {
            &BinaryExpressionOperator::Exponentiation => true,
            _ => false,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            &BinaryExpressionOperator::Exponentiation => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

/// true | false
#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

/// a()
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<ArgumentKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    pub param: Binding,
    pub body: BlockStatement,
}

/// class A {}
///
/// class A extends B {}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub identifier: Identifier,
    pub extends: Option<Expression>,
    pub body: Vec<ClassPropertyKind>,
}

/// let a = class {}
///
/// let a = class extends B {}
///
/// let a = class B {}
///
/// let a = class B extends C {}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassExpression {
    pub identifier: Option<Identifier>,
    pub body: Vec<ClassPropertyKind>,
    pub extends: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClassPropertyKind {
    Constructor(ClassConstructor),
    Method(ClassMethod),
    MethodComputed(ClassMethodComputed),
    MethodGet(ClassMethodGet),
    MethodGetComputed(ClassMethodGetComputed),
    MethodSet(ClassMethodSet),
    MethodSetComputed(ClassMethodSetComputed),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassConstructor {
    pub is_static: bool,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethod {
    pub is_static: bool,
    pub identifier: LiteralPropertyName,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethodComputed {
    pub is_static: bool,
    pub key: Expression,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethodGet {
    pub is_static: bool,
    pub identifier: LiteralPropertyName,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethodGetComputed {
    pub is_static: bool,
    pub key: Expression,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethodSet {
    pub is_static: bool,
    pub identifier: LiteralPropertyName,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethodSetComputed {
    pub is_static: bool,
    pub key: Expression,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

/// a ? b : c
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternate: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DebuggerStatement {}

#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmptyStatement {}

/// import a * from "b";
#[derive(Debug, PartialEq, Clone)]
pub struct ExportAllDeclaration {
    pub source: StringLiteral,
}

/// `export default function a() {}`
///
/// `export default Class A() {}`
///
/// `export default 3 + 3;`
#[derive(Debug, PartialEq, Clone)]
pub struct ExportDefaultDeclaration {
    pub declaration: ExportDefaultDeclarationKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportDefaultDeclarationKind {
    AnonymousDefaultExportedFunctionDeclaration(AnonymousDefaultExportedFunctionDeclaration),
    AnonymousDefaultExportedClassDeclaration(AnonymousDefaultExportedClassDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    Expression(Expression),
    ClassDeclaration(ClassDeclaration),
}

/// export function a() {}
///
/// export let a;
#[derive(Debug, PartialEq, Clone)]
pub struct ExportNamedDeclaration {
    pub declaration: ExportNamedDeclarationKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportNamedDeclarationKind {
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    ClassDeclaration(ClassDeclaration),
}

/// export { a, b, c };
///
/// export { a as b };
#[derive(Debug, PartialEq, Clone)]
pub struct ExportNamedSpecifiers {
    pub specifiers: Vec<ExportNamedSpecifier>,
    pub source: Option<StringLiteral>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExportNamedSpecifier {
    pub exported: Identifier,
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForInStatement {
    pub left: Box<Statement>,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForOfStatement {
    pub left: Box<Statement>,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ForStatementInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Binding(Binding),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub identifier: Identifier,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
    pub generator: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub identifier: Option<Identifier>,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
    pub generator: bool,
}

/// a
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    pub default: Option<Identifier>,
    pub namespace: Option<Identifier>,
    pub specifiers: Vec<ImportDeclarationSpecifier>,
    pub source: StringLiteral,
}

/// import { a }
#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclarationSpecifier {
    pub local: Identifier,
    pub imported: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabeledStatement {
    pub identifier: Identifier,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpression {
    pub left: Box<Expression>,
    pub operator: LogicalExpressionOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalExpressionOperator {
    Or,
    And,
    NullishCoalescing,
}

impl PrecedenceInfo for LogicalExpressionOperator {
    fn precedence(&self) -> Precedence {
        match self {
            LogicalExpressionOperator::Or => Precedence::LogicalOr,
            LogicalExpressionOperator::And => Precedence::LogicalAnd,
            LogicalExpressionOperator::NullishCoalescing => Precedence::NullishCoalescing,
        }
    }

    fn is_right_associative(&self) -> bool {
        false
    }

    fn is_left_associative(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
}

/// a[b] | a.b
#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<ArgumentKind>,
}

/// null
#[derive(Debug, PartialEq, Clone)]
pub struct NullLiteral {}

/// 1
#[derive(Debug, PartialEq, Clone)]
pub struct NumericLiteral {
    pub value: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectExpressionPropertyKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectExpressionPropertyKind {
    Spread(SpreadElement),
    Property(ObjectExpressionProperty),
    Shorthand(ObjectExpressionPropertyShorthand),
    Computed(ObjectExpressionPropertyComputed),
    Method(ObjectExpressionMethod),
    MethodComputed(ObjectExpressionMethodComputed),
    MethodGet(ObjectExpressionMethodGet),
    MethodGetComputed(ObjectExpressionMethodGetComputed),
    MethodSet(ObjectExpressionMethodSet),
    MethodSetComputed(ObjectExpressionMethodSetComputed),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionProperty {
    pub key: LiteralPropertyName,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionPropertyComputed {
    pub key: Expression,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionPropertyShorthand {
    pub key: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionMethod {
    pub key: LiteralPropertyName,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionMethodComputed {
    pub key: Expression,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionMethodGet {
    pub key: LiteralPropertyName,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionMethodGetComputed {
    pub key: Expression,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionMethodSet {
    pub key: LiteralPropertyName,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionMethodSetComputed {
    pub key: Expression,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBinding {
    pub properties: Vec<ObjectBindingPropertyKind>,
}

/// Most other AST specs defines a more generic type
/// for the properties and rely on the logic to prohibit
/// illegal properties, packet however defines a stricter AST
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectBindingPropertyKind {
    Property(ObjectBindingProperty),
    Computed(ObjectBindingPropertyComputed),
    Rest(ObjectBindingPropertyRest),
    Shorthand(ObjectBindingPropertyShorthand),
}

/// ({ a: b } = c)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingProperty {
    pub key: LiteralPropertyName,
    pub binding: Binding,
    pub initializer: Option<Expression>,
}

/// ({ [a]: b } = c)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingPropertyComputed {
    pub key: Expression,
    pub binding: Binding,
    pub initializer: Option<Expression>,
}

/// ({ ...a } = b)
///
/// Not that the rest binding is different for objects,
/// since the key/element can only be an identifier and not another binding.
///
/// This is valid ({ ...a } = b)
/// But this is not ({ ...{ a } = b)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingPropertyRest {
    pub key: Identifier,
}

/// ({ a } = b)
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectBindingPropertyShorthand {
    pub key: Identifier,
    pub initializer: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterKind {
    Parameter(Parameter),
    Rest(RestElement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub binding: Binding,
    pub initializer: Option<Expression>,
}

/// /abc/
#[derive(Debug, PartialEq, Clone)]
pub struct RegexpLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RestElement {
    pub binding: Binding,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SequenceExpression {
    pub expressions: Vec<Expression>,
}

/// ...a
/// ...3 + 3
#[derive(Debug, PartialEq, Clone)]
pub struct SpreadElement {
    pub element: Expression,
}

/// "a"
#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
}

/// super() | super.
#[derive(Debug, PartialEq, Clone)]
pub struct SuperExpression {}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchStatementCase>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatementCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Box<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TemplateLiteral {
    pub head: String,
    pub parts: Vec<TemplateLiteralPart>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TemplateLiteralPart {
    pub expression: Expression,
    pub text: String,
}

/// this
#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpression {}

#[derive(Debug, PartialEq, Clone)]
pub struct ThrowStatement {
    pub argument: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryStatement {
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

/// +a | -a | ~a | !a | void a | typeof a | delete a
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryExpressionOperator,
    pub argument: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryExpressionOperator {
    Positive,
    Negative,
    BinaryNot,
    LogicalNot,
    Void,
    Typeof,
    Delete,
}

impl PrecedenceInfo for UnaryExpressionOperator {
    fn precedence(&self) -> Precedence {
        match self {
            UnaryExpressionOperator::Positive => Precedence::Prefix,
            UnaryExpressionOperator::Negative => Precedence::Prefix,
            UnaryExpressionOperator::BinaryNot => Precedence::Prefix,
            UnaryExpressionOperator::LogicalNot => Precedence::Prefix,
            UnaryExpressionOperator::Void => Precedence::Prefix,
            UnaryExpressionOperator::Typeof => Precedence::Prefix,
            UnaryExpressionOperator::Delete => Precedence::Prefix,
        }
    }

    fn is_right_associative(&self) -> bool {
        true
    }

    fn is_left_associative(&self) -> bool {
        false
    }
}

/// ++a | --a | a++ | a--
#[derive(Debug, PartialEq, Clone)]
pub struct UpdateExpression {
    pub operator: UpdateExpressionOperator,
    pub argument: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UpdateExpressionOperator {
    PrefixIncrement,
    PrefixDecrement,
    PostfixIncrement,
    PostfixDecrement,
}

impl PrecedenceInfo for UpdateExpressionOperator {
    fn precedence(&self) -> Precedence {
        match self {
            UpdateExpressionOperator::PrefixIncrement => Precedence::Prefix,
            UpdateExpressionOperator::PrefixDecrement => Precedence::Prefix,
            UpdateExpressionOperator::PostfixIncrement => Precedence::Postfix,
            UpdateExpressionOperator::PostfixDecrement => Precedence::Postfix,
        }
    }

    fn is_right_associative(&self) -> bool {
        match self {
            UpdateExpressionOperator::PrefixIncrement => true,
            UpdateExpressionOperator::PrefixDecrement => true,
            UpdateExpressionOperator::PostfixIncrement => false,
            UpdateExpressionOperator::PostfixDecrement => false,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            UpdateExpressionOperator::PrefixIncrement => false,
            UpdateExpressionOperator::PrefixDecrement => false,
            UpdateExpressionOperator::PostfixIncrement => false,
            UpdateExpressionOperator::PostfixDecrement => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub binding: Binding,
    pub initializer: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WithStatement {
    pub object: Expression,
    pub body: Box<Statement>,
}
