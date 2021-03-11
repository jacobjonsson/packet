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
    open_paren_token: ("(", Token::OpenParen),
    close_paren_token: (")", Token::CloseParen),
    open_brace_token: ("{{", Token::OpenBrace),
    close_brace_token: ("}}", Token::CloseBrace),
    open_bracket_token: ("[", Token::OpenBracket),
    close_bracket_token: ("]", Token::CloseBracket),
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

// Literals
token_tests! {
    numeric_literal: ("123", Token::NumericLiteral("123".into())),
}

// Keywords
token_tests! {
    if_keyword: ("if", Token::If),
    else_keyword: ("else", Token::Else),
    function_keyword: ("function", Token::Function),
    break_keyword: ("break", Token::Break),
    continue_keyword: ("continue", Token::Continue),
}
