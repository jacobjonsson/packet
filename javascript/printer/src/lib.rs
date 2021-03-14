use javascript_ast::{expression::*, statement::*, Program};

pub struct Printer {
    text: String,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            text: String::new(),
        }
    }

    pub fn print_program(&mut self, program: &Program) -> String {
        for statement in &program.statements {
            self.print_statement(statement);
        }

        return self.text.clone();
    }
}

impl Printer {
    fn print_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration(v) => {
                match v.kind {
                    VariableDeclarationKind::Const => {
                        self.print_declaration_statement("const", &v.declarations)
                    }
                    VariableDeclarationKind::Let => {
                        self.print_declaration_statement("let", &v.declarations)
                    }
                    VariableDeclarationKind::Var => {
                        self.print_declaration_statement("var", &v.declarations)
                    }
                };
            }

            Statement::Expression(e) => {
                self.print_expression(&e.expression);
            }

            Statement::ImportDeclaration(i) => {
                let mut items = 0;

                self.print("import");
                self.print_space();

                let default_import = &i.specifiers.iter().find_map(|i| match i {
                    ImportClause::ImportDefault(i) => Some(i),
                    _ => None,
                });

                let namespace_import = &i.specifiers.iter().find_map(|i| match i {
                    ImportClause::ImportNamespace(i) => Some(i),
                    _ => None,
                });

                let named_imports: &Vec<&ImportSpecifier> = &i
                    .specifiers
                    .iter()
                    .filter_map(|i| match i {
                        ImportClause::Import(i) => Some(i),
                        _ => None,
                    })
                    .collect();

                if let Some(i) = default_import {
                    self.print(&i.local.name);
                    items += 1;
                }

                if named_imports.len() > 0 {
                    if items > 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print("{");
                    self.print_space();
                    for (idx, named_import) in named_imports.iter().enumerate() {
                        if idx != 0 {
                            self.print(",");
                            self.print_space();
                        }

                        self.print(&named_import.imported.name);
                        if named_import.imported.name != named_import.local.name {
                            self.print(" as ");
                            self.print(&named_import.local.name);
                        }
                    }
                    self.print_space();
                    self.print("}");
                }

                if let Some(i) = namespace_import {
                    if items > 0 {
                        self.print(",");
                        self.print_space();
                    }

                    self.print("*");
                    self.print_space();
                    self.print("as ");
                    self.print(&i.local.name);
                }

                self.print_space();
                self.print("from");
                self.print_space();
                self.print("\"");
                self.print(&i.source.value);
                self.print("\"");
            }

            Statement::Block(b) => self.print_block_statement(b),
            Statement::FunctionDeclaration(f) => self.print_function_declaration(f),
        };
    }

    fn print_block_statement(&mut self, block_statement: &BlockStatement) {
        self.print("{");
        for statement in &block_statement.statements {
            self.print_statement(statement);
        }
        self.print("}");
    }

    fn print_function_declaration(&mut self, function_declaration: &FunctionDeclaration) {
        self.print("function ");
        self.print_identifier(&function_declaration.id);
        self.print("(");
        for (idx, argument) in function_declaration.parameters.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_identifier(argument);
        }
        self.print(")");
        self.print_space();
        self.print_block_statement(&function_declaration.body);
    }

    fn print_declaration_statement(
        &mut self,
        keyword: &str,
        declarations: &Vec<VariableDeclarator>,
    ) {
        self.print(keyword);
        self.print_space();
        // TODO: We currently only handle one declaration.
        for declaration in declarations {
            self.print_identifier(&declaration.id);
            self.print_space();
            self.print("=");
            self.print_space();
            if let Some(expression) = &declaration.init {
                self.print_expression(expression);
            }
        }
        self.print(";");
    }

    fn print_expression(&mut self, expression: &Expression) {
        match &expression {
            Expression::BooleanExpression(e) => {
                match e.value {
                    true => self.print("true"),
                    false => self.print("false"),
                };
            }
            Expression::Identifier(e) => {
                self.print(&e.name);
            }
            Expression::IntegerLiteral(e) => {
                self.print(&e.value.to_string());
            }
            Expression::InfixExpression(e) => {
                self.print("(");
                self.print_expression(e.left.as_ref());
                self.print_space();
                self.print(&e.operator);
                self.print_space();
                self.print_expression(e.right.as_ref());
                self.print(")");
            }

            Expression::PrefixExpression(e) => {
                self.print("(");
                self.print(&e.operator);
                self.print_expression(e.right.as_ref());
                self.print(")");
            }

            Expression::StringLiteral(e) => {
                self.print("\"");
                self.print(&e.value);
                self.print("\"");
            }
        }
    }

    fn print_identifier(&mut self, id: &Identifier) {
        self.print(&id.name);
    }

    fn print_space(&mut self) {
        self.print(" ");
    }

    fn print(&mut self, text: &str) {
        self.text.push_str(text);
    }
}
