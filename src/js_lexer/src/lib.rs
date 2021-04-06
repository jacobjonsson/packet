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
use logger::compute_line_and_column;
mod unicode;

/// This means we've hit the end of the file
pub const EOF_CHAR: char = '\0';

pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current: usize,
    start: usize,
    end: usize,
    character: char,
    pub identifier: String,
    pub number: f64,
    pub token: Token,
}

/// Creates a new lexer state object
pub fn create<'a>(input: &'a str) -> Lexer<'a> {
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
    };

    bump(&mut lexer);
    scan_next_token(&mut lexer);
    lexer
}

/// Returns the raw input slice associated with the current token
pub fn raw<'a>(lexer: &'a Lexer) -> &'a str {
    &lexer.input[lexer.start..lexer.end]
}

/// Scans the next token in the stream
pub fn scan_next_token(lexer: &mut Lexer) {
    loop {
        lexer.start = lexer.end;
        eat_comment(lexer);

        match lexer.character {
            c if is_whitespace(c) => {
                bump(lexer);
                continue;
            }

            c if is_line_terminator(c) => {
                bump(lexer);
                continue;
            }

            // Identifier or keyword
            c if is_identifier_start(c) => {
                let identifier = read_identifer(lexer);
                lexer.token = lookup_identifer(&identifier);
                lexer.identifier = identifier;
            }

            '~' => {
                bump(lexer);
                lexer.token = Token::Tilde;
            }

            ';' => {
                bump(lexer);
                lexer.token = Token::Semicolon;
            }

            '(' => {
                bump(lexer);
                lexer.token = Token::OpenParen;
            }

            ')' => {
                bump(lexer);
                lexer.token = Token::CloseParen;
            }

            '{' => {
                bump(lexer);
                lexer.token = Token::OpenBrace;
            }

            '}' => {
                bump(lexer);
                lexer.token = Token::CloseBrace;
            }

            '[' => {
                bump(lexer);
                lexer.token = Token::OpenBracket;
            }

            ']' => {
                bump(lexer);
                lexer.token = Token::CloseBracket;
            }

            ',' => {
                bump(lexer);
                lexer.token = Token::Comma;
            }

            ':' => {
                bump(lexer);
                lexer.token = Token::Colon;
            }

            // + or ++ or +=
            '+' => {
                bump(lexer);
                lexer.token = match lexer.character {
                    '+' => {
                        bump(lexer);
                        Token::PlusPlus
                    }
                    '=' => {
                        bump(lexer);
                        Token::PlusEquals
                    }
                    _ => Token::Plus,
                };
            }

            // - or -- or -=
            '-' => {
                bump(lexer);
                lexer.token = match lexer.character {
                    '-' => {
                        bump(lexer);
                        Token::MinusMinus
                    }
                    '=' => {
                        bump(lexer);
                        Token::MinusEquals
                    }
                    _ => Token::Minus,
                };
            }

            // * or ** or *= or **=
            '*' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('=', _) => {
                        bump(lexer);
                        Token::AsteriskEquals
                    }
                    ('*', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::AsteriskAsteriskEquals
                    }
                    ('*', _) => {
                        bump(lexer);
                        Token::AsteriskAsterisk
                    }
                    _ => Token::Asterisk,
                }
            }

            // ? or '?.' or ?? or ??=
            '?' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('.', _) => {
                        bump(lexer);
                        Token::QuestionDot
                    }
                    ('?', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::QuestionQuestionEquals
                    }
                    ('?', _) => {
                        bump(lexer);
                        Token::QuestionQuestion
                    }
                    _ => Token::Question,
                }
            }

            // / or /=
            '/' => {
                bump(lexer);
                lexer.token = match lexer.character {
                    '=' => {
                        bump(lexer);
                        Token::SlashEquals
                    }
                    _ => Token::Slash,
                }
            }

            // = or == or === or =>
            '=' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('=', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::EqualsEqualsEquals
                    }
                    ('=', _) => {
                        bump(lexer);
                        Token::EqualsEquals
                    }
                    ('>', _) => {
                        bump(lexer);
                        Token::EqualsGreaterThan
                    }
                    _ => Token::Equals,
                }
            }

            // ! or != or !==
            '!' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('=', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::ExclamationEqualsEquals
                    }
                    ('=', _) => {
                        bump(lexer);
                        Token::ExclamationEquals
                    }
                    _ => Token::Exclamation,
                }
            }

            // % or %=
            '%' => {
                bump(lexer);
                lexer.token = match lexer.character {
                    '=' => {
                        bump(lexer);
                        Token::PercentEquals
                    }
                    _ => Token::Percent,
                };
            }

            // > or >> or >>> or >= or >>= or >>>=
            '>' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('>', '>') => {
                        bump(lexer);
                        bump(lexer);
                        match lexer.character {
                            '=' => {
                                bump(lexer);
                                Token::GreaterThanGreaterThanGreaterThanEquals
                            }
                            _ => Token::GreaterThanGreaterThanGreaterThan,
                        }
                    }
                    ('>', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::GreaterThanGreaterThanEquals
                    }
                    ('>', _) => {
                        bump(lexer);
                        Token::GreaterThanGreaterThan
                    }
                    ('=', _) => {
                        bump(lexer);
                        Token::GreaterThanEquals
                    }
                    _ => Token::GreaterThan,
                };
            }

            // < or << or <= or <<=
            '<' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('<', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::LessThanLessThanEquals
                    }
                    ('<', _) => {
                        bump(lexer);
                        Token::LessThanLessThan
                    }
                    ('=', _) => {
                        bump(lexer);
                        Token::LessThanEquals
                    }
                    _ => Token::LessThan,
                };
            }

            // | or || or |= or ||=
            '|' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('|', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::BarBarEquals
                    }
                    ('=', _) => {
                        bump(lexer);
                        Token::BarEquals
                    }
                    ('|', _) => {
                        bump(lexer);
                        Token::BarBar
                    }
                    _ => Token::Bar,
                };
            }

            // ^ or ^=
            '^' => {
                bump(lexer);
                lexer.token = match lexer.character {
                    '=' => {
                        bump(lexer);
                        Token::CaretEquals
                    }
                    _ => Token::Caret,
                };
            }

            // & or && or &= or &&=
            '&' => {
                bump(lexer);
                lexer.token = match (lexer.character, peek(lexer)) {
                    ('&', '=') => {
                        bump(lexer);
                        bump(lexer);
                        Token::AmpersandAmpersandEquals
                    }
                    ('=', _) => {
                        bump(lexer);
                        Token::AmpersandEquals
                    }
                    ('&', _) => {
                        bump(lexer);
                        Token::AmpersandAmpersand
                    }
                    _ => Token::Ampersand,
                }
            }

            // string literal
            '"' | '\'' => {
                let quote = lexer.character;
                bump(lexer);

                'string_literal: loop {
                    if lexer.character == quote {
                        bump(lexer);
                        break 'string_literal;
                    }

                    match lexer.character {
                        '\\' => bump(lexer),
                        EOF_CHAR => todo!(),
                        _ => {}
                    }
                    bump(lexer);
                }

                lexer.identifier = lexer.input[lexer.start + 1..lexer.end - 1].into();
                lexer.token = Token::StringLiteral;
            }

            // template literal
            '`' => {
                bump(lexer);

                let mut suffix_length = 1;
                lexer.token = Token::TemplateNoSubstitutionLiteral;
                'template_literal: loop {
                    match lexer.character {
                        '$' => {
                            bump(lexer);
                            if lexer.character == '{' {
                                bump(lexer);
                                suffix_length = 2;
                                lexer.token = Token::TemplateHead;
                                break 'template_literal;
                            }
                            continue 'template_literal;
                        }
                        '\\' => bump(lexer),
                        '`' => {
                            bump(lexer);
                            break 'template_literal;
                        }
                        EOF_CHAR => todo!(),
                        _ => {}
                    }
                    bump(lexer);
                }

                lexer.identifier = lexer.input[lexer.start + 1..lexer.end - suffix_length].into();
            }

            // . or ... or .123
            '.' => match peek(&lexer) {
                '.' => {
                    bump(lexer);
                    bump(lexer);
                    bump(lexer);
                    lexer.token = Token::DotDotDot;
                }

                '0'..='1' => {
                    read_number(lexer);
                }

                _ => {
                    bump(lexer);
                    lexer.token = Token::Dot;
                }
            },

            '0' => match peek(&lexer) {
                'b' => read_radix_number(lexer, 2),
                'o' => read_radix_number(lexer, 8),
                'x' => read_radix_number(lexer, 16),
                _ => read_number(lexer),
            },

            '1'..='9' => read_number(lexer),

            EOF_CHAR => lexer.token = Token::EndOfFile,

            _ => {
                bump(lexer);
                lexer.token = Token::Illegal;
            }
        };

        break;
    }
}

/// Scans the next token as part of an regexp
pub fn scan_regexp(lexer: &mut Lexer) {
    loop {
        match lexer.character {
            '/' => {
                bump(lexer);
                'inner: loop {
                    if is_identifier_continue(lexer.character) {
                        match lexer.character {
                            'g' | 'i' | 'm' | 's' | 'u' | 'y' => bump(lexer),
                            _ => todo!(),
                        }
                    } else {
                        break 'inner;
                    }
                }

                return;
            }

            '[' => {
                bump(lexer);
                while lexer.character != ']' {
                    if lexer.character == '\\' {
                        bump(lexer);
                    }

                    match lexer.character {
                        '\r' | '\n' | EOF_CHAR => {
                            todo!();
                        }
                        _ => bump(lexer),
                    };
                }
                bump(lexer);
            }

            _ => {
                if lexer.character == '\\' {
                    bump(lexer);
                }

                match lexer.character {
                    '\r' | '\n' | EOF_CHAR => {
                        todo!();
                    }
                    _ => bump(lexer),
                };
            }
        };
    }
}

/// Scans the next token as part of a template literal
pub fn scan_template_tail_or_middle(lexer: &mut Lexer) {
    expect_token(lexer, Token::CloseBrace);
    let mut suffix_length = 1;
    lexer.token = Token::TemplateTail;
    'template_literal: loop {
        match lexer.character {
            '$' => {
                bump(lexer);
                if lexer.character == '{' {
                    bump(lexer);
                    suffix_length = 2;
                    lexer.token = Token::TemplateMiddle;
                    break 'template_literal;
                }
                continue 'template_literal;
            }
            '\\' => bump(lexer),
            '`' => {
                bump(lexer);
                break 'template_literal;
            }
            EOF_CHAR => todo!(),
            _ => {}
        }
        bump(lexer);
    }

    lexer.identifier = lexer.input[lexer.start + 1..lexer.end - suffix_length].into();
}

/// Asserts that next token matches the given one
/// and advances to the lexer to the next token in the stream.
pub fn eat_token(lexer: &mut Lexer, token: Token) {
    expect_token(lexer, token);
    scan_next_token(lexer);
}

/// Asserts that the next token matches the given one
pub fn expect_token(lexer: &Lexer, token: Token) {
    if lexer.token != token {
        let (line, _, _, _) = compute_line_and_column(lexer.input, lexer.start);
        panic!(
            "Expected {} but got {} at line: {}",
            token, lexer.token, line
        );
    }
}

/// Bumps the lexer
fn bump(lexer: &mut Lexer) {
    lexer.character = lexer.chars.next().unwrap_or(EOF_CHAR);
    lexer.end = lexer.current;
    lexer.current += 1;
}

/// Return the next token without affecting the internal state
fn peek(lexer: &Lexer) -> char {
    lexer.chars.clone().nth(0).unwrap_or(EOF_CHAR)
}

/// Eats and discards the comment if the current token is the start of a comment
fn eat_comment(lexer: &mut Lexer) {
    match (lexer.character, peek(&lexer)) {
        // Single line comment
        ('/', '/') => {
            bump(lexer); // First /
            bump(lexer); // Second /

            // Loop until we reach a line terminator or EOF
            'single_line_comment: loop {
                match lexer.character {
                    c if is_line_terminator(c) => {
                        bump(lexer);
                        break 'single_line_comment;
                    }
                    EOF_CHAR => {
                        bump(lexer);
                        break 'single_line_comment;
                    }
                    _ => bump(lexer),
                }
            }
        }

        // Multi-line comment
        ('/', '*') => {
            bump(lexer); // /
            bump(lexer); // *

            'multi_line_comment: loop {
                match (lexer.character, peek(&lexer)) {
                    ('*', '/') => {
                        bump(lexer); // *
                        bump(lexer); // /
                        break 'multi_line_comment;
                    }
                    (EOF_CHAR, _) | (_, EOF_CHAR) => {
                        panic!("File ended without terminating multi-line comment")
                    }
                    _ => bump(lexer),
                }
            }
        }

        // For anything else, ignore.
        _ => {}
    };
}

/// Bumps and reads until the until the end of the identifier
fn read_identifer(lexer: &mut Lexer) -> String {
    let mut word = String::new();
    while is_identifier_continue(lexer.character) {
        word.push(lexer.character);
        bump(lexer);
    }
    word
}

/// Bumps and reads until the the end of the decimal number
fn read_number(lexer: &mut Lexer) {
    // 00
    if lexer.character == '0' && peek(lexer) == '0' {
        panic!("Legacy octal literals are not supported in strict mode");
    }

    // Means we've hit a fractal number .012
    if lexer.character == '.' {
        bump(lexer);
        let number = read_decimal_number(lexer);

        // Exponent
        if lexer.character == 'e' || lexer.character == 'E' {
            todo!()
        }

        lexer.token = Token::NumericLiteral;
        lexer.number = format!("0.{}", number)
            .parse::<f64>()
            .expect(&format!("Failed to parse .{} into an f64", number));
        return;
    }

    let mut number = read_decimal_number(lexer);

    // Exponent
    if lexer.character == 'e' || lexer.character == 'E' {
        todo!()
    }

    // Fractal 1.1
    if lexer.character == '.' {
        bump(lexer);
        number = format!("{}.{}", number, read_decimal_number(lexer));
    }

    // BitInt
    if lexer.character == 'n' {
        bump(lexer);
        lexer.token = Token::BigIntegerLiteral;
        lexer.identifier = number;
        return;
    }

    lexer.token = Token::NumericLiteral;
    lexer.number = number
        .parse::<f64>()
        .expect(&format!("Failed to parse .{} into an f64", number));
}

/// Reads a radix number (0b, 0x, 0o)
fn read_radix_number(lexer: &mut Lexer, radix: u32) {
    bump(lexer);
    bump(lexer);

    let number = match radix {
        2 => read_binary_number(lexer),
        8 => read_octal_number(lexer),
        16 => read_hexadecimal_number(lexer),

        _ => todo!(),
    };

    // Exponent
    if lexer.character == 'e' || lexer.character == 'E' {
        todo!()
    }

    // Means we've hit a big int literal
    // We do not attempt to convert the string into
    // a number since that could mean precision loss.
    if lexer.character == 'n' {
        bump(lexer);
        lexer.token = Token::BigIntegerLiteral;
        lexer.identifier = match radix {
            2 => format!("0b{}", number),
            8 => format!("0o{}", number),
            16 => format!("0x{}", number),
            _ => todo!(),
        };
        return;
    }

    lexer.number = i64::from_str_radix(&number, radix)
        .expect(&format!("[Packet]: Failed to convert {} to an i64", number))
        as f64;
    lexer.token = Token::NumericLiteral;
}

/// Reads a binary number
fn read_binary_number(lexer: &mut Lexer) -> String {
    let mut num = String::new();
    loop {
        match lexer.character {
            '0' | '1' => num.push(lexer.character),
            '_' => {}
            _ => break,
        }
        bump(lexer);
    }
    return num;
}

/// Reads an octal number
fn read_octal_number(lexer: &mut Lexer) -> String {
    let mut num = String::new();
    loop {
        match lexer.character {
            '0'..='8' => num.push(lexer.character),
            '_' => {}
            _ => break,
        }
        bump(lexer);
    }
    return num;
}

/// Reads a decimal number
fn read_decimal_number(lexer: &mut Lexer) -> String {
    let mut num = String::new();
    loop {
        match lexer.character {
            '0'..='9' => num.push(lexer.character),
            '_' => {}
            _ => break,
        }
        bump(lexer);
    }
    return num;
}

/// Reads a hexadecimal number
fn read_hexadecimal_number(lexer: &mut Lexer) -> String {
    let mut num = String::new();
    loop {
        match lexer.character {
            '0'..='9' | 'a'..='f' | 'A'..='F' => num.push(lexer.character),
            '_' => {}
            _ => break,
        }
        bump(lexer);
    }
    return num;
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
