use span::Span;

#[derive(Debug, Clone)]
pub struct BindingIdentifier {
    pub span: Span,
    pub name: String,
}
