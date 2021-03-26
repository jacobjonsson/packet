use js_ast::{expression::*, function::*, object::*};
use js_token::Token;

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_object_expression(&mut self) -> ParseResult<ObjectExpression> {
        self.lexer.next_token();
        let mut properties: Vec<ObjectExpressionProperty> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            let property = self.parse_object_expression_property()?;
            properties.push(property);
            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(ObjectExpression { properties })
    }

    fn parse_object_expression_property(&mut self) -> ParseResult<ObjectExpressionProperty> {
        match self.lexer.identifier.as_str() {
            "get" => {
                self.lexer.next_token();
                if self.lexer.token == Token::OpenParen || self.lexer.token == Token::Colon {
                    self.parse_object_property_or_method(Some(Identifier {
                        name: String::from("get"),
                    }))
                } else {
                    self.parse_object_get_method()
                }
            }
            "set" => {
                self.lexer.next_token();
                if self.lexer.token == Token::OpenParen || self.lexer.token == Token::Colon {
                    self.parse_object_property_or_method(Some(Identifier {
                        name: String::from("set"),
                    }))
                } else {
                    self.parse_object_set_method()
                }
            }
            _ => self.parse_object_property_or_method(None),
        }
    }

    fn parse_object_property_or_method(
        &mut self,
        conditional_identifier: Option<Identifier>,
    ) -> ParseResult<ObjectExpressionProperty> {
        // This means we've hit a computed property.
        // Short syntax is not support here so we can assume
        // that the value or method must exist, otherwise it's a syntax error.
        if self.lexer.token == Token::OpenBracket {
            let key = self.parse_computed_property_name()?;

            // [a]() {}
            if self.lexer.token == Token::OpenParen {
                let parameters = self.parse_function_parameters()?;
                let body = self.parse_block_statement()?;
                return Ok(ObjectExpressionProperty::ComputedObjectMethod(
                    ComputedObjectMethod {
                        key,
                        value: FunctionExpression {
                            body,
                            parameters,
                            id: None,
                        },
                    },
                ));
            }

            self.lexer.expect_token(Token::Colon);
            self.lexer.next_token();
            let value = self.parse_expression(Precedence::Comma)?;
            return Ok(ObjectExpressionProperty::ComputedObjectProperty(
                ComputedObjectProperty { key, value },
            ));
        }

        // If we were passed a conditional identifier,
        // use that one instead of parsing the current token.
        let identifier: LiteralPropertyName;
        if let Some(id) = conditional_identifier {
            identifier = LiteralPropertyName::Identifier(id);
        } else {
            identifier = self.parse_literal_property_name()?;
        }
        if self.lexer.token == Token::OpenParen {
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            return Ok(ObjectExpressionProperty::ObjectMethod(ObjectMethod {
                identifier,
                value: FunctionExpression {
                    body,
                    parameters,
                    id: None,
                },
            }));
        }

        // Means we've hit the shorthand syntax
        if self.lexer.token != Token::Colon {
            // Identifier is only allowed to be of type Identifier in the shorthand
            // syntax. If the identifier is not of the correct type, we report a syntax error.
            return Ok(ObjectExpressionProperty::ObjectPropertyShorthand(
                ObjectPropertyShorthand {
                    identifier: match identifier {
                        LiteralPropertyName::Identifier(i) => i,
                        _ => self.lexer.unexpected(),
                    },
                },
            ));
        }

        self.lexer.expect_token(Token::Colon);
        self.lexer.next_token();
        let value = self.parse_expression(Precedence::Comma)?;
        return Ok(ObjectExpressionProperty::ObjectProperty(ObjectProperty {
            identifier,
            value,
        }));
    }

    fn parse_object_get_method(&mut self) -> ParseResult<ObjectExpressionProperty> {
        if self.lexer.token == Token::OpenBracket {
            let key = self.parse_computed_property_name()?;
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            return Ok(ObjectExpressionProperty::ComputedObjectGetMethod(
                ComputedObjectGetMethod {
                    key,
                    value: FunctionExpression {
                        body,
                        parameters,
                        id: None,
                    },
                },
            ));
        }

        let identifier = self.parse_literal_property_name()?;
        let parameters = self.parse_function_parameters()?;
        let body = self.parse_block_statement()?;
        Ok(ObjectExpressionProperty::ObjectGetMethod(ObjectGetMethod {
            identifier,
            value: FunctionExpression {
                body,
                parameters,
                id: None,
            },
        }))
    }

    fn parse_object_set_method(&mut self) -> ParseResult<ObjectExpressionProperty> {
        if self.lexer.token == Token::OpenBracket {
            let key = self.parse_computed_property_name()?;
            let parameters = self.parse_function_parameters()?;
            let body = self.parse_block_statement()?;
            return Ok(ObjectExpressionProperty::ComputedObjectSetMethod(
                ComputedObjectSetMethod {
                    key,
                    value: FunctionExpression {
                        body,
                        parameters,
                        id: None,
                    },
                },
            ));
        }

        let identifier = self.parse_literal_property_name()?;
        let parameters = self.parse_function_parameters()?;
        let body = self.parse_block_statement()?;
        Ok(ObjectExpressionProperty::ObjectSetMethod(ObjectSetMethod {
            identifier,
            value: FunctionExpression {
                body,
                parameters,
                id: None,
            },
        }))
    }

    pub(crate) fn parse_computed_property_name(&mut self) -> ParseResult<Expression> {
        self.lexer.next_token();
        let key = self.parse_expression(Precedence::Comma)?;
        self.lexer.expect_token(Token::CloseBracket);
        self.lexer.next_token();
        Ok(key)
    }
}
