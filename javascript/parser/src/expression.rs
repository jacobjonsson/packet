use javascript_ast::expression::{Expression, FunctionExpression, Identifier};
use javascript_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_call_expression_arguments(&mut self) -> ParseResult<Vec<Box<Expression>>> {
        let mut arguments: Vec<Box<Expression>> = Vec::new();
        self.lexer.next_token();
        if self.lexer.token == Token::CloseParen {
            self.lexer.next_token();
            return Ok(arguments);
        }
        arguments.push(Box::new(self.parse_expression(OperatorPrecedence::Lowest)?));
        while self.lexer.token == Token::Comma {
            self.lexer.next_token();
            arguments.push(Box::new(self.parse_expression(OperatorPrecedence::Lowest)?));
        }
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();
        Ok(arguments)
    }

    /// Parse function expression
    /// let a = function() {}
    /// a(function() {})
    pub(crate) fn parse_function_expression(&mut self) -> ParseResult<FunctionExpression> {
        self.lexer.next_token();
        self.lexer.expect_token(Token::OpenParen);
        let parameters = self.parse_function_parameters()?;
        self.lexer.expect_token(Token::OpenBrace);
        let body = self.parse_block_statement()?;

        Ok(FunctionExpression { parameters, body })
    }

    /// Parses function parameters
    /// Note that we currently only return a vector of identifiers.
    /// In the future we need to support all of the patterns, see here: https://github.com/estree/estree/blob/master/es5.md#patterns
    pub(crate) fn parse_function_parameters(&mut self) -> ParseResult<Vec<Identifier>> {
        let mut parameters: Vec<Identifier> = Vec::new();

        // Means there aren't any parameters to parse
        self.lexer.next_token();
        if self.lexer.token == Token::CloseParen {
            self.lexer.next_token(); // Skip the closing paren
            return Ok(Vec::new());
        }

        // Parse the first parameter
        parameters.push(self.parse_identifer()?);

        // As long as the next token is a comma, we keep parsing identifiers.
        while self.lexer.token == Token::Comma {
            self.lexer.next_token();
            parameters.push(self.parse_identifer()?);
        }
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();
        Ok(parameters)
    }
}
