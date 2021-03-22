use js_token::{lookup_identifer, TokenType};
use logger::Logger;

pub struct Lexer<'a> {
    input: String,
    /// The position of the current character
    current: usize,
    /// The start position of the current token
    start: usize,
    /// The end position of the current token
    end: usize,
    /// The next character to parsed
    character: Option<char>,
    /// The value of the currently parsed token.
    pub token_value: String,
    /// The currently parsed token
    pub token: TokenType,

    logger: &'a dyn Logger,
}

/// Public
impl<'a> Lexer<'a> {
    pub fn new<'b>(input: &str, logger: &'b impl Logger) -> Lexer<'b> {
        let mut lexer = Lexer {
            input: input.into(),
            token_value: String::new(),
            token: TokenType::EndOfFile,
            start: 0,
            current: 0,
            end: 0,
            character: input.chars().nth(0),
            logger,
        };

        lexer.next_token();
        return lexer;
    }

    /// Asserts that the current token matches the provided one
    pub fn expect_token(&self, token: TokenType) {
        if self.token != token {
            self.logger.add_error(
                &self.input,
                logger::Range {
                    start: self.start,
                    end: self.end,
                },
                format!("Expected \"{}\" but found \"{}\"", token, self.token),
            );
            std::process::exit(1);
        }
    }

    pub fn is_identifier_or_keyword(&self) -> bool {
        match &self.token {
            TokenType::Identifier => true,
            TokenType::Await => true,
            TokenType::As => true,
            TokenType::Break => true,
            TokenType::Case => true,
            TokenType::Catch => true,
            TokenType::Class => true,
            TokenType::Const => true,
            TokenType::Continue => true,
            TokenType::Debugger => true,
            TokenType::Default => true,
            TokenType::Delete => true,
            TokenType::Do => true,
            TokenType::Else => true,
            TokenType::Enum => true,
            TokenType::Export => true,
            TokenType::Extends => true,
            TokenType::From => true,
            TokenType::False => true,
            TokenType::Finally => true,
            TokenType::For => true,
            TokenType::Function => true,
            TokenType::Let => true,
            TokenType::If => true,
            TokenType::Import => true,
            TokenType::In => true,
            TokenType::Instanceof => true,
            TokenType::New => true,
            TokenType::Null => true,
            TokenType::Of => true,
            TokenType::Return => true,
            TokenType::Super => true,
            TokenType::Switch => true,
            TokenType::This => true,
            TokenType::Throw => true,
            TokenType::True => true,
            TokenType::Try => true,
            TokenType::Typeof => true,
            TokenType::Var => true,
            TokenType::Void => true,
            TokenType::While => true,
            TokenType::With => true,

            _ => false,
        }
    }

    /// Reports the current token as unexpected.
    /// Calls exit and will therefor never return.
    pub fn unexpected(&self) -> ! {
        self.logger.add_error(
            &self.input,
            logger::Range {
                start: self.start,
                end: self.end,
            },
            format!("Unexpected token \"{}\"", self.token),
        );
        std::process::exit(1);
    }

    pub fn next_token(&mut self) {
        self.skip_whitespace();

        self.start = self.end;

        let character = match self.character {
            Some(v) => v,
            None => {
                self.token = TokenType::EndOfFile;
                return;
            }
        };

        match character {
            '~' => {
                self.step();
                self.token = TokenType::Tilde;
            }
            '/' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::SlashEquals;
                } else {
                    self.token = TokenType::Slash;
                }
            }
            '.' => {
                self.step();
                if self.character == Some('.') {
                    self.step();
                    if self.character == Some('.') {
                        self.step();
                        self.token = TokenType::DotDotDot;
                    } else {
                        // Means we hit ".." but not "...",
                        // should this be an error?
                        self.token = TokenType::Dot;
                    }
                } else {
                    self.token = TokenType::Dot;
                }
            }
            '?' => {
                self.step();
                if self.character == Some('.') {
                    self.step();
                    self.token = TokenType::QuestionDot;
                } else if self.character == Some('?') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::QuestionQuestionEquals;
                    } else {
                        self.token = TokenType::QuestionQuestion;
                    }
                } else {
                    self.token = TokenType::Question;
                }
            }
            ';' => {
                self.step();
                self.token = TokenType::Semicolon;
            }
            '(' => {
                self.step();
                self.token = TokenType::OpenParen;
            }
            ')' => {
                self.step();
                self.token = TokenType::CloseParen;
            }
            '{' => {
                self.step();
                self.token = TokenType::OpenBrace;
            }
            '}' => {
                self.step();
                self.token = TokenType::CloseBrace;
            }
            ',' => {
                self.step();
                self.token = TokenType::Comma;
            }
            '+' => {
                self.step();
                if self.character == Some('+') {
                    self.step();
                    self.token = TokenType::PlusPlus;
                } else if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::PlusEquals;
                } else {
                    self.token = TokenType::Plus;
                }
            }
            '-' => {
                self.step();
                if self.character == Some('-') {
                    self.step();
                    self.token = TokenType::MinusMinus;
                } else if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::MinusEquals;
                } else {
                    self.token = TokenType::Minus;
                }
            }
            '*' => {
                self.step();
                if self.character == Some('*') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::AsteriskAsteriskEquals;
                    } else {
                        self.token = TokenType::AsteriskAsterisk;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::AsteriskEquals;
                } else {
                    self.token = TokenType::Asterisk;
                }
            }
            '<' => {
                self.step();
                if self.character == Some('<') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::LessThanLessThanEquals;
                    } else {
                        self.token = TokenType::LessThanLessThan;
                    }
                } else if self.character == Some('=') {
                    self.token = TokenType::LessThanEquals;
                } else {
                    self.token = TokenType::LessThan;
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
                            self.token = TokenType::GreaterThanGreaterThanGreaterThanEquals;
                        } else {
                            self.token = TokenType::GreaterThanGreaterThanGreaterThan;
                        }
                    } else if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::GreaterThanGreaterThanEquals;
                    } else {
                        self.token = TokenType::GreaterThanGreaterThan;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::GreaterThanEquals;
                } else {
                    self.token = TokenType::GreaterThan;
                }
            }
            '[' => {
                self.step();
                self.token = TokenType::OpenBracket;
            }
            ']' => {
                self.step();
                self.token = TokenType::CloseBracket;
            }
            '=' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::EqualsEqualsEquals;
                    } else {
                        self.token = TokenType::EqualsEquals;
                    }
                } else if self.character == Some('>') {
                    self.step();
                    self.token = TokenType::EqualsGreaterThan;
                } else {
                    self.token = TokenType::Equals;
                }
            }
            '!' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::ExclamationEqualsEquals;
                    } else {
                        self.token = TokenType::ExclamationEquals;
                    }
                } else {
                    self.token = TokenType::Exclamation;
                }
            }
            '%' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::PercentEquals;
                } else {
                    self.token = TokenType::Percent;
                }
            }
            ':' => {
                self.step();
                self.token = TokenType::Colon;
            }
            '|' => {
                self.step();
                if self.character == Some('|') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::BarBarEquals;
                    } else {
                        self.token = TokenType::BarBar;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::BarEquals;
                } else {
                    self.token = TokenType::Bar;
                }
            }
            '@' => {
                self.step();
                self.token = TokenType::At;
            }
            '^' => {
                self.step();
                if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::CaretEquals;
                } else {
                    self.token = TokenType::Caret;
                }
            }
            '&' => {
                self.step();
                if self.character == Some('&') {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = TokenType::AmpersandAmpersandEquals;
                    } else {
                        self.token = TokenType::AmpersandAmpersand;
                    }
                } else if self.character == Some('=') {
                    self.step();
                    self.token = TokenType::AmpersandEquals;
                } else {
                    self.token = TokenType::Ampersand;
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
                self.token_value = literal;
                self.token = TokenType::StringLiteral;
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
                self.token_value = literal;
                self.token = TokenType::StringLiteral;
            }

            c if Lexer::is_letter(c) => {
                let identifier = self.read_identifier();
                self.token = lookup_identifer(&identifier);
                self.token_value = identifier;
            }

            c if Lexer::is_digit(c) => {
                let number = self.read_number();
                self.token_value = number;
                self.token = TokenType::NumericLiteral;
            }

            _ => {
                self.step();
                self.token = TokenType::Illegal;
            }
        };
    }
}

/// Internal
impl<'a> Lexer<'a> {
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
