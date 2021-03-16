use javascript_lexer::Lexer;
use javascript_parser::Parser;
use javascript_printer::Printer;

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

fn expected_printed(content: &str, expected: &str) {
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    let output = Printer::new().print_program(&program);
    assert_eq!(output, expected);
}

#[test]
fn test_variable_declaration() {
    expected_printed("var a = 1;", "var a = 1;");
    expected_printed("let a = 1;", "let a = 1;");
    expected_printed("const a = 1;", "const a = 1;");
    expected_printed("var a;", "var a;");
    expected_printed("let a;", "let a;");
    expected_printed("const a;", "const a;");
}

#[test]
fn test_infix_expressions() {
    expected_printed("5 + 5", "(5 + 5)");
    expected_printed("5 - 5", "(5 - 5)");
    expected_printed("5 * 5", "(5 * 5)");
    expected_printed("5 / 5", "(5 / 5)");
    expected_printed("5 > 5", "(5 > 5)");
    expected_printed("5 < 5", "(5 < 5)");
    expected_printed("5 == 5", "(5 == 5)");
    expected_printed("5 === 5", "(5 === 5)");
    expected_printed("5 != 5", "(5 != 5)");
    expected_printed("5 !== 5", "(5 !== 5)");
    expected_printed("a + a", "(a + a)");
    expected_printed("a === a", "(a === a)");
    expected_printed("true === true", "(true === true)");
    expected_printed("true !== false", "(true !== false)");
}

#[test]
fn test_operator_precedence_parsing() {
    expected_printed("5 + 5", "(5 + 5)");
    expected_printed("true", "true");
    expected_printed("false", "false");
    expected_printed("5 + 5 + 5", "((5 + 5) + 5)");
    expected_printed("3 > 5 == false", "((3 > 5) == false)");
    expected_printed("3 > 5 == false", "((3 > 5) == false)");
    expected_printed("a + b + c", "((a + b) + c)");
    expected_printed("a + b / c", "(a + (b / c))");
    expected_printed(
        "3 + 4 * 5 == 3 * 1 + 4 * 5",
        "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
    );
    expected_printed("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)");
    expected_printed("(5 + 5) * 2", "((5 + 5) * 2)");
    expected_printed("2 / (5 + 5)", "(2 / (5 + 5))");
    expected_printed("-(5 + 5)", "(-(5 + 5))");
    expected_printed("!(true == true)", "(!(true == true))");
}

#[test]
fn test_import_statement() {
    expected_printed("import a from \"b\"", "import a from \"b\"");
    expected_printed("import { a } from \"b\"", "import { a } from \"b\"");
    expected_printed("import { a, b } from \"b\"", "import { a, b } from \"b\"");
    expected_printed(
        "import { a as b } from \"b\"",
        "import { a as b } from \"b\"",
    );
    expected_printed("import { a, b } from \"b\"", "import { a, b } from \"b\"");
    expected_printed(
        "import { a as b, b as c } from \"b\"",
        "import { a as b, b as c } from \"b\"",
    );
    expected_printed(
        "import a, { b as c } from \"b\"",
        "import a, { b as c } from \"b\"",
    );
    expected_printed("import a, { b } from \"b\"", "import a, { b } from \"b\"");
    expected_printed("import * as a from \"b\"", "import * as a from \"b\"");
    expected_printed("import a, * as b from \"b\"", "import a, * as b from \"b\"");
}

#[test]
fn parse_function_declaration() {
    expected_printed("function a() {}", "function a() {}");
    expected_printed("function a(b, c) {}", "function a(b, c) {}");
    expected_printed(
        "function a(b, c) { return b + c; }",
        "function a(b, c) { return (b + c); }",
    );
}

#[test]
fn parse_return_statement() {
    expected_printed("return;", "return;");
    expected_printed("return 5;", "return 5;");
    expected_printed("return 5 + 5;", "return (5 + 5);");
}

#[test]
fn parse_call_expression() {
    expected_printed("a()", "a()");
    expected_printed("a(a)", "a(a)");
    expected_printed("a(a, b)", "a(a, b)");
    expected_printed("a(3 + 3)", "a((3 + 3))");
}

#[test]
fn parse_if_statement() {
    expected_printed("if (true) {}", "if (true) {}");
    expected_printed("if (true) {} else {}", "if (true) {} else {}");
    expected_printed("if (x < 10) { return 10; }", "if ((x < 10)) { return 10; }");
    expected_printed(
        "if (false) {} else if (true) {}",
        "if (false) {} else if (true) {}",
    );
    expected_printed(
        "if (false) {} function a() {}",
        "if (false) {}function a() {}",
    );
}

#[test]
fn parse_function_expression() {
    expected_printed("let a = function() {}", "let a = function() {};");
    expected_printed("a(function() {})", "a(function() {})");
}

#[test]
fn parse_conditional_expression() {
    expected_printed("true ? 1 : 2", "true ? 1 : 2");
    expected_printed("3 > 2 ? 3 + 2 : 3 * 2", "(3 > 2) ? (3 + 2) : (3 * 2)");
}
