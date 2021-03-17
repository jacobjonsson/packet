use javascript_token::{lookup_identifer, Token};

pub struct Lexer {
    input: String,
    /// The position of the current character
    current: usize,
    /// The end position of current token
    end: usize,
    /// The next character to parsed
    character: Option<char>,
    /// The currently parsed token
    pub token: Token,
}

/// Public
impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.into(),
            token: Token::EndOfFile,
            current: 0,
            end: 0,
            character: input.chars().nth(0),
        };

        lexer.next_token();
        return lexer;
    }

    pub fn next_token(&mut self) {
        self.skip_whitespace();

        let character = match self.character {
            Some(v) => v,
            None => {
                self.token = Token::EndOfFile;
                return;
            }
        };

        match character {
            '~' => {
                self.step();
                self.token = Token::Tilde;
            }
            '/' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    self.token = Token::SlashEquals;
                } else {
                    self.token = Token::Slash;
                }
            }
            '.' => {
                self.step();
                if self.character == Some('.') {
                    self.step();
                    if self.character == Some('.') {
                        self.step();
                        self.token = Token::DotDotDot;
                    } else {
                        // Means we hit ".." but not "...",
                        // should this be an error?
                        self.token = Token::Dot;
                    }
                } else {
                    self.token = Token::Dot;
                }
            }
            '?' => {
                self.step();
                if self.character == Some('.') {
                    self.step();
                    self.token = Token::QuestionDot;
                } else if self.character == Some('?') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::QuestionQuestionEquals;
                    } else {
                        self.token = Token::QuestionQuestion;
                    }
                } else {
                    self.token = Token::Question;
                }
            }
            ';' => {
                self.step();
                self.token = Token::Semicolon;
            }
            '(' => {
                self.step();
                self.token = Token::OpenParen;
            }
            ')' => {
                self.step();
                self.token = Token::CloseParen;
            }
            '{' => {
                self.step();
                self.token = Token::OpenBrace;
            }
            '}' => {
                self.step();
                self.token = Token::CloseBrace;
            }
            ',' => {
                self.step();
                self.token = Token::Comma;
            }
            '+' => {
                self.step();
                if self.character == Some('+') {
                    self.step();
                    self.token = Token::PlusPlus;
                } else if self.character == Some('=') {
                    self.step();
                    self.token = Token::PlusEquals;
                } else {
                    self.token = Token::Plus;
                }
            }
            '-' => {
                self.step();
                if self.character == Some('-') {
                    self.step();
                    self.token = Token::MinusMinus;
                } else if self.character == Some('=') {
                    self.step();
                    self.token = Token::MinusEquals;
                } else {
                    self.token = Token::Minus;
                }
            }
            '*' => {
                self.step();
                if self.character == Some('*') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::AsteriskAsteriskEquals;
                    } else {
                        self.token = Token::AsteriskAsterisk;
                    }
                } else {
                    self.token = Token::Asterisk;
                }
            }
            '<' => {
                self.step();
                if self.character == Some('<') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::LessThanLessThanEquals;
                    } else {
                        self.token = Token::LessThanLessThan;
                    }
                } else if self.character == Some('=') {
                    self.token = Token::LessThanEquals;
                } else {
                    self.token = Token::LessThan;
                }
            }
            '>' => {
                self.step();
                if self.character == Some('>') {
                    self.step();
                    if self.character == Some('>') {
                        self.step();
                        if self.character == Some('=') {
                            self.step();
                            self.token = Token::GreaterThanGreaterThanGreaterThanEquals;
                        } else {
                            self.token = Token::GreaterThanGreaterThanGreaterThan;
                        }
                    } else if self.character == Some('=') {
                        self.step();
                        self.token = Token::GreaterThanGreaterThanEquals;
                    } else {
                        self.token = Token::GreaterThanGreaterThan;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = Token::GreaterThanEquals;
                } else {
                    self.token = Token::GreaterThan;
                }
            }
            '[' => {
                self.step();
                self.token = Token::OpenBracket;
            }
            ']' => {
                self.step();
                self.token = Token::CloseBracket;
            }
            '=' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::EqualsEqualsEquals;
                    } else {
                        self.token = Token::EqualsEquals;
                    }
                } else if self.character == Some('>') {
                    self.step();
                    self.token = Token::EqualsGreaterThan;
                } else {
                    self.token = Token::Equals;
                }
            }
            '!' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::ExclamationEqualsEquals;
                    } else {
                        self.token = Token::ExclamationEquals;
                    }
                } else {
                    self.token = Token::Exclamation;
                }
            }
            '%' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    self.token = Token::PercentEquals;
                } else {
                    self.token = Token::Percent;
                }
            }
            ':' => {
                self.step();
                self.token = Token::Colon;
            }
            '|' => {
                self.step();
                if self.character == Some('|') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::BarBarEquals;
                    } else {
                        self.token = Token::BarBar;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = Token::BarEquals;
                } else {
                    self.token = Token::Bar;
                }
            }
            '@' => {
                self.step();
                self.token = Token::At;
            }
            '^' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    self.token = Token::CaretEquals;
                } else {
                    self.token = Token::Caret;
                }
            }
            '&' => {
                self.step();
                if self.character == Some('&') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::AmpersandAmpersandEquals;
                    } else {
                        self.token = Token::AmpersandAmpersand;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = Token::AmpersandEquals;
                } else {
                    self.token = Token::Ampersand;
                }
            }

            '"' => {
                self.step();
                let mut literal = String::new();
                while self.character != Some('"') {
                    if let Some(character) = self.character {
                        literal.push(character);
                        self.step();
                    } else {
                        break;
                    }
                }
                // Consume the ending "
                self.step();

                self.token = Token::StringLiteral(literal);
            }

            '\'' => {
                self.step();
                let mut literal = String::new();
                while self.character != Some('\'') {
                    if let Some(character) = self.character {
                        literal.push(character);
                        self.step();
                    } else {
                        break;
                    }
                }
                // Consume the ending '
                self.step();

                self.token = Token::StringLiteral(literal);
            }

            c if Lexer::is_letter(c) => {
                let identifier = self.read_identifier();
                self.token = lookup_identifer(&identifier);
            }

            c if Lexer::is_digit(c) => {
                let number = self.read_number();
                self.token = Token::NumericLiteral(number);
            }

            _ => {
                self.step();
                self.token = Token::Illegal;
            }
        };
    }
}

/// Internal
impl Lexer {
    fn step(&mut self) {
        self.end = self.current;
        self.current += 1;
        self.character = self.input.chars().nth(self.current);
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.character {
            match character {
                ' ' | '\t' | '\n' | '\r' => self.step(),
                _ => break,
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut word = String::new();
        while let Some(character) = self.character {
            if Lexer::is_letter(character) || Lexer::is_digit(character) {
                word.push(character);
                self.step();
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
                self.step();
            } else {
                break;
            }
        }
        return number;
    }

    fn is_letter(character: char) -> bool {
        return character.is_alphabetic() || character == '_';
    }

    fn is_digit(character: char) -> bool {
        return character.is_numeric();
    }
}
