mod expression;
mod import;
mod statement;

use javascript_ast::{
    expression::{
        BooleanExpression, Expression, Identifier, InfixExpression, IntegerLiteral,
        PrefixExpression, StringLiteral,
    },
    statement::*,
    Program,
};
use javascript_lexer::Lexer;
use javascript_token::{Token, TokenLiteral};

/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table
/// https://github.com/evanw/esbuild/blob/51b785f89933426afe675b4e633cf531d5a9890d/internal/js_ast/js_ast.go#L29
#[derive(Debug, PartialEq, PartialOrd)]
enum OperatorPrecedence {
    Lowest = 1,
    Conditional = 2,
    Equals = 3,
    LessGreater = 4,
    Sum = 5,
    Product = 6,
    Prefix = 7,
    Call = 8,
}

fn token_to_precedence(token: &Token) -> OperatorPrecedence {
    match token {
        Token::EqualsEquals => OperatorPrecedence::Equals,
        Token::EqualsEqualsEquals => OperatorPrecedence::Equals,
        Token::ExclamationEquals => OperatorPrecedence::Equals,
        Token::ExclamationEqualsEquals => OperatorPrecedence::Equals,
        Token::LessThan => OperatorPrecedence::LessGreater,
        Token::GreaterThan => OperatorPrecedence::LessGreater,
        Token::Plus => OperatorPrecedence::Sum,
        Token::Minus => OperatorPrecedence::Sum,
        Token::Slash => OperatorPrecedence::Product,
        Token::Asterisk => OperatorPrecedence::Product,
        Token::OpenParen => OperatorPrecedence::Call,
        Token::Question => OperatorPrecedence::Conditional,
        _ => OperatorPrecedence::Lowest,
    }
}

pub struct ParserError(String);
pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
}

/// Public
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer,
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::<Statement>::new();

        while &self.lexer.token != &Token::EndOfFile {
            match self.parse_statement() {
                Ok(s) => statements.push(s),
                Err(err) => self.errors.push(err.0),
            }
        }

        Program { statements }
    }
}

/// Private
impl Parser {
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match &self.lexer.token {
            Token::Const => self.parse_var_statement(VariableDeclarationKind::Const),
            Token::Var => self.parse_var_statement(VariableDeclarationKind::Var),
            Token::Let => self.parse_var_statement(VariableDeclarationKind::Let),
            Token::Import => self.parse_import_statement(),
            Token::Function => self
                .parse_function_declaration()
                .map(Statement::FunctionDeclaration),
            Token::Return => self.parse_return_statement().map(Statement::Return),
            Token::If => self.parse_if_statement().map(Statement::If),
            Token::OpenBrace => self.parse_block_statement().map(Statement::Block),
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
        let mut left = self.parse_prefix()?;

        while self.lexer.token != Token::Semicolon && precedence < self.current_precedence() {
            if let Ok(s) = self.parse_infix(left.clone()) {
                left = s;
            } else {
                return Ok(left);
            }
        }

        Ok(left)
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
            t => Err(ParserError(format!("No prefix parser for {:?} found", t))),
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

    fn parse_infix(&mut self, left: Expression) -> ParseResult<Expression> {
        match &self.lexer.token {
            Token::Plus => self.parse_infix_expression(left),
            Token::Minus => self.parse_infix_expression(left),
            Token::Slash => self.parse_infix_expression(left),
            Token::Asterisk => self.parse_infix_expression(left),
            Token::EqualsEquals => self.parse_infix_expression(left),
            Token::EqualsEqualsEquals => self.parse_infix_expression(left),
            Token::ExclamationEquals => self.parse_infix_expression(left),
            Token::ExclamationEqualsEquals => self.parse_infix_expression(left),
            Token::LessThan => self.parse_infix_expression(left),
            Token::GreaterThan => self.parse_infix_expression(left),
            Token::OpenParen => self
                .parse_call_expression(left)
                .map(Expression::CallExpression),
            Token::Question => self
                .parse_conditional_expression(left)
                .map(Expression::ConditionalExpression),
            t => Err(ParserError(format!(
                "No infix parse function for {} found",
                t.token_literal()
            ))),
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> ParseResult<Expression> {
        let operator = self.lexer.token.token_literal();
        let precedence = self.current_precedence();
        self.lexer.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Expression::InfixExpression(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.lexer.next_token();
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.expect_token(Token::CloseParen);
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

    fn parse_var_statement(&mut self, kind: VariableDeclarationKind) -> ParseResult<Statement> {
        self.lexer.next_token();
        let id = self.parse_identifer()?;
        // Means we hit a variable declaration without an assignment (eg: let a;)
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
            return Ok(Statement::VariableDeclaration(VariableDeclaration {
                declarations: vec![VariableDeclarator { id, init: None }],
                kind,
            }));
        }

        self.expect_token(Token::Equals);
        self.lexer.next_token();

        let init = Some(self.parse_expression(OperatorPrecedence::Lowest)?);
        // We can't expect a semicolon here since they are optional in JS.
        // But we should insert semicolons instead of just skipping when they are missing,
        // it will make printing easier.
        self.consume_semicolon();

        Ok(Statement::VariableDeclaration(VariableDeclaration {
            declarations: vec![VariableDeclarator { id, init }],
            kind: kind,
        }))
    }

    fn current_precedence(&self) -> OperatorPrecedence {
        token_to_precedence(&self.lexer.token)
    }

    /// Consumes the next semicolon
    fn consume_semicolon(&mut self) {
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
        }
    }

    fn expect_token(&mut self, token: Token) {
        if self.lexer.token != token {
            panic!("Expected {} but got {}", token, self.lexer.token);
        }
    }
}
