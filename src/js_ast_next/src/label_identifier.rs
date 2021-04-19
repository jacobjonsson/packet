use span::Span;

#[derive(Debug, Clone)]
pub struct LabelIdentifier {
    pub span: Span,
    pub name: String,
}
