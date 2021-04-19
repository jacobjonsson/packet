use js_codegen::Codegen;
use js_error::JSErrorKind;
use js_lexer_next::Lexer;
use js_parser_next::Parser;

fn expect_printed(content: &str, expected: &str) {
    let lexer = Lexer::new(content);
    let ast = Parser::new(lexer).parse().unwrap();
    let output = Codegen::new().generate(ast);
    assert_eq!(output, expected);
}

fn expect_error(content: &str, kind: JSErrorKind) {
    let lexer = Lexer::new(content);
    let error = Parser::new(lexer).parse().unwrap_err();
    assert_eq!(error.kind, kind);
}

#[test]
fn test_array_expressions() {
    expect_printed("[]", "[];\n");
    expect_printed("[,]", "[,];\n");
    expect_printed("[,,]", "[,,];\n");
    expect_printed("[1]", "[1];\n");
    expect_printed("[1,]", "[1];\n");
    expect_printed("[1,2]", "[1,2];\n");
    expect_printed("[1,,2]", "[1,,2];\n");
    expect_printed("[,,1,2]", "[,,1,2];\n");
    expect_printed("[1,2,,]", "[1,2,,];\n");
    expect_printed("[\"h\",1,2]", "[\"h\",1,2];\n");
}

#[test]
fn test_string_expressions() {
    expect_printed("'a'", "\"a\";\n");
    expect_printed("\"a\"", "\"a\";\n");
}

#[test]
fn test_boolean_expressions() {
    expect_printed("true", "true;\n");
    expect_printed("false", "false;\n");
}

#[test]
fn test_numeric_expressions() {
    expect_printed("123", "123;\n");
    expect_printed("1_23", "123;\n");
    expect_printed("0b10", "2;\n");
    expect_printed("0o10", "8;\n");
    expect_printed("10", "10;\n");
    expect_printed("0x10", "16;\n");
}

#[test]
fn test_regexp() {
    expect_printed("/x/g", "/x/g;\n");
    expect_printed("/x/i", "/x/i;\n");
    expect_printed("/x/m", "/x/m;\n");
    expect_printed("/x/s", "/x/s;\n");
    expect_printed("/x/u", "/x/u;\n");
    expect_printed("/x/y", "/x/y;\n");
}

#[test]
fn test_variable_statements() {
    expect_printed("var a;", "var a;\n");
    expect_printed("var let;", "var let;\n");
    expect_printed("var async;", "var async;\n");
    expect_printed("var a = 1;", "var a = 1;\n");
    expect_printed("var a = \"b\";", "var a = \"b\";\n");
}

#[test]
fn test_lexical_bindings() {
    expect_printed("const a = 1;", "const a = 1;\n");
    expect_printed("const a = \"b\";", "const a = \"b\";\n");

    expect_error("const a;", JSErrorKind::MissingConstInitializer);
}
