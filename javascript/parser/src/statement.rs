use javascript_ast::{
    expression::Identifier,
    statement::{BlockStatement, FunctionDeclaration, ReturnStatement, Statement},
};
use javascript_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser};

impl Parser {
    /// Parses function declarations
    /// function a() {}
    /// function a(arg1) {}
    pub(crate) fn parse_function_declaration(&mut self) -> ParseResult<FunctionDeclaration> {
        self.next_token(); // Skip the function keyword.
        let identifier = self.parse_identifer()?;
        self.expect_peek_token(Token::OpenParen)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_peek_token(Token::OpenBrace)?;
        let body = self.parse_block_statement()?;
        Ok(FunctionDeclaration {
            id: identifier,
            parameters,
            body,
        })
    }

    /// Parses a function declarations parameters
    /// Note that we currently only return a vector of identifiers.
    /// In the future we need to support all of the patterns, see here: https://github.com/estree/estree/blob/master/es5.md#patterns
    fn parse_function_parameters(&mut self) -> ParseResult<Vec<Identifier>> {
        let mut parameters: Vec<Identifier> = Vec::new();

        // Means there aren't any parameters to parse
        if self.peek_token == Token::CloseParen {
            self.next_token(); // Skip the closing paren
            return Ok(Vec::new());
        }

        self.next_token();

        // Parse the first parameter
        parameters.push(self.parse_identifer()?);

        // As long as the next token is a comma, we keep parsing identifiers.
        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            parameters.push(self.parse_identifer()?);
        }
        self.expect_peek_token(Token::CloseParen)?;
        Ok(parameters)
    }

    pub(crate) fn parse_block_statement(&mut self) -> ParseResult<BlockStatement> {
        let mut statements: Vec<Statement> = Vec::new();
        self.next_token();
        while self.current_token != Token::CloseBrace {
            println!("{}", self.current_token);
            statements.push(self.parse_statement()?);
            self.next_token();
        }
        Ok(BlockStatement { statements })
    }

    /// Parse return statements
    /// return;
    /// return 1 + 1;
    pub(crate) fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        if self.peek_token == Token::Semicolon {
            self.next_token();
            return Ok(ReturnStatement { expression: None });
        }

        self.next_token();
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.consume_semicolon();
        Ok(ReturnStatement {
            expression: Some(expression),
        })
    }
}
