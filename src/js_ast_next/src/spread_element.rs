use span::Span;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct SpreadElement {
    pub span: Span,
    pub argument: Expression,
}
