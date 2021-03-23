use js_token::{lookup_identifer, Token};
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
    /// The number of the currently parsed token.
    pub number: f64,
    /// The currently parsed token
    pub token: Token,

    logger: &'a dyn Logger,
}

/// Public
impl<'a> Lexer<'a> {
    pub fn new<'b>(input: &str, logger: &'b impl Logger) -> Lexer<'b> {
        let mut lexer = Lexer {
            input: input.into(),
            token_value: String::new(),
            number: 0.,
            token: Token::EndOfFile,
            start: 0,
            current: 0,
            end: 0,
            character: input.chars().nth(0),
            logger,
        };

        lexer.step();
        lexer.next_token();
        return lexer;
    }

    /// Asserts that the current token matches the provided one
    pub fn expect_token(&self, token: Token) {
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
            Token::Identifier => true,
            Token::Await => true,
            Token::As => true,
            Token::Break => true,
            Token::Case => true,
            Token::Catch => true,
            Token::Class => true,
            Token::Const => true,
            Token::Continue => true,
            Token::Debugger => true,
            Token::Default => true,
            Token::Delete => true,
            Token::Do => true,
            Token::Else => true,
            Token::Enum => true,
            Token::Export => true,
            Token::Extends => true,
            Token::From => true,
            Token::False => true,
            Token::Finally => true,
            Token::For => true,
            Token::Function => true,
            Token::Let => true,
            Token::If => true,
            Token::Import => true,
            Token::In => true,
            Token::Instanceof => true,
            Token::New => true,
            Token::Null => true,
            Token::Of => true,
            Token::Return => true,
            Token::Super => true,
            Token::Switch => true,
            Token::This => true,
            Token::Throw => true,
            Token::True => true,
            Token::Try => true,
            Token::Typeof => true,
            Token::Var => true,
            Token::Void => true,
            Token::While => true,
            Token::With => true,

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

    // Returns the raw slice of input related to the current token.
    pub fn raw(&self) -> String {
        self.input[self.start..self.end].into()
    }

    pub fn scan_regexp(&mut self) {
        loop {
            match self.character {
                Some('/') => {
                    self.step();
                    'inner: loop {
                        let character = match self.character {
                            Some(v) => v,
                            None => break 'inner,
                        };

                        if Lexer::is_letter(character) {
                            match character {
                                'g' | 'i' | 'm' | 's' | 'u' | 'y' => self.step(),
                                _ => self.unexpected(),
                            }
                        } else {
                            break 'inner;
                        }
                    }

                    return;
                }

                Some('[') => {
                    self.step();
                    while self.character != Some(']') {
                        if self.character == Some('\\') {
                            self.step();
                        }

                        match self.character {
                            Some('\r') | Some('\n') | None => {
                                self.unexpected();
                            }
                            _ => self.step(),
                        };
                    }
                    self.step();
                }

                _ => {
                    if self.character == Some('\\') {
                        self.step();
                    }

                    match self.character {
                        Some('\r') | Some('\n') | None => {
                            self.unexpected();
                        }
                        _ => self.step(),
                    };
                }
            };
        }
    }

    pub fn next_token(&mut self) {
        loop {
            self.start = self.end;

            let character = match self.character {
                Some(v) => v,
                None => {
                    self.token = Token::EndOfFile;
                    return;
                }
            };

            match character {
                ' ' | '\t' | '\n' | '\r' => {
                    self.step();
                    continue;
                }
                '~' => {
                    self.step();
                    self.token = Token::Tilde;
                }
                '/' => {
                    self.step();
                    if self.character == Some('=') {
                        self.step();
                        self.token = Token::SlashEquals;
                    } else if self.character == Some('/') {
                        'single_line_comment: loop {
                            self.step();
                            if self.character == Some('\n') || self.character == Some('\r') {
                                break 'single_line_comment;
                            } else if self.character == None {
                                break 'single_line_comment;
                            }
                        }
                        self.next_token();
                    } else if self.character == Some('*') {
                        'multi_line_comment: loop {
                            self.step();
                            if self.character == Some('*') {
                                self.step();
                                if self.character == Some('/') {
                                    self.step();
                                    break 'multi_line_comment;
                                }
                            } else if self.character == None {
                                break 'multi_line_comment;
                            }
                        }
                        self.next_token();
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
                    } else if self.character == Some('=') {
                        self.step();
                        self.token = Token::AsteriskEquals;
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
                        self.step();
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

                '"' | '\'' => {
                    let quote = self.character.clone();
                    self.step();

                    'string_literal: loop {
                        if self.character == quote {
                            self.step();
                            break 'string_literal;
                        }

                        match self.character {
                            Some('\\') => self.step(),
                            None => self.unexpected(),
                            _ => {}
                        }
                        self.step();
                    }

                    self.token_value = self.input[self.start + 1..self.end - 1].into();
                    self.token = Token::StringLiteral;
                }

                c if Lexer::is_letter(c) => {
                    let identifier = self.read_identifier();
                    self.token = lookup_identifer(&identifier);
                    self.token_value = identifier;
                }

                c if Lexer::is_digit(c) => {
                    let number = self.read_number();
                    self.number = number.parse::<f64>().expect("Failed to parse number");
                    self.token = Token::NumericLiteral;
                }

                _ => {
                    self.step();
                    self.token = Token::Illegal;
                }
            };

            return;
        }
    }
}

/// Internal
impl<'a> Lexer<'a> {
    fn step(&mut self) {
        self.character = self.input.chars().nth(self.current);
        self.end = self.current;
        self.current += 1;
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
        return character.is_alphabetic() || character == '_' || character == '$';
    }

    fn is_digit(character: char) -> bool {
        return character.is_numeric();
    }
}
