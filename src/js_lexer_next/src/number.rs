use crate::Lexer;
use crate::Token;
use crate::{identifier::is_identifier_start, LexerResult};

impl<'a> Lexer<'a> {
    /// Scans a floating point
    /// .123
    pub(crate) fn scan_floating_point(&mut self) -> LexerResult<Token> {
        let start = self.current_position();
        self.index += 1; // .
        loop {
            let character = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if matches!(character, '0'..='9') {
                self.index += 1;
                continue;
            }

            if character == '_' {
                // TODO: Underscores aren't allowed in succession
                self.index += 1;
                continue;
            }

            // Break on exponentiation
            if matches!(character, 'e' | 'E') {
                break;
            }

            // Floating points cannot be a big integer
            if character == 'n' {
                panic!("Invalid big integer")
            }

            if is_identifier_start(character) {
                panic!("Identifiers are not allowed directly after a number")
            }

            // Any other character is breaking
            break;
        }
        let end = self.current_position();
        let text = &self.input[start..end];
        self.token_number = format!("0{}", text).parse::<f64>().unwrap();
        Ok(Token::Number)
    }

    /// Scans a number staring with a zero
    pub(crate) fn scan_zero(&mut self) -> LexerResult<Token> {
        match self.next_character() {
            Some('b') => self.scan_binary_number(),
            Some('o') => self.scan_octal_number(),
            Some('x') => self.scan_hexadecimal_number(),
            _ => self.scan_decimal_number(),
        }
    }

    /// Scans a binary number
    /// 0b101
    pub(crate) fn scan_binary_number(&mut self) -> LexerResult<Token> {
        self.index += 2; // 0b
        let start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if matches!(c, '0' | '1') {
                self.index += 1;
                continue;
            }

            if c == '_' {
                self.index += 1;
                continue;
            }

            if matches!(c, 'n') {
                break;
            }

            if is_identifier_start(c) {
                panic!("Identifiers are not allowed directly after a number")
            }

            break;
        }

        let c = self.current_character();
        if c == Some('n') {
            let end = self.current_position();
            self.token_text = &self.input[start - 2..end];
            self.index += 1;
            return Ok(Token::BigInt);
        }

        let end = self.current_position();
        let text = &self.input[start..end];
        self.token_number = i64::from_str_radix(&text.replace('_', ""), 2).unwrap() as f64;
        Ok(Token::Number)
    }

    /// Scans an octal number
    /// 0o123
    pub(crate) fn scan_octal_number(&mut self) -> LexerResult<Token> {
        self.index += 2; // 0o
        let start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if matches!(c, '0'..='7') {
                self.index += 1;
                continue;
            }

            if c == '_' {
                self.index += 1;
                continue;
            }

            if matches!(c, 'n') {
                break;
            }

            if is_identifier_start(c) {
                panic!("Identifiers are not allowed directly after a number")
            }

            break;
        }

        let c = self.current_character();
        if c == Some('n') {
            let end = self.current_position();
            self.token_text = &self.input[start - 2..end];
            self.index += 1;
            return Ok(Token::BigInt);
        }

        let end = self.current_position();
        let text = &self.input[start..end];
        self.token_number = i64::from_str_radix(&text.replace('_', ""), 8).unwrap() as f64;
        Ok(Token::Number)
    }

    /// Scans a decimal number
    /// 123
    pub(crate) fn scan_decimal_number(&mut self) -> LexerResult<Token> {
        let start = self.index;
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if matches!(c, '0'..='9') {
                self.index += 1;
                continue;
            }

            if c == '_' {
                self.index += 1;
                continue;
            }

            if c == '.' {
                self.index += 1;
                continue;
            }

            if matches!(c, 'n' | 'e' | 'E') {
                break;
            }

            if is_identifier_start(c) {
                panic!("Identifiers are not allowed directly after a number")
            }

            break;
        }

        let c = self.current_character();

        if matches!(c, Some('e') | Some('E')) {
            todo!()
        }

        if c == Some('n') {
            let end = self.current_position();
            self.token_text = &self.input[start..end];
            self.index += 1;
            return Ok(Token::BigInt);
        }

        let end = self.current_position();
        let text = &self.input[start..end];
        self.token_number = text.replace('_', "").parse::<f64>().unwrap();
        Ok(Token::Number)
    }

    /// Scans a hexadecimal number
    /// 0x1af
    pub(crate) fn scan_hexadecimal_number(&mut self) -> LexerResult<Token> {
        self.index += 2; // 0x
        let start = self.current_position();
        loop {
            let c = match self.current_character() {
                Some(c) => c,
                None => break,
            };

            if matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F') {
                self.index += 1;
                continue;
            }

            if c == '_' {
                self.index += 1;
                continue;
            }

            if matches!(c, 'n') {
                break;
            }

            if is_identifier_start(c) {
                panic!("Identifiers are not allowed directly after a number")
            }

            break;
        }

        let c = self.current_character();
        if c == Some('n') {
            let end = self.current_position();
            self.token_text = &self.input[start - 2..end];
            self.index += 1;
            return Ok(Token::BigInt);
        }

        let end = self.current_position();
        let text = &self.input[start..end];
        self.token_number = i64::from_str_radix(&text.replace('_', ""), 16).unwrap() as f64;
        Ok(Token::Number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floating_points() {
        let tests = vec![(".12", 0.12), (".0001", 0.0001)];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(Token::Number, lexer.token);
            assert_eq!(lexer.token_number, test.1);
        }
    }

    #[test]
    fn test_number() {
        let tests = vec![
            ("0", 0.),
            ("0.1", 0.1),
            ("10", 10.0),
            ("10.10", 10.10),
            ("99999", 99999.),
            ("0b10", 2.),
            ("0o10", 8.),
            ("0x10", 16.),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(Token::Number, lexer.token);
            assert_eq!(lexer.token_number, test.1);
        }
    }

    #[test]
    fn test_big_int() {
        let tests = vec![
            ("1n", "1"),
            ("10n", "10"),
            ("0b11n", "0b11"),
            ("0o11n", "0o11"),
            ("0x11n", "0x11"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0);
            assert_eq!(lexer.next(), Ok(()));
            assert_eq!(Token::BigInt, lexer.token);
            assert_eq!(test.1, lexer.token_text);
        }
    }
}
