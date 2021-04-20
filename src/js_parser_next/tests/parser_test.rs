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
    expect_printed("var async;", "var async;\n");
    expect_printed("var a = 1;", "var a = 1;\n");
    expect_printed("var a = \"b\";", "var a = \"b\";\n");

    expect_error(
        "var yield;",
        JSErrorKind::UnexpectedYieldAsBindingIdentifier,
    );
    expect_error(
        "var await;",
        JSErrorKind::UnexpectedAwaitAsBindingIdentifier,
    );
    expect_error("var let;", JSErrorKind::StrictModeReserved);
    expect_error("var implements;", JSErrorKind::StrictModeReserved);
    expect_error("var package;", JSErrorKind::StrictModeReserved);
    expect_error("var private;", JSErrorKind::StrictModeReserved);
    expect_error("var protected;", JSErrorKind::StrictModeReserved);
    expect_error("var public;", JSErrorKind::StrictModeReserved);
    expect_error("var static;", JSErrorKind::StrictModeReserved);

    // Patterns
    expect_printed("var [a] = \"b\";", "var [a] = \"b\";\n");
    expect_printed("var [...a] = \"b\";", "var [...a] = \"b\";\n");
    expect_printed("var [a = 1] = \"b\";", "var [a = 1] = \"b\";\n");
    expect_printed("var [[[[a]]]] = \"b\";", "var [[[[a]]]] = \"b\";\n");
    expect_printed(
        "var [{a: [b], ...c} = 1] = \"b\";",
        "var [{a: [b], ...c} = 1] = \"b\";\n",
    );
    expect_printed("var {a} = 1;", "var {a} = 1;\n");
    expect_printed("var {async} = 1;", "var {async} = 1;\n");
    expect_printed("var {null: a} = 1;", "var {null: a} = 1;\n");
    expect_printed("var {null: a = 2} = 1;", "var {null: a = 2} = 1;\n");
    expect_printed("var {\"c\": a = 2} = 1;", "var {\"c\": a = 2} = 1;\n");

    expect_error("var {\"a\"} = 1;", JSErrorKind::InvalidShorthandPropertyKey);
    expect_error("var {1} = 1;", JSErrorKind::InvalidShorthandPropertyKey);
    expect_error("var {...a, c} = 1;", JSErrorKind::RestElementMustBeLast);
    expect_error("var {...a,} = 1;", JSErrorKind::RestElementMustBeLast);
    expect_error("var [...a, c] = 1;", JSErrorKind::RestElementMustBeLast);
    expect_error("var [...a,] = 1;", JSErrorKind::RestElementMustBeLast);
    // expect_error("var {null} = 1;", JSErrorKind::UnexpectedKeyword);
    // expect_error("var {default} = 1;", JSErrorKind::UnexpectedKeyword);
}

#[test]
fn test_lexical_bindings() {
    expect_printed("const a = 1;", "const a = 1;\n");
    expect_printed("const a = \"b\";", "const a = \"b\";\n");
    expect_printed("const [a] = \"b\";", "const [a] = \"b\";\n");
    expect_printed("const [...a] = \"b\";", "const [...a] = \"b\";\n");
    expect_printed("const [a = 1] = \"b\";", "const [a = 1] = \"b\";\n");
    expect_printed(
        "const [{a: [b], ...c} = 1] = \"b\";",
        "const [{a: [b], ...c} = 1] = \"b\";\n",
    );
    expect_printed("const [[[[a]]]] = \"b\";", "const [[[[a]]]] = \"b\";\n");
    expect_printed("const {a} = 1;", "const {a} = 1;\n");
    expect_printed("const {null: a} = 1;", "const {null: a} = 1;\n");

    expect_printed("let a;", "let a;\n");
    expect_printed("let a = 1;", "let a = 1;\n");
    expect_printed("let a = \"b\";", "let a = \"b\";\n");
    expect_printed("let [a] = \"b\";", "let [a] = \"b\";\n");
    expect_printed("let [...a] = \"b\";", "let [...a] = \"b\";\n");
    expect_printed("let [a = 1] = \"b\";", "let [a = 1] = \"b\";\n");
    expect_printed("let [[[[a]]]] = \"b\";", "let [[[[a]]]] = \"b\";\n");
    expect_printed("let {a} = 1;", "let {a} = 1;\n");
    expect_printed("let {null: a} = 1;", "let {null: a} = 1;\n");

    expect_error("const a;", JSErrorKind::MissingConstInitializer);
    expect_error("const let;", JSErrorKind::StrictModeReserved);
    expect_error("const implements;", JSErrorKind::StrictModeReserved);
    expect_error("const package;", JSErrorKind::StrictModeReserved);
    expect_error("const private;", JSErrorKind::StrictModeReserved);
    expect_error("const protected;", JSErrorKind::StrictModeReserved);
    expect_error("const public;", JSErrorKind::StrictModeReserved);
    expect_error("const static;", JSErrorKind::StrictModeReserved);
    expect_error(
        "const {\"a\"} = 1;",
        JSErrorKind::InvalidShorthandPropertyKey,
    );
    expect_error("const {1} = 1;", JSErrorKind::InvalidShorthandPropertyKey);

    expect_error("let let;", JSErrorKind::StrictModeReserved);
    expect_error("let implements;", JSErrorKind::StrictModeReserved);
    expect_error("let package;", JSErrorKind::StrictModeReserved);
    expect_error("let private;", JSErrorKind::StrictModeReserved);
    expect_error("let protected;", JSErrorKind::StrictModeReserved);
    expect_error("let public;", JSErrorKind::StrictModeReserved);
    expect_error("let static;", JSErrorKind::StrictModeReserved);
    expect_error("let {\"a\"} = 1;", JSErrorKind::InvalidShorthandPropertyKey);
    expect_error("let {1} = 1;", JSErrorKind::InvalidShorthandPropertyKey);
}
