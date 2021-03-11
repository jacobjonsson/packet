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

token_tests! {
    identifier_1: ("hello", Token::Identifier("hello".into())),
    identifier_2: ("hello_world", Token::Identifier("hello_world".into())),
    if_token: ("if", Token::If),
    else_token: ("else", Token::Else),
    function_token: ("function", Token::Function),
    break_token: ("break", Token::Break),
    continue_token: ("continue", Token::Continue),
}
