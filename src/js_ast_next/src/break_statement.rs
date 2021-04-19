use span::Span;

use crate::label_identifier::LabelIdentifier;

#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub span: Span,
    pub id: Option<LabelIdentifier>,
}
