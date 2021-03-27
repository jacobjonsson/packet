use js_ast::{
    binding::*,
    expression::{Expression, Precedence},
    object::LiteralPropertyName,
};
use js_token::Token;
use logger::Logger;

use crate::{ParseResult, Parser};

impl<'a, L: Logger> Parser<'a, L> {
    pub(crate) fn parse_binding(&mut self) -> ParseResult<Binding> {
        match self.lexer.token {
            Token::Identifier => Ok(Binding::Identifier(self.parse_identifer()?)),
            Token::OpenBrace => self.parse_object_binding().map(Binding::ObjectBinding),
            Token::OpenBracket => self.parse_array_binding().map(Binding::ArrayBinding),
            _ => self.lexer.unexpected(),
        }
    }

    fn parse_object_binding(&mut self) -> ParseResult<ObjectBinding> {
        self.lexer.next_token();
        let mut properties: Vec<ObjectBindingProperty> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            match self.lexer.token {
                // { ...a }
                Token::DotDotDot => {
                    self.lexer.next_token();
                    // Note that the rest element inside of object has different constraints compared
                    // to arrays, hence why we hand code the parsing of the rest element here instead of using
                    // parse_rest_element_binding. The only node that is be a rest element inside an object
                    // is an identifier, anything else is a syntax error.
                    let identifier = self.parse_identifer()?;
                    properties.push(ObjectBindingProperty {
                        default_value: None,
                        property: ObjectBindingPropertyKind::ObjectBindingRestProperty(
                            ObjectBindingRestProperty { identifier },
                        ),
                    })
                }

                // { [a]: b }
                Token::OpenBracket => {
                    let key = self.parse_computed_property_name()?;
                    self.lexer.expect_token(Token::Colon);
                    self.lexer.next_token();
                    let value = self.parse_binding()?;
                    let default_value = self.parse_optional_default_value()?;
                    properties.push(ObjectBindingProperty {
                        default_value,
                        property: ObjectBindingPropertyKind::ObjectBindingComputedProperty(
                            ObjectBindingComputedProperty { key, value },
                        ),
                    })
                }

                // Anything else: { a, a: b, "a": b, 2: b, null: b, undefined: b }
                _ => {
                    let identifier = self.parse_literal_property_name()?;
                    // Means we've hit a shorthand property.
                    if self.lexer.token != Token::Colon {
                        let default_value = self.parse_optional_default_value()?;
                        properties.push(ObjectBindingProperty {
                            default_value,
                            property: ObjectBindingPropertyKind::ObjectBindingShorthandProperty(
                                ObjectBindingShorthandProperty {
                                    identifier: match identifier {
                                        LiteralPropertyName::Identifier(i) => i,
                                        _ => self.lexer.unexpected(),
                                    },
                                },
                            ),
                        });
                    } else {
                        self.lexer.expect_token(Token::Colon);
                        self.lexer.next_token();
                        let value = self.parse_binding()?;
                        let default_value = self.parse_optional_default_value()?;
                        properties.push(ObjectBindingProperty {
                            default_value,
                            property: ObjectBindingPropertyKind::ObjectBindingStaticProperty(
                                ObjectBindingStaticProperty { identifier, value },
                            ),
                        })
                    }
                }
            }

            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(ObjectBinding { properties })
    }

    fn parse_array_binding(&mut self) -> ParseResult<ArrayBinding> {
        self.lexer.next_token();
        let mut items: Vec<ArrayBindingItem> = Vec::new();
        while self.lexer.token != Token::CloseBracket {
            match self.lexer.token {
                Token::DotDotDot => items.push(ArrayBindingItem {
                    default_value: None,
                    value: Binding::RestElementBinding(self.parse_rest_element_binding()?),
                }),

                Token::OpenBracket => {
                    let value = self.parse_array_binding()?;
                    let default_value = self.parse_optional_default_value()?;
                    items.push(ArrayBindingItem {
                        value: Binding::ArrayBinding(value),
                        default_value,
                    })
                }

                Token::OpenBrace => {
                    let value = self.parse_object_binding()?;
                    let default_value = self.parse_optional_default_value()?;
                    items.push(ArrayBindingItem {
                        value: Binding::ObjectBinding(value),
                        default_value,
                    })
                }

                Token::Identifier => {
                    let value = self.parse_identifer()?;
                    let default_value = self.parse_optional_default_value()?;
                    items.push(ArrayBindingItem {
                        value: Binding::Identifier(value),
                        default_value,
                    })
                }

                _ => self.lexer.unexpected(),
            };

            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseBracket);
        self.lexer.next_token();
        Ok(ArrayBinding { items })
    }

    fn parse_rest_element_binding(&mut self) -> ParseResult<RestElementBinding> {
        self.lexer.next_token();
        let key = match &self.lexer.token {
            Token::Identifier => RestElementBindingKey::Identifier(self.parse_identifer()?),
            Token::OpenBracket => RestElementBindingKey::ArrayBinding(self.parse_array_binding()?),
            Token::OpenBrace => RestElementBindingKey::ObjectBinding(self.parse_object_binding()?),

            _ => self.lexer.unexpected(),
        };
        Ok(RestElementBinding { key })
    }

    fn parse_optional_default_value(&mut self) -> ParseResult<Option<Expression>> {
        if self.lexer.token != Token::Equals {
            return Ok(None);
        }
        self.lexer.next_token();
        self.parse_expression(Precedence::Comma).map(Some)
    }
}
