use std::fmt;

#[derive(Debug, PartialEq)]
pub enum JSError {
    SyntaxError,
    IdentifierAfterNumber,
    UnterminatedBlockComment,
    UnterminatedStringLiteral,
    UnterminatedTemplateLiteral,
    UnterminatedRegexp,
    InvalidRegexpFlag,
}

impl fmt::Display for JSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSError::SyntaxError => write!(f, "Syntax error"),
            JSError::IdentifierAfterNumber => {
                write!(f, "Identifiers are not allowed directly after a number")
            }
            JSError::UnterminatedBlockComment => write!(f, "Unterminated block comment"),
            JSError::UnterminatedStringLiteral => write!(f, "Unterminated string literal"),
            JSError::UnterminatedTemplateLiteral => write!(f, "Unterminated template literal"),
            JSError::UnterminatedRegexp => write!(f, "Unterminated regexp"),
            JSError::InvalidRegexpFlag => write!(f, "The regexp flag is invalid"),
        }
    }
}
