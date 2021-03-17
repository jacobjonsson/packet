use javascript_lexer::Lexer;
use javascript_parser::Parser;

#[test]
fn fixture_1_test() {
    let input = "import a from \"./a\";

function main(arg1, arg2, arg3) {
    return arg1 + arg2 * arg3;
}

const result = main(1,2,3);
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 3);
}
