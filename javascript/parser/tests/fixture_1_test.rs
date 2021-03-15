use javascript_lexer::Lexer;
use javascript_parser::Parser;

fn check_parser_errors(parser: &Parser) {
    let errors = parser.errors();
    if errors.len() > 0 {
        println!("Parser has parser errors:");
        for error in errors {
            println!("parser error: {}", error);
        }
        panic!("PARSER ERROR");
    }
}

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
    check_parser_errors(&parser);
    assert_eq!(program.statements.len(), 3);
}
