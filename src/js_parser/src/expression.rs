use js_ast::expression::*;
use js_token::Token;

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Expression> {
        let left = self.parse_prefix()?;

        self.parse_suffix(precedence, left)
    }

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        match &self.lexer.token {
            Token::Null => {
                self.lexer.next_token();
                Ok(Expression::NullLiteral(NullLiteral {}))
            }

            Token::NumericLiteral => {
                let value = self.lexer.number;
                self.lexer.next_token();
                Ok(Expression::IntegerLiteral(IntegerLiteral { value }))
            }

            Token::Slash | Token::SlashEquals => {
                self.lexer.scan_regexp();
                let value = self.lexer.raw();
                self.lexer.next_token();
                Ok(Expression::RegexpLiteral(RegexpLiteral { value }))
            }

            Token::Identifier => self.parse_identifer().map(Expression::Identifier),
            Token::StringLiteral => self.parse_string_literal().map(Expression::StringLiteral),

            // !a
            Token::Exclamation => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryLogicalNot,
                    expression: Box::new(right),
                }))
            }
            // ~a
            Token::Tilde => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryBinaryNot,
                    expression: Box::new(right),
                }))
            }
            // +a
            Token::Plus => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryPositive,
                    expression: Box::new(right),
                }))
            }
            // ++a
            Token::PlusPlus => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryPrefixIncrement,
                    expression: Box::new(right),
                }))
            }
            // -a
            Token::Minus => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryNegative,
                    expression: Box::new(right),
                }))
            }
            // --a
            Token::MinusMinus => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryPrefixDecrement,
                    expression: Box::new(right),
                }))
            }
            // typeof a
            Token::Typeof => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryTypeof,
                    expression: Box::new(right),
                }))
            }
            // delete a
            Token::Delete => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryDelete,
                    expression: Box::new(right),
                }))
            }
            // void a
            Token::Void => {
                self.lexer.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expression::UnaryExpression(UnaryExpression {
                    op_code: OpCode::UnaryVoid,
                    expression: Box::new(right),
                }))
            }

            Token::True => {
                self.lexer.next_token();
                Ok(Expression::BooleanExpression(BooleanExpression {
                    value: true,
                }))
            }
            Token::False => {
                self.lexer.next_token();
                Ok(Expression::BooleanExpression(BooleanExpression {
                    value: false,
                }))
            }
            Token::OpenParen => {
                self.lexer.next_token();
                let expression = self.parse_expression(Precedence::Lowest)?;
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                Ok(expression)
            }
            Token::OpenBrace => self
                .parse_object_expression()
                .map(Expression::ObjectExpression),

            Token::New => {
                self.lexer.next_token();
                let callee = Box::new(self.parse_expression(Precedence::Member)?);
                self.lexer.expect_token(Token::OpenParen);
                self.lexer.next_token();
                let mut arguments: Vec<Expression> = Vec::new();
                while self.lexer.token != Token::CloseParen {
                    arguments.push(self.parse_expression(Precedence::Comma)?);
                    if self.lexer.token == Token::Comma {
                        self.lexer.next_token();
                    }
                }
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                Ok(Expression::NewExpression(NewExpression {
                    arguments,
                    callee,
                }))
            }
            Token::Function => self
                .parse_function_expression()
                .map(Expression::FunctionExpression),
            Token::This => {
                self.lexer.next_token();
                Ok(Expression::ThisExpression(ThisExpression {}))
            }
            Token::OpenBracket => self
                .parse_array_expression()
                .map(Expression::ArrayExpression),

            _ => self.lexer.unexpected(),
        }
    }

    fn parse_object_expression(&mut self) -> ParseResult<ObjectExpression> {
        self.lexer.next_token();

        let mut properties: Vec<Property> = Vec::new();

        while self.lexer.token != Token::CloseBrace {
            let property = self.parse_property()?;
            properties.push(property);

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
                _ => elements.push(Some(Box::new(self.parse_expression(Precedence::Comma)?))),
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

    pub(crate) fn parse_suffix(
        &mut self,
        precedence: Precedence,
        left: Expression,
    ) -> ParseResult<Expression> {
        let mut expression = left;

        loop {
            match &self.lexer.token {
                // a[b][c]
                Token::OpenBracket => {
                    self.lexer.next_token();
                    let property = self.parse_expression(Precedence::Lowest)?;
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
                    let property = self.parse_expression(Precedence::Member)?;
                    expression = Expression::MemberExpression(MemberExpression {
                        object: Box::new(expression),
                        computed: false,
                        property: Box::new(property),
                    });
                }

                // a = 1
                Token::Equals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a += 1
                Token::PlusEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryAdditionAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a -= 1
                Token::MinusEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinarySubstitutionAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a *= 1
                Token::AsteriskEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryMultiplyAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a /= 1
                Token::SlashEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryDivisionAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a %= 1
                Token::PercentEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryReminderAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a <<= 1
                Token::LessThanLessThanEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLeftShiftAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a >>= 1
                Token::GreaterThanGreaterThanEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryRightShiftAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a >>>= 1
                Token::GreaterThanGreaterThanGreaterThanEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryUnsignedRightShiftAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a |= 1
                Token::BarEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryBitwiseOrAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a ^= 1
                Token::CaretEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryBitwiseXorAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // a &= 1
                Token::AmpersandEquals => {
                    if precedence >= Precedence::Assign {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryBitwiseAndAssign,
                        right: Box::new(self.parse_expression(Precedence::Assign.lower())?),
                    })
                }

                // 1 + 2
                Token::Plus => {
                    if precedence >= Precedence::Add {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryAddition,
                        right: Box::new(self.parse_expression(Precedence::Add)?),
                    });
                }

                // 1 - 2
                Token::Minus => {
                    if precedence >= Precedence::Add {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinarySubstitution,
                        right: Box::new(self.parse_expression(Precedence::Add)?),
                    });
                }

                // 1 % 2
                Token::Percent => {
                    if precedence >= Precedence::Multiply {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryReminder,
                        right: Box::new(self.parse_expression(Precedence::Add)?),
                    });
                }

                // 1 / 2
                Token::Slash => {
                    if precedence >= Precedence::Multiply {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryDivision,
                        right: Box::new(self.parse_expression(Precedence::Multiply)?),
                    });
                }

                // 1 * 2
                Token::Asterisk => {
                    if precedence >= Precedence::Multiply {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryMultiply,
                        right: Box::new(self.parse_expression(Precedence::Multiply)?),
                    });
                }

                // 1 | 2
                Token::Bar => {
                    if precedence >= Precedence::BitwiseOr {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryBitwiseOr,
                        right: Box::new(self.parse_expression(Precedence::BitwiseOr)?),
                    });
                }

                // 1 & 2
                Token::Ampersand => {
                    if precedence >= Precedence::BitwiseAnd {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryBitwiseAnd,
                        right: Box::new(self.parse_expression(Precedence::BitwiseAnd)?),
                    });
                }

                // 1 ^ 2
                Token::Caret => {
                    if precedence >= Precedence::BitwiseXor {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryBitwiseXor,
                        right: Box::new(self.parse_expression(Precedence::BitwiseXor)?),
                    });
                }

                // 1 << 2
                Token::LessThanLessThan => {
                    if precedence >= Precedence::Shift {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLeftShift,
                        right: Box::new(self.parse_expression(Precedence::Shift)?),
                    });
                }

                // 1 >> 2
                Token::GreaterThanGreaterThan => {
                    if precedence >= Precedence::Shift {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryRightShift,
                        right: Box::new(self.parse_expression(Precedence::Shift)?),
                    });
                }

                // 1 >>> 2
                Token::GreaterThanGreaterThanGreaterThan => {
                    if precedence >= Precedence::Shift {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryUnsignedRightShift,
                        right: Box::new(self.parse_expression(Precedence::Shift)?),
                    });
                }

                // 1 >= 0
                Token::GreaterThanEquals => {
                    if precedence >= Precedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryGreaterThanEquals,
                        right: Box::new(self.parse_expression(Precedence::Compare)?),
                    });
                }

                // 1 <= 0
                Token::LessThanEquals => {
                    if precedence >= Precedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLessThanEquals,
                        right: Box::new(self.parse_expression(Precedence::Compare)?),
                    });
                }

                // 1 == 1
                Token::EqualsEquals => {
                    if precedence >= Precedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLooseEquals,
                        right: Box::new(self.parse_expression(Precedence::Equals)?),
                    });
                }

                // 1 === 1
                Token::EqualsEqualsEquals => {
                    if precedence >= Precedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryStrictEquals,
                        right: Box::new(self.parse_expression(Precedence::Equals)?),
                    });
                }

                // 1 != 2
                Token::ExclamationEquals => {
                    if precedence >= Precedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLooseNotEquals,
                        right: Box::new(self.parse_expression(Precedence::Equals)?),
                    });
                }

                // 1 !== 2
                Token::ExclamationEqualsEquals => {
                    if precedence >= Precedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryStrictNotEquals,
                        right: Box::new(self.parse_expression(Precedence::Equals)?),
                    });
                }

                // 1 < 2
                Token::LessThan => {
                    if precedence >= Precedence::Compare {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLessThan,
                        right: Box::new(self.parse_expression(Precedence::Compare)?),
                    });
                }

                // 1 > 2
                Token::GreaterThan => {
                    if precedence >= Precedence::Compare {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryGreaterThan,
                        right: Box::new(self.parse_expression(Precedence::Compare)?),
                    });
                }

                // a instanceof b
                Token::Instanceof => {
                    if precedence >= Precedence::Compare {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryInstanceof,
                        right: Box::new(self.parse_expression(Precedence::Compare)?),
                    })
                }

                // a in b
                Token::In => {
                    if precedence >= Precedence::Compare || !self.allow_in {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryIn,
                        right: Box::new(self.parse_expression(Precedence::Compare)?),
                    })
                }

                // a, b, c
                Token::Comma => {
                    if precedence >= Precedence::Comma {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryComma,
                        right: Box::new(self.parse_expression(Precedence::Comma)?),
                    })
                }

                Token::OpenParen => {
                    if precedence >= Precedence::Call {
                        return Ok(expression);
                    }

                    let arguments = self.parse_call_expression_arguments()?;

                    expression = Expression::CallExpression(CallExpression {
                        arguments,
                        callee: Box::new(expression),
                    });
                }

                Token::Question => {
                    if precedence >= Precedence::Conditional {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    let consequence = self.parse_expression(Precedence::Comma)?;
                    self.lexer.expect_token(Token::Colon);
                    self.lexer.next_token();
                    let alternate = self.parse_expression(Precedence::Comma)?;
                    expression = Expression::ConditionalExpression(ConditionalExpression {
                        test: Box::new(expression),
                        consequence: Box::new(consequence),
                        alternate: Box::new(alternate),
                    });
                }

                // 1++
                Token::PlusPlus => {
                    if precedence >= Precedence::Postfix {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::UnaryExpression(UnaryExpression {
                        op_code: OpCode::UnaryPostfixIncrement,
                        expression: Box::new(expression),
                    })
                }

                // 1--
                Token::MinusMinus => {
                    if precedence >= Precedence::Postfix {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::UnaryExpression(UnaryExpression {
                        op_code: OpCode::UnaryPostfixDecrement,
                        expression: Box::new(expression),
                    })
                }

                // a || b
                Token::BarBar => {
                    if precedence >= Precedence::LogicalOr {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLogicalOr,
                        right: Box::new(self.parse_expression(Precedence::LogicalOr)?),
                    })
                }

                // a && b
                Token::AmpersandAmpersand => {
                    if precedence >= Precedence::LogicalAnd {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression),
                        op_code: OpCode::BinaryLogicalAnd,
                        right: Box::new(self.parse_expression(Precedence::LogicalOr)?),
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
        arguments.push(Box::new(self.parse_expression(Precedence::Comma)?));
        while self.lexer.token == Token::Comma {
            self.lexer.next_token();
            arguments.push(Box::new(self.parse_expression(Precedence::Comma)?));
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

    pub(crate) fn parse_property(&mut self) -> ParseResult<Property> {
        let key: Expression;
        let mut computed = false;

        match self.lexer.token {
            Token::OpenBracket => {
                computed = true;
                self.lexer.next_token();
                key = self.parse_expression(Precedence::Comma)?;
                self.lexer.expect_token(Token::CloseBracket);
                self.lexer.next_token();
            }

            Token::NumericLiteral => {
                key = Expression::IntegerLiteral(IntegerLiteral {
                    value: self.lexer.number,
                });
            }

            Token::StringLiteral => {
                key = Expression::StringLiteral(self.parse_string_literal()?);
            }

            _ => {
                let name = self.lexer.token_value.clone();
                if !self.lexer.is_identifier_or_keyword() {
                    self.lexer.expect_token(Token::Identifier);
                }
                self.lexer.next_token();
                key = Expression::Identifier(Identifier { name });
                // TODO: Support shorthand syntax here.
            }
        };
        self.lexer.expect_token(Token::Colon);
        self.lexer.next_token();
        let value = self.parse_expression(Precedence::Comma)?;
        Ok(Property {
            computed,
            key,
            kind: PropertyKind::Init,
            value,
        })
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
            right: Box::new(self.parse_expression(Precedence::Assign)?),
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

    pub(crate) fn parse_string_literal(&mut self) -> ParseResult<StringLiteral> {
        let string_literal = StringLiteral {
            value: self.lexer.token_value.clone(),
        };
        self.lexer.next_token();
        Ok(string_literal)
    }
}
