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
    LogicalOr,
    LogicalAnd,
    Equals,
    Compare,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Postfix,
    Call,
    Member,
}

// Keep these in sync with the above order
impl OperatorPrecedence {
    pub fn lower(&self) -> OperatorPrecedence {
        match &self {
            OperatorPrecedence::Lowest => OperatorPrecedence::Lowest,
            OperatorPrecedence::Assignment => OperatorPrecedence::Lowest,
            OperatorPrecedence::Conditional => OperatorPrecedence::Assignment,
            OperatorPrecedence::LogicalOr => OperatorPrecedence::Conditional,
            OperatorPrecedence::LogicalAnd => OperatorPrecedence::LogicalOr,
            OperatorPrecedence::Equals => OperatorPrecedence::LogicalAnd,
            OperatorPrecedence::Compare => OperatorPrecedence::Equals,
            OperatorPrecedence::LessGreater => OperatorPrecedence::LessGreater,
            OperatorPrecedence::Sum => OperatorPrecedence::Sum,
            OperatorPrecedence::Product => OperatorPrecedence::Sum,
            OperatorPrecedence::Prefix => OperatorPrecedence::Product,
            OperatorPrecedence::Postfix => OperatorPrecedence::Prefix,
            OperatorPrecedence::Call => OperatorPrecedence::Postfix,
            OperatorPrecedence::Member => OperatorPrecedence::Call,
        }
    }

    #[allow(dead_code)]
    pub fn raise(&self) -> OperatorPrecedence {
        match &self {
            OperatorPrecedence::Lowest => OperatorPrecedence::Assignment,
            OperatorPrecedence::Assignment => OperatorPrecedence::Conditional,
            OperatorPrecedence::Conditional => OperatorPrecedence::LogicalOr,
            OperatorPrecedence::LogicalOr => OperatorPrecedence::LogicalAnd,
            OperatorPrecedence::LogicalAnd => OperatorPrecedence::Equals,
            OperatorPrecedence::Equals => OperatorPrecedence::Compare,
            OperatorPrecedence::Compare => OperatorPrecedence::LessGreater,
            OperatorPrecedence::LessGreater => OperatorPrecedence::Sum,
            OperatorPrecedence::Sum => OperatorPrecedence::Product,
            OperatorPrecedence::Product => OperatorPrecedence::Prefix,
            OperatorPrecedence::Prefix => OperatorPrecedence::Postfix,
            OperatorPrecedence::Postfix => OperatorPrecedence::Call,
            OperatorPrecedence::Call => OperatorPrecedence::Member,
            OperatorPrecedence::Member => OperatorPrecedence::Member,
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
            Token::For => self.parse_for_statement(),
            Token::Continue => {
                self.lexer.next_token();
                let mut label: Option<Identifier> = None;
                if self.lexer.token != Token::Semicolon {
                    label = Some(self.parse_identifer()?);
                }
                self.consume_semicolon();
                Ok(Statement::ContinueStatement(ContinueStatement { label }))
            }
            Token::Break => {
                self.lexer.next_token();
                let mut label: Option<Identifier> = None;
                if self.lexer.token != Token::Semicolon {
                    label = Some(self.parse_identifer()?);
                }
                self.consume_semicolon();
                Ok(Statement::BreakStatement(BreakStatement { label }))
            }
            Token::Semicolon => {
                self.lexer.next_token();
                Ok(Statement::EmptyStatement(EmptyStatement {}))
            }
            Token::While => {
                self.lexer.next_token();
                self.lexer.expect_token(Token::OpenParen);
                self.lexer.next_token();
                let test = self.parse_expression(OperatorPrecedence::Lowest)?;
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                let body = self.parse_statement()?;
                Ok(Statement::WhileStatement(WhileStatement {
                    body: Box::new(body),
                    test,
                }))
            }
            Token::Do => {
                self.lexer.next_token();
                let body = self.parse_statement()?;
                self.lexer.expect_token(Token::While);
                self.lexer.next_token();
                self.lexer.expect_token(Token::OpenParen);
                self.lexer.next_token();
                let test = self.parse_expression(OperatorPrecedence::Lowest)?;
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                Ok(Statement::DoWhileStatement(DoWhileStatement {
                    body: Box::new(body),
                    test,
                }))
            }
            Token::Switch => {
                self.lexer.next_token();
                self.lexer.expect_token(Token::OpenParen);
                self.lexer.next_token();
                let discriminant = self.parse_expression(OperatorPrecedence::Lowest)?;
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                self.lexer.expect_token(Token::OpenBrace);
                self.lexer.next_token();

                let mut cases: Vec<SwitchCase> = Vec::new();
                let mut found_default = false;
                while self.lexer.token != Token::CloseBrace {
                    let mut test: Option<Expression> = None;
                    let mut consequent: Vec<Box<Statement>> = Vec::new();

                    if self.lexer.token == Token::Default {
                        if found_default {
                            panic!("Multiple default clauses are not allowed");
                        }
                        self.lexer.next_token();
                        self.lexer.expect_token(Token::Colon);
                        self.lexer.next_token();
                        found_default = true;
                    } else {
                        self.lexer.expect_token(Token::Case);
                        self.lexer.next_token();
                        test = Some(self.parse_expression(OperatorPrecedence::Lowest)?);
                        self.lexer.expect_token(Token::Colon);
                        self.lexer.next_token();
                    }

                    'case_body: loop {
                        match &self.lexer.token {
                            Token::CloseBrace | Token::Case | Token::Default => break 'case_body,
                            _ => consequent.push(Box::new(self.parse_statement()?)),
                        };
                    }

                    cases.push(SwitchCase { consequent, test })
                }
                self.lexer.expect_token(Token::CloseBrace);
                self.lexer.next_token();
                Ok(Statement::SwitchStatement(SwitchStatement {
                    cases,
                    discriminant,
                }))
            }
            Token::Debugger => {
                self.lexer.next_token();
                Ok(Statement::DebuggerStatement(DebuggerStatement {}))
            }
            Token::With => {
                self.lexer.next_token();
                self.lexer.expect_token(Token::OpenParen);
                self.lexer.next_token();
                let object = self.parse_expression(OperatorPrecedence::Lowest)?;
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                let body = self.parse_statement()?;
                Ok(Statement::WithStatement(WithStatement {
                    body: Box::new(body),
                    object,
                }))
            }
            Token::Identifier(_) => {
                let identifier = self.parse_identifer()?;
                // Parse a labeled statement
                if self.lexer.token == Token::Colon {
                    self.lexer.next_token();
                    let body = self.parse_statement()?;
                    return Ok(Statement::LabeledStatement(LabeledStatement {
                        body: Box::new(body),
                        identifier,
                    }));
                } else {
                    // Parse a normal expression
                    let expression = self.parse_suffix(
                        OperatorPrecedence::Lowest,
                        Expression::Identifier(identifier),
                    )?;
                    return Ok(Statement::Expression(ExpressionStatement { expression }));
                }
            }
            Token::Throw => {
                self.lexer.next_token();
                let argument = self.parse_expression(OperatorPrecedence::Lowest)?;
                Ok(Statement::ThrowStatement(ThrowStatement { argument }))
            }
            Token::Try => {
                self.lexer.next_token();
                let block = self.parse_block_statement()?;
                let mut handler: Option<CatchClause> = None;
                let mut finalizer: Option<BlockStatement> = None;
                // Either catch or finally must be present.
                if self.lexer.token != Token::Catch && self.lexer.token != Token::Finally {
                    self.lexer.unexpected();
                    panic!();
                }
                if self.lexer.token == Token::Catch {
                    self.lexer.next_token();
                    self.lexer.expect_token(Token::OpenParen);
                    self.lexer.next_token();
                    let param = self.parse_identifer()?;
                    self.lexer.expect_token(Token::CloseParen);
                    self.lexer.next_token();
                    self.lexer.expect_token(Token::OpenBrace);
                    let body = self.parse_block_statement()?;
                    handler = Some(CatchClause { body, param });
                }
                if self.lexer.token == Token::Finally {
                    self.lexer.next_token();
                    self.lexer.expect_token(Token::OpenBrace);
                    finalizer = Some(self.parse_block_statement()?);
                }

                Ok(Statement::TryStatement(TryStatement {
                    block,
                    handler,
                    finalizer,
                }))
            }
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
            Token::StringLiteral(s) => {
                let expression = Expression::StringLiteral(StringLiteral { value: s.clone() });
                self.lexer.next_token();
                Ok(expression)
            }
            Token::Exclamation => self.parse_prefix_expression(),
            Token::Plus => self.parse_prefix_expression(),
            Token::Minus => self.parse_prefix_expression(),
            Token::True => self.parse_boolean().map(Expression::BooleanExpression),
            Token::False => self.parse_boolean().map(Expression::BooleanExpression),
            Token::OpenParen => self.parse_grouped_expression(),

            Token::New => {
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
                Ok(Expression::NewExpression(NewExpression {
                    arguments,
                    callee,
                }))
            }

            Token::OpenBrace => {
                self.lexer.next_token();

                let mut properties: Vec<Property> = Vec::new();

                while self.lexer.token != Token::CloseBrace {
                    let key: PropertyKey;
                    if self.lexer.token == Token::OpenBracket {
                        self.lexer.next_token();
                        key = PropertyKey::Identifier(self.parse_identifer()?);
                        self.lexer.expect_token(Token::CloseBracket);
                        self.lexer.next_token();
                    } else {
                        key = PropertyKey::StringLiteral(self.parse_string_literal()?);
                    }

                    self.lexer.expect_token(Token::Colon);
                    self.lexer.next_token();

                    let value = self.parse_expression(OperatorPrecedence::Lowest)?;
                    properties.push(Property {
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

                Ok(Expression::ObjectExpression(ObjectExpression {
                    properties,
                }))
            }

            Token::Function => self
                .parse_function_expression()
                .map(Expression::FunctionExpression),
            Token::This => {
                self.lexer.next_token();
                Ok(Expression::ThisExpression(ThisExpression {}))
            }
            Token::OpenBracket => {
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

                Ok(Expression::ArrayExpression(ArrayExpression { elements }))
            }
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

        let mut declarations: Vec<VariableDeclarator> = Vec::new();
        loop {
            let mut value: Option<Expression> = None;
            let identifier = self.parse_identifer()?;
            if self.lexer.token == Token::Equals {
                self.lexer.next_token();
                value = Some(self.parse_expression(OperatorPrecedence::Lowest)?);
            }
            declarations.push(VariableDeclarator {
                id: identifier,
                init: value,
            });

            if self.lexer.token != Token::Comma {
                break;
            }
            self.lexer.next_token();
        }

        self.consume_semicolon();

        Ok(VariableDeclaration {
            declarations,
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
