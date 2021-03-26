use crate::{binding::Binding, class::ClassDeclaration, expression::*, literal::StringLiteral};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    BlockStatement(BlockStatement),
    ReturnStatement(ReturnStatement),
    IfStatement(IfStatement),
    DebuggerStatement(DebuggerStatement),
    ForStatement(ForStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    EmptyStatement(EmptyStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    FunctionDeclaration(FunctionDeclaration),
    AnonymousDefaultExportedFunctionDeclaration(AnonymousDefaultExportedFunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    Expression(ExpressionStatement),
    ImportDeclaration(ImportDeclaration),
    SwitchStatement(SwitchStatement),
    WithStatement(WithStatement),
    LabeledStatement(LabeledStatement),
    ThrowStatement(ThrowStatement),
    TryStatement(TryStatement),
    ExportAllDeclaration(ExportAllDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
    ExportDefaultDeclaration(ExportDefaultDeclaration),
    ClassDeclaration(ClassDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmptyStatement {}

#[derive(Debug, PartialEq, Clone)]
pub struct DebuggerStatement {}

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

#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    pub param: Binding,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WithStatement {
    pub object: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

/* ------------------------------ Start for-statements ----------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum ForStatementInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Pattern(Binding),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
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

/* ---------------------------- End for-statements --------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Box<Statement>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabeledStatement {
    pub identifier: Identifier,
    pub body: Box<Statement>,
}

/* -------------------------------------------------------------------------- */
/*                                  Function                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub parameters: Vec<Binding>,
    pub body: BlockStatement,
}

/// This is only allowed in export statements.
#[derive(Debug, PartialEq, Clone)]
pub struct AnonymousDefaultExportedFunctionDeclaration {
    pub parameters: Vec<Binding>,
    pub body: BlockStatement,
}

/* -------------------------------------------------------------------------- */
/*                                  Variables                                 */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub id: Binding, // NB: IdentifierBinding is not permitted here.
    pub init: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

/* -------------------------------------------------------------------------- */
/*                                   Import                                   */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDeclaration {
    pub source: StringLiteral,
    pub specifiers: Vec<ImportClause>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportClause {
    Import(ImportSpecifier),
    ImportDefault(ImportDefaultSpecifier),
    ImportNamespace(ImportNamespaceSpecifier),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportSpecifier {
    pub local: Identifier,
    pub imported: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportDefaultSpecifier {
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImportNamespaceSpecifier {
    pub local: Identifier,
}

/* -------------------------------------------------------------------------- */
/*                                   Export                                   */
/* -------------------------------------------------------------------------- */
#[derive(Debug, PartialEq, Clone)]
pub struct ExportAllDeclaration {
    pub source: StringLiteral,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExportNamedDeclaration {
    pub declaration: Option<Declaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<StringLiteral>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExportSpecifier {
    pub local: Identifier,
    pub exported: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportDefaultDeclarationKind {
    FunctionDeclaration(FunctionDeclaration),
    Expression(Expression),
    AnonymousDefaultExportedFunctionDeclaration(AnonymousDefaultExportedFunctionDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExportDefaultDeclaration {
    pub declaration: ExportDefaultDeclarationKind,
}
