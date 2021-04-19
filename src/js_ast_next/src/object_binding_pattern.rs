use span::Span;

use crate::{binding_identifier::BindingIdentifier, ObjectPropertyKey, TargetBindingPattern};

#[derive(Debug, Clone)]
pub struct ObjectBindingProperty {
    pub span: Span,
    pub key: ObjectPropertyKey,
    pub value: TargetBindingPattern,
}

#[derive(Debug, Clone)]
pub struct BindingObjectPattern {
    pub span: Span,
    pub elements: Vec<ObjectBindingProperty>,
    pub rest: Option<BindingIdentifier>,
}
