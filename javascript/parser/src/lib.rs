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

#[derive(Debug, PartialEq, PartialOrd)]
enum OperatorPrecedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
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
        _ => OperatorPrecedence::Lowest,
    }
}

pub struct ParserError(String);
pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    current_token: Token,
    peek_token: Token,
}

/// Public
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer: lexer,
            errors: Vec::new(),
            current_token: Token::EndOfFile,
            peek_token: Token::EndOfFile,
        };

        // We run this twice to load both current and peek.
        parser.next_token();
        parser.next_token();
        return parser;
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::<Statement>::new();

        while &self.current_token != &Token::EndOfFile {
            match self.parse_statement() {
                Ok(s) => statements.push(s),
                Err(err) => self.errors.push(err.0),
            }
            self.next_token();
        }

        Program { statements }
    }
}

/// Private
impl Parser {
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match &self.current_token {
            Token::Const => self.parse_var_statement(VariableDeclarationKind::Const),
            Token::Var => self.parse_var_statement(VariableDeclarationKind::Var),
            Token::Let => self.parse_var_statement(VariableDeclarationKind::Let),
            Token::Import => self.parse_import_statement(),
            Token::Function => self
                .parse_function_declaration()
                .map(Statement::FunctionDeclaration),
            Token::Return => self.parse_return_statement().map(Statement::Return),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_string_literal(&mut self) -> ParseResult<StringLiteral> {
        let string_literal = StringLiteral {
            value: self.current_token.token_literal(),
        };
        self.next_token();
        Ok(string_literal)
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::Expression(ExpressionStatement { expression }))
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> ParseResult<Expression> {
        let mut left = self.parse_prefix()?;

        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            self.next_token();

            if let Ok(s) = self.parse_infix(left.clone()) {
                left = s;
            } else {
                return Ok(left);
            }
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        match &self.current_token {
            Token::NumericLiteral(_) => self.parse_numeric_literal(),
            Token::Identifier(_) => self.parse_identifer().map(Expression::Identifier),
            Token::Exclamation => self.parse_prefix_expression(),
            Token::Plus => self.parse_prefix_expression(),
            Token::Minus => self.parse_prefix_expression(),
            Token::True => self.parse_boolean(),
            Token::False => self.parse_boolean(),
            Token::OpenParen => self.parse_grouped_expression(),
            t => Err(ParserError(format!("No prefix parser for {:?} found", t))),
        }
    }

    fn parse_infix(&mut self, left: Expression) -> ParseResult<Expression> {
        match &self.current_token {
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
            // Token::OpenParen => self.parse_call_expression(left),
            t => Err(ParserError(format!(
                "No infix parse function for {} found",
                t.token_literal()
            ))),
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.current_token.token_literal();
        self.next_token();
        let right = self.parse_expression(OperatorPrecedence::Prefix)?;
        Ok(Expression::PrefixExpression(PrefixExpression {
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();

        let expression = self.parse_expression(OperatorPrecedence::Lowest);

        self.expect_peek_token(Token::CloseParen)?;

        return expression;
    }

    fn parse_identifer(&mut self) -> ParseResult<Identifier> {
        match &self.current_token {
            Token::Identifier(i) => Ok(Identifier { name: i.clone() }),
            t => Err(ParserError(format!("Expected identifier but got {}", t))),
        }
    }

    fn parse_boolean(&mut self) -> ParseResult<Expression> {
        Ok(Expression::BooleanExpression(BooleanExpression {
            value: match &self.current_token {
                Token::True => true,
                Token::False => false,
                c => {
                    return Err(ParserError(format!(
                        "Expected to get true or false but got {:?}",
                        c
                    )));
                }
            },
        }))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> ParseResult<Expression> {
        let operator = self.current_token.token_literal();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Expression::InfixExpression(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_numeric_literal(&mut self) -> ParseResult<Expression> {
        let value = self
            .current_token
            .token_literal()
            .parse::<i64>()
            .map_err(|_| {
                ParserError(format!(
                    "Failed to parse {} as number",
                    self.current_token.token_literal()
                ))
            })?;

        Ok(Expression::IntegerLiteral(IntegerLiteral { value }))
    }

    fn parse_var_statement(&mut self, kind: VariableDeclarationKind) -> ParseResult<Statement> {
        self.next_token();

        let name = match &self.current_token {
            Token::Identifier(i) => i.clone(),
            t => return Err(ParserError(format!("Expected identifier but got {}", t))),
        };
        let id = Identifier { name };

        // Means we hit a variable declaration without an assignment (eg: let a;)
        if self.peek_token == Token::Semicolon {
            self.next_token();
            return Ok(Statement::VariableDeclaration(VariableDeclaration {
                declarations: vec![VariableDeclarator { id, init: None }],
                kind,
            }));
        }

        self.expect_peek_token(Token::Equals)?;
        self.next_token();

        let init = Some(self.parse_expression(OperatorPrecedence::Lowest)?);
        // We can't expect a semicolon here since they are optional in JS.
        // But we should insert semicolons instead of just skipping when they are missing,
        // it will make printing easier.
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::VariableDeclaration(VariableDeclaration {
            declarations: vec![VariableDeclarator { id, init }],
            kind: kind,
        }))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn peek_precedence(&self) -> OperatorPrecedence {
        token_to_precedence(&self.peek_token)
    }

    fn current_precedence(&self) -> OperatorPrecedence {
        token_to_precedence(&self.current_token)
    }

    /// Asserts that the peek token is the given one and increments the lexer.
    /// If the peek token is not the given one it returns an error and does not increment the lexer.
    fn expect_peek_token(&mut self, token: Token) -> ParseResult<()> {
        if self.peek_token != token {
            return Err(ParserError(format!(
                "Expected {} but got {}",
                token, self.peek_token
            )));
        }
        self.next_token();
        Ok(())
    }
}
