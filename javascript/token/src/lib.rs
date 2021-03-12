pub trait TokenLiteral {
    fn token_literal(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EndOfFile,

    Hashbang,

    // Literals
    StringLiteral(String),
    NumericLiteral(String),

    // Identifiers
    Identifier(String),

    // Punctuation
    Ampersand,
    AmpersandAmpersand,
    Asterisk,
    AsteriskAsterisk,
    At,
    Bar,
    BarBar,
    Caret,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    Dot,
    DotDotDot,
    EqualsEquals,
    EqualsEqualsEquals,
    EqualsGreaterThan,
    Exclamation,
    ExclamationEquals,
    ExclamationEqualsEquals,
    GreaterThan,
    GreaterThanEquals,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    LessThan,
    LessThanEquals,
    LessThanLessThan,
    Minus,
    MinusMinus,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Percent,
    Plus,
    PlusPlus,
    Question,
    QuestionDot,
    QuestionQuestion,
    Semicolon,
    Slash,
    Tilde,

    // Assignments (keep in sync with IsAssign() below)
    AmpersandAmpersandEquals,
    AmpersandEquals,
    AsteriskAsteriskEquals,
    AsteriskEquals,
    BarBarEquals,
    BarEquals,
    CaretEquals,
    Equals,
    GreaterThanGreaterThanEquals,
    GreaterThanGreaterThanGreaterThanEquals,
    LessThanLessThanEquals,
    MinusEquals,
    PercentEquals,
    PlusEquals,
    QuestionQuestionEquals,
    SlashEquals,

    // Keywords
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    Let,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, "illegal"),
            Token::EndOfFile => write!(f, "eof"),

            Token::Hashbang => write!(f, "!"),

            Token::Identifier(w) => write!(f, "Identifier({})", w),
            Token::NumericLiteral(w) => write!(f, "NumericLiteral({})", w),

            Token::StringLiteral(w) => write!(f, "\"{}\"", w),

            // Punctuation
            Token::Ampersand => write!(f, "%"),
            Token::AmpersandAmpersand => write!(f, "%%"),
            Token::Asterisk => write!(f, "*"),
            Token::AsteriskAsterisk => write!(f, "**"),
            Token::At => write!(f, "@"),
            Token::Bar => write!(f, "|"),
            Token::BarBar => write!(f, "||"),
            Token::Caret => write!(f, "^"),
            Token::CloseBrace => write!(f, "}}"),
            Token::CloseBracket => write!(f, "]"),
            Token::CloseParen => write!(f, ")"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::DotDotDot => write!(f, "..."),
            Token::EqualsEquals => write!(f, "=="),
            Token::EqualsEqualsEquals => write!(f, "==="),
            Token::EqualsGreaterThan => write!(f, "=>"),
            Token::Exclamation => write!(f, "!"),
            Token::ExclamationEquals => write!(f, "!="),
            Token::ExclamationEqualsEquals => write!(f, "!=="),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanEquals => write!(f, ">="),
            Token::GreaterThanGreaterThan => write!(f, ">>"),
            Token::GreaterThanGreaterThanGreaterThan => write!(f, ">>>"),
            Token::LessThan => write!(f, "<"),
            Token::LessThanEquals => write!(f, "<="),
            Token::LessThanLessThan => write!(f, "<<"),
            Token::Minus => write!(f, "-"),
            Token::MinusMinus => write!(f, "--"),
            Token::OpenBrace => write!(f, "{{"),
            Token::OpenBracket => write!(f, "["),
            Token::OpenParen => write!(f, "("),
            Token::Percent => write!(f, "%"),
            Token::Plus => write!(f, "+"),
            Token::PlusPlus => write!(f, "++"),
            Token::Question => write!(f, "?"),
            Token::QuestionDot => write!(f, "?."),
            Token::QuestionQuestion => write!(f, "??"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::Tilde => write!(f, "~"),

            // Assignments
            Token::AmpersandAmpersandEquals => write!(f, "%%="),
            Token::AmpersandEquals => write!(f, "%="),
            Token::AsteriskAsteriskEquals => write!(f, "**="),
            Token::AsteriskEquals => write!(f, "*="),
            Token::BarBarEquals => write!(f, "||="),
            Token::BarEquals => write!(f, "|="),
            Token::CaretEquals => write!(f, "^="),
            Token::Equals => write!(f, "="),
            Token::GreaterThanGreaterThanEquals => write!(f, ">>="),
            Token::GreaterThanGreaterThanGreaterThanEquals => write!(f, ">>>="),
            Token::LessThanLessThanEquals => write!(f, "<<="),
            Token::MinusEquals => write!(f, "-="),
            Token::PercentEquals => write!(f, "%="),
            Token::PlusEquals => write!(f, "+="),
            Token::QuestionQuestionEquals => write!(f, "??="),
            Token::SlashEquals => write!(f, "/="),

            // Keywords
            Token::Break => write!(f, "break"),
            Token::Case => write!(f, "case"),
            Token::Catch => write!(f, "catch"),
            Token::Class => write!(f, "class"),
            Token::Const => write!(f, "const"),
            Token::Continue => write!(f, "continue"),
            Token::Debugger => write!(f, "debugger"),
            Token::Default => write!(f, "default"),
            Token::Delete => write!(f, "delete"),
            Token::Do => write!(f, "do"),
            Token::Else => write!(f, "else"),
            Token::Enum => write!(f, "enum"),
            Token::Export => write!(f, "export"),
            Token::Extends => write!(f, "extends"),
            Token::False => write!(f, "false"),
            Token::Finally => write!(f, "finally"),
            Token::For => write!(f, "for"),
            Token::Function => write!(f, "function"),
            Token::Let => write!(f, "let"),
            Token::If => write!(f, "if"),
            Token::Import => write!(f, "import"),
            Token::In => write!(f, "in"),
            Token::Instanceof => write!(f, "instanceof"),
            Token::New => write!(f, "new"),
            Token::Null => write!(f, "null"),
            Token::Return => write!(f, "return"),
            Token::Super => write!(f, "super"),
            Token::Switch => write!(f, "switch"),
            Token::This => write!(f, "this"),
            Token::Throw => write!(f, "throw"),
            Token::True => write!(f, "true"),
            Token::Try => write!(f, "try"),
            Token::Typeof => write!(f, "typeof"),
            Token::Var => write!(f, "var"),
            Token::Void => write!(f, "void"),
            Token::While => write!(f, "while"),
            Token::With => write!(f, "white"),
        }
    }
}

impl TokenLiteral for Token {
    fn token_literal(&self) -> String {
        match self {
            Token::Illegal => "illegal".into(),
            Token::EndOfFile => "eof".into(),

            Token::Hashbang => "!".into(),

            Token::Identifier(w) => w.clone(),
            Token::NumericLiteral(w) => w.clone(),
            Token::StringLiteral(w) => w.clone(),

            // Punctuation
            Token::Ampersand => "%".into(),
            Token::AmpersandAmpersand => "%%".into(),
            Token::Asterisk => "*".into(),
            Token::AsteriskAsterisk => "**".into(),
            Token::At => "@".into(),
            Token::Bar => "|".into(),
            Token::BarBar => "||".into(),
            Token::Caret => "^".into(),
            Token::CloseBrace => "}}".into(),
            Token::CloseBracket => "]".into(),
            Token::CloseParen => ")".into(),
            Token::Colon => ":".into(),
            Token::Comma => ",".into(),
            Token::Dot => ".".into(),
            Token::DotDotDot => "...".into(),
            Token::EqualsEquals => "==".into(),
            Token::EqualsEqualsEquals => "===".into(),
            Token::EqualsGreaterThan => "=>".into(),
            Token::Exclamation => "!".into(),
            Token::ExclamationEquals => "!=".into(),
            Token::ExclamationEqualsEquals => "!==".into(),
            Token::GreaterThan => ">".into(),
            Token::GreaterThanEquals => ">=".into(),
            Token::GreaterThanGreaterThan => ">>".into(),
            Token::GreaterThanGreaterThanGreaterThan => ">>>".into(),
            Token::LessThan => "<".into(),
            Token::LessThanEquals => "<=".into(),
            Token::LessThanLessThan => "<<".into(),
            Token::Minus => "-".into(),
            Token::MinusMinus => "--".into(),
            Token::OpenBrace => "{{".into(),
            Token::OpenBracket => "[".into(),
            Token::OpenParen => "(".into(),
            Token::Percent => "%".into(),
            Token::Plus => "+".into(),
            Token::PlusPlus => "++".into(),
            Token::Question => "?".into(),
            Token::QuestionDot => "?.".into(),
            Token::QuestionQuestion => "??".into(),
            Token::Semicolon => ";".into(),
            Token::Slash => "/".into(),
            Token::Tilde => "~".into(),

            // Assignments
            Token::AmpersandAmpersandEquals => "%%=".into(),
            Token::AmpersandEquals => "%=".into(),
            Token::AsteriskAsteriskEquals => "**=".into(),
            Token::AsteriskEquals => "*=".into(),
            Token::BarBarEquals => "||=".into(),
            Token::BarEquals => "|=".into(),
            Token::CaretEquals => "^=".into(),
            Token::Equals => "=".into(),
            Token::GreaterThanGreaterThanEquals => ">>=".into(),
            Token::GreaterThanGreaterThanGreaterThanEquals => ">>>=".into(),
            Token::LessThanLessThanEquals => "<<=".into(),
            Token::MinusEquals => "-=".into(),
            Token::PercentEquals => "%=".into(),
            Token::PlusEquals => "+=".into(),
            Token::QuestionQuestionEquals => "??=".into(),
            Token::SlashEquals => "/=".into(),

            // Keywords
            Token::Break => "break".into(),
            Token::Case => "case".into(),
            Token::Catch => "catch".into(),
            Token::Class => "class".into(),
            Token::Const => "const".into(),
            Token::Continue => "continue".into(),
            Token::Debugger => "debugger".into(),
            Token::Default => "default".into(),
            Token::Delete => "delete".into(),
            Token::Do => "do".into(),
            Token::Else => "else".into(),
            Token::Enum => "enum".into(),
            Token::Export => "export".into(),
            Token::Extends => "extends".into(),
            Token::False => "false".into(),
            Token::Finally => "finally".into(),
            Token::For => "for".into(),
            Token::Function => "function".into(),
            Token::Let => "let".into(),
            Token::If => "if".into(),
            Token::Import => "import".into(),
            Token::In => "in".into(),
            Token::Instanceof => "instanceof".into(),
            Token::New => "new".into(),
            Token::Null => "null".into(),
            Token::Return => "return".into(),
            Token::Super => "super".into(),
            Token::Switch => "switch".into(),
            Token::This => "this".into(),
            Token::Throw => "throw".into(),
            Token::True => "true".into(),
            Token::Try => "try".into(),
            Token::Typeof => "typeof".into(),
            Token::Var => "var".into(),
            Token::Void => "void".into(),
            Token::While => "while".into(),
            Token::With => "white".into(),
        }
    }
}

pub fn lookup_identifer(identifier: &str) -> Token {
    match identifier {
        "break" => Token::Break,
        "case" => Token::Case,
        "catch" => Token::Catch,
        "class" => Token::Class,
        "const" => Token::Const,
        "continue" => Token::Continue,
        "debugger" => Token::Debugger,
        "default" => Token::Default,
        "delete" => Token::Delete,
        "do" => Token::Do,
        "else" => Token::Else,
        "enum" => Token::Enum,
        "export" => Token::Export,
        "extends" => Token::Extends,
        "false" => Token::False,
        "finally" => Token::Finally,
        "for" => Token::For,
        "function" => Token::Function,
        "let" => Token::Let,
        "if" => Token::If,
        "import" => Token::Import,
        "in" => Token::In,
        "instanceof" => Token::Instanceof,
        "new" => Token::New,
        "null" => Token::Null,
        "return" => Token::Return,
        "super" => Token::Super,
        "switch" => Token::Switch,
        "this" => Token::This,
        "throw" => Token::Throw,
        "true" => Token::True,
        "try" => Token::Try,
        "typeof" => Token::Typeof,
        "var" => Token::Var,
        "void" => Token::Void,
        "while" => Token::While,
        "with" => Token::With,
        _ => Token::Identifier(identifier.into()),
    }
}
