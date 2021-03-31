use js_parser::parse;
use js_printer::Printer;
use logger::LoggerImpl;
use source::Source;

fn expect_printed(content: &str, expected: &str) {
    let source = Source {
        absolute_path: "/test.js",
        pretty_path: "./test.js",
        content: content.into(),
    };

    let logger = LoggerImpl::new();
    let ast = parse(&source, &logger);
    let output = Printer::new().print_program(&ast);
    assert_eq!(output, expected);
}

#[test]
fn test_string_literal() {
    expect_printed("\"hello_world\"", "\"hello_world\";\n");
    expect_printed("'hello_world'", "\"hello_world\";\n");
}

#[test]
fn test_variable_declaration() {
    expect_printed("var a = 1;", "var a = 1;\n");
    expect_printed("let a = 1;", "let a = 1;\n");
    expect_printed("const a = 1;", "const a = 1;\n");
    expect_printed("var a;", "var a;\n");
    expect_printed("let a;", "let a;\n");
    expect_printed("const a;", "const a;\n");
    expect_printed("const a", "const a;\n");
    expect_printed("const a = 1", "const a = 1;\n");
    expect_printed(
        "let a = window.document, b = window.navigation;",
        "let a = window.document, b = window.navigation;\n",
    );
    expect_printed("typeof undefined", "typeof undefined;\n");
    expect_printed("var a =  +y, b = c", "var a = +y, b = c;\n");
    expect_printed("let a = 1, b = 2, c = 3", "let a = 1, b = 2, c = 3;\n");
    expect_printed("const a = 1, b = 2, c = 3", "const a = 1, b = 2, c = 3;\n");
    expect_printed("var a = 1, b = 2, c = 3", "var a = 1, b = 2, c = 3;\n");
    expect_printed("let { a: b } = c;", "let { a: b } = c;\n");
    expect_printed("let { [a]: b } = c;", "let { [a]: b } = c;\n");
    expect_printed("let [ a ] = b;", "let [a] = b;\n");
    expect_printed("let { ...a } = b;", "let { ...a } = b;\n");
    expect_printed("let [...a] = b;", "let [...a] = b;\n");
    expect_printed("let { a = b } = c;", "let { a = b } = c;\n");
}

#[test]
fn test_binding() {
    expect_printed("let a = b", "let a = b;\n");
    expect_printed("let {} = b", "let {} = b;\n");
    expect_printed("let { a } = b", "let { a } = b;\n");
    expect_printed("let { a = c } = b", "let { a = c } = b;\n");
    expect_printed("let { a, b } = c", "let { a,\nb } = c;\n");
    expect_printed("let { ...a } = b", "let { ...a } = b;\n");
    expect_printed(
        "let { a: { b: [...a] } } = b",
        "let { a: { b: [...a] } } = b;\n",
    );
    expect_printed(
        "let { undefined: { null: { 3000: a } } } = b",
        "let { undefined: { null: { 3000: a } } } = b;\n",
    );
    expect_printed("let [] = b", "let [] = b;\n");
    expect_printed("let [a] = b", "let [a] = b;\n");
    expect_printed("let [...[...[a]]] = b", "let [...[...[a]]] = b;\n");

    expect_printed("let { a, b, c } = b", "let { a,\nb,\nc } = b;\n");
}

#[test]
fn test_prefix_expressions() {
    expect_printed("+5", "+5;\n");
    expect_printed("-5", "-5;\n");
    expect_printed("!5", "!5;\n");
    expect_printed("~5", "~5;\n");
    expect_printed("typeof a", "typeof a;\n");
    expect_printed("void a", "void a;\n");
    expect_printed("delete a", "delete a;\n");
}

#[test]
fn test_binary_expressions() {
    expect_printed("5 + 5", "5 + 5;\n");
    expect_printed("5 - 5", "5 - 5;\n");
    expect_printed("5 * 5", "5 * 5;\n");
    expect_printed("5 / 5", "5 / 5;\n");
    expect_printed("5 % 5", "5 % 5;\n");
    expect_printed("5 > 5", "5 > 5;\n");
    expect_printed("5 < 5", "5 < 5;\n");
    expect_printed("5 ^ 5", "5 ^ 5;\n");
    expect_printed("5 <= 4", "5 <= 4;\n");
    expect_printed("5 >= 4", "5 >= 4;\n");
    expect_printed("5 == 5", "5 == 5;\n");
    expect_printed("5 === 5", "5 === 5;\n");
    expect_printed("5 != 5", "5 != 5;\n");
    expect_printed("5 !== 5", "5 !== 5;\n");
    expect_printed("a + a", "a + a;\n");
    expect_printed("a === a", "a === a;\n");
    expect_printed("a instanceof b", "a instanceof b;\n");
    expect_printed("a in b", "a in b;\n");
    expect_printed("true === true", "true === true;\n");
    expect_printed("true !== false", "true !== false;\n");
    expect_printed("a | b", "a | b;\n");
    expect_printed("a & b", "a & b;\n");
    expect_printed("a ^ b", "a ^ b;\n");
    expect_printed("a << b", "a << b;\n");
    expect_printed("a >> b", "a >> b;\n");
    expect_printed("a >>> b", "a >>> b;\n");
}

#[test]
fn test_operator_precedence_parsing() {
    expect_printed("5 + 5", "5 + 5;\n");
    expect_printed("5 + 5 * 5", "5 + 5 * 5;\n");
    expect_printed("(5 + 5) * 5", "(5 + 5) * 5;\n");
    expect_printed(
        "3 + 4 * 5 == 3 * (1 + 4) * 5",
        "3 + 4 * 5 == 3 * (1 + 4) * 5;\n",
    );
}

#[test]
fn test_import_statement() {
    expect_printed("import a from \"b\"", "import a from \"b\";\n");
    expect_printed("import { a } from \"b\"", "import { a } from \"b\";\n");
    expect_printed(
        "import { a, b } from \"b\"",
        "import { a, b } from \"b\";\n",
    );
    expect_printed(
        "import { a as b } from \"b\";",
        "import { a as b } from \"b\";\n",
    );
    expect_printed(
        "import { a, b } from \"b\"",
        "import { a, b } from \"b\";\n",
    );
    expect_printed(
        "import { a as b, b as c } from \"b\";",
        "import { a as b, b as c } from \"b\";\n",
    );
    expect_printed(
        "import a, { b as c } from \"b\";",
        "import a, { b as c } from \"b\";\n",
    );
    expect_printed(
        "import a, { b } from \"b\"",
        "import a, { b } from \"b\";\n",
    );
    expect_printed("import * as a from \"b\"", "import * as a from \"b\";\n");
    expect_printed(
        "import a, * as b from \"b\"",
        "import a, * as b from \"b\";\n",
    );
}

#[test]
fn test_function_declaration() {
    expect_printed("function a() {}", "function a() {}");
    expect_printed("function a(b, c) {}", "function a(b, c) {}");
    expect_printed("function a({ ...b }, c) {}", "function a({ ...b }, c) {}");
    expect_printed(
        "function a(b, c) { return b + c; }",
        "function a(b, c) { return b + c;\n }",
    );
    expect_printed("function a({ b }) {}", "function a({ b }) {}");
    expect_printed("function a(...b) {}", "function a(...b) {}");
}

#[test]
fn parse_return_statement() {
    expect_printed("return;", "return;\n");
    expect_printed("return 5;", "return 5;\n");
    expect_printed("return 5 + 5;", "return 5 + 5;\n");
}

#[test]
fn test_call_expression() {
    expect_printed("a()", "a();\n");
    expect_printed("a(a)", "a(a);\n");
    expect_printed("a(a, b)", "a(a, b);\n");
    expect_printed("a(3 + 3)", "a(3 + 3);\n");
    expect_printed("a();b();", "a();\nb();\n");
    expect_printed("a(b, ...c, ...d)", "a(b, ...c, ...d);\n");
    expect_printed("a(b, c, ...d)", "a(b, c, ...d);\n");
}

#[test]
fn test_if_statement() {
    expect_printed("if (true) {}", "if (true) {}");
    expect_printed("if (true) {} else {}", "if (true) {} else {}");
    expect_printed("if (x < 10) { return 10; }", "if (x < 10) { return 10;\n }");
    expect_printed(
        "if (false) {} else if (true) {}",
        "if (false) {} else if (true) {}",
    );
    expect_printed(
        "if (false) {} function a() {}",
        "if (false) {}function a() {}",
    );
    expect_printed(
        "if (i in items && a[i] === elem) {}",
        "if (i in items && a[i] === elem) {}",
    );
    expect_printed(
        "if (true) if (true) if (true) if (true) {}",
        "if (true) if (true) if (true) if (true) {}",
    );
    expect_printed(
        "if (a) a(); else if (b) b(); else c();",
        "if (a) a();\n else if (b) b();\n else c();\n",
    );
}

#[test]
fn test_function_expression() {
    expect_printed("let a = function() {}", "let a = function() {};\n");
    expect_printed("a(function() {})", "a(function() {});\n");
    expect_printed("(function() {})", "(function() {});\n");
    expect_printed("(function() {})()", "(function() {})();\n");
    expect_printed("(function a() {})", "(function a() {});\n");
    expect_printed("let a = function b() {}", "let a = function b() {};\n");
    expect_printed(
        "let a = function b({ ...c }) {}",
        "let a = function b({ ...c }) {};\n",
    );
    expect_printed(
        "let a = function b([...c]) {}",
        "let a = function b([...c]) {};\n",
    );
}

#[test]
fn test_conditional_expression() {
    expect_printed("true ? 1 : 2", "true ? 1 : 2;\n");
    expect_printed("3 > 2 ? 3 + 2 : 3 * 2", "3 > 2 ? 3 + 2 : 3 * 2;\n");
    expect_printed("(a > b ? a : b)[k] = c", "(a > b ? a : b)[k] = c;\n");
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
    expect_printed("for (; a < 10; a++) {}", "for (; a < 10; a++) {}");
    expect_printed(
        "for (i = 0, l = 10; i < l; i++) {}",
        "for (i = 0, l = 10; i < l; i++) {}",
    );
}

#[test]
fn test_for_in_statement() {
    expect_printed("for (const a in items) {}", "for (const a in items) {}");
    expect_printed("for (var a in items) {}", "for (var a in items) {}");
    expect_printed("for (let a in items) {}", "for (let a in items) {}");
    expect_printed("for (a in items) {}", "for (a in items) {}");
    expect_printed(
        "for (let a in items) { return 3 + 3; }",
        "for (let a in items) { return 3 + 3;\n }",
    );
}

#[test]
fn test_for_of_statement() {
    expect_printed("for (const a of items) {}", "for (const a of items) {}");
    expect_printed("for (var a of items) {}", "for (var a of items) {}");
    expect_printed("for (let a of items) {}", "for (let a of items) {}");
    expect_printed(
        "for (let a of items) { return 3 + 3; }",
        "for (let a of items) { return 3 + 3;\n }",
    );
}

#[test]
fn test_update_expression() {
    expect_printed("++a", "++a;\n");
    expect_printed("a++", "a++;\n");
    expect_printed("--a", "--a;\n");
    expect_printed("a--", "a--;\n");
}

#[test]
fn test_assignment_expression() {
    expect_printed("a = 1", "a = 1;\n");
    expect_printed("a = 3 * 3", "a = 3 * 3;\n");
    expect_printed("a += 1", "a += 1;\n");
    expect_printed("a += 3 * 3", "a += 3 * 3;\n");
    expect_printed("a -= 1", "a -= 1;\n");
    expect_printed("a -= 3 * 3", "a -= 3 * 3;\n");
    expect_printed("a *= 1", "a *= 1;\n");
    expect_printed("a *= 3 * 3", "a *= 3 * 3;\n");
    expect_printed("a /= 1", "a /= 1;\n");
    expect_printed("a /= 3 * 3", "a /= 3 * 3;\n");
    expect_printed("a %= 1", "a %= 1;\n");
    expect_printed("a %= 3 * 3", "a %= 3 * 3;\n");
    expect_printed("a <<= 1", "a <<= 1;\n");
    expect_printed("a <<= 3 * 3", "a <<= 3 * 3;\n");
    expect_printed("a >>= 1", "a >>= 1;\n");
    expect_printed("a >>= 3 * 3", "a >>= 3 * 3;\n");
    expect_printed("a >>>= 1", "a >>>= 1;\n");
    expect_printed("a >>>= 3 * 3", "a >>>= 3 * 3;\n");
    expect_printed("a |= 1", "a |= 1;\n");
    expect_printed("a |= 3 * 3", "a |= 3 * 3;\n");
    expect_printed("a ^= 1", "a ^= 1;\n");
    expect_printed("a ^= 3 * 3", "a ^= 3 * 3;\n");
    expect_printed("a &= 1", "a &= 1;\n");
    expect_printed("a &= 3 * 3", "a &= 3 * 3;\n");
    expect_printed("a **= 3 * 3", "a **= 3 * 3;\n");
    expect_printed("[a] = b", "[a] = b;\n");
    expect_printed("[...a] = b", "[...a] = b;\n");
    expect_printed("({ a } = b)", "{ a } = b;\n");
    expect_printed("({ ...a } = b)", "{ ...a } = b;\n");
    expect_printed("a = 1, b = 2, c = 3", "a = 1, b = 2, c = 3;\n");
}

#[test]
fn test_logical_expression() {
    expect_printed("3 + 3 || 1 * 2", "3 + 3 || 1 * 2;\n");
    expect_printed("3 + 3 && 1 * 2", "3 + 3 && 1 * 2;\n");
    expect_printed("a || b && c", "a || b && c;\n");
}

#[test]
fn test_continue_statement() {
    expect_printed("continue;", "continue;\n");
    expect_printed("continue label1;", "continue label1;\n");
    expect_printed("continue", "continue;\n")
}

#[test]
fn test_break_statement() {
    expect_printed("break;", "break;\n");
    expect_printed("break label1;", "break label1;\n");
    expect_printed("break", "break;\n")
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
        "while (1 < 10) { return 3;\n }",
    );
}

#[test]
fn test_do_while_statement() {
    expect_printed("do {} while (true)", "do {} while (true);\n");
    expect_printed("do {} while (1 < 10)", "do {} while (1 < 10);\n");
    expect_printed(
        "do { return 3; } while (1 < 10)",
        "do { return 3;\n } while (1 < 10);\n",
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
    expect_printed("debugger", "debugger;\n");
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
    expect_printed("throw 3 + 3", "throw 3 + 3;\n");
    expect_printed("throw err", "throw err;\n");
    expect_printed("throw new Error()", "throw new Error();\n");
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
    expect_printed("this", "this;\n");
    expect_printed("this.hello()", "this.hello();\n");
}

#[test]
fn test_super_expression() {
    expect_printed("super", "super;\n");
    expect_printed("super.hello()", "super.hello();\n");
}

#[test]
fn test_array_expression() {
    expect_printed("[1, 2, 3, 4, 5]", "[1, 2, 3, 4, 5];\n");
    expect_printed("[\"a\", 2]", "[\"a\", 2];\n");
    expect_printed("let a = []", "let a = [];\n");
    expect_printed("let a = [,,,]", "let a = [, , ,];\n");
    expect_printed("let a = [null, undefined];", "let a = [null, undefined];\n");
    expect_printed("[...[...[ a]]]", "[...[...[a]]];\n");
}

#[test]
fn test_object_expression() {
    expect_printed("({ [a]: b })", "({ [a]: b });\n");
    expect_printed("({ [a]() {} })", "({ [a]() {} });\n");
    expect_printed("({ a: b })", "({ a: b });\n");
    expect_printed("({ \"a\": b })", "({ \"a\": b });\n");
    expect_printed("({ 3: b })", "({ 3: b });\n");
    expect_printed("({ null: b })", "({ null: b });\n");
    expect_printed("({ undefined() {} })", "({ undefined() {} });\n");
    expect_printed("({ undefined(a, b) {} })", "({ undefined(a, b) {} });\n");
    expect_printed("({ \"a\": \"hello\" })", "({ \"a\": \"hello\" });\n");
    expect_printed("({})", "({});\n");
    expect_printed("({ a: b, c: d })", "({ a: b, c: d });\n");
    expect_printed("({ [a]: b, [c]: d })", "({ [a]: b, [c]: d });\n");
    expect_printed(
        "({ [a]: { [b]: { [c]: { [d]: {} } } } })",
        "({ [a]: { [b]: { [c]: { [d]: {} } } } });\n",
    );
    expect_printed("({ [a]: 3 * 3 / 2 })", "({ [a]: 3 * 3 / 2 });\n");
    expect_printed(
        "({ a: function() {}, b: function() {} })",
        "({ a: function() {}, b: function() {} });\n",
    );
    expect_printed("({ a: b ? c : d, e: f })", "({ a: b ? c : d, e: f });\n");
    expect_printed("({ for: b })", "({ for: b });\n");
    expect_printed("({ a })", "({ a });\n");
    expect_printed("({ get a() {} })", "({ get a() {} });\n");
    expect_printed("({ set a() {} })", "({ set a() {} });\n");
    expect_printed("({ a() {} })", "({ a() {} });\n");
    expect_printed("({ get() {} })", "({ get() {} });\n");
    expect_printed("({ set() {} })", "({ set() {} });\n");
    expect_printed("({ get: 3 * 3 })", "({ get: 3 * 3 });\n");
    expect_printed("({ set: 3 * 3 })", "({ set: 3 * 3 });\n");
    expect_printed("({ ...a })", "({ ...a });\n");
    expect_printed(
        "({ ...{ ...{ ...{ a } } } })",
        "({ ...{ ...{ ...{ a } } } });\n",
    );
}

#[test]
fn test_new_expression() {
    expect_printed("new MyClass()", "new MyClass();\n");
    expect_printed("new MyClass(a, b, c)", "new MyClass(a, b, c);\n");
    expect_printed("new function() {}()", "new function() {}();\n");
    expect_printed("new a.b.c(e)", "new a.b.c(e);\n");
    expect_printed("new a.b.c(...e, a)", "new a.b.c(...e, a);\n");
    expect_printed("new a", "new a();\n");
}

#[test]
fn test_member_expression() {
    expect_printed("a.b.c", "a.b.c;\n");
    expect_printed("a[b].d.[c]", "a[b].d.[c];\n");
    expect_printed("a['a' + 'b'].d.[c]", "a[\"a\" + \"b\"].d.[c];\n");
    expect_printed("a.b.c.d()", "a.b.c.d();\n");
    expect_printed("a.b.c.d(e)", "a.b.c.d(e);\n");
}

#[test]
fn test_export_named_declaration() {
    expect_printed("export { a }", "export { a };\n");
    expect_printed("export { a as b }", "export { a as b };\n");
    expect_printed("export { a } from \"b\"", "export { a } from \"b\";\n");
    expect_printed(
        "export { a as b } from \"c\"",
        "export { a as b } from \"c\";\n",
    );
    expect_printed(
        "export { default as a } from \"b\";",
        "export { default as a } from \"b\";\n",
    );
    expect_printed("export function a() {}", "export function a() {}");
    expect_printed("export const a = 1;", "export const a = 1;\n");
    expect_printed("export class A {}", "export class A {}");
    expect_printed("export class A extends B {}", "export class A extends B {}");
    expect_printed("export class A { b() {} }", "export class A { b() {} }");
}

#[test]
fn test_export_all_declaration() {
    expect_printed("export * from \"a\";", "export * from \"a\";\n");
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
    expect_printed("export default class A {}", "export default class A {}");
    expect_printed("export default class {}", "export default class {}");
    expect_printed("export default 3 + 3", "export default 3 + 3;\n");
    expect_printed("export default { a: c }", "export default { a: c };\n");
}

#[test]
fn test_regexp_literal() {
    expect_printed("/hello/", "/hello/;\n");
    expect_printed("/hello/gi", "/hello/gi;\n");
    expect_printed("/hello/gi.test(\"hello\")", "/hello/gi.test(\"hello\");\n");
}

#[test]
fn test_class_declaration() {
    expect_printed("class A {}", "class A {}");
    expect_printed("class A extends B {}", "class A extends B {}");
    expect_printed("class A { b() {} }", "class A { b() {} }");
    expect_printed("class A { null() {} }", "class A { null() {} }");
    expect_printed("class A { undefined() {} }", "class A { undefined() {} }");
    expect_printed("class A { 123() {} }", "class A { 123() {} }");
    expect_printed("class A { \"abc\"() {} }", "class A { \"abc\"() {} }");
    expect_printed("class A { [b]() {} }", "class A { [b]() {} }");
    expect_printed(
        "class A { constructor(a) {}\nb() {} }",
        "class A { constructor(a) {}\nb() {} }",
    );
    expect_printed("class A { get b() {} }", "class A { get b() {} }");
    expect_printed("class A { get [b]() {} }", "class A { get [b]() {} }");
    expect_printed("class A { set [b]() {} }", "class A { set [b]() {} }");
    expect_printed("class A { b() {}; c() {}; }", "class A { b() {}\nc() {} }");
    expect_printed("class A {;}", "class A {}");
}

#[test]
fn test_class_expression() {
    expect_printed("let a = class {}", "let a = class {};\n");
    expect_printed(
        "let a = class extends B {}",
        "let a = class extends B {};\n",
    );
    expect_printed(
        "let a = class B extends C {}",
        "let a = class B extends C {};\n",
    );
    expect_printed(
        "let a = class A { b() {} }",
        "let a = class A { b() {} };\n",
    );
    expect_printed(
        "let a = class A { constructor(a) {}\nb() {} }",
        "let a = class A { constructor(a) {}\nb() {} };\n",
    );
    expect_printed(
        "let a = class A { get b() {} }",
        "let a = class A { get b() {} };\n",
    );
    expect_printed(
        "let a = class A { get [b]() {} }",
        "let a = class A { get [b]() {} };\n",
    );
    expect_printed(
        "let a = class A { set [b]() {} }",
        "let a = class A { set [b]() {} };\n",
    );
}

#[test]
fn test_array_function_expression() {
    expect_printed("a => a", "(a) => a;\n");
    expect_printed("() => 3 * 3", "() => 3 * 3;\n");
    expect_printed("() => {}", "() => {};\n");
    expect_printed(
        "(a, b, c) => { return a + b + c;\n }",
        "(a, b, c) => { return a + b + c;\n };\n",
    );
    expect_printed("(a, b, ...c) => {}", "(a, b, ...c) => {};\n");
    expect_printed("a = b => {}", "a = (b) => {};\n");
    expect_printed("a = () => {}", "a = () => {};\n");
    expect_printed("let a = () => {}", "let a = () => {};\n");
    expect_printed("let a = b => {}", "let a = (b) => {};\n");
}
