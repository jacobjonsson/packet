use span::Span;

use crate::Expression;

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub span: Span,
    pub target: AssignmentExpressionTarget,
    pub operator: AssignmentExpressionOperator,
    pub argument: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum AssignmentExpressionTarget {}

#[derive(Debug, Clone)]
pub enum AssignmentExpressionOperator {
    Assign,
    AdditionAssign,
    SubstitutionAssign,
    MultiplicationAssign,
    DivisionAssign,
    ModulusAssign,
    ExponentiationAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseOrAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    NullishCoalescingAssign,
    LogicalOrAssign,
    LogicalAndAssign,
}
