use span::Span;

use crate::{Expression, TargetBindingPattern};

#[derive(Debug, Clone)]
pub struct LexicalBinding {
    pub span: Span,
    pub binding: TargetBindingPattern,
    pub initializer: Option<Expression>,
}
