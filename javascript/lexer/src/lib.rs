use javascript_token::{lookup_identifer, Token};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    character: Option<char>,
}

/// Public
impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.into(),
            position: 0,
            read_position: 0,
            character: input.chars().nth(0),
        };
        lexer.read_character();
        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let character = match self.character {
            Some(v) => v,
            None => return Token::EndOfFile,
        };

        let token = match character {
            '~' => Token::Tilde,
            '/' => {
                if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::SlashEquals
                } else {
                    Token::Slash
                }
            }
            '.' => {
                if self.peek_character() == Some('.') {
                    self.read_character();
                    if self.peek_character() == Some('.') {
                        self.read_character();
                        Token::DotDotDot
                    } else {
                        // Means we hit ".." but not "...",
                        // should this be an error?
                        Token::Dot
                    }
                } else {
                    Token::Dot
                }
            }
            '?' => {
                if self.peek_character() == Some('.') {
                    self.read_character();
                    Token::QuestionDot
                } else if self.peek_character() == Some('?') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::QuestionQuestionEquals
                    } else {
                        Token::QuestionQuestion
                    }
                } else {
                    Token::Question
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ',' => Token::Comma,
            '+' => {
                if self.peek_character() == Some('+') {
                    self.read_character();
                    Token::PlusPlus
                } else if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::PlusEquals
                } else {
                    Token::Plus
                }
            }
            '-' => {
                if self.peek_character() == Some('-') {
                    self.read_character();
                    Token::MinusMinus
                } else if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::MinusEquals
                } else {
                    Token::Minus
                }
            }
            '*' => {
                if self.peek_character() == Some('*') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::AsteriskAsteriskEquals
                    } else {
                        Token::AsteriskAsterisk
                    }
                } else {
                    Token::Asterisk
                }
            }
            '<' => {
                if self.peek_character() == Some('<') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::LessThanLessThanEquals
                    } else {
                        Token::LessThanLessThan
                    }
                } else if self.peek_character() == Some('=') {
                    Token::LessThanEquals
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                if self.peek_character() == Some('>') {
                    self.read_character();
                    if self.peek_character() == Some('>') {
                        self.read_character();
                        if self.peek_character() == Some('=') {
                            self.read_character();
                            Token::GreaterThanGreaterThanGreaterThanEquals
                        } else {
                            Token::GreaterThanGreaterThanGreaterThan
                        }
                    } else if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::GreaterThanGreaterThanEquals
                    } else {
                        Token::GreaterThanGreaterThan
                    }
                } else if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::GreaterThanEquals
                } else {
                    Token::GreaterThan
                }
            }
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '=' => {
                if self.peek_character() == Some('=') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::EqualsEqualsEquals
                    } else {
                        Token::EqualsEquals
                    }
                } else if self.peek_character() == Some('>') {
                    self.read_character();
                    Token::EqualsGreaterThan
                } else {
                    Token::Equals
                }
            }
            '!' => {
                if self.peek_character() == Some('=') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::ExclamationEqualsEquals
                    } else {
                        Token::ExclamationEquals
                    }
                } else {
                    Token::Exclamation
                }
            }
            '%' => {
                if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::PercentEquals
                } else {
                    Token::Percent
                }
            }
            ':' => Token::Colon,
            '|' => {
                if self.peek_character() == Some('|') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::BarBarEquals
                    } else {
                        Token::BarBar
                    }
                } else if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::BarEquals
                } else {
                    Token::Bar
                }
            }
            '@' => Token::At,
            '^' => {
                if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::CaretEquals
                } else {
                    Token::Caret
                }
            }
            '&' => {
                if self.peek_character() == Some('&') {
                    self.read_character();
                    if self.peek_character() == Some('=') {
                        self.read_character();
                        Token::AmpersandAmpersandEquals
                    } else {
                        Token::AmpersandAmpersand
                    }
                } else if self.peek_character() == Some('=') {
                    self.read_character();
                    Token::AmpersandEquals
                } else {
                    Token::Ampersand
                }
            }

            '"' => {
                let mut literal = String::new();
                while self.peek_character() != Some('"') {
                    self.read_character();
                    let character = match self.character {
                        Some(c) => c,
                        None => break, // Means we failed to read the character, prob because of EndOfFile. (We should ideally return an error here)
                    };

                    literal.push(character);
                }
                // Consume the ending "
                self.read_character();

                return Token::StringLiteral(literal);
            }

            '\'' => {
                let mut literal = String::new();
                while self.peek_character() != Some('\'') {
                    self.read_character();
                    let character = match self.character {
                        Some(c) => c,
                        None => break, // Means we failed to read the character, prob because of EndOfFile. (We should ideally return an error here)
                    };

                    literal.push(character);
                }
                // Consume the ending '
                self.read_character();

                return Token::StringLiteral(literal);
            }

            c if Lexer::is_letter(c) => {
                let identifier = self.read_identifier();
                return lookup_identifer(&identifier);
            }

            c if Lexer::is_digit(c) => {
                let number = self.read_number();
                return Token::NumericLiteral(number);
            }

            _ => Token::Illegal,
        };

        self.read_character();
        return token;
    }
}

/// Internal
impl Lexer {
    fn read_character(&mut self) {
        self.character = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.character {
            match character {
                ' ' | '\t' | '\n' | '\r' => self.read_character(),
                _ => break,
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut word = String::new();
        while let Some(ch) = self.character {
            if Lexer::is_letter(ch) {
                word.push(ch);
                self.read_character();
            } else {
                break;
            }
        }
        return word;
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(ch) = self.character {
            if Lexer::is_digit(ch) {
                number.push(ch);
                self.read_character();
            } else {
                break;
            }
        }
        return number;
    }

    fn peek_character(&self) -> Option<char> {
        return self.input.chars().nth(self.read_position);
    }

    fn is_letter(character: char) -> bool {
        return character.is_alphabetic() || character == '_';
    }

    fn is_digit(character: char) -> bool {
        return character.is_numeric();
    }
}
