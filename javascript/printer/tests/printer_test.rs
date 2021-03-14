use javascript_ast::{
    expression::{Expression, Identifier, StringLiteral},
    statement::{
        ExpressionStatement, ImportClause, ImportDeclaration, ImportDefaultSpecifier,
        ImportNamespaceSpecifier, ImportSpecifier, Statement,
    },
    Program,
};
use javascript_printer::Printer;

#[test]
fn print_string_literal() {
    let tests = vec![("hello world", "\"hello world\"")];

    for test in tests {
        let program = Program {
            statements: vec![Statement::Expression(ExpressionStatement {
                expression: Expression::StringLiteral(StringLiteral {
                    value: test.0.into(),
                }),
            })],
        };

        let text = Printer::new().print_program(&program);
        assert_eq!(text, test.1);
    }
}

#[test]
fn print_import_statements() {
    let tests = vec![
        (
            Program {
                statements: vec![Statement::ImportDeclaration(ImportDeclaration {
                    source: StringLiteral { value: "a".into() },
                    specifiers: vec![ImportClause::ImportDefault(ImportDefaultSpecifier {
                        local: Identifier { name: "a".into() },
                    })],
                })],
            },
            "import a from \"a\"",
        ),
        (
            Program {
                statements: vec![Statement::ImportDeclaration(ImportDeclaration {
                    source: StringLiteral { value: "a".into() },
                    specifiers: vec![ImportClause::Import(ImportSpecifier {
                        local: Identifier { name: "a".into() },
                        imported: Identifier { name: "a".into() },
                    })],
                })],
            },
            "import { a } from \"a\"",
        ),
        (
            Program {
                statements: vec![Statement::ImportDeclaration(ImportDeclaration {
                    source: StringLiteral { value: "a".into() },
                    specifiers: vec![ImportClause::Import(ImportSpecifier {
                        local: Identifier { name: "b".into() },
                        imported: Identifier { name: "a".into() },
                    })],
                })],
            },
            "import { a as b } from \"a\"",
        ),
        (
            Program {
                statements: vec![Statement::ImportDeclaration(ImportDeclaration {
                    source: StringLiteral { value: "a".into() },
                    specifiers: vec![
                        ImportClause::Import(ImportSpecifier {
                            local: Identifier { name: "a".into() },
                            imported: Identifier { name: "a".into() },
                        }),
                        ImportClause::Import(ImportSpecifier {
                            local: Identifier { name: "b".into() },
                            imported: Identifier { name: "b".into() },
                        }),
                    ],
                })],
            },
            "import { a, b } from \"a\"",
        ),
        (
            Program {
                statements: vec![Statement::ImportDeclaration(ImportDeclaration {
                    source: StringLiteral { value: "a".into() },
                    specifiers: vec![ImportClause::ImportNamespace(ImportNamespaceSpecifier {
                        local: Identifier { name: "a".into() },
                    })],
                })],
            },
            "import * as a from \"a\"",
        ),
        (
            Program {
                statements: vec![Statement::ImportDeclaration(ImportDeclaration {
                    source: StringLiteral { value: "a".into() },
                    specifiers: vec![
                        ImportClause::ImportDefault(ImportDefaultSpecifier {
                            local: Identifier { name: "a".into() },
                        }),
                        ImportClause::Import(ImportSpecifier {
                            local: Identifier { name: "b".into() },
                            imported: Identifier { name: "b".into() },
                        }),
                    ],
                })],
            },
            "import a, { b } from \"a\"",
        ),
    ];

    for test in tests {
        let text = Printer::new().print_program(&test.0);
        assert_eq!(text, test.1);
    }
}
