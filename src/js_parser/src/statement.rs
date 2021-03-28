use js_ast::{expression::*, statement::*};
use js_token::Token;
use logger::Logger;

use crate::{ParseResult, Parser};

impl<'a, L: Logger> Parser<'a, L> {
    pub(crate) fn parse_statement(&mut self) -> ParseResult<Statement> {
        match &self.lexer.token {
            Token::Const | Token::Var | Token::Let => self
                .parse_variable_declaration()
                .map(Statement::VariableDeclaration),
            Token::Import => self.parse_import_statement(),
            Token::Export => self.parse_export_statement(),
            Token::Function => self
                .parse_function_declaration()
                .map(Statement::FunctionDeclaration),
            Token::Return => self
                .parse_return_statement()
                .map(Statement::ReturnStatement),
            Token::If => self.parse_if_statement().map(Statement::IfStatement),
            Token::OpenBrace => self.parse_block_statement().map(Statement::BlockStatement),
            Token::For => self.parse_for_statement(),
            Token::Continue => {
                self.lexer.next_token();
                let mut label: Option<Identifier> = None;
                if self.lexer.token == Token::Identifier {
                    label = Some(self.parse_identifer()?);
                }
                self.consume_semicolon();
                Ok(Statement::ContinueStatement(ContinueStatement { label }))
            }
            Token::Break => {
                self.lexer.next_token();
                let mut label: Option<Identifier> = None;
                if self.lexer.token == Token::Identifier {
                    label = Some(self.parse_identifer()?);
                }
                self.consume_semicolon();
                Ok(Statement::BreakStatement(BreakStatement { label }))
            }
            Token::Semicolon => {
                self.lexer.next_token();
                Ok(Statement::EmptyStatement(EmptyStatement {}))
            }
            Token::Class => self
                .parse_class_declaration()
                .map(Statement::ClassDeclaration),

            Token::While => {
                self.lexer.next_token();
                self.lexer.expect_token(Token::OpenParen);
                self.lexer.next_token();
                let test = self.parse_expression(&Precedence::Lowest)?;
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
                let test = self.parse_expression(&Precedence::Lowest)?;
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
                let discriminant = self.parse_expression(&Precedence::Lowest)?;
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
                        test = Some(self.parse_expression(&Precedence::Lowest)?);
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
                let object = self.parse_expression(&Precedence::Lowest)?;
                self.lexer.expect_token(Token::CloseParen);
                self.lexer.next_token();
                let body = self.parse_statement()?;
                Ok(Statement::WithStatement(WithStatement {
                    body: Box::new(body),
                    object,
                }))
            }
            Token::Identifier => {
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
                    let expression =
                        self.parse_suffix(&Precedence::Lowest, Expression::Identifier(identifier))?;
                    self.consume_semicolon();
                    return Ok(Statement::Expression(ExpressionStatement { expression }));
                }
            }
            Token::Throw => {
                self.lexer.next_token();
                let argument = self.parse_expression(&Precedence::Lowest)?;
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
                }
                if self.lexer.token == Token::Catch {
                    self.lexer.next_token();
                    self.lexer.expect_token(Token::OpenParen);
                    self.lexer.next_token();
                    let param = self.parse_binding()?;
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
            _ => {
                let expression = self.parse_expression(&Precedence::Lowest)?;
                self.consume_semicolon();

                Ok(Statement::Expression(ExpressionStatement { expression }))
            }
        }
    }

    /// Parses a block statement
    /// {
    ///     statement1;
    ///     statement2;
    /// }
    pub(crate) fn parse_block_statement(&mut self) -> ParseResult<BlockStatement> {
        self.lexer.next_token();
        let mut statements: Vec<Statement> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            statements.push(self.parse_statement()?);
        }
        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(BlockStatement { statements })
    }

    /// Parse return statements
    /// return;
    /// return 1 + 1;
    pub(crate) fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        self.lexer.next_token();
        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
            return Ok(ReturnStatement { expression: None });
        }

        let expression = self.parse_expression(&Precedence::Lowest)?;
        self.consume_semicolon();
        Ok(ReturnStatement {
            expression: Some(expression),
        })
    }

    /// Parses an if statement
    /// if (test) { consequent } else { alternate }
    /// if (test) { consequent } else alternate
    pub(crate) fn parse_if_statement(&mut self) -> ParseResult<IfStatement> {
        self.lexer.next_token();
        self.lexer.expect_token(Token::OpenParen);
        self.lexer.next_token();
        let test = self.parse_expression(&Precedence::Lowest)?;
        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();

        // TODO: Function declarations are not valid in strict mode.
        let consequent = self.parse_statement()?;

        let mut alternate: Option<Box<Statement>> = None;
        if self.lexer.token == Token::Else {
            self.lexer.next_token();
            // TODO: Function declarations are not valid in strict mode.
            alternate = self.parse_statement().map(Box::new).map(Some)?;
        }

        Ok(IfStatement {
            alternate: alternate,
            consequent: Box::new(consequent),
            test,
        })
    }

    /// Parses for statement
    /// for (let a = 1; a < 10; a++) {}
    /// for (let a in items) {}
    /// for (let a of items) {}
    pub(crate) fn parse_for_statement(&mut self) -> ParseResult<Statement> {
        self.lexer.next_token();

        if self.lexer.token == Token::Await {
            panic!("\"for await\" syntax is not yet supported");
        }

        self.lexer.expect_token(Token::OpenParen);
        self.lexer.next_token();

        self.allow_in = false;

        let mut test: Option<Expression> = None;
        let mut update: Option<Expression> = None;

        let init = match self.lexer.token {
            Token::Const | Token::Let | Token::Var => self
                .parse_variable_declaration()
                .map(Statement::VariableDeclaration)
                .map(Box::new)
                .map(Some)?,
            Token::Semicolon => None,
            _ => self
                .parse_expression(&Precedence::Lowest)
                .map(|expression| Statement::Expression(ExpressionStatement { expression }))
                .map(Box::new)
                .map(Some)?,
        };

        self.allow_in = true;

        if self.lexer.token == Token::Of {
            // TODO: We should check for declarations here and forbid them if they exist.
            self.lexer.next_token();
            let right = self.parse_expression(&Precedence::Lowest)?;
            self.lexer.expect_token(Token::CloseParen);
            self.lexer.next_token();
            let body = self.parse_statement()?;
            if let Some(left) = init {
                return Ok(Statement::ForOfStatement(ForOfStatement {
                    body: Box::new(body),
                    left,
                    right,
                }));
            } else {
                // This essentially means we've somehow reached something like
                // "for (in <expression>) {}"" which should be impossible to reach.
                self.lexer.unexpected();
            }
        }

        if self.lexer.token == Token::In {
            // TODO: We should check for declarations here and forbid them if they exist.
            self.lexer.next_token();
            let right = self.parse_expression(&Precedence::Lowest)?;
            self.lexer.expect_token(Token::CloseParen);
            self.lexer.next_token();
            let body = self.parse_statement()?;
            if let Some(left) = init {
                return Ok(Statement::ForInStatement(ForInStatement {
                    body: Box::new(body),
                    left,
                    right,
                }));
            } else {
                // This essentially means we've somehow reached something like
                // "for (in <expression>) {}"" which should be impossible to reach.
                self.lexer.unexpected();
            }
        }

        if self.lexer.token == Token::Semicolon {
            self.lexer.next_token();
        }

        if self.lexer.token != Token::Semicolon {
            test = self.parse_expression(&Precedence::Lowest).map(Some)?;
        }

        self.lexer.expect_token(Token::Semicolon);
        self.lexer.next_token();

        if self.lexer.token != Token::CloseParen {
            update = self.parse_expression(&Precedence::Lowest).map(Some)?;
        }

        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();

        let body = self.parse_statement().map(Box::new)?;
        Ok(Statement::ForStatement(ForStatement {
            body,
            init,
            test,
            update,
        }))
    }

    /// Parses a variable declaration (var, const and let)
    /// var a = 1;
    /// var a = 1, b = 2;
    /// var a;
    pub(crate) fn parse_variable_declaration(&mut self) -> ParseResult<VariableDeclaration> {
        let kind = match self.lexer.token {
            Token::Const => VariableDeclarationKind::Const,
            Token::Let => VariableDeclarationKind::Let,
            Token::Var => VariableDeclarationKind::Var,
            _ => self.lexer.unexpected(),
        };
        self.lexer.next_token();

        let mut declarations: Vec<VariableDeclarator> = Vec::new();
        loop {
            let mut init: Option<Expression> = None;
            let id = self.parse_binding()?;
            if self.lexer.token == Token::Equals {
                self.lexer.next_token();
                init = Some(self.parse_expression(&Precedence::Assign)?);
            }
            declarations.push(VariableDeclarator { id, init });
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
}
