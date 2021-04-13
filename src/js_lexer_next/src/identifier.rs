use crate::{token::lookup_keyword, Lexer, LexerResult, Token};

/// True if `c` is considered a identifier start according to the ECMAScript specification
///
/// See [ECMAScript specification](https://262.ecma-international.org/11.0/#sec-names-and-keywords)
pub fn is_identifier_start(c: char) -> bool {
    // We start by fast-checking the ASCII characters and
    // if no match is find we run the slower unicode check.
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || c == '_'
        || c == '$'
        || unicode::id_start(c)
}

/// True if `c` is considered a identifier start according to the ECMAScript specification.
///
/// See [ECMAScript specification](https://262.ecma-international.org/11.0/#sec-names-and-keywords)
pub fn is_identifier_continue(c: char) -> bool {
    // We start by fast-checking the ASCII characters and
    // if no match is find we run the slower unicode check.
    ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
        || ('0'..='9').contains(&c)
        || c == '\u{200C}'
        || c == '\u{200D}'
        || c == '_'
        || c == '$'
        || unicode::id_continue(c)
}

impl<'a> Lexer<'a> {
    pub(crate) fn scan_identifier(&mut self) -> LexerResult<Token> {
        let start = self.current_position();

        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if !is_identifier_continue(character) {
                break;
            }

            self.index += 1;
        }

        let end = self.current_position();
        let identifier = &self.input[start..end];

        Ok(lookup_keyword(identifier).unwrap_or(Token::Identifier {
            name: identifier.into(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_identifiers() {
        let tests = vec![
            ("a;", "a"),
            ("_a", "_a"),
            ("$a", "$a"),
            ("a_b", "a_b"),
            ("let", "let"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            match lexer.next() {
                Ok(Token::Identifier { name }) => assert_eq!(test.1, name),
                _ => panic!(),
            };
        }
    }

    #[test]
    fn test_valid_keywords() {
        let tests = vec![
            ("break", Token::Break),
            ("case", Token::Case),
            ("catch", Token::Catch),
            ("class", Token::Class),
            ("const", Token::Const),
            ("continue", Token::Continue),
            ("debugger", Token::Debugger),
            ("default", Token::Default),
            ("delete", Token::Delete),
            ("do", Token::Do),
            ("else", Token::Else),
            ("enum", Token::Enum),
            ("export", Token::Export),
            ("extends", Token::Extends),
            ("false", Token::False),
            ("finally", Token::Finally),
            ("for", Token::For),
            ("function", Token::Function),
            ("if", Token::If),
            ("import", Token::Import),
            ("in", Token::In),
            ("instanceof", Token::Instanceof),
            ("new", Token::New),
            ("null", Token::Null),
            ("return", Token::Return),
            ("super", Token::Super),
            ("switch", Token::Switch),
            ("this", Token::This),
            ("throw", Token::Throw),
            ("true", Token::True),
            ("try", Token::Try),
            ("typeof", Token::Typeof),
            ("var", Token::Var),
            ("void", Token::Void),
            ("while", Token::While),
            ("with", Token::With),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(test.1));
        }
    }
}
