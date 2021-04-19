use span::Span;

#[derive(Debug, Clone)]
pub struct IdentifierName {
    pub span: Span,
    pub name: String,
}
