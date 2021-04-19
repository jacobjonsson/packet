use crate::{identifier::is_identifier_continue, Token};
use crate::{Lexer, LexerResult};
use js_error::{JSError, JSErrorKind};
use span::Span;

impl<'a> Lexer<'a> {
    // Scans the next token as part of a regexp instead of a normal token
    // Calling this function assumes that the leading slash has already been consumed
    pub fn next_as_regexp(&mut self) -> LexerResult<()> {
        self.token_start = self.previous_position(); // Include the leading slash
        self.scan_regexp_pattern()?;
        self.index += 1; // Skip over the ending slash
        self.scan_regexp_flags()?;
        self.token_end = self.current_position();

        self.token_text = &self.input[self.token_start..self.token_end];
        self.token = Token::Regexp;
        Ok(())
    }

    fn scan_regexp_pattern(&mut self) -> LexerResult<()> {
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
        Ok(())
    }

    fn scan_regexp_flags(&mut self) -> LexerResult<()> {
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
            return Ok(());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_regexp() {
        let tests = vec![("/abc/", "/abc/"), ("/abc/g", "/abc/g")];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, Token::Slash);
            assert_eq!(lexer.next_as_regexp(), Ok(()));
            assert_eq!(lexer.token_text, test.1);
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
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, Token::Slash);
            assert_eq!(lexer.next_as_regexp().unwrap_err().kind, test.1);
        }
    }
}
