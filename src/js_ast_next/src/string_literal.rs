use span::Span;

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub span: Span,
    pub value: String,
}
