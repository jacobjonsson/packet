use js_ast::{
    expression::{
        BinaryExpression, BinaryOperator, BooleanExpression, Expression, Identifier,
        IntegerLiteral, LogicalExpression, LogicalOperator,
    },
    statement::{ExpressionStatement, Statement},
    Program,
};
use js_lexer::Lexer;
use js_parser::Parser;
use js_printer::Printer;
use logger::LoggerImpl;

fn expect_printed(content: &str, expected: &str) {
    let logger = LoggerImpl::new();
    let lexer = Lexer::new(content, &logger);
    let mut parser = Parser::new(lexer, &logger);
    let program = parser.parse_program();
    let output = Printer::new().print_program(&program);
    assert_eq!(output, expected);
}

fn expect_ast(content: &str, expected: Program) {
    let logger = LoggerImpl::new();
    let lexer = Lexer::new(content, &logger);
    let mut parser = Parser::new(lexer, &logger);
    let program = parser.parse_program();
    assert_eq!(program, expected);
}

#[test]
fn test_string_literal() {
    expect_printed("\"hello_world\"", "\"hello_world\"");
    expect_printed("'hello_world'", "\"hello_world\"");
}

#[test]
fn test_variable_declaration() {
    expect_printed("var a = 1;", "var a = 1;");
    expect_printed("let a = 1;", "let a = 1;");
    expect_printed("const a = 1;", "const a = 1;");
    expect_printed("var a;", "var a;");
    expect_printed("let a;", "let a;");
    expect_printed("const a;", "const a;");
    expect_printed("const a", "const a;");
    expect_printed("const a = 1", "const a = 1;");

    expect_printed("let a = 1, b = 2, c = 3", "let a = 1, b = 2, c = 3;");
    expect_printed("const a = 1, b = 2, c = 3", "const a = 1, b = 2, c = 3;");
    expect_printed("var a = 1, b = 2, c = 3", "var a = 1, b = 2, c = 3;");
}

#[test]
fn test_infix_expressions() {
    expect_printed("5 + 5", "5 + 5");
    expect_printed("5 - 5", "5 - 5");
    expect_printed("5 * 5", "5 * 5");
    expect_printed("5 / 5", "5 / 5");
    expect_printed("5 > 5", "5 > 5");
    expect_printed("5 < 5", "5 < 5");
    expect_printed("5 == 5", "5 == 5");
    expect_printed("5 === 5", "5 === 5");
    expect_printed("5 != 5", "5 != 5");
    expect_printed("5 !== 5", "5 !== 5");
    expect_printed("a + a", "a + a");
    expect_printed("a === a", "a === a");
    expect_printed("true === true", "true === true");
    expect_printed("true !== false", "true !== false");
}

#[test]
fn test_operator_precedence_parsing() {
    expect_ast(
        "5 + 5",
        Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::BinaryExpression(BinaryExpression {
                    left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                    operator: BinaryOperator::Plus,
                    right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                }),
            })],
        },
    );
    expect_ast(
        "true",
        Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::BooleanExpression(BooleanExpression { value: true }),
            })],
        },
    );
    expect_ast(
        "false",
        Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::BooleanExpression(BooleanExpression { value: false }),
            })],
        },
    );
    expect_ast(
        "5 + 5 + 5",
        Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::BinaryExpression(BinaryExpression {
                    left: Box::new(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                        operator: BinaryOperator::Plus,
                        right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                    })),
                    operator: BinaryOperator::Plus,
                    right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                }),
            })],
        },
    );

    expect_ast(
        "3 + 4 * 5 == 3 * (1 + 4) * 5",
        Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::BinaryExpression(BinaryExpression {
                    left: Box::new(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 3 })),
                        operator: BinaryOperator::Plus,
                        right: Box::new(Expression::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 4 })),
                            operator: BinaryOperator::Asterisk,
                            right: Box::new(Expression::IntegerLiteral(IntegerLiteral {
                                value: 5,
                            })),
                        })),
                    })),
                    operator: BinaryOperator::EqualsEquals,
                    right: Box::new(Expression::BinaryExpression(BinaryExpression {
                        left: Box::new(Expression::BinaryExpression(BinaryExpression {
                            left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 3 })),
                            operator: BinaryOperator::Asterisk,
                            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::IntegerLiteral(IntegerLiteral {
                                    value: 1,
                                })),
                                operator: BinaryOperator::Plus,
                                right: Box::new(Expression::IntegerLiteral(IntegerLiteral {
                                    value: 4,
                                })),
                            })),
                        })),
                        operator: BinaryOperator::Asterisk,
                        right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                    })),
                }),
            })],
        },
    );
}

#[test]
fn test_import_statement() {
    expect_printed("import a from \"b\"", "import a from \"b\";");
    expect_printed("import { a } from \"b\"", "import { a } from \"b\";");
    expect_printed("import { a, b } from \"b\"", "import { a, b } from \"b\";");
    expect_printed(
        "import { a as b } from \"b\";",
        "import { a as b } from \"b\";",
    );
    expect_printed("import { a, b } from \"b\"", "import { a, b } from \"b\";");
    expect_printed(
        "import { a as b, b as c } from \"b\";",
        "import { a as b, b as c } from \"b\";",
    );
    expect_printed(
        "import a, { b as c } from \"b\";",
        "import a, { b as c } from \"b\";",
    );
    expect_printed("import a, { b } from \"b\"", "import a, { b } from \"b\";");
    expect_printed("import * as a from \"b\"", "import * as a from \"b\";");
    expect_printed(
        "import a, * as b from \"b\"",
        "import a, * as b from \"b\";",
    );
}

#[test]
fn test_function_declaration() {
    expect_printed("function a() {}", "function a() {}");
    expect_printed("function a(b, c) {}", "function a(b, c) {}");
    expect_printed(
        "function a(b, c) { return b + c; }",
        "function a(b, c) { return b + c; }",
    );
}

#[test]
fn parse_return_statement() {
    expect_printed("return;", "return;");
    expect_printed("return 5;", "return 5;");
    expect_printed("return 5 + 5;", "return 5 + 5;");
}

#[test]
fn test_call_expression() {
    expect_printed("a()", "a()");
    expect_printed("a(a)", "a(a)");
    expect_printed("a(a, b)", "a(a, b)");
    expect_printed("a(3 + 3)", "a(3 + 3)");
}

#[test]
fn test_if_statement() {
    expect_printed("if (true) {}", "if (true) {}");
    expect_printed("if (true) {} else {}", "if (true) {} else {}");
    expect_printed("if (x < 10) { return 10; }", "if (x < 10) { return 10; }");
    expect_printed(
        "if (false) {} else if (true) {}",
        "if (false) {} else if (true) {}",
    );
    expect_printed(
        "if (false) {} function a() {}",
        "if (false) {}function a() {}",
    );
}

#[test]
fn test_function_expression() {
    expect_printed("let a = function() {}", "let a = function() {};");
    expect_printed("a(function() {})", "a(function() {})");
    expect_printed("(function() {})", "(function() {})");
    expect_printed("(function() {})()", "(function() {})()");
    expect_printed("(function a() {})", "(function a() {})");
    expect_printed("let a = function b() {}", "let a = function b() {};");
}

#[test]
fn test_conditional_expression() {
    expect_printed("true ? 1 : 2", "true ? 1 : 2");
    expect_printed("3 > 2 ? 3 + 2 : 3 * 2", "3 > 2 ? 3 + 2 : 3 * 2");
}

#[test]
fn test_for_statement() {
    expect_printed(
        "for (let a = 1; a < 10; a++) {}",
        "for (let a = 1; a < 10; a++) {}",
    );
    expect_printed(
        "for (const a = 1; a < 10; a++) {}",
        "for (const a = 1; a < 10; a++) {}",
    );
    expect_printed(
        "for (let a = 1; a < 10; a++) {}",
        "for (let a = 1; a < 10; a++) {}",
    );
}

#[test]
fn parse_for_in_statement() {
    expect_printed("for (const a in items) {}", "for (const a in items) {}");
    expect_printed("for (var a in items) {}", "for (var a in items) {}");
    expect_printed("for (let a in items) {}", "for (let a in items) {}");
    expect_printed(
        "for (let a in items) { return 3 + 3; }",
        "for (let a in items) { return 3 + 3; }",
    );
}

#[test]
fn parse_for_of_statement() {
    expect_printed("for (const a of items) {}", "for (const a of items) {}");
    expect_printed("for (var a of items) {}", "for (var a of items) {}");
    expect_printed("for (let a of items) {}", "for (let a of items) {}");
    expect_printed(
        "for (let a of items) { return 3 + 3; }",
        "for (let a of items) { return 3 + 3; }",
    );
}

#[test]
fn test_update_expression() {
    expect_printed("++a", "++a");
    expect_printed("a++", "a++");
    expect_printed("--a", "--a");
    expect_printed("a--", "a--");
}

#[test]
fn test_assignment_expression() {
    expect_printed("a = 1", "a = 1");
    expect_printed("a = 3 * 3", "a = 3 * 3");
    expect_printed("a += 1", "a += 1");
    expect_printed("a += 3 * 3", "a += 3 * 3");
    expect_printed("a -= 1", "a -= 1");
    expect_printed("a -= 3 * 3", "a -= 3 * 3");
    expect_printed("a *= 1", "a *= 1");
    expect_printed("a *= 3 * 3", "a *= 3 * 3");
    expect_printed("a /= 1", "a /= 1");
    expect_printed("a /= 3 * 3", "a /= 3 * 3");
    expect_printed("a %= 1", "a %= 1");
    expect_printed("a %= 3 * 3", "a %= 3 * 3");
    expect_printed("a <<= 1", "a <<= 1");
    expect_printed("a <<= 3 * 3", "a <<= 3 * 3");
    expect_printed("a >>= 1", "a >>= 1");
    expect_printed("a >>= 3 * 3", "a >>= 3 * 3");
    expect_printed("a >>>= 1", "a >>>= 1");
    expect_printed("a >>>= 3 * 3", "a >>>= 3 * 3");
    expect_printed("a |= 1", "a |= 1");
    expect_printed("a |= 3 * 3", "a |= 3 * 3");
    expect_printed("a ^= 1", "a ^= 1");
    expect_printed("a ^= 3 * 3", "a ^= 3 * 3");
    expect_printed("a &= 1", "a &= 1");
    expect_printed("a &= 3 * 3", "a &= 3 * 3");
}

#[test]
fn test_logical_expression() {
    expect_printed("3 + 3 || 1 * 2", "3 + 3 || 1 * 2");
    expect_printed("3 + 3 && 1 * 2", "3 + 3 && 1 * 2");
    expect_ast(
        "a || b && c",
        Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::LogicalExpression(LogicalExpression {
                    left: Box::new(Expression::Identifier(Identifier { name: "a".into() })),
                    operator: LogicalOperator::BarBar,
                    right: Box::new(Expression::LogicalExpression(LogicalExpression {
                        left: Box::new(Expression::Identifier(Identifier { name: "b".into() })),
                        operator: LogicalOperator::AmpersandAmpersand,
                        right: Box::new(Expression::Identifier(Identifier { name: "c".into() })),
                    })),
                }),
            })],
        },
    )
}

#[test]
fn test_continue_statement() {
    expect_printed("continue;", "continue");
    expect_printed("continue label1;", "continue label1");
}

#[test]
fn test_break_statement() {
    expect_printed("break;", "break");
    expect_printed("break label1;", "break label1");
}

#[test]
fn test_empty_statement() {
    expect_printed(";", ";");
}

#[test]
fn test_while_statement() {
    expect_printed("while (true) {}", "while (true) {}");
    expect_printed("while (1 < 10) {}", "while (1 < 10) {}");
    expect_printed(
        "while (1 < 10) { return 3; }",
        "while (1 < 10) { return 3; }",
    );
}

#[test]
fn test_do_while_statement() {
    expect_printed("do {} while (true)", "do {} while (true)");
    expect_printed("do {} while (1 < 10)", "do {} while (1 < 10)");
    expect_printed(
        "do { return 3; } while (1 < 10)",
        "do { return 3; } while (1 < 10)",
    );
}

#[test]
fn test_switch_statement() {
    expect_printed(
        "switch (a) { case \"1\": {} }",
        "switch (a) { case \"1\": {} }",
    );
    expect_printed(
        "switch (a) { case \"1\": {} default: {} }",
        "switch (a) { case \"1\": {} default: {} }",
    );
    expect_printed("switch (a) { default: {} }", "switch (a) { default: {} }");
}

#[test]
fn test_debugger_statement() {
    expect_printed("debugger", "debugger");
}

#[test]
fn test_with_statement() {
    expect_printed("with (a) {}", "with (a) {}")
}

#[test]
fn test_labeled_statement() {
    expect_printed("label1: function a() {}", "label1: function a() {}");
    expect_printed("label1: while (true) {}", "label1: while (true) {}");
}

#[test]
fn test_throw_statement() {
    expect_printed("throw 3 + 3", "throw 3 + 3");
    expect_printed("throw err", "throw err");
    expect_printed("throw new Error()", "throw new Error()");
}

#[test]
fn test_try_statement() {
    expect_printed("try {} catch (err) {}", "try {} catch (err) {}");
    expect_printed("try {} finally {}", "try {} finally {}");
    expect_printed(
        "try {} catch (err) {} finally {}",
        "try {} catch (err) {} finally {}",
    );
}

#[test]
fn test_this_expression() {
    expect_printed("this", "this");
    expect_printed("this.hello()", "this.hello()");
}

#[test]
fn test_array_expression() {
    expect_printed("[1, 2, 3, 4, 5]", "[1, 2, 3, 4, 5]");
    expect_printed("[\"a\", 2]", "[\"a\", 2]");
    expect_printed("let a = []", "let a = [];");
    expect_printed("let a = [,,,]", "let a = [, , ,];");
}

#[test]
fn test_object_expression() {
    expect_printed("let a = { a: b }", "let a = { \"a\": b };");
    expect_printed(
        "let a = { \"a\": \"hello\" }",
        "let a = { \"a\": \"hello\" };",
    );
    expect_printed("let a = {}", "let a = {};");
    expect_printed("let a = { a: b, c: d }", "let a = { \"a\": b, \"c\": d };");
    expect_printed("let a = { [a]: b, [c]: d }", "let a = { [a]: b, [c]: d };");
    expect_printed(
        "let a = { [a]: { [b]: { [c]: { [d]: {} } } } }",
        "let a = { [a]: { [b]: { [c]: { [d]: {} } } } };",
    );
    expect_printed("let a = { [a]: 3 * 3 / 2 }", "let a = { [a]: 3 * 3 / 2 };");
}

#[test]
fn test_new_expression() {
    expect_printed("new MyClass()", "new MyClass()");
    expect_printed("new MyClass(a, b, c)", "new MyClass(a, b, c)");
    expect_printed("new function() {}()", "new function() {}()");
}

#[test]
fn test_member_expression() {
    expect_printed("a.b.c", "a.b.c");
    expect_printed("a[b].d.[c]", "a[b].d.[c]");
    expect_printed("a['a' + 'b'].d.[c]", "a[\"a\" + \"b\"].d.[c]");
}

#[test]
fn test_export_named_declaration() {
    expect_printed("export { a }", "export { a };");
    expect_printed("export { a as b }", "export { a as b };");
    expect_printed("export { a } from \"b\"", "export { a } from \"b\";");
    expect_printed(
        "export { a as b } from \"c\"",
        "export { a as b } from \"c\";",
    );
    expect_printed(
        "export { default as a } from \"b\";",
        "export { default as a } from \"b\";",
    );
    expect_printed("export function a() {}", "export function a() {}");
    expect_printed("export const a = 1;", "export const a = 1;");
}

#[test]
fn test_export_all_declaration() {
    expect_printed("export * from \"a\";", "export * from \"a\";");
}

#[test]
fn test_export_default_declaration() {
    expect_printed(
        "export default function a() {}",
        "export default function a() {}",
    );
    expect_printed(
        "export default function() {}",
        "export default function() {}",
    );
    expect_printed("export default 3 + 3", "export default 3 + 3;");
    expect_printed("export default { a: c }", "export default { \"a\": c };");
}
