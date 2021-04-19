use js_error::{JSError, JSErrorKind};
use span::Span;

use crate::Token;
use crate::{Lexer, LexerResult};

impl<'a> Lexer<'a> {
    /// Scans a template
    pub(crate) fn scan_template(&mut self) -> LexerResult<Token> {
        self.index += 1;
        let start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => {
                    return Err(JSError::new(
                        JSErrorKind::UnterminatedTemplateLiteral,
                        Span::new(start, self.current_position()),
                    ))
                }
            };

            if c == '`' {
                let end = self.current_position();
                self.index += 1;
                self.token_text = &self.input[start..end];
                return Ok(Token::String);
            }

            if c == '$' {
                self.index += 1;
                if self.current_character() == Some('{') {
                    let end = self.previous_position();
                    self.index += 1;
                    self.token_text = &self.input[start..end];
                    return Ok(Token::TemplateHead);
                } else {
                    continue;
                }
            }

            self.index += 1;
        }
    }

    /// Scans the next token as part of a template span
    /// This function assumes that the current character is `}`
    pub fn next_as_template_span(&mut self) -> LexerResult<()> {
        self.index += 1; // Skip the leading }
        self.token_start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => {
                    return Err(JSError::new(
                        JSErrorKind::UnterminatedTemplateLiteral,
                        Span::new(self.token_start, self.current_position()),
                    ))
                }
            };

            if c == '`' {
                let end = self.current_position();
                self.index += 1;
                self.token_text = &self.input[self.token_start..end];
                self.token = Token::TemplateTail;
                self.token_end = end;
                return Ok(());
            }

            if c == '$' {
                self.index += 1;
                if self.current_character() == Some('{') {
                    let end = self.previous_position();
                    self.index += 1;
                    self.token_text = &self.input[self.token_start..end];
                    self.token = Token::TemplateMiddle;
                    self.token_end = end;
                    return Ok(());
                } else {
                    continue;
                }
            }

            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_template() {
        let tests = vec![
            ("`hello`", "hello"),
            ("`hello $`", "hello $"),
            ("`hello $}`", "hello $}"),
            ("`hello }`", "hello }"),
            ("`hello {`", "hello {"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, Token::String);
            assert_eq!(lexer.token_text, test.1);
        }
    }

    #[test]
    fn test_template_head() {
        let tests = vec![("`hello ${`", "hello "), ("`h ${`", "h "), ("`${`", "")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, Token::TemplateHead);
            assert_eq!(lexer.token_text, test.1);
        }
    }

    #[test]
    fn test_template_middle() {
        let tests = vec![("}hello ${`", "hello "), ("}h ${`", "h "), ("}${`", "")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next_as_template_span(), Ok(()));
            assert_eq!(lexer.token, Token::TemplateMiddle);
            assert_eq!(lexer.token_text, test.1);
        }
    }

    #[test]
    fn test_template_tail() {
        let tests = vec![("}hello`", "hello"), ("}h`", "h"), ("}`", "")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next_as_template_span(), Ok(()));
            assert_eq!(lexer.token, Token::TemplateTail);
            assert_eq!(lexer.token_text, test.1);
        }
    }

    #[test]
    fn test_invalid_template() {
        let tests = vec![
            ("`", JSErrorKind::UnterminatedTemplateLiteral),
            (
                "`

            ",
                JSErrorKind::UnterminatedTemplateLiteral,
            ),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next_as_template_span().unwrap_err().kind, test.1);
        }
    }
}
