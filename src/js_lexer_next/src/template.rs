use js_error::JSError;

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
                None => return Err(JSError::UnterminatedTemplateLiteral),
            };

            if c == '`' {
                let end = self.current_position();
                self.index += 1;
                let text = &self.input[start..end];
                return Ok(Token::NoSubstationTemplate { value: text.into() });
            }

            if c == '$' {
                self.index += 1;
                if self.current_character() == Some('{') {
                    let end = self.previous_position();
                    self.index += 1;
                    let text = &self.input[start..end];
                    return Ok(Token::TemplateHead { value: text.into() });
                } else {
                    continue;
                }
            }

            self.index += 1;
        }
    }

    /// Scans the next token as part of a template span
    /// This function assumes that the current character is `}`
    pub fn next_as_template_span(&mut self) -> LexerResult<Token> {
        self.index += 1; // Skip the leading }
        let start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => return Err(JSError::UnterminatedTemplateLiteral),
            };

            if c == '`' {
                let end = self.current_position();
                self.index += 1;
                let text = &self.input[start..end];
                return Ok(Token::TemplateTail { value: text.into() });
            }

            if c == '$' {
                self.index += 1;
                if self.current_character() == Some('{') {
                    let end = self.previous_position();
                    self.index += 1;
                    let text = &self.input[start..end];
                    return Ok(Token::TemplateMiddle { value: text.into() });
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
    fn test_no_substitution_template() {
        let tests = vec![
            ("`hello`", "hello"),
            ("`hello $`", "hello $"),
            ("`hello $}`", "hello $}"),
            ("`hello }`", "hello }"),
            ("`hello {`", "hello {"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(
                Token::NoSubstationTemplate {
                    value: test.1.into()
                },
                lexer.next().unwrap()
            );
        }
    }

    #[test]
    fn test_template_head() {
        let tests = vec![("`hello ${`", "hello "), ("`h ${`", "h "), ("`${`", "")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(
                Token::TemplateHead {
                    value: test.1.into()
                },
                lexer.next().unwrap()
            );
        }
    }

    #[test]
    fn test_template_middle() {
        let tests = vec![("}hello ${`", "hello "), ("}h ${`", "h "), ("}${`", "")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(
                Token::TemplateMiddle {
                    value: test.1.into()
                },
                lexer.next_as_template_span().unwrap(),
            );
        }
    }

    #[test]
    fn test_template_tail() {
        let tests = vec![("}hello`", "hello"), ("}h`", "h"), ("}`", "")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(
                Token::TemplateTail {
                    value: test.1.into()
                },
                lexer.next_as_template_span().unwrap(),
            );
        }
    }

    #[test]
    fn test_invalid_template() {
        let tests = vec![
            ("`", JSError::UnterminatedTemplateLiteral),
            (
                "`

            ",
                JSError::UnterminatedTemplateLiteral,
            ),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(Err(test.1), lexer.next());
        }
    }
}
