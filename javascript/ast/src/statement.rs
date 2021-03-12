use crate::expression::{Expression, Identifier};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(ExpressionStatement),
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
