mod binding;
mod class;
mod expression;
mod function;
mod literal;
mod module;
mod object;
mod statement;

use js_ast::{statement::*, AST};
use js_lexer::Lexer;
use js_token::Token;
use logger::Logger;
use source::Source;

/// Parses the given source into an AST.
pub fn parse<'a>(source: &Source, logger: &'a impl Logger) -> AST {
    let lexer = Lexer::new(source.content, logger);
    let mut parser = Parser::new(lexer, logger);
    let ast = parser.parse_program();
    ast
}

pub struct ParserError(String);
pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    #[allow(dead_code)]
    logger: &'a dyn Logger,
    /// in statement are only allowed in certain expressions.
    allow_in: bool,
}

/// Public
impl<'a> Parser<'a> {
    pub fn new<'b>(lexer: Lexer<'b>, logger: &'b impl Logger) -> Parser<'b> {
        Parser {
            lexer: lexer,
            logger,
            allow_in: true,
        }
    }

    pub fn parse_program(&mut self) -> AST {
        let mut statements = Vec::<Statement>::new();

        while &self.lexer.token != &Token::EndOfFile {
            match self.parse_statement() {
                Ok(s) => statements.push(s),
                Err(err) => panic!(err.0),
            }
        }

        AST { statements }
    }
}

/// Private
impl<'a> Parser<'a> {
    /// Consumes the next semicolon
    fn consume_semicolon(&mut self) {
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
        }
    }
}
