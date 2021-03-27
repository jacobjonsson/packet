pub mod binding;
pub mod class;
pub mod expression;
pub mod function;
pub mod literal;
pub mod object;
pub mod statement;

#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    pub statements: Vec<statement::Statement>,
}
