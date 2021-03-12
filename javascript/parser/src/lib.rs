use javascript_ast::{
    expression::{Expression, Identifier, IntegerLiteral},
    statement::*,
    Program,
};
use javascript_lexer::Lexer;
use javascript_token::{Token, TokenLiteral};

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
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        todo!()
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        let left = self.parse_prefix()?;

        // TODO: Add infix parsing

        Ok(left)
    }

    fn parse_prefix(&mut self) -> ParseResult<Expression> {
        match &self.current_token {
            Token::NumericLiteral(_) => self.parse_numeric_literal(),
            t => Err(ParserError(format!("No prefix parser for {:?} found", t))),
        }
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

        let init = Some(self.parse_expression()?);
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
