use crate::Token;
use crate::{Lexer, LexerResult};

/// Implementations of scanning functions for the punctuators used in JavaScript.
///
/// See [spec](https://tc39.es/ecma262/#sec-punctuators)
impl<'a> Lexer<'a> {
    /// ~
    pub(crate) fn scan_tilde(&mut self) -> Token {
        self.index += 1;
        Token::Tilde
    }

    /// (
    pub(crate) fn scan_open_paren(&mut self) -> Token {
        self.index += 1;
        Token::OpenParen
    }

    /// )
    pub(crate) fn scan_close_paren(&mut self) -> Token {
        self.index += 1;
        Token::CloseParen
    }

    /// {
    pub(crate) fn scan_open_brace(&mut self) -> Token {
        self.index += 1;
        Token::OpenBrace
    }

    /// }
    pub(crate) fn scan_close_brace(&mut self) -> Token {
        self.index += 1;
        Token::CloseBrace
    }

    /// [
    pub(crate) fn scan_open_bracket(&mut self) -> Token {
        self.index += 1;
        Token::OpenBracket
    }

    /// ]
    pub(crate) fn scan_close_bracket(&mut self) -> Token {
        self.index += 1;
        Token::CloseBracket
    }

    /// ;
    pub(crate) fn scan_semicolon(&mut self) -> Token {
        self.index += 1;
        Token::Semicolon
    }

    /// :
    pub(crate) fn scan_colon(&mut self) -> Token {
        self.index += 1;
        Token::Colon
    }

    /// :
    pub(crate) fn scan_comma(&mut self) -> Token {
        self.index += 1;
        Token::Comma
    }

    /// /, /=
    pub(crate) fn scan_slash(&mut self) -> Token {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            Token::SlashEquals
        } else {
            Token::Slash
        }
    }

    /// &, &=, &&, &&=
    pub(crate) fn scan_ampersand(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('&') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::AmpersandAmpersandEquals
            } else {
                Token::AmpersandAmpersand
            }
        } else if character == Some('=') {
            self.index += 1;
            Token::AmpersandEquals
        } else {
            Token::Ampersand
        }
    }

    /// -, --, -=
    pub(crate) fn scan_minus(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('-') {
            self.index += 1;
            Token::MinusMinus
        } else if character == Some('=') {
            self.index += 1;
            Token::MinusEquals
        } else {
            Token::Minus
        }
    }

    /// !, !=, !==
    pub(crate) fn scan_exclamation(&mut self) -> Token {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::ExclamationEqualsEquals
            } else {
                Token::ExclamationEquals
            }
        } else {
            Token::Exclamation
        }
    }

    /// ?, ?., ??, ??=
    pub(crate) fn scan_question_mark(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('?') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::QuestionQuestionEquals
            } else {
                Token::QuestionQuestion
            }
        } else if character == Some('.') {
            Token::QuestionDot
        } else {
            Token::Question
        }
    }

    /// ., ..., .123
    pub(crate) fn scan_dot(&mut self) -> LexerResult<Token> {
        let next = self.next_character();
        if next >= Some('0') && next <= Some('9') {
            self.scan_floating_point()
        } else if next == Some('.') {
            self.index += 3;
            Ok(Token::DotDotDot)
        } else {
            self.index += 1;
            Ok(Token::Dot)
        }
    }

    /// *, *=, **, **=
    pub(crate) fn scan_asterisk(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('*') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::AsteriskAsteriskEquals
            } else {
                Token::AsteriskAsterisk
            }
        } else if character == Some('=') {
            self.index += 1;
            Token::AsteriskEquals
        } else {
            Token::Asterisk
        }
    }

    /// %, %=
    pub(crate) fn scan_percent(&mut self) -> Token {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            Token::PercentEquals
        } else {
            Token::Percent
        }
    }

    /// ^, ^=
    pub(crate) fn scan_caret(&mut self) -> Token {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            Token::CaretEquals
        } else {
            Token::Caret
        }
    }

    /// +, ++, +=
    pub(crate) fn scan_plus(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('=') {
            self.index += 1;
            Token::PlusEquals
        } else if character == Some('+') {
            self.index += 1;
            Token::PlusPlus
        } else {
            Token::Plus
        }
    }

    /// <, <<, <<=, <=
    pub(crate) fn scan_less_than(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('<') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::LessThanLessThanEquals
            } else {
                Token::LessThanLessThan
            }
        } else if character == Some('=') {
            self.index += 1;
            Token::LessThanEquals
        } else {
            Token::LessThan
        }
    }

    /// =, ==, ===, =>
    pub(crate) fn scan_equals(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('=') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::EqualsEqualsEquals
            } else {
                Token::EqualsEquals
            }
        } else if character == Some('>') {
            self.index += 1;
            Token::EqualsGreaterThan
        } else {
            Token::Equals
        }
    }

    /// >, >=, >>, >>=, >>>, >>>=
    pub(crate) fn scan_greater_than(&mut self) -> Token {
        self.index += 1;
        let mut character = self.current_character();
        if character == Some('>') {
            self.index += 1;
            character = self.current_character();
            if character == Some('>') {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    Token::GreaterThanGreaterThanGreaterThanEquals
                } else {
                    Token::GreaterThanGreaterThanGreaterThan
                }
            } else if character == Some('=') {
                self.index += 1;
                Token::GreaterThanGreaterThanEquals
            } else {
                Token::GreaterThanGreaterThan
            }
        } else if character == Some('=') {
            self.index += 1;
            Token::GreaterThanEquals
        } else {
            Token::GreaterThan
        }
    }

    /// |, |=, ||, ||=
    pub(crate) fn scan_bar(&mut self) -> Token {
        self.index += 1;
        let character = self.current_character();
        if character == Some('|') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                Token::BarBarEquals
            } else {
                Token::BarBar
            }
        } else if character == Some('=') {
            self.index += 1;
            Token::BarEquals
        } else {
            Token::Bar
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punctuators() {
        let tests = vec![
            ("--", Token::MinusMinus),
            ("-", Token::Minus),
            ("-=", Token::MinusEquals),
            (",", Token::Comma),
            (";", Token::Semicolon),
            (":", Token::Colon),
            ("!", Token::Exclamation),
            ("!=", Token::ExclamationEquals),
            ("!==", Token::ExclamationEqualsEquals),
            ("??", Token::QuestionQuestion),
            ("??=", Token::QuestionQuestionEquals),
            ("?.", Token::QuestionDot),
            ("?", Token::Question),
            ("...", Token::DotDotDot),
            (".", Token::Dot),
            ("(", Token::OpenParen),
            (")", Token::CloseParen),
            ("[", Token::OpenBracket),
            ("]", Token::CloseBracket),
            ("{", Token::OpenBrace),
            ("}", Token::CloseBrace),
            ("*", Token::Asterisk),
            ("**", Token::AsteriskAsterisk),
            ("**=", Token::AsteriskAsteriskEquals),
            ("*=", Token::AsteriskEquals),
            ("/", Token::Slash),
            ("/=", Token::SlashEquals),
            ("&", Token::Ampersand),
            ("&&", Token::AmpersandAmpersand),
            ("&&=", Token::AmpersandAmpersandEquals),
            ("&=", Token::AmpersandEquals),
            ("%", Token::Percent),
            ("%=", Token::PercentEquals),
            ("^", Token::Caret),
            ("^=", Token::CaretEquals),
            ("+", Token::Plus),
            ("++", Token::PlusPlus),
            ("+=", Token::PlusEquals),
            ("<", Token::LessThan),
            ("<<", Token::LessThanLessThan),
            ("<<=", Token::LessThanLessThanEquals),
            ("<=", Token::LessThanEquals),
            ("=", Token::Equals),
            ("==", Token::EqualsEquals),
            ("===", Token::EqualsEqualsEquals),
            ("=>", Token::EqualsGreaterThan),
            (">", Token::GreaterThan),
            (">=", Token::GreaterThanEquals),
            (">>", Token::GreaterThanGreaterThan),
            (">>=", Token::GreaterThanGreaterThanEquals),
            (">>>", Token::GreaterThanGreaterThanGreaterThan),
            (">>>", Token::GreaterThanGreaterThanGreaterThan),
            ("|", Token::Bar),
            ("|=", Token::BarEquals),
            ("||", Token::BarBar),
            ("||=", Token::BarBarEquals),
            ("~", Token::Tilde),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(lexer.token, test.1);
        }
    }
}
