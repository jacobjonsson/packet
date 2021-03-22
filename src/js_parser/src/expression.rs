use crate::ParserError;
use js_ast::expression::*;
use js_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(
        &mut self,
        precedence: OperatorPrecedence,
    ) -> ParseResult<Expression> {
        let left = self.parse_prefix()?;

        self.parse_suffix(precedence, left)
    }

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        match &self.lexer.token {
            Token::Null => self.parse_null_literal().map(Expression::NullLiteral),
            Token::NumericLiteral => self.parse_numeric_literal(),
            Token::Identifier => self.parse_identifer().map(Expression::Identifier),
            Token::StringLiteral => self.parse_string_literal().map(Expression::StringLiteral),
            Token::Exclamation => self.parse_unary_expression(UnaryOperator::Exclamation),
            Token::Tilde => self.parse_unary_expression(UnaryOperator::Tilde),
            Token::Plus => self.parse_unary_expression(UnaryOperator::Plus),
            Token::Minus => self.parse_unary_expression(UnaryOperator::Minus),
            Token::Typeof => self.parse_unary_expression(UnaryOperator::Typeof),
            Token::Delete => self.parse_unary_expression(UnaryOperator::Delete),
            Token::Void => self.parse_unary_expression(UnaryOperator::Void),
            Token::True => self.parse_boolean().map(Expression::BooleanExpression),
            Token::False => self.parse_boolean().map(Expression::BooleanExpression),
            Token::OpenParen => self.parse_group_expression(),
            Token::New => self.parse_new_expression().map(Expression::NewExpression),
            Token::OpenBrace => self
                .parse_object_expression()
                .map(Expression::ObjectExpression),
            Token::Function => self
                .parse_function_expression()
                .map(Expression::FunctionExpression),
            Token::This => self.parse_this_expression().map(Expression::ThisExpression),
            Token::OpenBracket => self
                .parse_array_expression()
                .map(Expression::ArrayExpression),
            Token::PlusPlus | Token::MinusMinus => self
                .parse_update_expression(true)
                .map(Expression::UpdateExpression),
            _ => {
                self.lexer.unexpected();
            }
        }
    }

    fn parse_null_literal(&mut self) -> ParseResult<NullLiteral> {
        self.lexer.next_token();
        Ok(NullLiteral {})
    }

    fn parse_this_expression(&mut self) -> ParseResult<ThisExpression> {
        self.lexer.next_token();
        Ok(ThisExpression {})
    }

    fn parse_group_expression(&mut self) -> ParseResult<Expression> {
        self.lexer.next_token();
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();
        Ok(expression)
    }

    fn parse_update_expression(&mut self, prefix: bool) -> ParseResult<UpdateExpression> {
        let operator = match self.lexer.token {
            Token::PlusPlus => UpdateOperator::Increment,
            Token::MinusMinus => UpdateOperator::Decrement,
            _ => self.lexer.unexpected(),
        };
        self.lexer.next_token();
        let argument = self.parse_expression(OperatorPrecedence::Prefix)?;
        Ok(UpdateExpression {
            argument: Box::new(argument),
            operator,
            prefix,
        })
    }

    fn parse_new_expression(&mut self) -> ParseResult<NewExpression> {
        self.lexer.next_token();
        let callee = Box::new(self.parse_expression(OperatorPrecedence::Member)?);
        self.lexer.expect_token(Token::OpenParen);
        self.lexer.next_token();
        let mut arguments: Vec<Expression> = Vec::new();
        while self.lexer.token != Token::CloseParen {
            arguments.push(self.parse_expression(OperatorPrecedence::Lowest)?);
            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();
        Ok(NewExpression { arguments, callee })
    }

    fn parse_object_expression(&mut self) -> ParseResult<ObjectExpression> {
        self.lexer.next_token();

        let mut properties: Vec<Property> = Vec::new();

        while self.lexer.token != Token::CloseBrace {
            let (key, computed) = self.parse_property_key()?;
            self.lexer.expect_token(Token::Colon);
            self.lexer.next_token();

            let value = self.parse_expression(OperatorPrecedence::Lowest)?;
            properties.push(Property {
                computed,
                value,
                key,
                kind: PropertyKind::Init,
            });

            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }

        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();

        Ok(ObjectExpression { properties })
    }

    fn parse_array_expression(&mut self) -> ParseResult<ArrayExpression> {
        self.lexer.next_token();
        let mut elements: Vec<Option<Box<Expression>>> = Vec::new();
        while self.lexer.token != Token::CloseBracket {
            match self.lexer.token {
                Token::Comma => elements.push(None),
                Token::DotDotDot => self.lexer.unexpected(),
                _ => elements.push(Some(Box::new(
                    self.parse_expression(OperatorPrecedence::Lowest)?,
                ))),
            };

            if self.lexer.token != Token::Comma {
                break;
            } else {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseBracket);
        self.lexer.next_token();

        Ok(ArrayExpression { elements })
    }

    fn parse_unary_expression(
        &mut self,
        operator: UnaryOperator,
    ) -> Result<Expression, ParserError> {
        self.lexer.next_token();
        let argument = self.parse_expression(OperatorPrecedence::Prefix)?;
        Ok(Expression::UnaryExpression(UnaryExpression {
            prefix: true,
            operator,
            argument: Box::new(argument),
        }))
    }

    pub(crate) fn parse_suffix(
        &mut self,
        level: OperatorPrecedence,
        left: Expression,
    ) -> ParseResult<Expression> {
        let mut expression = left;

        loop {
            match &self.lexer.token {
                // a[b][c]
                Token::OpenBracket => {
                    self.lexer.next_token();
                    let property = self.parse_expression(OperatorPrecedence::Lowest)?;
                    self.lexer.expect_token(Token::CloseBracket);
                    self.lexer.next_token();
                    expression = Expression::MemberExpression(MemberExpression {
                        object: Box::new(expression),
                        computed: true,
                        property: Box::new(property),
                    })
                }
                // a.b.c
                Token::Dot => {
                    self.lexer.next_token();
                    let property = self.parse_expression(OperatorPrecedence::Member)?;
                    expression = Expression::MemberExpression(MemberExpression {
                        object: Box::new(expression),
                        computed: false,
                        property: Box::new(property),
                    });
                }

                // a = 1
                Token::Equals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::Equals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a += 1
                Token::PlusEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::PlusEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a -= 1
                Token::MinusEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::MinusEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a *= 1
                Token::AsteriskEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::AsteriskEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a /= 1
                Token::SlashEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::SlashEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a %= 1
                Token::PercentEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::PercentEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a <<= 1
                Token::LessThanLessThanEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::LessThanLessThanEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a >>= 1
                Token::GreaterThanGreaterThanEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::GreaterThanGreaterThanEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a >>>= 1
                Token::GreaterThanGreaterThanGreaterThanEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::GreaterThanGreaterThanGreaterThanEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a |= 1
                Token::BarEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::BarEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a ^= 1
                Token::CaretEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::CaretEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // a &= 1
                Token::AmpersandEquals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(AssignmentExpressionLeft::Expression(expression)),
                        operator: AssignmentOperator::AmpersandEquals,
                        right: Box::new(
                            self.parse_expression(OperatorPrecedence::Assignment.lower())?,
                        ),
                    })
                }

                // 1 + 2
                Token::Plus => {
                    if level >= OperatorPrecedence::Sum {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::Plus,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Sum)?),
                    });
                }

                // 1 - 2
                Token::Minus => {
                    if level >= OperatorPrecedence::Sum {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::Minus,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Sum)?),
                    });
                }

                // 1 / 2
                Token::Slash => {
                    if level >= OperatorPrecedence::Product {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::Slash,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Product)?),
                    });
                }

                // 1 * 2
                Token::Asterisk => {
                    if level >= OperatorPrecedence::Product {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::Asterisk,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Product)?),
                    });
                }

                // 1 ^ 2
                Token::Caret => {
                    if level >= OperatorPrecedence::Product {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::Caret,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Product)?),
                    });
                }

                // 1 >= 0
                Token::GreaterThanEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::GreaterThanEquals,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Equals)?),
                    });
                }

                // 1 <= 0
                Token::LessThanEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::LessThanEquals,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Equals)?),
                    });
                }

                // 1 == 1
                Token::EqualsEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::EqualsEquals,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Equals)?),
                    });
                }

                // 1 === 1
                Token::EqualsEqualsEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::EqualsEqualsEquals,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Equals)?),
                    });
                }

                // 1 != 2
                Token::ExclamationEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::ExclamationEquals,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Equals)?),
                    });
                }

                // 1 !== 2
                Token::ExclamationEqualsEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::ExclamationEqualsEquals,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Equals)?),
                    });
                }

                // 1 < 2
                Token::LessThan => {
                    if level >= OperatorPrecedence::Compare {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::LessThan,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Compare)?),
                    });
                }

                // 1 > 2
                Token::GreaterThan => {
                    if level >= OperatorPrecedence::Compare {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::GreaterThan,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Compare)?),
                    });
                }

                // a instanceof b
                Token::Instanceof => {
                    if level >= OperatorPrecedence::Compare {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::Instanceof,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Compare)?),
                    })
                }

                // a in b
                Token::In => {
                    if level >= OperatorPrecedence::Compare || !self.allow_in {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        operator: BinaryOperator::In,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Compare)?),
                    })
                }

                Token::OpenParen => {
                    if level >= OperatorPrecedence::Call {
                        return Ok(expression);
                    }

                    let arguments = self.parse_call_expression_arguments()?;

                    expression = Expression::CallExpression(CallExpression {
                        arguments,
                        callee: Box::new(expression),
                    });
                }

                Token::Question => {
                    if level >= OperatorPrecedence::Conditional {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    let consequence = self.parse_expression(OperatorPrecedence::Lowest)?;
                    self.lexer.expect_token(Token::Colon);
                    self.lexer.next_token();
                    let alternate = self.parse_expression(OperatorPrecedence::Lowest)?;
                    expression = Expression::ConditionalExpression(ConditionalExpression {
                        test: Box::new(expression),
                        consequence: Box::new(consequence),
                        alternate: Box::new(alternate),
                    });
                }

                // 1++
                Token::PlusPlus => {
                    if level >= OperatorPrecedence::Postfix {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::UpdateExpression(UpdateExpression {
                        argument: Box::new(expression),
                        operator: UpdateOperator::Increment,
                        prefix: false,
                    });
                }

                // 1--
                Token::MinusMinus => {
                    if level >= OperatorPrecedence::Postfix {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::UpdateExpression(UpdateExpression {
                        argument: Box::new(expression),
                        operator: UpdateOperator::Decrement,
                        prefix: false,
                    });
                }

                // a || b
                Token::BarBar => {
                    if level >= OperatorPrecedence::LogicalOr {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::LogicalExpression(LogicalExpression {
                        left: Box::new(expression),
                        operator: LogicalOperator::BarBar,
                        right: Box::new(self.parse_expression(OperatorPrecedence::LogicalOr)?),
                    })
                }

                // a && b
                Token::AmpersandAmpersand => {
                    if level >= OperatorPrecedence::LogicalAnd {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::LogicalExpression(LogicalExpression {
                        left: Box::new(expression),
                        operator: LogicalOperator::AmpersandAmpersand,
                        right: Box::new(self.parse_expression(OperatorPrecedence::LogicalAnd)?),
                    })
                }

                _ => {
                    return Ok(expression);
                }
            };
        }
    }

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

    pub(crate) fn parse_property_key(&mut self) -> ParseResult<(PropertyKey, bool)> {
        let (property_key, computed) = match self.lexer.token {
            Token::OpenBracket => {
                self.lexer.next_token();
                let pk = PropertyKey::Identifier(self.parse_identifer()?);
                self.lexer.expect_token(Token::CloseBracket);
                self.lexer.next_token();
                (pk, true)
            }
            Token::Identifier => (PropertyKey::Identifier(self.parse_identifer()?), false),
            Token::StringLiteral => (
                PropertyKey::StringLiteral(self.parse_string_literal()?),
                false,
            ),
            _ => self.lexer.unexpected(),
        };

        Ok((property_key, computed))
    }

    pub(crate) fn parse_pattern(&mut self) -> ParseResult<Pattern> {
        match &self.lexer.token {
            Token::Identifier => Ok(Pattern::Identifier(self.parse_identifer()?)),
            Token::OpenBrace => Ok(Pattern::ObjectPattern(self.parse_object_pattern()?)),
            Token::OpenBracket => Ok(Pattern::ArrayPattern(self.parse_array_pattern()?)),
            Token::DotDotDot => Ok(Pattern::RestElement(self.parse_rest_element()?)),
            Token::Equals => Ok(Pattern::AssignmentPattern(self.parse_assignment_pattern()?)),
            _ => todo!(),
        }
    }

    pub(crate) fn parse_object_pattern(&mut self) -> ParseResult<ObjectPattern> {
        self.lexer.next_token();
        let mut properties: Vec<ObjectPatternProperty> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            if self.lexer.token == Token::DotDotDot {
                properties.push(ObjectPatternProperty::RestElement(
                    self.parse_rest_element()?,
                ))
            } else {
                let (key, _) = self.parse_property_key()?;
                let value: Pattern;
                if self.lexer.token == Token::Equals {
                    value = self.parse_pattern()?;
                } else {
                    self.lexer.expect_token(Token::Colon);
                    self.lexer.next_token();
                    value = self.parse_pattern()?;
                }
                properties.push(ObjectPatternProperty::AssignmentProperty(
                    AssignmentProperty {
                        key,
                        value: Box::new(value),
                    },
                ));
            }
        }
        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(ObjectPattern { properties })
    }

    pub(crate) fn parse_array_pattern(&mut self) -> ParseResult<ArrayPattern> {
        self.lexer.next_token();
        let mut properties: Vec<Option<Pattern>> = Vec::new();
        while self.lexer.token != Token::CloseBracket {
            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
                properties.push(None);
            } else if self.lexer.token == Token::DotDotDot {
                properties.push(Some(Pattern::RestElement(self.parse_rest_element()?)));
            } else {
                properties.push(Some(self.parse_pattern()?));
            }
            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseBracket);
        self.lexer.next_token();
        Ok(ArrayPattern { properties })
    }

    pub(crate) fn parse_rest_element(&mut self) -> ParseResult<RestElement> {
        self.lexer.next_token();
        Ok(RestElement {
            argument: Box::new(self.parse_pattern()?),
        })
    }

    pub(crate) fn parse_assignment_pattern(&mut self) -> ParseResult<AssignmentPattern> {
        self.lexer.next_token();
        Ok(AssignmentPattern {
            right: Box::new(self.parse_expression(OperatorPrecedence::Assignment)?),
        })
    }

    /// Parse function expression
    /// let a = function() {}
    /// a(function() {})
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

    /// Parses function parameters
    pub(crate) fn parse_function_parameters(&mut self) -> ParseResult<Vec<Pattern>> {
        let mut parameters: Vec<Pattern> = Vec::new();

        // Means there aren't any parameters to parse
        self.lexer.next_token();
        if self.lexer.token == Token::CloseParen {
            self.lexer.next_token(); // Skip the closing paren
            return Ok(Vec::new());
        }

        while self.lexer.token != Token::CloseParen {
            // Parse the first parameter
            parameters.push(self.parse_pattern()?);
            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }
        }
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();

        Ok(parameters)
    }

    pub(crate) fn parse_identifer(&mut self) -> ParseResult<Identifier> {
        self.lexer.expect_token(Token::Identifier);
        let identifier = Identifier {
            name: self.lexer.token_value.clone(),
        };
        self.lexer.next_token();
        Ok(identifier)
    }

    pub(crate) fn parse_boolean(&mut self) -> ParseResult<BooleanExpression> {
        let boolean_expression = match &self.lexer.token {
            Token::True => BooleanExpression { value: true },
            Token::False => BooleanExpression { value: false },
            _ => self.lexer.unexpected(),
        };
        self.lexer.next_token();
        Ok(boolean_expression)
    }

    pub(crate) fn parse_numeric_literal(&mut self) -> ParseResult<Expression> {
        self.lexer.expect_token(Token::NumericLiteral);
        let value = self.lexer.token_value.parse::<i64>().map_err(|_| {
            ParserError(format!(
                "Failed to parse {} as number",
                self.lexer.token_value
            ))
        })?;

        self.lexer.next_token();
        Ok(Expression::IntegerLiteral(IntegerLiteral { value }))
    }

    pub(crate) fn parse_string_literal(&mut self) -> ParseResult<StringLiteral> {
        let string_literal = StringLiteral {
            value: self.lexer.token_value.clone(),
        };
        self.lexer.next_token();
        Ok(string_literal)
    }
}
