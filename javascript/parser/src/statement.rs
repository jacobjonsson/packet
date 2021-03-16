use javascript_ast::{
    statement::{BlockStatement, FunctionDeclaration, IfStatement, ReturnStatement, Statement},
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

    pub(crate) fn parse_if_statement(&mut self) -> ParseResult<IfStatement> {
        self.expect_peek_token(Token::OpenParen)?;
        self.next_token();
        let test = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.expect_peek_token(Token::CloseParen)?;
        self.next_token();
        // TODO: Function declarations are not valid in strict mode.
        let consequent = self.parse_statement()?;

        let mut alternate: Option<Box<Statement>> = None;
        if self.peek_token == Token::Else {
            self.next_token();
            self.next_token();

            // TODO: Function declarations are not valid in strict mode.
            alternate = Some(Box::new(self.parse_statement()?));
        }

        Ok(IfStatement {
            alternate: alternate,
            consequent: Box::new(consequent),
            test,
        })
    }
}
