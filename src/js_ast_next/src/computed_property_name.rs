use span::Span;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct ComputedPropertyName {
    pub span: Span,
    pub name: Expression,
}
