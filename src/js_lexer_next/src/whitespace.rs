use js_error::JSError;

use crate::{Lexer, LexerResult};

/// True if `c` is considered whitespace according to the ECMAScript specification
///
/// See [ECMAScript specification](https://262.ecma-international.org/11.0/#sec-white-space)
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}' // Tab
        | '\u{000B}' // Vertical tab
        | '\u{000C}' // Form feed
        | '\u{0020}' // Space
        | '\u{00A0}' // No-break space
        | '\u{FEFF}' // Zero width no-break space
        | '\u{1680}' // Orgham space mark
        | '\u{2000}' // En quad
        | '\u{2001}' // Em quad
        | '\u{2002}' // En space
        | '\u{2003}' // Em space
        | '\u{2004}' // Three per em space
        | '\u{2005}' // Four per em space
        | '\u{2006}' // Six per em space
        | '\u{2007}' // Figure space
        | '\u{2008}' // Punctuation space
        | '\u{2009}' // Thin space
        | '\u{200A}' // Hair space
        | '\u{202F}' // Narrow no-break space
        | '\u{205F}' // Medium mathematical space
        | '\u{3000}' // Ideographic space
    )
}

/// True if `c` is considered a line terminator according to the ECMAScript specification
///
/// See [ECMAScript specification](https://262.ecma-international.org/11.0/#sec-line-terminators)
pub fn is_line_terminator(c: char) -> bool {
    matches!(
        c,
        '\u{000A}' // Line feed
        | '\u{000D}' // Carriage return
        | '\u{2028}' // Line separator
        | '\u{2029}' // Paragraph separator
    )
}

impl<'a> Lexer<'a> {
    /// Skips over whitespace and comments
    pub(crate) fn skip_whitespace(&mut self) -> LexerResult<()> {
        while let Some(character) = self.current_character() {
            match character {
                c if is_whitespace(c) => {
                    self.index += 1;
                }

                c if is_line_terminator(c) => {
                    self.index += 1;
                }

                '/' => {
                    let next = self.next_character();
                    if next == Some('/') {
                        self.skip_single_line_comment();
                    } else if next == Some('*') {
                        self.skip_block_comment()?;
                    } else {
                        break;
                    }
                }

                _ => break,
            }
        }

        Ok(())
    }

    fn skip_single_line_comment(&mut self) {
        self.index += 2;

        while let Some(character) = self.current_character() {
            if is_line_terminator(character) {
                break;
            }

            self.index += 1;
        }
    }

    fn skip_block_comment(&mut self) -> LexerResult<()> {
        self.index += 2;

        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => return Err(JSError::UnterminatedBlockComment),
            };

            if c == '*' {
                if self.next_character() == Some('/') {
                    self.index += 2;
                    break;
                }
            }

            self.index += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

    #[test]
    fn test_single_line_comment() {
        let tests = vec![
            "// this is a comment",
            "// hello world // hello",
            "// // // // ",
        ];

        for test in tests {
            let mut lexer = Lexer::new(test);
            assert_eq!(lexer.next().unwrap(), Token::Eof);
        }
    }

    #[test]
    fn test_block_comment() {
        let tests = vec![
            "/* this a block comment */",
            "/* \n\n\n\n\n\n */",
            "/*
              *
              *
              *
              */",
        ];

        for test in tests {
            let mut lexer = Lexer::new(test);
            assert_eq!(lexer.next().unwrap(), Token::Eof);
        }
    }

    #[test]
    fn test_whitespace() {
        let tests = vec![
            "                  ",
            "\n\n\n\n",
            "



            ",
            "\t\t\t\t\t",
        ];

        for test in tests {
            let mut lexer = Lexer::new(test);
            assert_eq!(lexer.next().unwrap(), Token::Eof);
        }
    }

    #[test]
    fn test_whitespace_context() {
        let tests = vec![
            (
                "/* hello world */identifier",
                Token::Identifier {
                    name: "identifier".into(),
                },
            ),
            ("/* hello world */123", Token::Number { value: 123. }),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next().unwrap(), test.1);
        }
    }

    #[test]
    fn test_invalid_comments() {
        let tests = vec![
            ("/*", JSError::UnterminatedBlockComment),
            ("/****", JSError::UnterminatedBlockComment),
            (
                "/*


            *",
                JSError::UnterminatedBlockComment,
            ),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Err(test.1));
        }
    }
}
