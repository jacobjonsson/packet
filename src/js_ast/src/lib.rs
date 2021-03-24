pub mod class;
pub mod expression;
pub mod statement;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<statement::Statement>,
}
