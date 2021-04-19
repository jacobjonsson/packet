use span::Span;

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}
