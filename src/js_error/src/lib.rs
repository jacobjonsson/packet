use std::fmt;

use span::Span;

#[derive(Debug, PartialEq)]
pub struct JSError {
    pub kind: JSErrorKind,
    pub span: Span,
}

impl JSError {
    pub fn new(kind: JSErrorKind, span: Span) -> JSError {
        JSError { kind, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum JSErrorKind {
    SyntaxError,
    IdentifierAfterNumber,
    UnterminatedBlockComment,
    UnterminatedStringLiteral,
    UnterminatedTemplateLiteral,
    UnterminatedRegexp,
    InvalidRegexpFlag,
    MissingConstInitializer,
    StrictModeReserved,
    UnexpectedYieldAsBindingIdentifier,
    UnexpectedAwaitAsBindingIdentifier,
    ExpectedBindingIdentifier,
    InvalidShorthandPropertyKey,
    RestElementMustBeLast,
    TrailingCommaAfterRestElement,
}

impl fmt::Display for JSErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSErrorKind::SyntaxError => write!(f, "Syntax error"),
            JSErrorKind::IdentifierAfterNumber => {
                write!(f, "Identifiers are not allowed directly after a number")
            }
            JSErrorKind::UnterminatedBlockComment => write!(f, "Unterminated block comment"),
            JSErrorKind::UnterminatedStringLiteral => write!(f, "Unterminated string literal"),
            JSErrorKind::UnterminatedTemplateLiteral => write!(f, "Unterminated template literal"),
            JSErrorKind::UnterminatedRegexp => write!(f, "Unterminated regexp"),
            JSErrorKind::InvalidRegexpFlag => write!(f, "The regexp flag is invalid"),
            JSErrorKind::MissingConstInitializer => write!(f, "Missing const initializer"),
            JSErrorKind::StrictModeReserved => write!(f, "Unexpected reserved word in struct mode"),
            JSErrorKind::UnexpectedYieldAsBindingIdentifier => {
                write!(f, "Unexpected yield as binding identifier in this context")
            }
            JSErrorKind::UnexpectedAwaitAsBindingIdentifier => {
                write!(f, "Unexpected await as binding identifier in this context")
            }
            JSErrorKind::ExpectedBindingIdentifier => write!(f, "Expected binding identifier"),
            JSErrorKind::InvalidShorthandPropertyKey => write!(f, "Invalid shorthand property key"),
            JSErrorKind::RestElementMustBeLast => {
                write!(f, "Rest element must be the last element")
            }
            JSErrorKind::TrailingCommaAfterRestElement => {
                write!(f, "Unexpected trailing comma after rest element")
            }
        }
    }
}
