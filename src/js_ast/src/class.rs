use crate::{
    expression::{Expression, FunctionExpression, Identifier},
    object::LiteralPropertyName,
};

/// Class declaration
/// class A {}
/// class A extends B {}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub identifier: Identifier,
    pub extends: Option<Expression>,
    pub body: Vec<ClassProperty>,
}

/// Class expression
/// let a = class {}
/// let a = class extends B {}
/// let a = class B {}
/// let a = class B extends C {}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassExpression {
    pub identifier: Option<Identifier>,
    pub body: Vec<ClassProperty>,
    pub extends: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClassProperty {
    ClassConstructor(ClassConstructor),
    ClassMethod(ClassMethod),
    ComputedClassMethod(ComputedClassMethod),
    ClassGetMethod(ClassGetMethod),
    ComputedClassGetMethod(ComputedClassGetMethod),
    ClassSetMethod(ClassSetMethod),
    ComputedClassSetMethod(ComputedClassSetMethod),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassConstructor {
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassMethod {
    pub is_static: bool,
    pub identifier: LiteralPropertyName,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedClassMethod {
    pub is_static: bool,
    pub key: Expression,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassGetMethod {
    pub is_static: bool,
    pub identifier: LiteralPropertyName,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedClassGetMethod {
    pub is_static: bool,
    pub key: Expression,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassSetMethod {
    pub is_static: bool,
    pub identifier: LiteralPropertyName,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedClassSetMethod {
    pub is_static: bool,
    pub key: Expression,
    pub value: FunctionExpression,
}
