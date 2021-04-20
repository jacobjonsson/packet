pub mod array_binding_pattern;
pub mod array_expression;
pub mod array_hole;
pub mod assignment_expression;
pub mod binary_expression;
pub mod binding_identifier;
pub mod block_statement;
pub mod boolean_literal;
pub mod break_statement;
pub mod computed_property_name;
pub mod empty_statement;
pub mod expression_statement;
pub mod identifier_name;
pub mod identifier_reference;
pub mod label_identifier;
pub mod lexical_binding;
pub mod lexical_declaration;
pub mod null_literal;
pub mod numeric_literal;
pub mod object_binding_pattern;
pub mod precedence;
pub mod regexp_literal;
pub mod sequence_expression;
pub mod spread_element;
pub mod string_literal;
pub mod unary_expression;
pub mod update_expression;
pub mod variable_declaration;
pub mod variable_statement;

use array_binding_pattern::ArrayBindingPattern;
use array_expression::ArrayExpression;
use assignment_expression::AssignmentExpression;
use binary_expression::BinaryExpression;
use binding_identifier::BindingIdentifier;
use block_statement::BlockStatement;
use boolean_literal::BooleanLiteral;
use break_statement::BreakStatement;
use computed_property_name::ComputedPropertyName;
use empty_statement::EmptyStatement;
use expression_statement::ExpressionStatement;
use identifier_name::IdentifierName;
use lexical_declaration::LexicalDeclaration;
use null_literal::NullLiteral;
use numeric_literal::NumericLiteral;
use object_binding_pattern::ObjectBindingPattern;
use regexp_literal::RegexpLiteral;
use span::Span;
use string_literal::StringLiteral;
use unary_expression::UnaryExpression;
use update_expression::UpdateExpression;
use variable_statement::VariableStatement;

/// The top level ast node
#[derive(Debug, Clone)]
pub struct AST {
    pub statements: Vec<Statement>,
}

/// The top level expression union
#[derive(Debug, Clone)]
pub enum Expression {
    ArrayExpression(ArrayExpression),
    AssignmentExpression(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    BooleanLiteral(BooleanLiteral),
    NullLiteral(NullLiteral),
    NumericLiteral(NumericLiteral),
    RegexpLiteral(RegexpLiteral),
    StringLiteral(StringLiteral),
    UnaryExpression(UnaryExpression),
    UpdateExpression(UpdateExpression),
}

#[derive(Debug, Clone)]
pub enum LiteralPropertyName {
    IdentifierName(IdentifierName),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
}

#[derive(Debug, Clone)]
pub enum ObjectPropertyKey {
    LiteralPropertyName(LiteralPropertyName),
    ComputedPropertyName(ComputedPropertyName),
}

/// The top level statement union
#[derive(Debug, Clone)]
pub enum Statement {
    BreakStatement(BreakStatement),
    BlockStatement(BlockStatement),
    EmptyStatement(EmptyStatement),
    ExpressionStatement(ExpressionStatement),
    VariableStatement(VariableStatement),
    LexicalDeclaration(LexicalDeclaration),
}

#[derive(Debug, Clone)]
pub enum TargetBindingPattern {
    BindingIdentifier(BindingIdentifier),
    BindingArrayPattern(ArrayBindingPattern),
    BindingObjectPattern(ObjectBindingPattern),
}
