use span::Span;

use crate::Expression;

/// (a,b,c)
#[derive(Debug, Clone)]
pub struct SequenceExpression {
    pub span: Span,
    pub expressions: Vec<Expression>,
}
