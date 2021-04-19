use span::Span;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct UpdateExpression {
    pub span: Span,
    pub argument: Box<Expression>,
    pub operator: UpdateExpressionOperator,
    /// Is this a prefix or postfix operation
    pub prefix: bool,
}

#[derive(Debug, Clone)]
pub enum UpdateExpressionOperator {
    Increment,
    Decrement,
}
