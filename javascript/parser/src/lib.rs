mod expression;
mod import;
mod statement;

use javascript_ast::{expression::*, statement::*, Program};
use javascript_lexer::Lexer;
use javascript_token::{Token, TokenLiteral};

/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table
/// https://github.com/evanw/esbuild/blob/51b785f89933426afe675b4e633cf531d5a9890d/internal/js_ast/js_ast.go#L29
#[derive(Debug, PartialEq, PartialOrd)]
enum OperatorPrecedence {
    Lowest = 0,
    Assignment,
    Conditional,
    Equals,
    Compare,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Postfix,
    Call,
}

// Keep these in sync with the above order
impl OperatorPrecedence {
    pub fn lower(&self) -> OperatorPrecedence {
        match &self {
            OperatorPrecedence::Lowest => OperatorPrecedence::Lowest,
            OperatorPrecedence::Assignment => OperatorPrecedence::Lowest,
            OperatorPrecedence::Conditional => OperatorPrecedence::Assignment,
            OperatorPrecedence::Equals => OperatorPrecedence::Conditional,
            OperatorPrecedence::Compare => OperatorPrecedence::Equals,
            OperatorPrecedence::LessGreater => OperatorPrecedence::LessGreater,
            OperatorPrecedence::Sum => OperatorPrecedence::Sum,
            OperatorPrecedence::Product => OperatorPrecedence::Sum,
            OperatorPrecedence::Prefix => OperatorPrecedence::Product,
            OperatorPrecedence::Postfix => OperatorPrecedence::Prefix,
            OperatorPrecedence::Call => OperatorPrecedence::Postfix,
        }
    }

    #[allow(dead_code)]
    pub fn raise(&self) -> OperatorPrecedence {
        match &self {
            OperatorPrecedence::Lowest => OperatorPrecedence::Assignment,
            OperatorPrecedence::Assignment => OperatorPrecedence::Conditional,
            OperatorPrecedence::Conditional => OperatorPrecedence::Equals,
            OperatorPrecedence::Equals => OperatorPrecedence::Compare,
            OperatorPrecedence::Compare => OperatorPrecedence::LessGreater,
            OperatorPrecedence::LessGreater => OperatorPrecedence::Sum,
            OperatorPrecedence::Sum => OperatorPrecedence::Product,
            OperatorPrecedence::Product => OperatorPrecedence::Prefix,
            OperatorPrecedence::Prefix => OperatorPrecedence::Postfix,
            OperatorPrecedence::Postfix => OperatorPrecedence::Call,
            OperatorPrecedence::Call => OperatorPrecedence::Call,
        }
    }
}

pub struct ParserError(String);
pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    lexer: Lexer,
}

/// Public
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer: lexer }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::<Statement>::new();

        while &self.lexer.token != &Token::EndOfFile {
            match self.parse_statement() {
                Ok(s) => statements.push(s),
                Err(err) => panic!(err.0),
            }
        }

        Program { statements }
    }
}

/// Private
impl Parser {
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match &self.lexer.token {
            Token::Const => self
                .parse_var_statement(VariableDeclarationKind::Const)
                .map(Statement::VariableDeclaration),
            Token::Var => self
                .parse_var_statement(VariableDeclarationKind::Var)
                .map(Statement::VariableDeclaration),
            Token::Let => self
                .parse_var_statement(VariableDeclarationKind::Let)
                .map(Statement::VariableDeclaration),
            Token::Import => self.parse_import_statement(),
            Token::Function => self
                .parse_function_declaration()
                .map(Statement::FunctionDeclaration),
            Token::Return => self.parse_return_statement().map(Statement::Return),
            Token::If => self.parse_if_statement().map(Statement::If),
            Token::OpenBrace => self.parse_block_statement().map(Statement::Block),
            Token::For => self.parse_for_statement().map(Statement::For),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_string_literal(&mut self) -> ParseResult<StringLiteral> {
        let string_literal = StringLiteral {
            value: self.lexer.token.token_literal(),
        };
        self.lexer.next_token();
        Ok(string_literal)
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.consume_semicolon();

        Ok(Statement::Expression(ExpressionStatement { expression }))
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> ParseResult<Expression> {
        let left = self.parse_prefix()?;

        self.parse_suffix(precedence, left)
    }

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        match &self.lexer.token {
            Token::NumericLiteral(_) => self.parse_numeric_literal(),
            Token::Identifier(_) => self.parse_identifer().map(Expression::Identifier),
            Token::Exclamation => self.parse_prefix_expression(),
            Token::Plus => self.parse_prefix_expression(),
            Token::Minus => self.parse_prefix_expression(),
            Token::True => self.parse_boolean().map(Expression::BooleanExpression),
            Token::False => self.parse_boolean().map(Expression::BooleanExpression),
            Token::OpenParen => self.parse_grouped_expression(),
            Token::Function => self
                .parse_function_expression()
                .map(Expression::FunctionExpression),
            Token::PlusPlus => {
                self.lexer.next_token();
                self.parse_expression(OperatorPrecedence::Prefix)
                    .map(|e| UpdateExpression {
                        operator: UpdateOperator::Increment,
                        argument: Box::new(e),
                        prefix: true,
                    })
                    .map(Expression::UpdateExpression)
                    .map(Ok)?
            }
            Token::MinusMinus => {
                self.lexer.next_token();
                self.parse_expression(OperatorPrecedence::Prefix)
                    .map(|e| UpdateExpression {
                        operator: UpdateOperator::Decrement,
                        argument: Box::new(e),
                        prefix: true,
                    })
                    .map(Expression::UpdateExpression)
                    .map(Ok)?
            }
            _ => {
                self.lexer.unexpected();
                return Err(ParserError("".into()));
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.lexer.token.token_literal();
        self.lexer.next_token();
        let right = self.parse_expression(OperatorPrecedence::Prefix)?;
        self.lexer.next_token();
        Ok(Expression::PrefixExpression(PrefixExpression {
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_suffix(
        &mut self,
        level: OperatorPrecedence,
        left: Expression,
    ) -> ParseResult<Expression> {
        let mut expression = left;

        loop {
            match &self.lexer.token {
                // a = 1
                Token::Equals => {
                    if level >= OperatorPrecedence::Assignment {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::AssignmentExpression(AssignmentExpression {
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
                        operator: BinaryOperator::Asterisk,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Product)?),
                    });
                }

                // 1 == 1
                Token::EqualsEquals => {
                    if level >= OperatorPrecedence::Equals {
                        return Ok(expression);
                    }
                    self.lexer.next_token();
                    expression = Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
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
                        left: Box::new(expression.clone()),
                        operator: BinaryOperator::GreaterThan,
                        right: Box::new(self.parse_expression(OperatorPrecedence::Compare)?),
                    });
                }

                Token::OpenParen => {
                    if level >= OperatorPrecedence::Call {
                        return Ok(expression);
                    }

                    let function = match expression {
                        Expression::Identifier(i) => i,
                        _ => {
                            self.lexer.unexpected();
                            panic!(); // We panic above, this is just to satisfy the compiler.
                        }
                    };
                    let arguments = self.parse_call_expression_arguments()?;

                    expression = Expression::CallExpression(CallExpression {
                        arguments,
                        function,
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

                _ => {
                    //
                    return Ok(expression);
                }
            };
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.lexer.next_token();
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();
        Ok(expression)
    }

    fn parse_identifer(&mut self) -> ParseResult<Identifier> {
        let identifier = match &self.lexer.token {
            Token::Identifier(i) => Identifier { name: i.clone() },
            t => return Err(ParserError(format!("Expected identifier but got {}", t))),
        };
        self.lexer.next_token();
        Ok(identifier)
    }

    fn parse_boolean(&mut self) -> ParseResult<BooleanExpression> {
        let boolean_expression = match &self.lexer.token {
            Token::True => BooleanExpression { value: true },
            Token::False => BooleanExpression { value: false },
            c => {
                return Err(ParserError(format!(
                    "Expected to get true or false but got {:?}",
                    c
                )));
            }
        };
        self.lexer.next_token();
        Ok(boolean_expression)
    }

    fn parse_numeric_literal(&mut self) -> ParseResult<Expression> {
        let value = self
            .lexer
            .token
            .token_literal()
            .parse::<i64>()
            .map_err(|_| {
                ParserError(format!(
                    "Failed to parse {} as number",
                    self.lexer.token.token_literal()
                ))
            })?;

        self.lexer.next_token();
        Ok(Expression::IntegerLiteral(IntegerLiteral { value }))
    }

    fn parse_var_statement(
        &mut self,
        kind: VariableDeclarationKind,
    ) -> ParseResult<VariableDeclaration> {
        self.lexer.next_token();
        let id = self.parse_identifer()?;
        // Means we hit a variable declaration without an assignment (eg: let a;)
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
            return Ok(VariableDeclaration {
                declarations: vec![VariableDeclarator { id, init: None }],
                kind,
            });
        }

        self.lexer.expect_token(Token::Equals);
        self.lexer.next_token();

        let init = Some(self.parse_expression(OperatorPrecedence::Lowest)?);
        // We can't expect a semicolon here since they are optional in JS.
        // But we should insert semicolons instead of just skipping when they are missing,
        // it will make printing easier.
        self.consume_semicolon();

        Ok(VariableDeclaration {
            declarations: vec![VariableDeclarator { id, init }],
            kind: kind,
        })
    }

    /// Consumes the next semicolon
    fn consume_semicolon(&mut self) {
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
        }
    }
}
