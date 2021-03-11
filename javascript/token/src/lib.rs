pub trait TokenLiteral {
    fn token_literal(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Identifier(String),
    Number(String),

    Semicolon,
    Comma,
    Plus,
    Minus,
    Slash,
    Asterisk,
    LessThan,
    GreaterThan,

    Equals,
    EqualsEquals,
    EqualsEqualsEquals,
    Exclamation,
    ExclamationEquals,
    ExclamationEqualsEquals,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Continue,
    Break,
    If,
    ElseIf,
    Else,
    Function,
    Let,
    Const,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, "illegal"),
            Token::Eof => write!(f, "eof"),

            Token::Identifier(w) => write!(f, "Identifier({})", w),
            Token::Number(w) => write!(f, "Number({})", w),

            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),

            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),

            Token::Equals => write!(f, "="),
            Token::EqualsEquals => write!(f, "=="),
            Token::EqualsEqualsEquals => write!(f, "==="),
            Token::Exclamation => write!(f, "!"),
            Token::ExclamationEquals => write!(f, "!="),
            Token::ExclamationEqualsEquals => write!(f, "!=="),
            Token::LeftBrace => write!(f, "{{"),
            Token::LeftParen => write!(f, "("),
            Token::RightBrace => write!(f, "}}"),
            Token::RightParen => write!(f, ")"),

            Token::Continue => write!(f, "continue"),
            Token::Break => write!(f, "break"),
            Token::Const => write!(f, "const"),
            Token::Else => write!(f, "else"),
            Token::ElseIf => write!(f, "else if"),
            Token::Function => write!(f, "function"),
            Token::If => write!(f, "if"),
            Token::Let => write!(f, "let"),
        }
    }
}

impl TokenLiteral for Token {
    fn token_literal(&self) -> String {
        match self {
            Token::Illegal => "illegal".into(),
            Token::Eof => "eof".into(),

            Token::Identifier(w) => format!("Identifier({})", w),
            Token::Number(w) => format!("Number({})", w),

            Token::Semicolon => ";".into(),
            Token::Comma => ",".into(),

            Token::Plus => "+".into(),
            Token::Minus => "-".into(),
            Token::Slash => "/".into(),
            Token::Asterisk => "*".into(),
            Token::LessThan => "<".into(),
            Token::GreaterThan => ">".into(),

            Token::Equals => "=".into(),
            Token::EqualsEquals => "==".into(),
            Token::EqualsEqualsEquals => "===".into(),
            Token::Exclamation => "!".into(),
            Token::ExclamationEquals => "!=".into(),
            Token::ExclamationEqualsEquals => "!==".into(),
            Token::LeftBrace => "{".into(),
            Token::LeftParen => "(".into(),
            Token::RightBrace => "}".into(),
            Token::RightParen => ")".into(),

            Token::Continue => "continue".into(),
            Token::Break => "break".into(),
            Token::Const => "const".into(),
            Token::Else => "else".into(),
            Token::ElseIf => "else if".into(),
            Token::Function => "function".into(),
            Token::If => "if".into(),
            Token::Let => "let".into(),
        }
    }
}

pub fn lookup_identifer(identifier: &str) -> Token {
    match identifier {
        "function" => Token::Function,
        "continue" => Token::Continue,
        "break" => Token::Break,
        "if" => Token::If,
        "else" => Token::Else,
        "else if" => Token::ElseIf,
        _ => Token::Identifier(identifier.into()),
    }
}
