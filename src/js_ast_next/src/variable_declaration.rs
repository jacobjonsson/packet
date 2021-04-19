use span::Span;

use crate::{Expression, TargetBindingPattern};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub span: Span,
    pub binding: TargetBindingPattern,
    pub initializer: Option<Expression>,
}
