use javascript_ast::{
    expression::{Identifier, StringLiteral},
    statement::{
        ImportClause, ImportDeclaration, ImportDefaultSpecifier, ImportNamespaceSpecifier,
        ImportSpecifier, Statement,
    },
};
use javascript_token::Token;

use crate::{ParseResult, Parser, ParserError};

impl Parser {
    pub(crate) fn parse_import_statement(&mut self) -> ParseResult<Statement> {
        let source: StringLiteral;
        let mut specifiers: Vec<ImportClause> = Vec::new();

        self.next_token();

        if self.current_token == Token::OpenParen {
            return Err(ParserError(
                "Import expression are not supported yet".into(),
            ));
        }

        match &self.current_token {
            // import "module";
            Token::StringLiteral(_) => {
                source = self.parse_string_literal()?;
            }
            // import * as abc from "module";
            Token::Asterisk => {
                specifiers.push(self.parse_namespace_import_clause()?);
                self.expect_peek_token(Token::From)?;
                self.next_token();
                source = self.parse_string_literal()?;
            }
            // import { a } from "module";
            Token::OpenBrace => {
                specifiers.append(&mut self.parse_named_import_clause()?);
                self.expect_peek_token(Token::From)?;
                self.next_token();
                source = self.parse_string_literal()?;
            }
            // import a from "module";
            Token::Identifier(_) => {
                specifiers.append(&mut self.parse_default_import_clause()?);
                self.expect_peek_token(Token::From)?;
                self.next_token();
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
        self.expect_peek_token(Token::As)?;
        self.next_token();
        let identifier = self.parse_identifer()?;
        Ok(ImportClause::ImportNamespace(ImportNamespaceSpecifier {
            local: identifier,
        }))
    }

    fn parse_named_import_clause(&mut self) -> ParseResult<Vec<ImportClause>> {
        self.next_token();
        let mut specifiers: Vec<ImportClause> = Vec::new();
        let imported_identifier = self.parse_identifer()?;
        let mut local_identifier: Option<Identifier> = None;
        if self.peek_token == Token::As {
            self.next_token();
            self.next_token();
            local_identifier = Some(self.parse_identifer()?);
        }
        specifiers.push(ImportClause::Import(ImportSpecifier {
            imported: imported_identifier.clone(),
            local: local_identifier.unwrap_or(imported_identifier.clone()),
        }));

        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            let imported_identifier = self.parse_identifer()?;
            let mut local_identifier: Option<Identifier> = None;
            if self.peek_token == Token::As {
                self.next_token();
                self.next_token();
                local_identifier = Some(self.parse_identifer()?);
            }
            specifiers.push(ImportClause::Import(ImportSpecifier {
                imported: imported_identifier.clone(),
                local: local_identifier.unwrap_or(imported_identifier.clone()),
            }));
        }

        self.expect_peek_token(Token::CloseBrace)?;

        Ok(specifiers)
    }

    fn parse_default_import_clause(&mut self) -> ParseResult<Vec<ImportClause>> {
        let mut specifiers: Vec<ImportClause> = Vec::new();
        specifiers.push(ImportClause::ImportDefault(ImportDefaultSpecifier {
            local: self.parse_identifer()?,
        }));

        if self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            match &self.current_token {
                Token::Asterisk => specifiers.push(self.parse_namespace_import_clause()?),
                Token::OpenBrace => specifiers.append(&mut self.parse_named_import_clause()?),
                t => return Err(ParserError(format!("Unexpected token {}", t))),
            };
        }

        Ok(specifiers)
    }
}
