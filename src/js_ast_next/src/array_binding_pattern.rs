use span::Span;

use crate::{array_hole::ArrayHole, Expression, TargetBindingPattern};

#[derive(Debug, Clone)]
pub struct ArrayBindingElement {
    pub span: Span,
    pub binding: TargetBindingPattern,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum ArrayBindingElementKind {
    ArrayHole(ArrayHole),
    ArrayBindingElement(ArrayBindingElement),
}

#[derive(Debug, Clone)]
pub struct ArrayBindingPattern {
    pub span: Span,
    pub elements: Vec<ArrayBindingElementKind>,
    pub rest: Option<Box<TargetBindingPattern>>,
}
