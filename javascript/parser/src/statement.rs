use javascript_ast::statement::{
    BlockStatement, FunctionDeclaration, IfStatement, ReturnStatement, Statement,
};
use javascript_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser};

impl Parser {
    /// Parses function declarations
    /// function a() {}
    /// function a(arg1) {}
    pub(crate) fn parse_function_declaration(&mut self) -> ParseResult<FunctionDeclaration> {
        self.lexer.next_token(); // Skip the function keyword.
        let identifier = self.parse_identifer()?;
        self.expect_token(Token::OpenParen);
        let parameters = self.parse_function_parameters()?;
        self.expect_token(Token::OpenBrace);
        let body = self.parse_block_statement()?;
        Ok(FunctionDeclaration {
            id: identifier,
            parameters,
            body,
        })
    }

    /// Parses a block statement
    /// { statement1; statement2; }
    pub(crate) fn parse_block_statement(&mut self) -> ParseResult<BlockStatement> {
        self.lexer.next_token();
        let mut statements: Vec<Statement> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            statements.push(self.parse_statement()?);
        }
        self.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(BlockStatement { statements })
    }

    /// Parse return statements
    /// return;
    /// return 1 + 1;
    pub(crate) fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        self.lexer.next_token();
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
            return Ok(ReturnStatement { expression: None });
        }

        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.consume_semicolon();
        Ok(ReturnStatement {
            expression: Some(expression),
        })
    }

    /// Parses an if statement
    /// if (test) { consequent } else { alternate }
    /// if (test) { consequent } else alternate
    pub(crate) fn parse_if_statement(&mut self) -> ParseResult<IfStatement> {
        self.lexer.next_token();
        self.expect_token(Token::OpenParen);
        self.lexer.next_token();
        let test = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.expect_token(Token::CloseParen);
        self.lexer.next_token();
        // TODO: Function declarations are not valid in strict mode.
        let consequent = self.parse_statement()?;
        let mut alternate: Option<Box<Statement>> = None;
        if self.lexer.token == Token::Else {
            self.lexer.next_token();
            // TODO: Function declarations are not valid in strict mode.
            alternate = self.parse_statement().map(Box::new).map(Some)?;
        }

        Ok(IfStatement {
            alternate: alternate,
            consequent: Box::new(consequent),
            test,
        })
    }
}
