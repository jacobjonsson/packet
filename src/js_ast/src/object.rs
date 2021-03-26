use crate::{expression::*, function::*, literal::*};

/// This is shared with the class ast.
#[derive(Debug, PartialEq, Clone)]
pub enum LiteralPropertyName {
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectExpressionProperty>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectExpressionProperty {
    SpreadExpression(SpreadExpression),
    ObjectProperty(ObjectProperty),
    ObjectPropertyShorthand(ObjectPropertyShorthand),
    ComputedObjectProperty(ComputedObjectProperty),
    ObjectMethod(ObjectMethod),
    ComputedObjectMethod(ComputedObjectMethod),
    ObjectGetMethod(ObjectGetMethod),
    ComputedObjectGetMethod(ComputedObjectGetMethod),
    ObjectSetMethod(ObjectSetMethod),
    ComputedObjectSetMethod(ComputedObjectSetMethod),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectSpreadProperty {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectProperty {
    pub identifier: LiteralPropertyName,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectPropertyShorthand {
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedObjectProperty {
    pub key: Expression,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectMethod {
    pub identifier: LiteralPropertyName,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedObjectMethod {
    pub key: Expression,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectGetMethod {
    pub identifier: LiteralPropertyName,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedObjectGetMethod {
    pub key: Expression,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectSetMethod {
    pub identifier: LiteralPropertyName,
    pub value: FunctionExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComputedObjectSetMethod {
    pub key: Expression,
    pub value: FunctionExpression,
}
