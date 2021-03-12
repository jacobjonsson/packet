use javascript_ast::{
    expression::{Expression, Identifier, IntegerLiteral},
    statement::*,
};
use javascript_lexer::Lexer;
use javascript_parser::Parser;

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
