use js_lexer::Lexer;
use js_token::Token;
use logger::LoggerImpl;

#[test]
fn tokenize_multiple_lines() {
    let input = "import a from \"a\";

    let b = 5;

    5 + 5;
";

    let expected_tokens = vec![
        (Token::Import, None),
        (Token::Identifier, Some("a")),
        (Token::From, None),
        (Token::StringLiteral, Some("a")),
        (Token::Semicolon, None),
        (Token::Let, None),
        (Token::Identifier, Some("b")),
        (Token::Equals, None),
        (Token::NumericLiteral, Some("5")),
        (Token::Semicolon, None),
        (Token::NumericLiteral, Some("5")),
        (Token::Plus, None),
        (Token::NumericLiteral, Some("5")),
        (Token::Semicolon, None),
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
    assert_eq!(lexer.token, Token::StringLiteral);
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
    assert_eq!(lexer.token, Token::Identifier);
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
        ("&", Token::Ampersand),
        ("&&", Token::AmpersandAmpersand),
        ("*", Token::Asterisk),
        ("**", Token::AsteriskAsterisk),
        ("@", Token::At),
        ("|", Token::Bar),
        ("||", Token::BarBar),
        ("^", Token::Caret),
        ("}}", Token::CloseBrace),
        ("]", Token::CloseBracket),
        (")", Token::CloseParen),
        (":", Token::Colon),
        (",", Token::Comma),
        (".", Token::Dot),
        ("...", Token::DotDotDot),
        ("==", Token::EqualsEquals),
        ("===", Token::EqualsEqualsEquals),
        ("=>", Token::EqualsGreaterThan),
        ("!", Token::Exclamation),
        ("!=", Token::ExclamationEquals),
        ("!==", Token::ExclamationEqualsEquals),
        (">", Token::GreaterThan),
        (">=", Token::GreaterThanEquals),
        (">>", Token::GreaterThanGreaterThan),
        (">>>", Token::GreaterThanGreaterThanGreaterThan),
        ("<", Token::LessThan),
        ("<=", Token::LessThanEquals),
        ("<<", Token::LessThanLessThan),
        ("-", Token::Minus),
        ("--", Token::MinusMinus),
        ("{{", Token::OpenBrace),
        ("[", Token::OpenBracket),
        ("(", Token::OpenParen),
        ("%", Token::Percent),
        ("+", Token::Plus),
        ("++", Token::PlusPlus),
        ("?", Token::Question),
        ("?.", Token::QuestionDot),
        ("??", Token::QuestionQuestion),
        (";", Token::Semicolon),
        ("/", Token::Slash),
        ("~", Token::Tilde),
        //
        // Assignments
        ("&&=", Token::AmpersandAmpersandEquals),
        ("&=", Token::AmpersandEquals),
        ("*=", Token::AsteriskEquals),
        ("**=", Token::AsteriskAsteriskEquals),
        ("||=", Token::BarBarEquals),
        ("|=", Token::BarEquals),
        ("^=", Token::CaretEquals),
        ("=", Token::Equals),
        (">>=", Token::GreaterThanGreaterThanEquals),
        (">>>", Token::GreaterThanGreaterThanGreaterThan),
        ("<<=", Token::LessThanLessThanEquals),
        ("-=", Token::MinusEquals),
        ("%=", Token::PercentEquals),
        ("+=", Token::PlusEquals),
        ("??=", Token::QuestionQuestionEquals),
        ("/=", Token::SlashEquals),
        //
        // Keywords
        ("await", Token::Await),
        ("as", Token::As),
        ("break", Token::Break),
        ("case", Token::Case),
        ("catch", Token::Catch),
        ("class", Token::Class),
        ("const", Token::Const),
        ("continue", Token::Continue),
        ("debugger", Token::Debugger),
        ("default", Token::Default),
        ("delete", Token::Delete),
        ("do", Token::Do),
        ("else", Token::Else),
        ("enum", Token::Enum),
        ("export", Token::Export),
        ("extends", Token::Extends),
        ("false", Token::False),
        ("finally", Token::Finally),
        ("for", Token::For),
        ("function", Token::Function),
        ("if", Token::If),
        ("import", Token::Import),
        ("in", Token::In),
        ("instanceof", Token::Instanceof),
        ("new", Token::New),
        ("null", Token::Null),
        ("return", Token::Return),
        ("super", Token::Super),
        ("switch", Token::Switch),
        ("this", Token::This),
        ("throw", Token::Throw),
        ("true", Token::True),
        ("try", Token::Try),
        ("typeof", Token::Typeof),
        ("var", Token::Var),
        ("void", Token::Void),
        ("while", Token::While),
        ("with", Token::With),
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
    assert_eq!(lexer.token, Token::NumericLiteral);
    assert_eq!(lexer.token_value, expected);
}

#[test]
fn test_numeric_literals() {
    expect_number("1", "1");
    expect_number("120", "120");
}

#[test]
fn test_comments() {
    let input = "// comment 1
    let a;
    /* comment 2 */
    let b;
    ";

    let tokens = vec![
        Token::Let,
        Token::Identifier,
        Token::Semicolon,
        Token::Let,
        Token::Identifier,
        Token::Semicolon,
    ];

    let logger = LoggerImpl::new();
    let mut lexer = Lexer::new(input, &logger);
    for (idx, token) in tokens.iter().enumerate() {
        if idx != 0 {
            lexer.next_token();
        }
        assert_eq!(token, &lexer.token);
    }
}