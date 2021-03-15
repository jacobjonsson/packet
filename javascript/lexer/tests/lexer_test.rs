use javascript_lexer::Lexer;
use javascript_token::Token;

macro_rules! token_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            let mut lexer = Lexer::new(input);
            assert_eq!(lexer.next_token(), expected);
        }
    )*
    }
}

// Punctuations
token_tests! {
    ampersand_token: ("&", Token::Ampersand),
    ampersand_ampersand_token: ("&&", Token::AmpersandAmpersand),
    asterisk_token: ("*", Token::Asterisk),
    asterisk_asterisk_token: ("**", Token::AsteriskAsterisk),
    at_token: ("@", Token::At),
    bar_token: ("|", Token::Bar),
    bar_bar_token: ("||", Token::BarBar),
    caret_token: ("^", Token::Caret),
    close_brace_token: ("}}", Token::CloseBrace),
    close_bracket_token: ("]", Token::CloseBracket),
    close_paren_token: (")", Token::CloseParen),
    colon_token: (":", Token::Colon),
    comma_token: (",", Token::Comma),
    dot_token: (".", Token::Dot),
    dot_dot_dot_token: ("...", Token::DotDotDot),
    equals_equals_token: ("==", Token::EqualsEquals),
    equals_equals_equals_token: ("===", Token::EqualsEqualsEquals),
    equals_greater_than: ("=>", Token::EqualsGreaterThan),
    exclamation_token: ("!", Token::Exclamation),
    exclamation_equals_token: ("!=", Token::ExclamationEquals),
    exclamation_equals_equals_token: ("!==", Token::ExclamationEqualsEquals),
    greater_than_token: (">", Token::GreaterThan),
    greater_than_equals_token: (">=", Token::GreaterThanEquals),
    greater_than_greater_than_token: (">>", Token::GreaterThanGreaterThan),
    greater_than_greater_than_greater_than_token: (">>>", Token::GreaterThanGreaterThanGreaterThan),
    less_than_token: ("<", Token::LessThan),
    less_than_equals_token: ("<=", Token::LessThanEquals),
    less_than_less_than_token: ("<<", Token::LessThanLessThan),
    minus_token: ("-", Token::Minus),
    minus_minus_token: ("--", Token::MinusMinus),
    open_brace_token: ("{{", Token::OpenBrace),
    open_bracket_token: ("[", Token::OpenBracket),
    open_paren_token: ("(", Token::OpenParen),
    percent_token: ("%", Token::Percent),
    plus_token: ("+", Token::Plus),
    plus_plus_token: ("++", Token::PlusPlus),
    question_token: ("?", Token::Question),
    question_dot_token: ("?.", Token::QuestionDot),
    question_question_token: ("??", Token::QuestionQuestion),
    semicolon_token: (";", Token::Semicolon),
    slash_token: ("/", Token::Slash),
    tilde_token: ("~", Token::Tilde),
}

// Assignments
token_tests! {
    ampersand_ampersand_equals_token: ("&&=", Token::AmpersandAmpersandEquals),
    ampersand_equals_token: ("&=", Token::AmpersandEquals),
    asterisk_asterisk_equals_token: ("**=", Token::AsteriskAsteriskEquals),
    bar_bar_equals: ("||=", Token::BarBarEquals),
    bar_equals: ("|=", Token::BarEquals),
    caret_equals: ("^=", Token::CaretEquals),
    equals_token: ("=", Token::Equals),
    greater_than_greater_than_equals_token: (">>=", Token::GreaterThanGreaterThanEquals),
    greater_than_greater_than_greater_than_equals_token: (">>>", Token::GreaterThanGreaterThanGreaterThan),
    less_than_less_than_equals_token: ("<<=", Token::LessThanLessThanEquals),
    minus_equals_token: ("-=", Token::MinusEquals),
    percent_equals_token: ("%=", Token::PercentEquals),
    plus_equals_token: ("+=", Token::PlusEquals),
    question_question_equals_token: ("??=", Token::QuestionQuestionEquals),
    slash_equals_token: ("/=", Token::SlashEquals),
}

// Identifiers
token_tests! {
    hello_identifier: ("hello", Token::Identifier("hello".into())),
    hello_world_identifier: ("hello_world", Token::Identifier("hello_world".into())),
    arg1_identifer: ("arg1", Token::Identifier("arg1".into())),
}

// Literals
token_tests! {
    numeric_literal: ("123", Token::NumericLiteral("123".into())),
    string_literal: ("\"hello world\"", Token::StringLiteral("hello world".into())),
    single_quote_string_literal: ("'hello world'", Token::StringLiteral("hello world".into())),
}

// Keywords
token_tests! {
    if_keyword: ("if", Token::If),
    else_keyword: ("else", Token::Else),
    function_keyword: ("function", Token::Function),
    break_keyword: ("break", Token::Break),
    continue_keyword: ("continue", Token::Continue),
}

#[test]
fn tokenize_multiple_lines() {
    let input = "import a from \"a\";
    
    let b = 5;

    5 + 5;
";

    let expected_tokens = vec![
        Token::Import,
        Token::Identifier("a".into()),
        Token::From,
        Token::StringLiteral("a".into()),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("b".into()),
        Token::Equals,
        Token::NumericLiteral("5".into()),
        Token::Semicolon,
        Token::NumericLiteral("5".into()),
        Token::Plus,
        Token::NumericLiteral("5".into()),
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input);
    for token in expected_tokens {
        assert_eq!(lexer.next_token(), token);
    }
}
