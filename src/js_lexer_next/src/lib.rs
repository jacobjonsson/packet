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

    /// The current token
    pub token: Token,
    /// The start position of the token
    pub token_start: usize,
    /// The end position of the token,
    pub token_end: usize,
    /// The string value of the token
    pub token_text: &'a str,
    /// The numeric value of the token
    pub token_number: f64,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(input: &'a str) -> Lexer {
        let characters: Vec<(usize, char)> = input.char_indices().collect();
        let last_position: usize = characters
            .last()
            .map(|(idx, char)| idx + char.len_utf8())
            .expect("Failed to extract the position of the last character");

        Lexer {
            input,
            characters: input.char_indices().collect(),
            index: 0,
            last_position,
            token: Token::Eof,
            token_start: 0,
            token_end: 0,
            token_text: "",
            token_number: 0.,
        }
    }

    /// Scans the next token and advances the lexer
    pub fn next(&mut self) -> LexerResult<()> {
        self.skip_whitespace()?;

        self.token_start = self.current_position();
        let character = match self.current_character() {
            Some(c) => c,
            None => {
                self.token = Token::Eof;
                return Ok(());
            }
        };

        self.token = match character {
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
            _ => Token::Illegal,
        };
        self.token_end = self.current_position();

        Ok(())
    }

    /// Asserts the the current token matches given kind and increments the next token
    pub fn consume(&mut self, kind: Token) -> LexerResult<()> {
        if self.token != kind {
            return Err(JSError::new(
                js_error::JSErrorKind::SyntaxError,
                Span::new(self.token_start, self.token_end),
            ));
        }

        self.next()
    }

    pub fn consume_optional(&mut self, kind: Token) -> LexerResult<()> {
        if self.token == kind {
            self.next()?;
        }
        Ok(())
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

    /// Returns the position in the source of the
    /// next character.
    #[allow(dead_code)]
    fn next_position(&self) -> usize {
        match self.characters.get(self.index + 1) {
            Some(v) => v.0,
            None => self.last_position,
        }
    }
}
