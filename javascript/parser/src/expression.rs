use javascript_ast::expression::{CallExpression, Expression};
use javascript_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser, ParserError};

impl Parser {
    /// Parse call expression
    /// a()
    /// a(3 + 3)
    pub(crate) fn parse_call_expression(
        &mut self,
        function: Expression,
    ) -> ParseResult<CallExpression> {
        let arguments = self.parse_call_expression_arguments()?;
        let function = match function {
            Expression::Identifier(i) => i,
            e => {
                return Err(ParserError(format!(
                    "Expected a call expression but got {:?}",
                    e
                )))
            }
        };

        Ok(CallExpression {
            arguments,
            function,
        })
    }

    fn parse_call_expression_arguments(&mut self) -> ParseResult<Vec<Box<Expression>>> {
        let mut arguments: Vec<Box<Expression>> = Vec::new();
        if self.peek_token == Token::CloseParen {
            self.next_token();
            return Ok(arguments);
        }
        self.next_token();
        arguments.push(Box::new(self.parse_expression(OperatorPrecedence::Lowest)?));
        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            arguments.push(Box::new(self.parse_expression(OperatorPrecedence::Lowest)?));
        }
        self.expect_peek_token(Token::CloseParen)?;
        Ok(arguments)
    }
}
