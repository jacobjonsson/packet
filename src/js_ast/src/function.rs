use crate::{
    binding::Binding,
    expression::{Expression, Identifier},
    statement::BlockStatement,
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

/// This is only allowed in export statements.
#[derive(Debug, PartialEq, Clone)]
pub struct AnonymousDefaultExportedFunctionDeclaration {
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub id: Option<Identifier>,
    pub parameters: Vec<ParameterKind>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParameterKind {
    Parameter(Parameter),
    RestParameter(RestParameter),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub binding: Binding,
    pub default_value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RestParameter {
    pub binding: Binding,
}
