use span::Span;

use crate::Statement;

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub span: Span,
    pub statements: Vec<Statement>,
}
