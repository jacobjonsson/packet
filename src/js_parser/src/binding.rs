use js_ast::{
    binding::*,
    expression::{Expression, Precedence},
    object::LiteralPropertyName,
};
use js_token::Token;

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
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

// pub(crate) fn parse_pattern(&mut self) -> ParseResult<Pattern> {
//     match &self.lexer.token {
//         Token::Identifier => Ok(Pattern::Identifier(self.parse_identifer()?)),
//         Token::OpenBrace => Ok(Pattern::ObjectPattern(self.parse_object_pattern()?)),
//         Token::OpenBracket => Ok(Pattern::ArrayPattern(self.parse_array_pattern()?)),
//         _ => todo!(),
//     }
// }

// fn parse_object_pattern_property(&mut self) -> ParseResult<ObjectPatternProperty> {
//     let key: Expression;
//     let mut is_computed = false;

//     match self.lexer.token {
//         Token::DotDotDot => {
//             self.lexer.next_token();
//             let value = Pattern::Identifier(self.parse_identifer()?);
//             return Ok(ObjectPatternProperty {
//                 is_computed: false,
//                 is_rest: true,
//                 key: None,
//                 value,
//                 default_value: None,
//             });
//         }

//         Token::OpenBracket => {
//             is_computed = true;
//             self.lexer.next_token();
//             key = self.parse_expression(Precedence::Comma)?;
//             self.lexer.expect_token(Token::CloseBracket);
//             self.lexer.next_token();
//         }

//         Token::NumericLiteral => {
//             key = Expression::NumericLiteral(NumericLiteral {
//                 value: self.lexer.number,
//             });
//             self.lexer.next_token();
//         }

//         Token::StringLiteral => {
//             key = Expression::StringLiteral(self.parse_string_literal()?);
//         }

//         _ => {
//             let name = self.lexer.identifier.clone();
//             if !self.lexer.is_identifier_or_keyword() {
//                 self.lexer.expect_token(Token::Identifier);
//             }
//             self.lexer.next_token();
//             key = Expression::StringLiteral(StringLiteral {
//                 value: name.clone(),
//             });

//             // If the key is not followed by colon then it means we've hit a shorthand syntax
//             // { a } = b
//             if self.lexer.token != Token::Colon {
//                 let value = Pattern::Identifier(Identifier { name: name.clone() });

//                 // { a = b } = c
//                 let mut default_value: Option<Expression> = None;
//                 if self.lexer.token == Token::Equals {
//                     self.lexer.next_token();
//                     default_value = Some(self.parse_expression(Precedence::Comma)?);
//                 }

//                 return Ok(ObjectPatternProperty {
//                     is_computed: false,
//                     is_rest: false,
//                     key: Some(key),
//                     value,
//                     default_value,
//                 });
//             }
//         }
//     }

//     self.lexer.expect_token(Token::Colon);
//     self.lexer.next_token();
//     let value = self.parse_pattern()?;

//     Ok(ObjectPatternProperty {
//         default_value: None,
//         is_computed,
//         is_rest: false,
//         key: Some(key),
//         value,
//     })
// }

// pub(crate) fn parse_object_pattern(&mut self) -> ParseResult<ObjectPattern> {
//     self.lexer.next_token();
//     let mut properties: Vec<ObjectPatternProperty> = Vec::new();
//     while self.lexer.token != Token::CloseBrace {
//         properties.push(self.parse_object_pattern_property()?);
//     }
//     self.lexer.expect_token(Token::CloseBrace);
//     self.lexer.next_token();
//     Ok(ObjectPattern { properties })
// }

// pub fn parse_array_pattern_item(&mut self) -> ParseResult<ArrayPatternItem> {
//     match self.lexer.token {
//         Token::DotDotDot => {
//             self.lexer.next_token();
//             let value = self.parse_pattern()?;
//             if self.lexer.token == Token::Comma {
//                 panic!("Comma is not allowed after rest element");
//             }
//             return Ok(ArrayPatternItem {
//                 default_value: None,
//                 is_rest: true,
//                 value: value,
//             });
//         }

//         _ => {
//             let value = self.parse_pattern()?;
//             // [a = b]
//             let mut default_value: Option<Expression> = None;
//             if self.lexer.token == Token::Equals {
//                 self.lexer.next_token();
//                 default_value = Some(self.parse_expression(Precedence::Comma)?);
//             }

//             if self.lexer.token == Token::Comma {
//                 self.lexer.next_token();
//             }

//             return Ok(ArrayPatternItem {
//                 default_value,
//                 is_rest: false,
//                 value,
//             });
//         }
//     }
// }

// pub(crate) fn parse_array_pattern(&mut self) -> ParseResult<ArrayPattern> {
//     self.lexer.next_token();
//     let mut properties: Vec<Option<ArrayPatternItem>> = Vec::new();
//     while self.lexer.token != Token::CloseBracket {
//         if self.lexer.token != Token::Comma {
//             properties.push(Some(self.parse_array_pattern_item()?));
//         } else {
//             self.lexer.next_token();
//             properties.push(None);
//         }

//         if self.lexer.token == Token::Comma {
//             self.lexer.next_token();
//         }
//     }
//     self.lexer.expect_token(Token::CloseBracket);
//     self.lexer.next_token();
//     Ok(ArrayPattern { properties })
// }
