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

            s => panic!("Does not know how to print {:?}", s),
        };
    }

    fn print_declaration_statement(
        &mut self,
        keyword: &str,
        declarations: &Vec<VariableDeclarator>,
    ) {
        self.print(keyword);
        self.print_space();
        // TODO: We currently only handle on declaration.
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
            e => panic!("Does not know how to print {:?}", e),
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
