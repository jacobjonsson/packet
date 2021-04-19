use span::Span;

use crate::lexical_binding::LexicalBinding;

#[derive(Debug, Clone)]
pub struct LexicalDeclaration {
    pub span: Span,
    pub declarations: Vec<LexicalBinding>,
    pub is_const: bool,
}
