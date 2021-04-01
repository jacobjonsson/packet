#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EndOfFile,

    Hashbang,

    // Literals
    StringLiteral,
    NumericLiteral,
    BigIntegerLiteral,
    // This is used when a template literal does not contain any
    // expression parts, e.g `hello`.
    TemplateNoSubstitutionLiteral,

    // Template literals
    TemplateHead,
    TemplateMiddle,
    TemplateTail,

    // Identifiers
    Identifier,

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
    Await,
    As,
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
    From,
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
    Of,
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

            Token::Identifier => write!(f, "Identifier"),
            Token::NumericLiteral => write!(f, "NumericLiteral"),
            Token::StringLiteral => write!(f, "StringLiteral"),
            Token::BigIntegerLiteral => write!(f, "BigIntegerLiteral"),
            Token::TemplateNoSubstitutionLiteral => write!(f, "TemplateNoSubstitutionLiteral"),

            Token::TemplateHead => write!(f, "TemplateHead"),
            Token::TemplateMiddle => write!(f, "TemplateMiddle"),
            Token::TemplateTail => write!(f, "TemplateTail"),

            // Punctuation
            Token::Ampersand => write!(f, "&"),
            Token::AmpersandAmpersand => write!(f, "&&"),
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
            Token::AmpersandAmpersandEquals => write!(f, "&&="),
            Token::AmpersandEquals => write!(f, "&="),
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
            Token::Await => write!(f, "await"),
            Token::As => write!(f, "as"),
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
            Token::From => write!(f, "from"),
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
            Token::Of => write!(f, "of"),
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
            Token::With => write!(f, "with"),
        }
    }
}

pub fn lookup_identifer(identifier: &str) -> Token {
    match identifier {
        "await" => Token::Await,
        "as" => Token::As,
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
        "from" => Token::From,
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
        "of" => Token::Of,
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
        _ => Token::Identifier,
    }
}
