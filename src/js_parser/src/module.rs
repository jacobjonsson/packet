use js_ast::{
    expression::{Identifier, StringLiteral},
    statement::{
        AnonymousDefaultExportedFunctionDeclaration, Declaration, ExportAllDeclaration,
        ExportDefaultDeclaration, ExportDefaultDeclarationKind, ExportNamedDeclaration,
        ExportSpecifier, FunctionDeclaration, ImportClause, ImportDeclaration,
        ImportDefaultSpecifier, ImportNamespaceSpecifier, ImportSpecifier, Statement,
    },
};
use js_token::Token;

use crate::{OperatorPrecedence, ParseResult, Parser, ParserError};

// Import parsing
impl<'a> Parser<'a> {
    pub(crate) fn parse_import_statement(&mut self) -> ParseResult<Statement> {
        let source: StringLiteral;
        let mut specifiers: Vec<ImportClause> = Vec::new();

        self.lexer.next_token();

        if self.lexer.token == Token::OpenParen {
            return Err(ParserError(
                "Import expression are not supported yet".into(),
            ));
        }

        match &self.lexer.token {
            // import "module";
            Token::StringLiteral => {
                source = self.parse_string_literal()?;
            }
            // import * as abc from "module";
            Token::Asterisk => {
                specifiers.push(self.parse_namespace_import_clause()?);
                self.lexer.expect_token(Token::From);
                self.lexer.next_token();
                source = self.parse_string_literal()?;
            }
            // import { a } from "module";
            Token::OpenBrace => {
                specifiers.append(&mut self.parse_named_import_clause()?);
                self.lexer.expect_token(Token::From);
                self.lexer.next_token();
                source = self.parse_string_literal()?;
            }
            // import a from "module";
            Token::Identifier => {
                specifiers.append(&mut self.parse_default_import_clause()?);
                self.lexer.expect_token(Token::From);
                self.lexer.next_token();
                source = self.parse_string_literal()?;
            }
            t => return Err(ParserError(format!("Unexpected token {}", t))),
        };

        self.consume_semicolon();

        Ok(Statement::ImportDeclaration(ImportDeclaration {
            source,
            specifiers,
        }))
    }

    fn parse_namespace_import_clause(&mut self) -> ParseResult<ImportClause> {
        self.lexer.next_token();
        self.lexer.expect_token(Token::As);
        self.lexer.next_token();
        let identifier = self.parse_identifer()?;
        Ok(ImportClause::ImportNamespace(ImportNamespaceSpecifier {
            local: identifier,
        }))
    }

    fn parse_named_import_clause(&mut self) -> ParseResult<Vec<ImportClause>> {
        self.lexer.next_token();
        let mut specifiers: Vec<ImportClause> = Vec::new();
        let imported_identifier = self.parse_identifer()?;
        let mut local_identifier: Option<Identifier> = None;
        if self.lexer.token == Token::As {
            self.lexer.next_token();
            local_identifier = Some(self.parse_identifer()?);
        }
        specifiers.push(ImportClause::Import(ImportSpecifier {
            imported: imported_identifier.clone(),
            local: local_identifier.unwrap_or(imported_identifier.clone()),
        }));

        while self.lexer.token == Token::Comma {
            self.lexer.next_token();
            let imported_identifier = self.parse_identifer()?;
            let mut local_identifier: Option<Identifier> = None;
            if self.lexer.token == Token::As {
                self.lexer.next_token();
                local_identifier = Some(self.parse_identifer()?);
            }
            specifiers.push(ImportClause::Import(ImportSpecifier {
                imported: imported_identifier.clone(),
                local: local_identifier.unwrap_or(imported_identifier.clone()),
            }));
        }

        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();

        Ok(specifiers)
    }

    fn parse_default_import_clause(&mut self) -> ParseResult<Vec<ImportClause>> {
        let mut specifiers: Vec<ImportClause> = Vec::new();
        specifiers.push(ImportClause::ImportDefault(ImportDefaultSpecifier {
            local: self.parse_identifer()?,
        }));

        if self.lexer.token == Token::Comma {
            self.lexer.next_token();
            match &self.lexer.token {
                Token::Asterisk => specifiers.push(self.parse_namespace_import_clause()?),
                Token::OpenBrace => specifiers.append(&mut self.parse_named_import_clause()?),
                t => return Err(ParserError(format!("Unexpected token {}", t))),
            };
        }

        Ok(specifiers)
    }
}

// Export
impl<'a> Parser<'a> {
    pub(crate) fn parse_export_statement(&mut self) -> ParseResult<Statement> {
        self.lexer.next_token();

        match &self.lexer.token {
            Token::Asterisk => self.parse_export_all().map(Statement::ExportAllDeclaration),
            Token::Default => self
                .parse_export_default()
                .map(Statement::ExportDefaultDeclaration),
            _ => self
                .parse_export_named()
                .map(Statement::ExportNamedDeclaration),
        }
    }

    fn parse_export_all(&mut self) -> ParseResult<ExportAllDeclaration> {
        self.lexer.next_token();
        self.lexer.expect_token(Token::From);
        self.lexer.next_token();
        let string_literal = self.parse_string_literal()?;
        self.consume_semicolon();
        Ok(ExportAllDeclaration {
            source: string_literal,
        })
    }

    fn parse_export_default(&mut self) -> ParseResult<ExportDefaultDeclaration> {
        self.lexer.next_token();

        match self.lexer.token {
            Token::Function => {
                self.lexer.next_token();
                if self.lexer.token == Token::Identifier {
                    let identifier = self.parse_identifer()?;
                    self.lexer.expect_token(Token::OpenParen);
                    let parameters = self.parse_function_parameters()?;
                    self.lexer.expect_token(Token::OpenBrace);
                    let body = self.parse_block_statement()?;
                    Ok(ExportDefaultDeclaration {
                        declaration: ExportDefaultDeclarationKind::FunctionDeclaration(
                            FunctionDeclaration {
                                id: identifier,
                                parameters,
                                body,
                            },
                        ),
                    })
                } else {
                    self.lexer.expect_token(Token::OpenParen);
                    let parameters = self.parse_function_parameters()?;
                    self.lexer.expect_token(Token::OpenBrace);
                    let body = self.parse_block_statement()?;
                    Ok(ExportDefaultDeclaration {
                        declaration: ExportDefaultDeclarationKind::AnonymousDefaultExportedFunctionDeclaration(
                            AnonymousDefaultExportedFunctionDeclaration { parameters, body },
                        ),
                    })
                }
            }
            _ => {
                let expression = self.parse_expression(OperatorPrecedence::Lowest)?;
                self.consume_semicolon();
                Ok(ExportDefaultDeclaration {
                    declaration: ExportDefaultDeclarationKind::Expression(expression),
                })
            }
        }
    }

    fn parse_export_named(&mut self) -> ParseResult<ExportNamedDeclaration> {
        match &self.lexer.token {
            Token::Const | Token::Let | Token::Var => {
                let variable_declaration = self.parse_var_statement()?;
                Ok(ExportNamedDeclaration {
                    declaration: Some(Declaration::VariableDeclaration(variable_declaration)),
                    source: None,
                    specifiers: Vec::new(),
                })
            }
            Token::Function => {
                let function_declaration = self.parse_function_declaration()?;
                Ok(ExportNamedDeclaration {
                    declaration: Some(Declaration::FunctionDeclaration(function_declaration)),
                    source: None,
                    specifiers: Vec::new(),
                })
            }
            Token::OpenBrace => {
                let specifiers = self.parse_export_clause()?;
                let mut source: Option<StringLiteral> = None;
                if self.lexer.token == Token::From {
                    self.lexer.next_token();
                    self.lexer.expect_token(Token::StringLiteral);
                    source = Some(self.parse_string_literal()?);
                }
                self.consume_semicolon();
                Ok(ExportNamedDeclaration {
                    declaration: None,
                    source,
                    specifiers,
                })
            }

            _ => self.lexer.unexpected(),
        }
    }

    fn parse_export_clause(&mut self) -> ParseResult<Vec<ExportSpecifier>> {
        self.lexer.expect_token(Token::OpenBrace);
        self.lexer.next_token();

        let mut specifiers: Vec<ExportSpecifier> = Vec::new();
        while self.lexer.token != Token::CloseBrace {
            // The name can actually be a keyword if we're really an "export from"
            // statement. However, we won't know until later. Allow keywords as
            // identifiers for now and throw an error later if there's no "from".
            //
            //   // This is fine
            //   export { default } from 'path'
            //
            //   // This is a syntax error
            //   export { default }
            //
            if !self.lexer.is_identifier_or_keyword() {
                self.lexer.unexpected();
            }

            let local = Identifier {
                name: self.lexer.token_value.clone(),
            };
            self.lexer.next_token();
            let mut exported: Option<Identifier> = None;
            if self.lexer.token == Token::As {
                self.lexer.next_token();
                exported = Some(self.parse_identifer()?);
            }
            if self.lexer.token == Token::Comma {
                self.lexer.next_token();
            }

            specifiers.push(ExportSpecifier {
                exported: exported.unwrap_or_else(|| local.clone()),
                local: local,
            })
        }
        self.lexer.expect_token(Token::CloseBrace);
        self.lexer.next_token();
        Ok(specifiers)
    }
}
