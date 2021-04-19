use span::Span;

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub span: Span,
    pub value: f64,
}
