use js_ast::{class::*, expression::*, function::*};
use js_token::Token;

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_class_declaration(&mut self) -> ParseResult<ClassDeclaration> {
        self.lexer.next_token();
        let identifier = self.parse_identifer()?;
        let extends = match self.lexer.token {
            Token::Extends => {
                self.lexer.next_token();
                self.parse_expression(Precedence::Comma).map(Some)?
            }
            _ => None,
        };
        let body = self.parse_class_body()?;
        Ok(ClassDeclaration {
            body,
            extends,
            identifier,
        })
    }

    pub(crate) fn parse_class_expression(&mut self) -> ParseResult<ClassExpression> {
        self.lexer.next_token();
        let identifier = match self.lexer.token {
            Token::Identifier => self.parse_identifer().map(Some)?,
            _ => None,
        };
        let extends = match self.lexer.token {
            Token::Extends => {
                self.lexer.next_token(); // Skip the extends token.
                self.parse_expression(Precedence::Comma)
                    .map(Box::new)
                    .map(Some)?
            }
            _ => None,
        };
        let body = self.parse_class_body()?;
        Ok(ClassExpression {
            body,
            extends,
            identifier,
        })
    }

    pub fn parse_class_body(&mut self) -> ParseResult<Vec<ClassProperty>> {
        self.lexer.expect_token(Token::OpenBrace);
        self.lexer.next_token();
        let mut properties: Vec<ClassProperty> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            properties.push(self.parse_class_property()?)
        }
        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(properties)
    }

    fn parse_class_property(&mut self) -> ParseResult<ClassProperty> {
        let is_static = match self.lexer.identifier.as_str() {
            "static" => {
                self.lexer.next_token();
                true
            }
            _ => false,
        };

        match self.lexer.identifier.as_str() {
            "constructor" => self.parse_class_constructor(),
            "get" => self.parse_class_get_method(is_static),
            "set" => self.parse_class_set_method(is_static),
            _ => self.parse_class_method(is_static),
        }
    }

    fn parse_class_constructor(&mut self) -> ParseResult<ClassProperty> {
        self.lexer.next_token();
        let parameters = self.parse_function_parameters()?;
        let body = self.parse_block_statement()?;
        Ok(ClassProperty::ClassConstructor(ClassConstructor {
            value: FunctionExpression {
                body,
                parameters,
                id: None,
            },
        }))
    }

    fn parse_class_method(&mut self, is_static: bool) -> ParseResult<ClassProperty> {
        if self.lexer.token == Token::OpenBracket {
            self.lexer.next_token();
            let key = self.parse_expression(Precedence::Comma)?;
            self.lexer.expect_token(Token::CloseBracket);
            self.lexer.next_token();
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            Ok(ClassProperty::ComputedClassMethod(ComputedClassMethod {
                is_static,
                key,
                value: FunctionExpression {
                    body,
                    parameters,
                    id: None,
                },
            }))
        } else {
            let identifier = self.parse_literal_property_name()?;
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            Ok(ClassProperty::ClassMethod(ClassMethod {
                is_static,
                identifier,
                value: FunctionExpression {
                    body,
                    parameters,
                    id: None,
                },
            }))
        }
    }

    fn parse_class_get_method(&mut self, is_static: bool) -> ParseResult<ClassProperty> {
        self.lexer.next_token();
        if self.lexer.token == Token::OpenBracket {
            self.lexer.next_token();
            let key = self.parse_expression(Precedence::Comma)?;
            self.lexer.expect_token(Token::CloseBracket);
            self.lexer.next_token();
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            Ok(ClassProperty::ComputedClassGetMethod(
                ComputedClassGetMethod {
                    is_static,
                    key,
                    value: FunctionExpression {
                        body,
                        parameters,
                        id: None,
                    },
                },
            ))
        } else {
            let identifier = self.parse_literal_property_name()?;
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            Ok(ClassProperty::ClassGetMethod(ClassGetMethod {
                is_static,
                identifier,
                value: FunctionExpression {
                    body,
                    parameters,
                    id: None,
                },
            }))
        }
    }

    fn parse_class_set_method(&mut self, is_static: bool) -> ParseResult<ClassProperty> {
        self.lexer.next_token();
        if self.lexer.token == Token::OpenBracket {
            self.lexer.next_token();
            let key = self.parse_expression(Precedence::Comma)?;
            self.lexer.expect_token(Token::CloseBracket);
            self.lexer.next_token();
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            Ok(ClassProperty::ComputedClassSetMethod(
                ComputedClassSetMethod {
                    is_static,
                    key,
                    value: FunctionExpression {
                        body,
                        parameters,
                        id: None,
                    },
                },
            ))
        } else {
            let identifier = self.parse_literal_property_name()?;
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            Ok(ClassProperty::ClassSetMethod(ClassSetMethod {
                is_static,
                identifier,
                value: FunctionExpression {
                    body,
                    parameters,
                    id: None,
                },
            }))
        }
    }
}
