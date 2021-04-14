use span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    /// Creates a new token
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,

    // Literals
    /// A big int literal: "123n", "0b1n", "0o1n", "0x1n"
    BigInt {
        value: String,
    },
    /// A numeric literal: "123", ".123", "0b1", "0o1", "0x1"
    Number {
        value: f64,
    },
    /// A string literal: ""abc"", "'abc'", "`abc`"
    String {
        value: String,
    },

    /// The start of a template literal: "`abc${"
    TemplateHead {
        value: String,
    },
    /// The middle part of a template literal: "}abc${"
    TemplateMiddle {
        value: String,
    },
    /// The end of a template literal: "}abc`"
    TemplateTail {
        value: String,
    },

    /// A regexp literal, i.e /abc/g
    Regexp {
        pattern: String,
        flags: Option<String>,
    },

    // Identifiers
    Identifier {
        name: String,
    },

    // Tokens
    /// "&"
    Ampersand,
    /// "&&"
    AmpersandAmpersand,
    /// "&&="
    AmpersandAmpersandEquals,
    /// "&="
    AmpersandEquals,
    /// "*"
    Asterisk,
    /// "**"
    AsteriskAsterisk,
    /// "**="
    AsteriskAsteriskEquals,
    /// "*="
    AsteriskEquals,
    /// "|"
    Bar,
    /// "||"
    BarBar,
    /// "||="
    BarBarEquals,
    /// "|="
    BarEquals,
    /// "^"
    Caret,
    /// "^="
    CaretEquals,
    /// "}"
    CloseBrace,
    /// "]"
    CloseBracket,
    /// ")"
    CloseParen,
    /// ":"
    Colon,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "..."
    DotDotDot,
    /// "="
    Equals,
    /// "=="
    EqualsEquals,
    /// "==="
    EqualsEqualsEquals,
    /// "=>"
    EqualsGreaterThan,
    /// "!"
    Exclamation,
    /// "!="
    ExclamationEquals,
    /// "!=="
    ExclamationEqualsEquals,
    /// ">"
    GreaterThan,
    /// ">="
    GreaterThanEquals,
    /// ">>"
    GreaterThanGreaterThan,
    /// ">>="
    GreaterThanGreaterThanEquals,
    /// ">>>"
    GreaterThanGreaterThanGreaterThan,
    /// ">>>="
    GreaterThanGreaterThanGreaterThanEquals,
    /// "<"
    LessThan,
    /// "<="
    LessThanEquals,
    /// "<<"
    LessThanLessThan,
    /// "<<="
    LessThanLessThanEquals,
    /// "-"
    Minus,
    /// "-="
    MinusEquals,
    /// "--"
    MinusMinus,
    /// "{"
    OpenBrace,
    /// "["
    OpenBracket,
    /// "("
    OpenParen,
    /// "%"
    Percent,
    /// "&="
    PercentEquals,
    /// "+"
    Plus,
    /// "+="
    PlusEquals,
    /// "++"
    PlusPlus,
    /// "?"
    Question,
    /// "?."
    QuestionDot,
    /// "??"
    QuestionQuestion,
    /// "??="
    QuestionQuestionEquals,
    /// ";"
    Semicolon,
    /// "/"
    Slash,
    /// "/="
    SlashEquals,
    /// "~"
    Tilde,

    // Reserved words
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

impl TokenKind {
    /// Creates a new token kind by first checking if the given identifier
    /// is a keyword and if so returns the keyword, otherwise returns an identifier
    pub fn from_potential_keyword(identifier: &str) -> TokenKind {
        match identifier {
            "break" => TokenKind::Break,
            "case" => TokenKind::Case,
            "catch" => TokenKind::Catch,
            "class" => TokenKind::Class,
            "const" => TokenKind::Const,
            "continue" => TokenKind::Continue,
            "debugger" => TokenKind::Debugger,
            "default" => TokenKind::Default,
            "delete" => TokenKind::Delete,
            "do" => TokenKind::Do,
            "else" => TokenKind::Else,
            "enum" => TokenKind::Enum,
            "export" => TokenKind::Export,
            "extends" => TokenKind::Extends,
            "false" => TokenKind::False,
            "finally" => TokenKind::Finally,
            "for" => TokenKind::For,
            "function" => TokenKind::Function,
            "if" => TokenKind::If,
            "import" => TokenKind::Import,
            "in" => TokenKind::In,
            "instanceof" => TokenKind::Instanceof,
            "new" => TokenKind::New,
            "null" => TokenKind::Null,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "switch" => TokenKind::Switch,
            "this" => TokenKind::This,
            "throw" => TokenKind::Throw,
            "true" => TokenKind::True,
            "try" => TokenKind::Try,
            "typeof" => TokenKind::Typeof,
            "var" => TokenKind::Var,
            "void" => TokenKind::Void,
            "while" => TokenKind::While,
            "with" => TokenKind::With,
            c => TokenKind::Identifier { name: c.into() },
        }
    }
}
