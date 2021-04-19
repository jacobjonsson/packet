use span::Span;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub span: Span,
    pub argument: Box<Expression>,
    pub operator: UnaryExpressionOperator,
}

#[derive(Debug, Clone)]
pub enum UnaryExpressionOperator {
    Delete,
    Void,
    Typeof,
    Positive,
    Negative,
    BitwiseNot,
    LogicalNot,
}
