mod identifier;
mod number;
mod punctuators;
mod regexp;
mod string;
mod template;
mod token;
mod whitespace;

use identifier::is_identifier_start;
use js_error::JSError;
use span::Span;
use token::{Token, TokenKind};

pub type LexerResult<T> = Result<T, JSError>;

pub struct Lexer<'a> {
    /// The source file to scan
    input: &'a str,
    /// The vector of characters
    characters: Vec<(usize, char)>,
    /// The current index
    index: usize,
    /// The last position
    last_position: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(input: &'a str) -> Lexer {
        let characters: Vec<(usize, char)> = input.char_indices().collect();
        let last_character: usize = characters
            .last()
            .map(|(idx, char)| idx + char.len_utf8())
            .expect("Failed to extract the last character");

        Lexer {
            input,
            characters: input.char_indices().collect(),
            index: 0,
            last_position: last_character,
        }
    }

    /// Scans the next token and advances the lexer
    pub fn next(&mut self) -> LexerResult<Token> {
        self.skip_whitespace()?;

        let character = match self.current_character() {
            Some(c) => c,
            None => return Ok(Token::new(TokenKind::Eof, Span::new(0, 0))),
        };

        let start = self.current_position();
        let kind = match character {
            c if is_identifier_start(c) => self.scan_identifier()?,
            '0' => self.scan_zero()?,
            '1'..='9' => self.scan_decimal_number()?,
            '"' | '\'' => self.scan_string(character)?,
            '`' => self.scan_template()?,
            '-' => self.scan_minus(),
            ',' => self.scan_comma(),
            ';' => self.scan_semicolon(),
            ':' => self.scan_colon(),
            '!' => self.scan_exclamation(),
            '?' => self.scan_question_mark(),
            '.' => self.scan_dot()?,
            '(' => self.scan_open_paren(),
            ')' => self.scan_close_paren(),
            '[' => self.scan_open_bracket(),
            ']' => self.scan_close_bracket(),
            '{' => self.scan_open_brace(),
            '}' => self.scan_close_brace(),
            '*' => self.scan_asterisk(),
            '/' => self.scan_slash(),
            '&' => self.scan_ampersand(),
            '%' => self.scan_percent(),
            '^' => self.scan_caret(),
            '+' => self.scan_plus(),
            '<' => self.scan_less_than(),
            '=' => self.scan_equals(),
            '>' => self.scan_greater_than(),
            '|' => self.scan_bar(),
            '~' => self.scan_tilde(),
            _ => TokenKind::Illegal,
        };

        Ok(Token::new(kind, Span::new(start, self.current_position())))
    }

    /// Returns the current character
    fn current_character(&self) -> Option<char> {
        match self.characters.get(self.index) {
            Some(v) => Some(v.1),
            None => None,
        }
    }

    /// Returns the next character
    fn next_character(&self) -> Option<char> {
        match self.characters.get(self.index + 1) {
            Some(v) => Some(v.1),
            None => None,
        }
    }

    /// Returns the current position in the source
    /// If the current character does not exist the we return
    /// the position of the last character instead.
    fn current_position(&self) -> usize {
        match self.characters.get(self.index) {
            Some(v) => v.0,
            None => self.last_position,
        }
    }

    /// Returns the position in the source of the
    /// previous character.
    fn previous_position(&self) -> usize {
        match self.characters.get(self.index - 1) {
            Some(v) => v.0,
            None => self.last_position,
        }
    }
}
