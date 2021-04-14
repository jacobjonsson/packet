use crate::TokenKind;
use crate::{Lexer, LexerResult};

/// Implementations of scanning functions for the punctuators used in JavaScript.
///
/// See [spec](https://tc39.es/ecma262/#sec-punctuators)
impl<'a> Lexer<'a> {
    /// ~
    pub(crate) fn scan_tilde(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::Tilde
    }

    /// (
    pub(crate) fn scan_open_paren(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::OpenParen
    }

    /// )
    pub(crate) fn scan_close_paren(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::CloseParen
    }

    /// {
    pub(crate) fn scan_open_brace(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::OpenBrace
    }

    /// }
    pub(crate) fn scan_close_brace(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::CloseBrace
    }

    /// [
    pub(crate) fn scan_open_bracket(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::OpenBracket
    }

    /// ]
    pub(crate) fn scan_close_bracket(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::CloseBracket
    }

    /// ;
    pub(crate) fn scan_semicolon(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::Semicolon
    }

    /// :
    pub(crate) fn scan_colon(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::Colon
    }

    /// :
    pub(crate) fn scan_comma(&mut self) -> TokenKind {
        self.index += 1;
        TokenKind::Comma
    }

    /// /, /=
    pub(crate) fn scan_slash(&mut self) -> TokenKind {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            TokenKind::SlashEquals
        } else {
            TokenKind::Slash
        }
    }

    /// &, &=, &&, &&=
    pub(crate) fn scan_ampersand(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('&') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::AmpersandAmpersandEquals
            } else {
                TokenKind::AmpersandAmpersand
            }
        } else if character == Some('=') {
            self.index += 1;
            TokenKind::AmpersandEquals
        } else {
            TokenKind::Ampersand
        }
    }

    /// -, --, -=
    pub(crate) fn scan_minus(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('-') {
            self.index += 1;
            TokenKind::MinusMinus
        } else if character == Some('=') {
            self.index += 1;
            TokenKind::MinusEquals
        } else {
            TokenKind::Minus
        }
    }

    /// !, !=, !==
    pub(crate) fn scan_exclamation(&mut self) -> TokenKind {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::ExclamationEqualsEquals
            } else {
                TokenKind::ExclamationEquals
            }
        } else {
            TokenKind::Exclamation
        }
    }

    /// ?, ?., ??, ??=
    pub(crate) fn scan_question_mark(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('?') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::QuestionQuestionEquals
            } else {
                TokenKind::QuestionQuestion
            }
        } else if character == Some('.') {
            TokenKind::QuestionDot
        } else {
            TokenKind::Question
        }
    }

    /// ., ..., .123
    pub(crate) fn scan_dot(&mut self) -> LexerResult<TokenKind> {
        let next = self.next_character();
        if next >= Some('0') && next <= Some('9') {
            self.scan_floating_point()
        } else if next == Some('.') {
            self.index += 3;
            Ok(TokenKind::DotDotDot)
        } else {
            self.index += 1;
            Ok(TokenKind::Dot)
        }
    }

    /// *, *=, **, **=
    pub(crate) fn scan_asterisk(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('*') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::AsteriskAsteriskEquals
            } else {
                TokenKind::AsteriskAsterisk
            }
        } else if character == Some('=') {
            self.index += 1;
            TokenKind::AsteriskEquals
        } else {
            TokenKind::Asterisk
        }
    }

    /// %, %=
    pub(crate) fn scan_percent(&mut self) -> TokenKind {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            TokenKind::PercentEquals
        } else {
            TokenKind::Percent
        }
    }

    /// ^, ^=
    pub(crate) fn scan_caret(&mut self) -> TokenKind {
        self.index += 1;
        if self.current_character() == Some('=') {
            self.index += 1;
            TokenKind::CaretEquals
        } else {
            TokenKind::Caret
        }
    }

    /// +, ++, +=
    pub(crate) fn scan_plus(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('=') {
            self.index += 1;
            TokenKind::PlusEquals
        } else if character == Some('+') {
            self.index += 1;
            TokenKind::PlusPlus
        } else {
            TokenKind::Plus
        }
    }

    /// <, <<, <<=, <=
    pub(crate) fn scan_less_than(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('<') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::LessThanLessThanEquals
            } else {
                TokenKind::LessThanLessThan
            }
        } else if character == Some('=') {
            self.index += 1;
            TokenKind::LessThanEquals
        } else {
            TokenKind::LessThan
        }
    }

    /// =, ==, ===, =>
    pub(crate) fn scan_equals(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('=') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::EqualsEqualsEquals
            } else {
                TokenKind::EqualsEquals
            }
        } else if character == Some('>') {
            self.index += 1;
            TokenKind::EqualsGreaterThan
        } else {
            TokenKind::Equals
        }
    }

    /// >, >=, >>, >>=, >>>, >>>=
    pub(crate) fn scan_greater_than(&mut self) -> TokenKind {
        self.index += 1;
        let mut character = self.current_character();
        if character == Some('>') {
            self.index += 1;
            character = self.current_character();
            if character == Some('>') {
                self.index += 1;
                if self.current_character() == Some('=') {
                    self.index += 1;
                    TokenKind::GreaterThanGreaterThanGreaterThanEquals
                } else {
                    TokenKind::GreaterThanGreaterThanGreaterThan
                }
            } else if character == Some('=') {
                self.index += 1;
                TokenKind::GreaterThanGreaterThanEquals
            } else {
                TokenKind::GreaterThanGreaterThan
            }
        } else if character == Some('=') {
            self.index += 1;
            TokenKind::GreaterThanEquals
        } else {
            TokenKind::GreaterThan
        }
    }

    /// |, |=, ||, ||=
    pub(crate) fn scan_bar(&mut self) -> TokenKind {
        self.index += 1;
        let character = self.current_character();
        if character == Some('|') {
            self.index += 1;
            if self.current_character() == Some('=') {
                self.index += 1;
                TokenKind::BarBarEquals
            } else {
                TokenKind::BarBar
            }
        } else if character == Some('=') {
            self.index += 1;
            TokenKind::BarEquals
        } else {
            TokenKind::Bar
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punctuators() {
        let tests = vec![
            ("--", TokenKind::MinusMinus),
            ("-", TokenKind::Minus),
            ("-=", TokenKind::MinusEquals),
            (",", TokenKind::Comma),
            (";", TokenKind::Semicolon),
            (":", TokenKind::Colon),
            ("!", TokenKind::Exclamation),
            ("!=", TokenKind::ExclamationEquals),
            ("!==", TokenKind::ExclamationEqualsEquals),
            ("??", TokenKind::QuestionQuestion),
            ("??=", TokenKind::QuestionQuestionEquals),
            ("?.", TokenKind::QuestionDot),
            ("?", TokenKind::Question),
            ("...", TokenKind::DotDotDot),
            (".", TokenKind::Dot),
            ("(", TokenKind::OpenParen),
            (")", TokenKind::CloseParen),
            ("[", TokenKind::OpenBracket),
            ("]", TokenKind::CloseBracket),
            ("{", TokenKind::OpenBrace),
            ("}", TokenKind::CloseBrace),
            ("*", TokenKind::Asterisk),
            ("**", TokenKind::AsteriskAsterisk),
            ("**=", TokenKind::AsteriskAsteriskEquals),
            ("*=", TokenKind::AsteriskEquals),
            ("/", TokenKind::Slash),
            ("/=", TokenKind::SlashEquals),
            ("&", TokenKind::Ampersand),
            ("&&", TokenKind::AmpersandAmpersand),
            ("&&=", TokenKind::AmpersandAmpersandEquals),
            ("&=", TokenKind::AmpersandEquals),
            ("%", TokenKind::Percent),
            ("%=", TokenKind::PercentEquals),
            ("^", TokenKind::Caret),
            ("^=", TokenKind::CaretEquals),
            ("+", TokenKind::Plus),
            ("++", TokenKind::PlusPlus),
            ("+=", TokenKind::PlusEquals),
            ("<", TokenKind::LessThan),
            ("<<", TokenKind::LessThanLessThan),
            ("<<=", TokenKind::LessThanLessThanEquals),
            ("<=", TokenKind::LessThanEquals),
            ("=", TokenKind::Equals),
            ("==", TokenKind::EqualsEquals),
            ("===", TokenKind::EqualsEqualsEquals),
            ("=>", TokenKind::EqualsGreaterThan),
            (">", TokenKind::GreaterThan),
            (">=", TokenKind::GreaterThanEquals),
            (">>", TokenKind::GreaterThanGreaterThan),
            (">>=", TokenKind::GreaterThanGreaterThanEquals),
            (">>>", TokenKind::GreaterThanGreaterThanGreaterThan),
            (">>>", TokenKind::GreaterThanGreaterThanGreaterThan),
            ("|", TokenKind::Bar),
            ("|=", TokenKind::BarEquals),
            ("||", TokenKind::BarBar),
            ("||=", TokenKind::BarBarEquals),
            ("~", TokenKind::Tilde),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next().unwrap().kind, test.1);
        }
    }
}
