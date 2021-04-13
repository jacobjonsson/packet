use js_error::JSError;

use crate::Token;
use crate::{whitespace::is_line_terminator, Lexer, LexerResult};

impl<'a> Lexer<'a> {
    /// Scans a string
    pub(crate) fn scan_string(&mut self, quote: char) -> LexerResult<Token> {
        self.index += 1; // Skip over the leading quote
        let start = self.current_position();

        let end: usize;
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => return Err(JSError::UnterminatedStringLiteral),
            };

            if is_line_terminator(c) {
                return Err(JSError::UnterminatedStringLiteral);
            }

            // Break on ending quote
            if c == quote {
                end = self.current_position();
                self.index += 1;
                break;
            }

            // A slash escapes any character after it so we skip twice and continue;
            if c == '\\' {
                self.index += 2;
                continue;
            }

            // Any other character is considered part of the string
            self.index += 1;
        }

        let value = &self.input[start..end];
        Ok(Token::String {
            value: value.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_strings() {
        let tests = vec![
            ("\"hello world\"", "hello world"),
            ("'hello world'", "hello world"),
            ("\"\"", ""),
            ("''", ""),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            match lexer.next().unwrap() {
                Token::String { value } => assert_eq!(test.1, value),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test_invalid_strings() {
        let tests = vec![
            ("\"hello", JSError::UnterminatedStringLiteral),
            ("'hello", JSError::UnterminatedStringLiteral),
            ("'hello\"", JSError::UnterminatedStringLiteral),
            ("\"hello'", JSError::UnterminatedStringLiteral),
            (
                "\"hello


            \"",
                JSError::UnterminatedStringLiteral,
            ),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Err(test.1));
        }
    }
}
