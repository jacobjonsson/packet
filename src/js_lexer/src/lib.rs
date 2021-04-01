/// This file contains the code for the lexer.
/// The lexers responsibility is to convert one or more chars
/// into tokens that will later be used by the parser to construct
/// ast nodes. The lexer only parses one token at a time and will
/// need to be called iteratively to parse the next token in contrast
/// to lexer that parses the entire file at once. The reason for this is
/// because some tokens will need to scanned differently depending on the context.
/// For example, the < token will, in a normal context, be scanned as a Token::LessThan
/// while in a jsx context it would be scanned as the start of a jsx token.
/// The same logic applies to regex's, this sequence: /abc/ would in a normal context
/// be scanned as Slash - Identifier - Slash while it actually should be scanned as a
/// regex. And since the scanner itself is not aware of the parsing rules
/// the context will need to be determined by the parser whom will need to
/// call the lexer differently depending on the context.
use std::str::Chars;

use js_token::{lookup_identifer, Token};
use logger::Logger;
mod unicode;

/// This means we've hit the end of the file
pub const EOF_CHAR: char = '\0';

pub struct Lexer<'a, L: Logger> {
    input: &'a str,
    chars: Chars<'a>,
    /// The position of the current character
    current: usize,
    /// The start of the current token
    start: usize,
    /// The end of the current token
    end: usize,
    /// The next character to parsed
    character: char,
    /// The value of the currently parsed string or identifier.
    pub identifier: String,
    /// The number of the currently parsed token.
    pub number: f64,
    /// The currently parsed token
    pub token: Token,

    logger: &'a L,
}

/// Public
impl<'a, L: Logger> Lexer<'a, L> {
    /// Creates a new lexer
    pub fn new(input: &'a str, logger: &'a L) -> Lexer<'a, L> {
        let mut lexer = Lexer {
            input,
            identifier: String::new(),
            number: 0.,
            token: Token::EndOfFile,
            start: 0,
            current: 0,
            end: 0,
            chars: input.chars(),
            character: EOF_CHAR,
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

    /// Asserts that current token matches the provided one,
    /// and if it does, increments the lexer.
    pub fn eat_token(&mut self, token: Token) {
        self.expect_token(token);
        self.next_token();
    }

    /// Returns a boolean indicating if the current token
    /// is either an identifier or a keyword.
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

    /// Returns the raw slice of input related to the current token.
    pub fn raw(&self) -> String {
        self.input[self.start..self.end].into()
    }

    /// Scans the current token as a regexp
    pub fn scan_regexp(&mut self) {
        loop {
            match self.character {
                '/' => {
                    self.step();
                    'inner: loop {
                        if is_identifier_continue(self.character) {
                            match self.character {
                                'g' | 'i' | 'm' | 's' | 'u' | 'y' => self.step(),
                                _ => self.unexpected(),
                            }
                        } else {
                            break 'inner;
                        }
                    }

                    return;
                }

                '[' => {
                    self.step();
                    while self.character != ']' {
                        if self.character == '\\' {
                            self.step();
                        }

                        match self.character {
                            '\r' | '\n' | EOF_CHAR => {
                                self.unexpected();
                            }
                            _ => self.step(),
                        };
                    }
                    self.step();
                }

                _ => {
                    if self.character == '\\' {
                        self.step();
                    }

                    match self.character {
                        '\r' | '\n' | EOF_CHAR => {
                            self.unexpected();
                        }
                        _ => self.step(),
                    };
                }
            };
        }
    }

    /// Scans the next token
    pub fn next_token(&mut self) {
        loop {
            self.start = self.end;

            self.consume_comment();

            match self.character {
                c if is_whitespace(c) => {
                    self.step();
                    continue;
                }

                c if is_line_terminator(c) => {
                    self.step();
                    continue;
                }

                c if is_identifier_start(c) => {
                    let identifier = self.read_identifier();
                    self.token = lookup_identifer(&identifier);
                    self.identifier = identifier;
                }

                '~' => {
                    self.step();
                    self.token = Token::Tilde;
                }

                '/' => {
                    self.step();
                    if self.character == '=' {
                        self.step();
                        self.token = Token::SlashEquals;
                    } else {
                        self.token = Token::Slash;
                    }
                }

                '?' => {
                    self.step();
                    if self.character == '.' {
                        self.step();
                        self.token = Token::QuestionDot;
                    } else if self.character == '?' {
                        self.step();
                        if self.character == '=' {
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
                    if self.character == '+' {
                        self.step();
                        self.token = Token::PlusPlus;
                    } else if self.character == '=' {
                        self.step();
                        self.token = Token::PlusEquals;
                    } else {
                        self.token = Token::Plus;
                    }
                }

                '-' => {
                    self.step();
                    if self.character == '-' {
                        self.step();
                        self.token = Token::MinusMinus;
                    } else if self.character == '=' {
                        self.step();
                        self.token = Token::MinusEquals;
                    } else {
                        self.token = Token::Minus;
                    }
                }

                '*' => {
                    self.step();
                    if self.character == '*' {
                        self.step();
                        if self.character == '=' {
                            self.step();
                            self.token = Token::AsteriskAsteriskEquals;
                        } else {
                            self.token = Token::AsteriskAsterisk;
                        }
                    } else if self.character == '=' {
                        self.step();
                        self.token = Token::AsteriskEquals;
                    } else {
                        self.token = Token::Asterisk;
                    }
                }

                '<' => {
                    self.step();
                    if self.character == '<' {
                        self.step();
                        if self.character == '=' {
                            self.step();
                            self.token = Token::LessThanLessThanEquals;
                        } else {
                            self.token = Token::LessThanLessThan;
                        }
                    } else if self.character == '=' {
                        self.step();
                        self.token = Token::LessThanEquals;
                    } else {
                        self.token = Token::LessThan;
                    }
                }

                '>' => {
                    self.step();
                    if self.character == '>' {
                        self.step();
                        if self.character == '>' {
                            self.step();
                            if self.character == '=' {
                                self.step();
                                self.token = Token::GreaterThanGreaterThanGreaterThanEquals;
                            } else {
                                self.token = Token::GreaterThanGreaterThanGreaterThan;
                            }
                        } else if self.character == '=' {
                            self.step();
                            self.token = Token::GreaterThanGreaterThanEquals;
                        } else {
                            self.token = Token::GreaterThanGreaterThan;
                        }
                    } else if self.character == '=' {
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
                    if self.character == '=' {
                        self.step();
                        if self.character == '=' {
                            self.step();
                            self.token = Token::EqualsEqualsEquals;
                        } else {
                            self.token = Token::EqualsEquals;
                        }
                    } else if self.character == '>' {
                        self.step();
                        self.token = Token::EqualsGreaterThan;
                    } else {
                        self.token = Token::Equals;
                    }
                }

                '!' => {
                    self.step();
                    if self.character == '=' {
                        self.step();
                        if self.character == '=' {
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
                    if self.character == '=' {
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
                    if self.character == '|' {
                        self.step();
                        if self.character == '=' {
                            self.step();
                            self.token = Token::BarBarEquals;
                        } else {
                            self.token = Token::BarBar;
                        }
                    } else if self.character == '=' {
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
                    if self.character == '=' {
                        self.step();
                        self.token = Token::CaretEquals;
                    } else {
                        self.token = Token::Caret;
                    }
                }

                '&' => {
                    self.step();
                    if self.character == '&' {
                        self.step();
                        if self.character == '=' {
                            self.step();
                            self.token = Token::AmpersandAmpersandEquals;
                        } else {
                            self.token = Token::AmpersandAmpersand;
                        }
                    } else if self.character == '=' {
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
                            '\\' => self.step(),
                            EOF_CHAR => self.unexpected(),
                            _ => {}
                        }
                        self.step();
                    }

                    self.identifier = self.input[self.start + 1..self.end - 1].into();
                    self.token = Token::StringLiteral;
                }

                '`' => {
                    self.step();

                    let mut suffix_length = 1;
                    self.token = Token::TemplateNoSubstitutionLiteral;
                    'template_literal: loop {
                        match self.character {
                            '$' => {
                                self.step();
                                if self.character == '{' {
                                    self.step();
                                    suffix_length = 2;
                                    self.token = Token::TemplateHead;
                                    break 'template_literal;
                                }
                                continue 'template_literal;
                            }
                            '\\' => self.step(),
                            '`' => {
                                self.step();
                                break 'template_literal;
                            }
                            EOF_CHAR => self.unexpected(),
                            _ => {}
                        }
                        self.step();
                    }

                    self.identifier = self.input[self.start + 1..self.end - suffix_length].into();
                }

                '.' => {
                    if self.peek() == '.' {
                        self.step();
                        self.step();
                        self.step();
                        self.token = Token::DotDotDot;
                    } else if self.peek() >= '0' && self.peek() <= '9' {
                        self.read_number();
                    } else {
                        self.step();
                        self.token = Token::Dot;
                    }
                }

                '0' => match self.peek() {
                    'b' => self.read_radix_number(2),
                    'o' => self.read_radix_number(8),
                    'x' => self.read_radix_number(16),
                    _ => self.read_number(),
                },

                '1'..='9' => self.read_number(),

                EOF_CHAR => self.token = Token::EndOfFile,

                _ => {
                    self.step();
                    self.token = Token::Illegal;
                }
            };

            return;
        }
    }

    /// Scans the next token as either a template tail
    /// or a template middle.
    pub fn scan_template_tail_or_middle(&mut self) {
        self.expect_token(Token::CloseBrace);
        let mut suffix_length = 1;
        self.token = Token::TemplateTail;
        'template_literal: loop {
            match self.character {
                '$' => {
                    self.step();
                    if self.character == '{' {
                        self.step();
                        suffix_length = 2;
                        self.token = Token::TemplateMiddle;
                        break 'template_literal;
                    }
                    continue 'template_literal;
                }
                '\\' => self.step(),
                '`' => {
                    self.step();
                    break 'template_literal;
                }
                EOF_CHAR => self.unexpected(),
                _ => {}
            }
            self.step();
        }

        self.identifier = self.input[self.start + 1..self.end - suffix_length].into();
    }
}

/// Internal
impl<'a, L: Logger> Lexer<'a, L> {
    fn step(&mut self) {
        self.character = self.chars.next().unwrap_or(EOF_CHAR);
        self.end = self.current;
        self.current += 1;
    }

    // Returns the next token without moving the current.
    fn peek(&mut self) -> char {
        self.chars.clone().nth(0).unwrap_or(EOF_CHAR)
    }

    /// Skip over the comment if the current character marks
    /// the start of a comment.
    fn consume_comment(&mut self) {
        match (self.character, self.peek()) {
            // Single line comment
            ('/', '/') => {
                self.step(); // First /
                self.step(); // Second /

                // Loop until we reach a line terminator or EOF
                'single_line_comment: loop {
                    match self.character {
                        c if is_line_terminator(c) => {
                            self.step();
                            break 'single_line_comment;
                        }
                        EOF_CHAR => {
                            self.step();
                            break 'single_line_comment;
                        }
                        _ => self.step(),
                    }
                }
            }

            // Multi-line comment
            ('/', '*') => {
                self.step(); // /
                self.step(); // *

                'multi_line_comment: loop {
                    match (self.character, self.peek()) {
                        ('*', '/') => {
                            self.step(); // *
                            self.step(); // /
                            break 'multi_line_comment;
                        }
                        (EOF_CHAR, _) | (_, EOF_CHAR) => {
                            panic!("File ended without terminating multi-line comment")
                        }
                        _ => self.step(),
                    }
                }
            }

            // For anything else, ignore.
            _ => {}
        };
    }

    fn read_identifier(&mut self) -> String {
        let mut word = String::new();
        while self.character != EOF_CHAR {
            if is_identifier_continue(self.character) {
                word.push(self.character);
                self.step();
            } else {
                break;
            }
        }
        return word;
    }

    fn read_number(&mut self) {
        // 00
        if self.character == '0' && self.peek() == '0' {
            panic!("Legacy octal literals are not supported in strict mode");
        }

        // Means we've hit a fractal number .012
        if self.character == '.' {
            self.step();
            let number = self.read_decimal_number();

            // Exponent
            if self.character == 'e' || self.character == 'E' {
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
        if self.character == 'e' || self.character == 'E' {
            todo!()
        }

        // Fractal 1.1
        if self.character == '.' {
            self.step();
            number = format!("{}.{}", number, self.read_decimal_number());
        }

        // BitInt
        if self.character == 'n' {
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
        if self.character == 'e' || self.character == 'E' {
            todo!()
        }

        // Means we've hit a big int literal
        // We do not attempt to convert the string into
        // a number since that could mean precision loss.
        if self.character == 'n' {
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
                '0' | '1' => num.push(self.character),
                '_' => {}
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
                '0'..='8' => num.push(self.character),
                '_' => {}
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
                '0'..='9' => num.push(self.character),

                '_' => {}
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
                '0'..='9' | 'a'..='f' | 'A'..='F' => num.push(self.character),
                '_' => {}
                _ => break,
            }
            self.step();
        }
        return num;
    }
}

/// True if `c` is considered whitespace according to the ECMAScript specification
///
/// See [ECMAScript specification](https://262.ecma-international.org/11.0/#sec-white-space)
fn is_whitespace(c: char) -> bool {
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
fn is_line_terminator(c: char) -> bool {
    matches!(
        c,
        '\u{000A}' // Line feed
        | '\u{000D}' // Carriage return
        | '\u{2028}' // Line separator
        | '\u{2029}' // Paragraph separator
    )
}

/// True if `c` is considered a identifier start according to the ECMAScript specification
///
/// See [ECMAScript specification](https://262.ecma-international.org/11.0/#sec-names-and-keywords)
fn is_identifier_start(c: char) -> bool {
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
fn is_identifier_continue(c: char) -> bool {
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
