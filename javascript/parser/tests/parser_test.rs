use javascript_ast::{
    expression::{Expression, Identifier, IntegerLiteral},
    statement::*,
};
use javascript_lexer::Lexer;
use javascript_parser::Parser;
use javascript_printer::Printer;

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
