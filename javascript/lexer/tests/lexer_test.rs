use javascript_lexer::Lexer;
use javascript_token::TokenType;
use logger::LoggerImpl;

#[test]
fn tokenize_multiple_lines() {
    let input = "import a from \"a\";

    let b = 5;

    5 + 5;
";

    let expected_tokens = vec![
        (TokenType::Import, None),
        (TokenType::Identifier, Some("a")),
        (TokenType::From, None),
        (TokenType::StringLiteral, Some("a")),
        (TokenType::Semicolon, None),
        (TokenType::Let, None),
        (TokenType::Identifier, Some("b")),
        (TokenType::Equals, None),
        (TokenType::NumericLiteral, Some("5")),
        (TokenType::Semicolon, None),
        (TokenType::NumericLiteral, Some("5")),
        (TokenType::Plus, None),
        (TokenType::NumericLiteral, Some("5")),
        (TokenType::Semicolon, None),
    ];

    let logger = LoggerImpl::new();
    let mut lexer = Lexer::new(input, &logger);
    for (idx, token) in expected_tokens.iter().enumerate() {
        if idx != 0 {
            lexer.next_token();
        }
        assert_eq!(&lexer.token, &token.0);
        if let Some(value) = token.1 {
            assert_eq!(lexer.token_value, value);
        }
    }
}

fn expect_string_literal(content: &str, expected: &str) {
    let logger = LoggerImpl::new();
    let lexer = Lexer::new(content, &logger);
    assert_eq!(lexer.token, TokenType::StringLiteral);
    assert_eq!(lexer.token_value, expected);
}

#[test]
fn test_string_literal() {
    expect_string_literal("'a'", "a");
    expect_string_literal("\"a\"", "a");
    expect_string_literal("'\n'", "\n");
    expect_string_literal("'\"'", "\"");
    expect_string_literal("\"'\"", "'");
}

fn expect_identifier(content: &str, expected: &str) {
    let logger = LoggerImpl::new();
    let lexer = Lexer::new(content, &logger);
    assert_eq!(lexer.token, TokenType::Identifier);
    assert_eq!(lexer.token_value, expected);
}

#[test]
fn test_identifiers() {
    expect_identifier("a", "a");
    expect_identifier("a1", "a1");
    expect_identifier("a_a", "a_a");
}

#[test]
fn test_tokens() {
    let tests = vec![
        // Punctuations
        ("&", TokenType::Ampersand),
        ("&&", TokenType::AmpersandAmpersand),
        ("*", TokenType::Asterisk),
        ("**", TokenType::AsteriskAsterisk),
        ("@", TokenType::At),
        ("|", TokenType::Bar),
        ("||", TokenType::BarBar),
        ("^", TokenType::Caret),
        ("}}", TokenType::CloseBrace),
        ("]", TokenType::CloseBracket),
        (")", TokenType::CloseParen),
        (":", TokenType::Colon),
        (",", TokenType::Comma),
        (".", TokenType::Dot),
        ("...", TokenType::DotDotDot),
        ("==", TokenType::EqualsEquals),
        ("===", TokenType::EqualsEqualsEquals),
        ("=>", TokenType::EqualsGreaterThan),
        ("!", TokenType::Exclamation),
        ("!=", TokenType::ExclamationEquals),
        ("!==", TokenType::ExclamationEqualsEquals),
        (">", TokenType::GreaterThan),
        (">=", TokenType::GreaterThanEquals),
        (">>", TokenType::GreaterThanGreaterThan),
        (">>>", TokenType::GreaterThanGreaterThanGreaterThan),
        ("<", TokenType::LessThan),
        ("<=", TokenType::LessThanEquals),
        ("<<", TokenType::LessThanLessThan),
        ("-", TokenType::Minus),
        ("--", TokenType::MinusMinus),
        ("{{", TokenType::OpenBrace),
        ("[", TokenType::OpenBracket),
        ("(", TokenType::OpenParen),
        ("%", TokenType::Percent),
        ("+", TokenType::Plus),
        ("++", TokenType::PlusPlus),
        ("?", TokenType::Question),
        ("?.", TokenType::QuestionDot),
        ("??", TokenType::QuestionQuestion),
        (";", TokenType::Semicolon),
        ("/", TokenType::Slash),
        ("~", TokenType::Tilde),
        //
        // Assignments
        ("&&=", TokenType::AmpersandAmpersandEquals),
        ("&=", TokenType::AmpersandEquals),
        ("*=", TokenType::AsteriskEquals),
        ("**=", TokenType::AsteriskAsteriskEquals),
        ("||=", TokenType::BarBarEquals),
        ("|=", TokenType::BarEquals),
        ("^=", TokenType::CaretEquals),
        ("=", TokenType::Equals),
        (">>=", TokenType::GreaterThanGreaterThanEquals),
        (">>>", TokenType::GreaterThanGreaterThanGreaterThan),
        ("<<=", TokenType::LessThanLessThanEquals),
        ("-=", TokenType::MinusEquals),
        ("%=", TokenType::PercentEquals),
        ("+=", TokenType::PlusEquals),
        ("??=", TokenType::QuestionQuestionEquals),
        ("/=", TokenType::SlashEquals),
        //
        // Keywords
        ("await", TokenType::Await),
        ("as", TokenType::As),
        ("break", TokenType::Break),
        ("case", TokenType::Case),
        ("catch", TokenType::Catch),
        ("class", TokenType::Class),
        ("const", TokenType::Const),
        ("continue", TokenType::Continue),
        ("debugger", TokenType::Debugger),
        ("default", TokenType::Default),
        ("delete", TokenType::Delete),
        ("do", TokenType::Do),
        ("else", TokenType::Else),
        ("enum", TokenType::Enum),
        ("export", TokenType::Export),
        ("extends", TokenType::Extends),
        ("false", TokenType::False),
        ("finally", TokenType::Finally),
        ("for", TokenType::For),
        ("function", TokenType::Function),
        ("if", TokenType::If),
        ("import", TokenType::Import),
        ("in", TokenType::In),
        ("instanceof", TokenType::Instanceof),
        ("new", TokenType::New),
        ("null", TokenType::Null),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("switch", TokenType::Switch),
        ("this", TokenType::This),
        ("throw", TokenType::Throw),
        ("true", TokenType::True),
        ("try", TokenType::Try),
        ("typeof", TokenType::Typeof),
        ("var", TokenType::Var),
        ("void", TokenType::Void),
        ("while", TokenType::While),
        ("with", TokenType::With),
    ];

    for test in tests {
        let logger = LoggerImpl::new();
        let lexer = Lexer::new(test.0, &logger);
        assert_eq!(lexer.token, test.1);
    }
}

fn expect_number(content: &str, expected: &str) {
    let logger = LoggerImpl::new();
    let lexer = Lexer::new(content, &logger);
    assert_eq!(lexer.token, TokenType::NumericLiteral);
    assert_eq!(lexer.token_value, expected);
}

#[test]
fn test_numeric_literals() {
    expect_number("1", "1");
    expect_number("120", "120");
}
