use js_lexer::{self, scan_next_token};
use js_token::Token;

enum StringOrFloat<'a> {
    String(&'a str),
    Float(f64),
}

#[test]
fn tokenize_multiple_lines() {
    let input = "import a from \"a\";

    let b = 5;

    5 + 5;
";

    let expected_tokens = vec![
        (Token::Import, None),
        (Token::Identifier, Some(StringOrFloat::String("a"))),
        (Token::From, None),
        (Token::StringLiteral, Some(StringOrFloat::String("a"))),
        (Token::Semicolon, None),
        (Token::Let, None),
        (Token::Identifier, Some(StringOrFloat::String("b"))),
        (Token::Equals, None),
        (Token::NumericLiteral, Some(StringOrFloat::Float(5.))),
        (Token::Semicolon, None),
        (Token::NumericLiteral, Some(StringOrFloat::Float(5.))),
        (Token::Plus, None),
        (Token::NumericLiteral, Some(StringOrFloat::Float(5.))),
        (Token::Semicolon, None),
    ];

    let mut lexer = js_lexer::create(input);
    for (idx, token) in expected_tokens.iter().enumerate() {
        if idx != 0 {
            js_lexer::scan_next_token(&mut lexer);
        }
        assert_eq!(&lexer.token, &token.0);
        if let Some(value) = &token.1 {
            match value {
                StringOrFloat::Float(f) => assert_eq!(&lexer.number, f),
                StringOrFloat::String(s) => assert_eq!(&lexer.identifier, s),
            }
        }
    }
}

fn expect_string_literal(content: &str, expected: &str) {
    let lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::StringLiteral);
    assert_eq!(lexer.identifier, expected);
}

#[test]
fn test_string_literal() {
    expect_string_literal("       'a'", "a");
    expect_string_literal("\"a\"", "a");
    expect_string_literal("'\n'", "\n");
    expect_string_literal("'\"'", "\"");
    expect_string_literal("\"'\"", "'");
    expect_string_literal("\"\\\"\"", "\\\"");
    expect_string_literal("\n\n\r  \t\"hello world\"", "hello world");
}

fn expect_identifier(content: &str, expected: &str) {
    let lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::Identifier);
    assert_eq!(lexer.identifier, expected);
}

#[test]
fn test_identifiers() {
    expect_identifier("a", "a");
    expect_identifier("a1", "a1");
    expect_identifier("a_a", "a_a");
    expect_identifier("$", "$");
    expect_identifier("_$", "_$");
    // expect_identifier("\\u0061s", "\\u0061"); // TODO
}

fn expect_regexp(content: &str, expected: &str) {
    let mut lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::Slash);
    js_lexer::scan_regexp(&mut lexer);
    assert_eq!(js_lexer::raw(&lexer), expected);
}

#[test]
fn test_regexp() {
    expect_regexp("/hello/gi", "/hello/gi");
    expect_regexp("/hello/", "/hello/");
    expect_regexp(
        "/^<(\\w+)\\s*\\/?>(?:<\\/\\1>|)$/",
        "/^<(\\w+)\\s*\\/?>(?:<\\/\\1>|)$/",
    )
}

#[test]
fn test_tokens() {
    let tests = vec![
        // Punctuations
        ("&", Token::Ampersand),
        ("&&", Token::AmpersandAmpersand),
        ("*", Token::Asterisk),
        ("**", Token::AsteriskAsterisk),
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
        let lexer = js_lexer::create(test.0);
        assert_eq!(lexer.token, test.1);
    }
}

fn expect_number(content: &str, expected: f64) {
    let lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::NumericLiteral);
    assert_eq!(lexer.number, expected);
}

#[test]
fn test_numeric_literals() {
    expect_number("1", 1.);
    expect_number("120", 120.);
    expect_number("120.2", 120.2);
    expect_number("1_2_0_2", 1202.);
    expect_number(".1", 0.1);
    expect_number("0b10", 2.);
    expect_number("0o10", 8.);
    expect_number("0x10", 16.);
}

fn expect_big_int(content: &str, expected: &str) {
    let lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::BigIntegerLiteral);
    assert_eq!(lexer.identifier, expected);
}

#[test]
fn test_big_int_literal() {
    expect_big_int("1n", "1");
    expect_big_int("2000000000n", "2000000000");
    expect_big_int("0b10n", "0b10");
    expect_big_int("0o10n", "0o10");
    expect_big_int("0x10n", "0x10");
}

fn expect_eof(content: &str) {
    let lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::EndOfFile);
}

#[test]
fn test_comments() {
    expect_eof("//");
    expect_eof("/* */");
    expect_eof("/**  **/");
    expect_eof(
        "/**
    *
    *
    **/",
    );
}

fn expect_no_substitution_template_literal(content: &str, expected: &str) {
    let lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::TemplateNoSubstitutionLiteral);
    assert_eq!(lexer.identifier, expected);
}

#[test]
fn test_no_substitution_template_literals() {
    expect_no_substitution_template_literal("`hello world`", "hello world");
    expect_no_substitution_template_literal("`$`", "$");
    expect_no_substitution_template_literal("`$}`", "$}");
    expect_no_substitution_template_literal("`}`", "}");
    expect_no_substitution_template_literal("`\\``", "\\`");
}

struct TemplateLiteralPart<'a> {
    text: &'a str,
    expression_tokens: Vec<Token>,
}

fn expect_template_literals(content: &str, head: &str, parts: Vec<TemplateLiteralPart>) {
    let mut lexer = js_lexer::create(content);
    assert_eq!(lexer.token, Token::TemplateHead);
    assert_eq!(lexer.identifier, head);
    for part in &parts {
        for token in &part.expression_tokens {
            js_lexer::scan_next_token(&mut lexer);
            assert_eq!(lexer.token, *token);
        }
        js_lexer::scan_next_token(&mut lexer);
        js_lexer::scan_template_tail_or_middle(&mut lexer);
        assert_eq!(lexer.identifier, part.text);
    }
}

#[test]
fn test_template_literals() {
    expect_template_literals(
        "`head ${a} \\` middle ${b} tail`",
        "head ",
        vec![
            TemplateLiteralPart {
                expression_tokens: vec![Token::Identifier],
                text: " \\` middle ",
            },
            TemplateLiteralPart {
                expression_tokens: vec![Token::Identifier],
                text: " tail",
            },
        ],
    )
}

fn expect_token_sequence(content: &str, tokens: &[Token]) {
    let mut lexer = js_lexer::create(content);
    assert_eq!(lexer.token, tokens[0]);
    for token in &tokens[1..] {
        scan_next_token(&mut lexer);
        assert_eq!(lexer.token, *token);
    }
}

#[test]
fn test_token_sequence() {
    expect_token_sequence(
        "++--+-*/[]",
        &vec![
            Token::PlusPlus,
            Token::MinusMinus,
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::OpenBracket,
            Token::CloseBracket,
        ],
    );
}
