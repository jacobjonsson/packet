use js_ast::{expression::Identifier, literal::NumericLiteral, object::LiteralPropertyName};
use js_token::Token;

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_literal_property_name(&mut self) -> ParseResult<LiteralPropertyName> {
        match self.lexer.token {
            Token::StringLiteral => {
                let string_literal = self.parse_string_literal()?;
                Ok(LiteralPropertyName::StringLiteral(string_literal))
            }

            Token::NumericLiteral => {
                let numeric_literal = NumericLiteral {
                    value: self.lexer.number.clone(),
                };
                self.lexer.next_token();
                Ok(LiteralPropertyName::NumericLiteral(numeric_literal))
            }

            Token::Identifier => {
                let identifier = self.parse_identifer()?;
                Ok(LiteralPropertyName::Identifier(identifier))
            }

            Token::Null => {
                let identifier = Identifier {
                    name: "null".into(),
                };
                self.lexer.next_token();
                Ok(LiteralPropertyName::Identifier(identifier))
            }

            // Treat anything else as an identifier (null, undefined etc)
            _ => {
                let identifier = Identifier {
                    name: self.lexer.identifier.clone(),
                };
                self.lexer.next_token();
                Ok(LiteralPropertyName::Identifier(identifier))
            }
        }
    }
}
