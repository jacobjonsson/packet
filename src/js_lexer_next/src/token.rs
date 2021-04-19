#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Literals
    /// A big int literal: "123n", "0b1n", "0o1n", "0x1n"
    BigInt,
    /// A numeric literal: "123", ".123", "0b1", "0o1", "0x1"
    Number,
    /// A string literal: ""abc"", "'abc'", "`abc`"
    String,

    /// The start of a template literal: "`abc${"
    TemplateHead,
    /// The middle part of a template literal: "}abc${"
    TemplateMiddle,
    /// The end of a template literal: "}abc`"
    TemplateTail,

    /// A regexp literal, i.e /abc/g
    /// 0 is the pattern
    /// 1 is the flags
    Regexp,

    // Identifiers
    Identifier,

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

impl Token {
    /// Creates a new token kind by first checking if the given identifier
    /// is a keyword and if so returns the keyword, otherwise returns an identifier
    pub fn from_potential_keyword(identifier: &str) -> Token {
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
            _ => Token::Identifier,
        }
    }

    /// Returns true if the token might indicate the start of a property key
    pub fn is_property_key(&self) -> bool {
        match self {
            Token::OpenBracket
            | Token::Identifier
            | Token::String
            | Token::Number
            | Token::False
            | Token::True => true,
            _ => false,
        }
    }

    /// Does the token indicate the potential start of pattern
    pub fn is_pattern_start(&self) -> bool {
        match self {
            Token::OpenBracket | Token::OpenBrace => true,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier => true,
            _ => false,
        }
    }

    pub fn is_identifier_or_pattern(&self) -> bool {
        self.is_identifier() || self.is_pattern_start()
    }
}
