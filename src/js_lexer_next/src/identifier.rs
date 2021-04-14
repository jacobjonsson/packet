use crate::{Lexer, LexerResult, TokenKind};

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
    pub(crate) fn scan_identifier(&mut self) -> LexerResult<TokenKind> {
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

        Ok(TokenKind::from_potential_keyword(identifier))
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
            match lexer.next().unwrap().kind {
                TokenKind::Identifier { name } => assert_eq!(test.1, name),
                _ => panic!(),
            };
        }
    }

    #[test]
    fn test_valid_keywords() {
        let tests = vec![
            ("break", TokenKind::Break),
            ("case", TokenKind::Case),
            ("catch", TokenKind::Catch),
            ("class", TokenKind::Class),
            ("const", TokenKind::Const),
            ("continue", TokenKind::Continue),
            ("debugger", TokenKind::Debugger),
            ("default", TokenKind::Default),
            ("delete", TokenKind::Delete),
            ("do", TokenKind::Do),
            ("else", TokenKind::Else),
            ("enum", TokenKind::Enum),
            ("export", TokenKind::Export),
            ("extends", TokenKind::Extends),
            ("false", TokenKind::False),
            ("finally", TokenKind::Finally),
            ("for", TokenKind::For),
            ("function", TokenKind::Function),
            ("if", TokenKind::If),
            ("import", TokenKind::Import),
            ("in", TokenKind::In),
            ("instanceof", TokenKind::Instanceof),
            ("new", TokenKind::New),
            ("null", TokenKind::Null),
            ("return", TokenKind::Return),
            ("super", TokenKind::Super),
            ("switch", TokenKind::Switch),
            ("this", TokenKind::This),
            ("throw", TokenKind::Throw),
            ("true", TokenKind::True),
            ("try", TokenKind::Try),
            ("typeof", TokenKind::Typeof),
            ("var", TokenKind::Var),
            ("void", TokenKind::Void),
            ("while", TokenKind::While),
            ("with", TokenKind::With),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next().unwrap().kind, test.1);
        }
    }
}
