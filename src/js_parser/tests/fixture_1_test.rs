use js_lexer::Lexer;
use js_parser::Parser;
use logger::LoggerImpl;

#[test]
fn fixture_1_test() {
    let input = "import a from \"./a\";

function main(arg1, arg2, arg3) {
    return arg1 + arg2 * arg3;
}

const result = main(1,2,3);
    ";

    let logger = LoggerImpl::new();
    let lexer = Lexer::new(input, &logger);
    let mut parser = Parser::new(lexer, &logger);
    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 3);
}
