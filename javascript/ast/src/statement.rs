use crate::expression::{Expression, Identifier, StringLiteral};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(ExpressionStatement),
    ImportDeclaration(ImportDeclaration),
}

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
