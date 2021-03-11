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

// Tokens
token_tests! {
    greater_than_token: (">", Token::GreaterThan),
    less_than_token: ("<", Token::LessThan),
    semicolon_token: (";", Token::Semicolon),
    comma_token: (",", Token::Comma),
    plus_token: ("+", Token::Plus),
    minus_token: ("-", Token::Minus),
    slash_token: ("/", Token::Slash),
    asterisk_token: ("*", Token::Asterisk),
    equals_token: ("=", Token::Equals),

    left_paren_token: ("(", Token::LeftParen),
    right_paren_token: (")", Token::RightParen),
    left_brace_token: ("{{", Token::LeftBrace),
    right_brace_token: ("}}", Token::RightBrace),
    left_bracket_token: ("[", Token::LeftBracket),
    right_bracket_token: ("]", Token::RightBracket),
    equals_equals_token: ("==", Token::EqualsEquals),
    equals_equals_equals_token: ("===", Token::EqualsEqualsEquals),
    exclamation_token: ("!", Token::Exclamation),
    exclamation_equals_token: ("!=", Token::ExclamationEquals),
    exclamation_equals_equals_token: ("!==", Token::ExclamationEqualsEquals),
}

// Identifiers
token_tests! {
    hello_identifier: ("hello", Token::Identifier("hello".into())),
    hello_world_identifier: ("hello_world", Token::Identifier("hello_world".into())),
}

// Keywords
token_tests! {
    if_keyword: ("if", Token::If),
    else_keyword: ("else", Token::Else),
    function_keyword: ("function", Token::Function),
    break_keyword: ("break", Token::Break),
    continue_keyword: ("continue", Token::Continue),
}
