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

    // Variable declarations
    Var,
    Let,
    Const,

    // Keywords
    Break,
    Case,
    Catch,
    Class,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
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
    Void,
    While,
    With,

    // Strict mode keywords
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,

    // Contextual keywords
    As,
    Async,
    Await,
    Constructor,
    Get,
    Set,
    From,
    Of,
    Enum,
}

impl Token {
    /// Creates a new token kind by first checking if the given identifier
    /// is a keyword and if so returns the keyword, otherwise returns an identifier
    pub fn from_potential_keyword(identifier: &str) -> Token {
        match identifier {
            "as" => Token::As,
            "async" => Token::Async,
            "await" => Token::Await,
            "break" => Token::Break,
            "case" => Token::Case,
            "catch" => Token::Catch,
            "class" => Token::Class,
            "const" => Token::Const,
            "constructor" => Token::Constructor,
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
            "from" => Token::From,
            "function" => Token::Function,
            "get" => Token::Get,
            "if" => Token::If,
            "implements" => Token::Implements,
            "import" => Token::Import,
            "in" => Token::In,
            "instanceof" => Token::Instanceof,
            "interface" => Token::Interface,
            "let" => Token::Let,
            "new" => Token::New,
            "null" => Token::Null,
            "of" => Token::Of,
            "package" => Token::Package,
            "private" => Token::Private,
            "protected" => Token::Protected,
            "public" => Token::Public,
            "return" => Token::Return,
            "set" => Token::Set,
            "static" => Token::Static,
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
            "yield" => Token::Yield,

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
            Token::Identifier
            | Token::As
            | Token::Async
            | Token::Await
            | Token::Constructor
            | Token::Get
            | Token::Set
            | Token::From
            | Token::Of => true,
            _ => false,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            Token::Break
            | Token::Case
            | Token::Catch
            | Token::Class
            | Token::Continue
            | Token::Debugger
            | Token::Default
            | Token::Delete
            | Token::Do
            | Token::Else
            | Token::Export
            | Token::Extends
            | Token::False
            | Token::Finally
            | Token::For
            | Token::Function
            | Token::If
            | Token::Import
            | Token::In
            | Token::Instanceof
            | Token::New
            | Token::Null
            | Token::Return
            | Token::Super
            | Token::Switch
            | Token::This
            | Token::Throw
            | Token::True
            | Token::Try
            | Token::Typeof
            | Token::Void
            | Token::While
            | Token::With => true,
            _ => false,
        }
    }

    pub fn is_future_reserved(&self) -> bool {
        match self {
            Token::Implements
            | Token::Interface
            | Token::Package
            | Token::Private
            | Token::Protected
            | Token::Public
            | Token::Static
            | Token::Yield
            | Token::Let => true,
            _ => false,
        }
    }

    pub fn is_identifier_or_pattern(&self) -> bool {
        self.is_identifier() || self.is_pattern_start()
    }
}
