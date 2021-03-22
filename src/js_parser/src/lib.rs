mod expression;
mod module;
mod statement;

use js_ast::{statement::*, Program};
use js_lexer::Lexer;
use js_token::Token;
use logger::Logger;

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

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    #[allow(dead_code)]
    logger: &'a dyn Logger,
}

/// Public
impl<'a> Parser<'a> {
    pub fn new<'b>(lexer: Lexer<'b>, logger: &'b impl Logger) -> Parser<'b> {
        Parser {
            lexer: lexer,
            logger,
        }
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
impl<'a> Parser<'a> {
    /// Consumes the next semicolon
    fn consume_semicolon(&mut self) {
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
        }
    }
}
