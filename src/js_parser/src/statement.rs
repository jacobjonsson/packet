use js_ast::{expression::*, statement::*};
use js_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser};

impl<'a> Parser<'a> {
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

    /// Parses function declarations
    /// function a() {}
    /// function a(arg1) {}
    pub(crate) fn parse_function_declaration(&mut self) -> ParseResult<FunctionDeclaration> {
        self.lexer.next_token(); // Skip the function keyword.
        let identifier = self.parse_identifer()?;
        self.lexer.expect_token(Token::OpenParen);
        let parameters = self.parse_function_parameters()?;
        self.lexer.expect_token(Token::OpenBrace);
        let body = self.parse_block_statement()?;
        Ok(FunctionDeclaration {
            id: identifier,
            parameters,
            body,
        })
    }

    pub(crate) fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
        self.consume_semicolon();

        Ok(Statement::Expression(ExpressionStatement { expression }))
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

        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
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
        let test = self.parse_expression(OperatorPrecedence::Lowest)?;
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

        let init: Option<ForStatementInit>;
        let mut test: Option<Expression> = None;
        let mut update: Option<Expression> = None;

        match self.lexer.token {
            Token::Const | Token::Let | Token::Var => {
                init = self
                    .parse_variable_declaration()
                    .map(ForStatementInit::VariableDeclaration)
                    .map(Some)?;
            }
            _ => {
                init = self
                    .parse_expression(OperatorPrecedence::Lowest)
                    .map(ForStatementInit::Expression)
                    .map(Some)?;
            }
        }

        if self.lexer.token == Token::Of {
            // TODO: We should check for declarations here and forbid them if they exist.
            self.lexer.next_token();
            let right = self.parse_expression(OperatorPrecedence::Lowest)?;
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
            let right = self.parse_expression(OperatorPrecedence::Lowest)?;
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
            test = self
                .parse_expression(OperatorPrecedence::Lowest)
                .map(Some)?;
        }

        self.lexer.expect_token(Token::Semicolon);
        self.lexer.next_token();

        if self.lexer.token != Token::CloseParen {
            update = self
                .parse_expression(OperatorPrecedence::Lowest)
                .map(Some)?;
        }

        self.lexer.expect_token(Token::CloseParen);
        self.lexer.next_token();

        let body = self.parse_statement().map(Box::new)?;
        Ok(Statement::For(ForStatement {
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
}