use span::Span;

#[derive(Debug, Clone)]
pub struct RegexpLiteral {
    pub span: Span,
    pub value: String,
}
