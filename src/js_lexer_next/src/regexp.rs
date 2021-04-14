use crate::{identifier::is_identifier_continue, TokenKind};
use crate::{Lexer, LexerResult};
use js_error::{JSError, JSErrorKind};
use span::Span;

impl<'a> Lexer<'a> {
    // Scans the next token as part of a regexp instead of a normal token
    // Calling this function assumes that the leading slash has already been consumed
    pub fn next_as_regexp(&mut self) -> LexerResult<TokenKind> {
        let pattern = self.scan_regexp_pattern()?;
        self.index += 1; // Skip over the ending slash
        let flags = self.scan_regexp_flags()?;

        Ok(TokenKind::Regexp { flags, pattern })
    }

    fn scan_regexp_pattern(&mut self) -> LexerResult<String> {
        let start = self.current_position();
        // Scan pattern
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => {
                    return Err(JSError::new(
                        JSErrorKind::UnterminatedRegexp,
                        Span::new(start, self.current_position()),
                    ))
                }
            };

            if c == '/' {
                break;
            }

            self.index += 1;
        }
        let end = self.current_position();
        Ok(self.input[start..end].into())
    }

    fn scan_regexp_flags(&mut self) -> LexerResult<Option<String>> {
        let start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            // If the character is not an identifier, break.
            if !is_identifier_continue(c) {
                break;
            }

            if matches!(c, 'i' | 'g' | 'm' | 's' | 'u' | 'y') {
                self.index += 1;
                continue;
            }

            return Err(JSError::new(
                JSErrorKind::InvalidRegexpFlag,
                Span::new(start, self.current_position()),
            ));
        }

        let end = self.current_position();
        if start == end {
            return Ok(None);
        }

        Ok(Some(self.input[start..end].into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_regexp() {
        let tests = vec![
            ("/abc/", "abc", None),
            ("/abc/g", "abc", Some(String::from("g"))),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next().unwrap().kind, TokenKind::Slash);
            assert_eq!(
                lexer.next_as_regexp().unwrap(),
                TokenKind::Regexp {
                    pattern: test.1.into(),
                    flags: test.2
                }
            );
        }
    }

    #[test]
    fn test_invalid_regexp() {
        let tests = vec![
            ("/abc", JSErrorKind::UnterminatedRegexp),
            ("/abc/bc", JSErrorKind::InvalidRegexpFlag),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next().unwrap().kind, TokenKind::Slash);
            assert_eq!(lexer.next_as_regexp().unwrap_err().kind, test.1);
        }
    }
}
