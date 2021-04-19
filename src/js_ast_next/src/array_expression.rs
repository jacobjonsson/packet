use crate::{array_hole::ArrayHole, spread_element::SpreadElement, Expression};
use span::Span;

#[derive(Debug, Clone)]
pub enum ArrayExpressionElement {
    Expression(Expression),
    Spread(SpreadElement),
    Hole(ArrayHole),
}

#[derive(Debug, Clone)]
pub struct ArrayExpression {
    pub span: Span,
    pub elements: Vec<ArrayExpressionElement>,
}
