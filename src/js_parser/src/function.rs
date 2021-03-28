use js_ast::{expression::*, function::*};
use js_token::Token;
use logger::Logger;

use crate::{ParseResult, Parser};

impl<'a, L: Logger> Parser<'a, L> {
    /// function a() {}
    pub(crate) fn parse_function_declaration(&mut self) -> ParseResult<FunctionDeclaration> {
        self.lexer.next_token(); // Skip the function keyword.
        let identifier = self.parse_identifer()?;
        self.lexer.expect_token(Token::OpenParen);
        let parameters = self.parse_function_parameters()?;
        self.lexer.expect_token(Token::OpenBrace);
        let body = self.parse_block_statement()?;
        Ok(FunctionDeclaration {
            id: identifier,
            parameters,
            body,
        })
    }

    /// let a = function a() {}
    pub(crate) fn parse_function_expression(&mut self) -> ParseResult<FunctionExpression> {
        self.lexer.next_token();
        let mut id: Option<Identifier> = None;
        if self.lexer.token == Token::Identifier {
            id = Some(self.parse_identifer()?);
        }
        self.lexer.expect_token(Token::OpenParen);
        let parameters = self.parse_function_parameters()?;
        self.lexer.expect_token(Token::OpenBrace);
        let body = self.parse_block_statement()?;

        Ok(FunctionExpression {
            parameters,
            body,
            id,
        })
    }

    /// let a = b() => {}
    #[allow(dead_code)]
    pub(crate) fn parse_arrow_function_expression(&mut self) {
        todo!()
    }

    /// Parses function parameters
    pub(crate) fn parse_function_parameters(&mut self) -> ParseResult<Vec<ParameterKind>> {
        self.lexer.next_token();
        let mut parameters: Vec<ParameterKind> = Vec::new();

        while self.lexer.token != Token::CloseParen {
            match &self.lexer.token {
                Token::DotDotDot => {
                    self.lexer.next_token();
                    let binding = self.parse_binding()?;
                    parameters.push(ParameterKind::RestParameter(RestParameter { binding }));
                }

                _ => {
                    let binding = self.parse_binding()?;
                    let mut default_value: Option<Expression> = None;
                    if self.lexer.token == Token::Equals {
                        self.lexer.next_token();
                        default_value = self.parse_expression(&Precedence::Comma).map(Some)?;
                    }
                    parameters.push(ParameterKind::Parameter(Parameter {
                        binding,
                        default_value,
                    }))
                }
            }

            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();

        Ok(parameters)
    }
}
