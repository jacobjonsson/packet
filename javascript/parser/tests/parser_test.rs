use javascript_ast::{expression::*, statement::*};
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

fn expect_integer_literal(expression: &Expression, value: i64) {
    assert_eq!(
        expression,
        &Expression::IntegerLiteral(IntegerLiteral { value })
    );
}

#[test]
fn test_let_declaration() {
    let input = "let a = 1;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(
        program.statements.len(),
        1,
        "Program should contain 1 statement"
    );

    let variable_declaration = match &program.statements[0] {
        Statement::VariableDeclaration(v) => v,
        s => panic!("Expected variable declaration but got {:?}", s),
    };

    assert_eq!(variable_declaration.kind, VariableDeclarationKind::Let);
    for declaration in &variable_declaration.declarations {
        assert_eq!(declaration.id, Identifier { name: "a".into() });
        match &declaration.init {
            Some(e) => expect_integer_literal(e, 1),
            None => panic!("Expected declaration.init to be Some but got None"),
        };
    }
}

#[test]
fn test_const_declaration() {
    let input = "const a = 1;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(
        program.statements.len(),
        1,
        "Program should contain 1 statement"
    );

    let variable_declaration = match &program.statements[0] {
        Statement::VariableDeclaration(v) => v,
        s => panic!("Expected variable declaration but got {:?}", s),
    };

    assert_eq!(variable_declaration.kind, VariableDeclarationKind::Const);
    for declaration in &variable_declaration.declarations {
        assert_eq!(declaration.id, Identifier { name: "a".into() });
        match &declaration.init {
            Some(e) => expect_integer_literal(e, 1),
            None => panic!("Expected declaration.init to be Some but got None"),
        };
    }
}

#[test]
fn test_var_declaration() {
    let input = "var a = 1;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(
        program.statements.len(),
        1,
        "Program should contain 1 statement"
    );

    let variable_declaration = match &program.statements[0] {
        Statement::VariableDeclaration(v) => v,
        s => panic!("Expected variable declaration but got {:?}", s),
    };

    assert_eq!(variable_declaration.kind, VariableDeclarationKind::Var);
    for declaration in &variable_declaration.declarations {
        assert_eq!(declaration.id, Identifier { name: "a".into() });
        match &declaration.init {
            Some(e) => expect_integer_literal(e, 1),
            None => panic!("Expected declaration.init to be Some but got None"),
        };
    }
}

#[test]
fn test_empty_variable_declaration() {
    let input = "let a;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert_eq!(
        program.statements.len(),
        1,
        "Program should contain 1 statement"
    );

    let variable_declaration = match &program.statements[0] {
        Statement::VariableDeclaration(v) => v,
        s => panic!("Expected variable declaration but got {:?}", s),
    };

    assert_eq!(variable_declaration.kind, VariableDeclarationKind::Let);
    for declaration in &variable_declaration.declarations {
        assert_eq!(declaration.id, Identifier { name: "a".into() });
        assert_eq!(declaration.init, None);
    }
}

enum Expected {
    Integer(i64),
    String(String),
    Boolean(bool),
}

fn test_integer_literal(expression: &Expression, value: i64) {
    let integer = match expression {
        Expression::IntegerLiteral(v) => v,
        e => panic!("Expected integer literal but got {:?}", e),
    };

    assert_eq!(integer.value, value, "Values should match");
}

fn test_boolean_literal(expression: &Expression, value: bool) {
    let literal = match expression {
        Expression::BooleanExpression(v) => v,
        e => panic!("Expected boolean identifier but got {:?}", e),
    };

    assert_eq!(literal.value, value);
}

fn test_identifier(expression: &Expression, value: String) {
    let identifier = match expression {
        Expression::Identifier(v) => v,
        e => panic!("Expected identifier but got {:?}", e),
    };

    assert_eq!(identifier.name, value);
}

fn test_literal_expression(expression: &Expression, value: Expected) {
    match value {
        Expected::String(v) => test_identifier(expression, v),
        Expected::Integer(v) => test_integer_literal(expression, v),
        Expected::Boolean(v) => test_boolean_literal(expression, v),
    }
}

fn test_infix_expression(expression: &Expression, left: Expected, operator: &str, right: Expected) {
    let infix_expression = match expression {
        Expression::InfixExpression(e) => e,
        e => panic!("Expected infix expression but got {:?}", e),
    };

    test_literal_expression(&infix_expression.left, left);
    assert_eq!(
        infix_expression.operator, operator,
        "Operators should match"
    );
    test_literal_expression(&infix_expression.right, right);
}

#[test]
fn test_infix_expressions() {
    let tests = vec![
        ("5 + 5;", Expected::Integer(5), "+", Expected::Integer(5)),
        ("5 - 5;", Expected::Integer(5), "-", Expected::Integer(5)),
        ("5 * 5;", Expected::Integer(5), "*", Expected::Integer(5)),
        ("5 / 5;", Expected::Integer(5), "/", Expected::Integer(5)),
        ("5 > 5;", Expected::Integer(5), ">", Expected::Integer(5)),
        ("5 < 5;", Expected::Integer(5), "<", Expected::Integer(5)),
        ("5 == 5;", Expected::Integer(5), "==", Expected::Integer(5)),
        (
            "5 === 5;",
            Expected::Integer(5),
            "===",
            Expected::Integer(5),
        ),
        (
            "5 !== 5;",
            Expected::Integer(5),
            "!==",
            Expected::Integer(5),
        ),
        (
            "a + b",
            Expected::String("a".into()),
            "+",
            Expected::String("b".into()),
        ),
        (
            "true == true;",
            Expected::Boolean(true),
            "==",
            Expected::Boolean(true),
        ),
        (
            "true === true;",
            Expected::Boolean(true),
            "===",
            Expected::Boolean(true),
        ),
        (
            "true != false;",
            Expected::Boolean(true),
            "!=",
            Expected::Boolean(false),
        ),
        (
            "true !== false;",
            Expected::Boolean(true),
            "!==",
            Expected::Boolean(false),
        ),
        (
            "false == false;",
            Expected::Boolean(false),
            "==",
            Expected::Boolean(false),
        ),
        (
            "false === false;",
            Expected::Boolean(false),
            "===",
            Expected::Boolean(false),
        ),
    ];

    for test in tests {
        let lexer = Lexer::new(test.0);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            1,
            "Program should contain 1 statement"
        );
        let statement = match &program.statements[0] {
            Statement::Expression(s) => s,
            s => panic!("Expected expression statement but got {:?}", s),
        };

        test_infix_expression(&statement.expression, test.1, test.2, test.3);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("5 + 5", "(5 + 5)"),
        ("true", "true"),
        ("false", "false"),
        ("5 + 5 + 5", "((5 + 5) + 5)"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b / c", "(a + (b / c))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
    ];

    for test in tests {
        let lexer = Lexer::new(test.0);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            1,
            "Program should contain 1 statement"
        );

        let text = Printer::new().print_program(&program);
        assert_eq!(text, test.1);
    }
}

#[test]
fn test_import_statement() {
    let tests: Vec<(&str, Vec<ImportClause>)> = vec![
        (
            "import a from \"b\"",
            vec![ImportClause::ImportDefault(ImportDefaultSpecifier {
                local: Identifier { name: "a".into() },
            })],
        ),
        (
            "import { a } from \"b\"",
            vec![ImportClause::Import(ImportSpecifier {
                local: Identifier { name: "a".into() },
                imported: Identifier { name: "a".into() },
            })],
        ),
        (
            "import { a as b } from \"b\"",
            vec![ImportClause::Import(ImportSpecifier {
                local: Identifier { name: "b".into() },
                imported: Identifier { name: "a".into() },
            })],
        ),
        (
            "import { a, b } from \"b\"",
            vec![
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "a".into() },
                    imported: Identifier { name: "a".into() },
                }),
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "b".into() },
                    imported: Identifier { name: "b".into() },
                }),
            ],
        ),
        (
            "import { a as b, b as c } from \"b\"",
            vec![
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "b".into() },
                    imported: Identifier { name: "a".into() },
                }),
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "c".into() },
                    imported: Identifier { name: "b".into() },
                }),
            ],
        ),
        (
            "import a, { b, c } from \"b\"",
            vec![
                ImportClause::ImportDefault(ImportDefaultSpecifier {
                    local: Identifier { name: "a".into() },
                }),
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "b".into() },
                    imported: Identifier { name: "b".into() },
                }),
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "c".into() },
                    imported: Identifier { name: "c".into() },
                }),
            ],
        ),
        (
            "import a, { b as c } from \"b\"",
            vec![
                ImportClause::ImportDefault(ImportDefaultSpecifier {
                    local: Identifier { name: "a".into() },
                }),
                ImportClause::Import(ImportSpecifier {
                    local: Identifier { name: "c".into() },
                    imported: Identifier { name: "b".into() },
                }),
            ],
        ),
        (
            "import a, * as b from \"b\"",
            vec![
                ImportClause::ImportDefault(ImportDefaultSpecifier {
                    local: Identifier { name: "a".into() },
                }),
                ImportClause::ImportNamespace(ImportNamespaceSpecifier {
                    local: Identifier { name: "b".into() },
                }),
            ],
        ),
        (
            "import * as a from \"b\"",
            vec![ImportClause::ImportNamespace(ImportNamespaceSpecifier {
                local: Identifier { name: "a".into() },
            })],
        ),
        ("import \"b\"", Vec::new()),
    ];

    for test in tests {
        let lexer = Lexer::new(test.0);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(
            program.statements.len(),
            1,
            "Program should contain 1 statement"
        );

        let import_declaration = match &program.statements[0] {
            Statement::ImportDeclaration(i) => i,
            t => panic!("Expected import declaration but {:?}", t),
        };

        assert_eq!(
            import_declaration.source,
            StringLiteral { value: "b".into() }
        );
        assert_eq!(import_declaration.specifiers, test.1);
    }
}

#[test]
fn parse_function_declaration() {
    let tests: Vec<(&str, &str, Vec<Identifier>, Vec<Statement>)> = vec![
        ("function a() {}", "a", vec![], vec![]),
        (
            "function a(b) {}",
            "a",
            vec![Identifier { name: "b".into() }],
            vec![],
        ),
        (
            "function a(b,c) {}",
            "a",
            vec![
                Identifier { name: "b".into() },
                Identifier { name: "c".into() },
            ],
            vec![],
        ),
        (
            "function a(b,c) {}",
            "a",
            vec![
                Identifier { name: "b".into() },
                Identifier { name: "c".into() },
            ],
            vec![],
        ),
        (
            "function a(b,c) { b + c; }",
            "a",
            vec![
                Identifier { name: "b".into() },
                Identifier { name: "c".into() },
            ],
            vec![Statement::Expression(ExpressionStatement {
                expression: Expression::InfixExpression(InfixExpression {
                    left: Box::new(Expression::Identifier(Identifier { name: "b".into() })),
                    operator: "+".into(),
                    right: Box::new(Expression::Identifier(Identifier { name: "c".into() })),
                }),
            })],
        ),
    ];

    for test in tests {
        let lexer = Lexer::new(test.0);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);
        let function_declaration = match &program.statements[0] {
            Statement::FunctionDeclaration(f) => f,
            s => panic!("Expected function declaration but got {:?}", s),
        };

        assert_eq!(
            function_declaration.id,
            Identifier {
                name: test.1.into()
            }
        );
        assert_eq!(function_declaration.parameters, test.2);
        assert_eq!(
            function_declaration.body,
            BlockStatement { statements: test.3 }
        );
    }
}

#[test]
fn parse_return_statement() {
    let tests: Vec<(&str, Option<Expression>)> = vec![
        ("return;", None),
        (
            "return 5;",
            Some(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
        ),
        (
            "return 5 + 5;",
            Some(Expression::InfixExpression(InfixExpression {
                left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
                operator: "+".into(),
                right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 5 })),
            })),
        ),
    ];

    for test in tests {
        let lexer = Lexer::new(test.0);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);
        let return_statement = match &program.statements[0] {
            Statement::Return(r) => r,
            s => panic!("Expected return statement but got {:?}", s),
        };
        assert_eq!(return_statement.expression, test.1);
    }
}

#[test]
fn parse_call_expression() {
    let tests: Vec<(&str, &str, Vec<Expression>)> = vec![
        ("a();", "a", vec![]),
        (
            "a(b);",
            "a",
            vec![Expression::Identifier(Identifier { name: "b".into() })],
        ),
        (
            "a(b, 3 + 3);",
            "a",
            vec![
                Expression::Identifier(Identifier { name: "b".into() }),
                Expression::InfixExpression(InfixExpression {
                    left: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 3 })),
                    operator: "+".into(),
                    right: Box::new(Expression::IntegerLiteral(IntegerLiteral { value: 3 })),
                }),
            ],
        ),
    ];

    for test in tests {
        let lexer = Lexer::new(&test.0);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);
        assert_eq!(program.statements.len(), 1);

        let statement = match &program.statements[0] {
            Statement::Expression(s) => s,
            s => panic!("Expected call expression statement but {:?}", s),
        };

        let call_expression = match &statement.expression {
            Expression::CallExpression(c) => c,
            e => panic!("Expected call expression but {:?}", e),
        };

        assert_eq!(
            call_expression.function,
            Identifier {
                name: test.1.into()
            }
        );

        for (idx, argument) in call_expression.arguments.iter().enumerate() {
            assert_eq!(argument.as_ref(), &test.2[idx]);
        }
    }
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
