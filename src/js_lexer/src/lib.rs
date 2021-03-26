use std::{iter::Peekable, str::Chars};

use js_token::{lookup_identifer, Token};
use logger::Logger;

pub struct Lexer<'a> {
    input: String,
    chars: Peekable<Chars<'a>>,
    /// The position of the current character
    current: usize,
    /// The start position of the current token
    start: usize,
    /// The end position of the current token
    end: usize,
    /// The next character to parsed
    character: Option<char>,
    /// The value of the currently parsed string or identifier.
    pub identifier: String,
    /// The number of the currently parsed token.
    pub number: f64,
    /// The currently parsed token
    pub token: Token,

    logger: &'a dyn Logger,
}

/// Public
impl<'a> Lexer<'a> {
    pub fn new<'b>(input: &'b str, logger: &'b impl Logger) -> Lexer<'b> {
        let mut lexer = Lexer {
            input: input.into(),
            identifier: String::new(),
            number: 0.,
            token: Token::EndOfFile,
            start: 0,
            current: 0,
            end: 0,
            chars: input.chars().peekable(),
            character: None,
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

            // Skip over comments
            if self.character == Some('/') {
                // Single line comment
                if self.peek() == Some(&'/') {
                    self.step();
                    'single_line_comment: loop {
                        match self.character {
                            Some('\n') | Some('\r') => {
                                self.step();
                                break 'single_line_comment;
                            }
                            None => {
                                self.step();
                                break 'single_line_comment;
                            }
                            _ => self.step(),
                        }
                    }
                } else if self.peek() == Some(&'*') {
                    // Multi line comment
                    self.step();

                    'multi_line_comment: loop {
                        match self.character {
                            Some('*') => {
                                self.step();
                                if self.character == Some('/') {
                                    self.step();
                                    break 'multi_line_comment;
                                }
                            }
                            None => panic!("File ended without terminating multi-line comment"),
                            _ => self.step(),
                        }
                    }
                }
            }

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
                    } else {
                        self.token = Token::Slash;
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

                    self.identifier = self.input[self.start + 1..self.end - 1].into();
                    self.token = Token::StringLiteral;
                }

                '.' => {
                    if self.peek() == Some(&'.') {
                        self.step();
                        self.step();
                        self.step();
                        self.token = Token::DotDotDot;
                    } else if self.peek() >= Some(&'0') && self.peek() <= Some(&'9') {
                        self.read_number();
                    } else {
                        self.step();
                        self.token = Token::Dot;
                    }
                }

                '0' => match self.peek() {
                    Some(&'b') => self.read_radix_number(2),
                    Some(&'o') => self.read_radix_number(8),
                    Some(&'x') => self.read_radix_number(16),
                    _ => self.read_number(),
                },

                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.read_number();
                }

                c if Lexer::is_letter(c) => {
                    let identifier = self.read_identifier();
                    self.token = lookup_identifer(&identifier);
                    self.identifier = identifier;
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
        self.character = self.chars.next();
        self.end = self.current;
        self.current += 1;
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn read_identifier(&mut self) -> String {
        let mut word = String::new();
        while let Some(character) = self.character {
            if Lexer::is_letter(character) || character.is_digit(10) {
                word.push(character);
                self.step();
            } else {
                break;
            }
        }
        return word;
    }

    fn is_letter(character: char) -> bool {
        return character.is_alphabetic() || character == '_' || character == '$';
    }

    fn read_number(&mut self) {
        // 00
        if self.character == Some('0') && self.peek() == Some(&'0') {
            panic!("Legacy octal literals are not supported in strict mode");
        }

        // Means we've hit a fractal number .012
        if self.character == Some('.') {
            self.step();
            let number = self.read_decimal_number();

            // Exponent
            if self.character == Some('e') || self.character == Some('E') {
                todo!()
            }

            self.token = Token::NumericLiteral;
            self.number = format!("0.{}", number)
                .parse::<f64>()
                .expect(&format!("Failed to parse .{} into an f64", number));

            return;
        }

        let mut number = self.read_decimal_number();

        // Exponent
        if self.character == Some('e') || self.character == Some('E') {
            todo!()
        }

        // Fractal 1.1
        if self.character == Some('.') {
            self.step();
            number = format!("{}.{}", number, self.read_decimal_number());
        }

        // BitInt
        if self.character == Some('n') {
            self.step();
            self.token = Token::BigIntegerLiteral;
            self.identifier = number;
            return;
        }

        self.token = Token::NumericLiteral;
        self.number = number
            .parse::<f64>()
            .expect(&format!("Failed to parse .{} into an f64", number));
    }

    /// Reads a radix number (0b, 0x, 0o)
    fn read_radix_number(&mut self, radix: u32) {
        self.step(); // 0
        self.step(); // x/b/o

        let number = match radix {
            2 => self.read_binary_number(),
            8 => self.read_octal_number(),
            16 => self.read_hexadecimal_number(),

            _ => self.unexpected(),
        };

        // Exponent
        if self.character == Some('e') || self.character == Some('E') {
            todo!()
        }

        // Means we've hit a big int literal
        // We do not attempt to convert the string into
        // a number since that could mean precision loss.
        if self.character == Some('n') {
            self.step();
            self.token = Token::BigIntegerLiteral;
            self.identifier = match radix {
                2 => format!("0b{}", number),
                8 => format!("0o{}", number),
                16 => format!("0x{}", number),
                _ => self.unexpected(),
            };
            return;
        }

        self.number = i64::from_str_radix(&number, radix)
            .expect(&format!("[Packet]: Failed to convert {} to an i64", number))
            as f64;
        self.token = Token::NumericLiteral;
    }

    fn read_binary_number(&mut self) -> String {
        let mut num = String::new();
        loop {
            match self.character {
                Some('0') | Some('1') => num.push(self.character.unwrap()),
                Some('_') => {}
                _ => break,
            }
            self.step();
        }
        return num;
    }

    fn read_octal_number(&mut self) -> String {
        let mut num = String::new();
        loop {
            match self.character {
                Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5')
                | Some('6') | Some('7') | Some('8') => num.push(self.character.unwrap()),
                Some('_') => {}
                _ => break,
            }
            self.step();
        }
        return num;
    }

    fn read_decimal_number(&mut self) -> String {
        let mut num = String::new();
        loop {
            match self.character {
                Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5')
                | Some('6') | Some('7') | Some('8') | Some('9') => {
                    num.push(self.character.unwrap())
                }
                Some('_') => {}
                _ => break,
            }
            self.step();
        }
        return num;
    }

    fn read_hexadecimal_number(&mut self) -> String {
        let mut num = String::new();
        loop {
            match self.character {
                Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5')
                | Some('6') | Some('7') | Some('8') | Some('9') | Some('A') | Some('B')
                | Some('C') | Some('D') | Some('E') | Some('F') | Some('a') | Some('b')
                | Some('c') | Some('d') | Some('e') | Some('f') => {
                    num.push(self.character.unwrap())
                }
                Some('_') => {}
                _ => break,
            }
            self.step();
        }
        return num;
    }
}
