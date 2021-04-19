use js_ast_next::{
    array_binding_pattern::{ArrayBindingElement, ArrayBindingElementKind, ArrayBindingPattern},
    array_hole::ArrayHole,
    binding_identifier::BindingIdentifier,
    object_binding_pattern::BindingObjectPattern,
    TargetBindingPattern,
};
use js_error::{JSError, JSErrorKind};
use js_lexer_next::Token;
use span::Span;

use crate::{Parser, ParserError};

impl<'a> Parser<'a> {
    /// Parses a binding identifier
    pub fn parse_binding_identifier(&mut self) -> ParserError<BindingIdentifier> {
        let name = self.lexer.token_text.to_string();

        if self.strict && self.lexer.token == Token::Yield {
            return Err(JSError::new(
                JSErrorKind::UnexpectedYieldAsBindingIdentifier,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        if self.module && self.lexer.token == Token::Await {
            return Err(JSError::new(
                JSErrorKind::UnexpectedAwaitAsBindingIdentifier,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        if self.lexer.token.is_keyword() {
            return Err(JSError::new(
                JSErrorKind::ExpectedBindingIdentifier,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        if self.strict && self.lexer.token.is_future_reserved() {
            return Err(JSError::new(
                JSErrorKind::StrictModeReserved,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        let start = self.lexer.token_start;
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(BindingIdentifier {
            name,
            span: Span::new(start, end),
        })
    }

    /// Parses a binding array pattern
    pub fn parse_array_binding_pattern(&mut self) -> ParserError<ArrayBindingPattern> {
        let start = self.lexer.token_start;
        self.lexer.consume(Token::OpenBracket)?;
        let mut elements: Vec<ArrayBindingElementKind> = Vec::new();
        let mut rest: Option<Box<TargetBindingPattern>> = None;
        while self.lexer.token != Token::CloseBracket {
            // [...a]
            if self.lexer.token == Token::DotDotDot {
                self.lexer.next()?;
                rest = self
                    .parse_binding_identifier()
                    .map(TargetBindingPattern::BindingIdentifier)
                    .map(Box::new)
                    .map(Some)?;
                continue;
            }

            // [,]
            if self.lexer.token == Token::Comma {
                let span = Span::new(self.lexer.token_start, self.lexer.token_end);
                self.lexer.next()?;
                elements.push(ArrayBindingElementKind::ArrayHole(ArrayHole { span }));
                continue;
            }

            let start = self.lexer.token_start;
            let binding = match self.lexer.token {
                Token::OpenBracket => self
                    .parse_array_binding_pattern()
                    .map(TargetBindingPattern::BindingArrayPattern)?,
                Token::OpenBrace => self
                    .parse_object_binding_pattern()
                    .map(TargetBindingPattern::BindingObjectPattern)?,
                _ => self
                    .parse_binding_identifier()
                    .map(TargetBindingPattern::BindingIdentifier)?,
            };
            let initializer = match self.lexer.token {
                Token::Equals => {
                    self.lexer.next()?;
                    self.parse_expression().map(Some)?
                }
                _ => None,
            };
            let end = self.lexer.token_start;
            elements.push(ArrayBindingElementKind::ArrayBindingElement(
                ArrayBindingElement {
                    span: Span::new(start, end),
                    binding,
                    initializer,
                },
            ));
            if self.lexer.token == Token::Comma {
                self.lexer.next()?;
            }
        }
        self.lexer.consume(Token::CloseBracket)?;
        let end = self.lexer.token_start;
        Ok(ArrayBindingPattern {
            elements,
            rest,
            span: Span::new(start, end),
        })
    }

    /// Parses a binding object pattern
    pub fn parse_object_binding_pattern(&mut self) -> ParserError<BindingObjectPattern> {
        todo!()
    }
}
