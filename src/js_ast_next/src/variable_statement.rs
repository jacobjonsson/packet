use span::Span;

use crate::variable_declaration::VariableDeclaration;

#[derive(Debug, Clone)]
pub struct VariableStatement {
    pub span: Span,
    pub declarations: Vec<VariableDeclaration>,
}
