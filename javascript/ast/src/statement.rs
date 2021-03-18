use crate::expression::{Expression, Identifier, StringLiteral};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Block(BlockStatement),
    Return(ReturnStatement),
    If(IfStatement),
    For(ForStatement),
    ForInStatement(ForInStatement),
    ForOfStatement(ForOfStatement),
    EmptyStatement(EmptyStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    Expression(ExpressionStatement),
    ImportDeclaration(ImportDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmptyStatement {}

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

#[derive(Debug, PartialEq, Clone)]
pub enum ForStatementInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub init: Option<ForStatementInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForInStatement {
    pub left: ForStatementInit,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForOfStatement {
    pub left: ForStatementInit,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

/* -------------------------------------------------------------------------- */
/*                                  Function                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub parameters: Vec<Identifier>, // TODO: es6 and upwards supports more patterns, see here: https://github.com/estree/estree/blob/master/es5.md#patterns
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
    pub id: Identifier, // TODO: Identifier is the only supported pattern for es5, there are more patterns for the es6, es7 and etc.
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
