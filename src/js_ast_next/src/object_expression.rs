use span::Span;

#[derive(Debug, Clone)]
pub struct ObjectExpression {
    pub span: Span,
    pub properties: Vec<()>,
}
