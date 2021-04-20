use span::Span;

use crate::{
    binding_identifier::BindingIdentifier, Expression, ObjectPropertyKey, TargetBindingPattern,
};

#[derive(Debug, Clone)]
pub struct ObjectBindingProperty {
    pub span: Span,
    pub key: ObjectPropertyKey,
    pub value: TargetBindingPattern,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct SingleNameBinding {
    pub span: Span,
    pub identifier: BindingIdentifier,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum ObjectBindingPropertyKind {
    ObjectBindingProperty(ObjectBindingProperty),
    SingleNameBinding(SingleNameBinding),
}

#[derive(Debug, Clone)]
pub struct ObjectBindingPattern {
    pub span: Span,
    pub properties: Vec<ObjectBindingPropertyKind>,
    pub rest: Option<BindingIdentifier>,
}
