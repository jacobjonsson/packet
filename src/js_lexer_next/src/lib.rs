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
pub use token::Token;

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
            None => return Ok(Token::Eof),
        };

        match character {
            c if is_identifier_start(c) => self.scan_identifier(),
            '0' => self.scan_zero(),
            '1'..='9' => self.scan_decimal_number(),
            '"' | '\'' => self.scan_string(character),
            '`' => self.scan_template(),
            '-' => Ok(self.scan_minus()),
            ',' => Ok(self.scan_comma()),
            ';' => Ok(self.scan_semicolon()),
            ':' => Ok(self.scan_colon()),
            '!' => Ok(self.scan_exclamation()),
            '?' => Ok(self.scan_question_mark()),
            '.' => self.scan_dot(),
            '(' => Ok(self.scan_open_paren()),
            ')' => Ok(self.scan_close_paren()),
            '[' => Ok(self.scan_open_bracket()),
            ']' => Ok(self.scan_close_bracket()),
            '{' => Ok(self.scan_open_brace()),
            '}' => Ok(self.scan_close_brace()),
            '*' => Ok(self.scan_asterisk()),
            '/' => Ok(self.scan_slash()),
            '&' => Ok(self.scan_ampersand()),
            '%' => Ok(self.scan_percent()),
            '^' => Ok(self.scan_caret()),
            '+' => Ok(self.scan_plus()),
            '<' => Ok(self.scan_less_than()),
            '=' => Ok(self.scan_equals()),
            '>' => Ok(self.scan_greater_than()),
            '|' => Ok(self.scan_bar()),
            '~' => Ok(self.scan_tilde()),
            _ => Ok(Token::Illegal),
        }
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
