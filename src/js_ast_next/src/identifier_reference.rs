use span::Span;

#[derive(Debug, Clone)]
pub struct IdentifierReference {
    pub span: Span,
    pub name: String,
}
