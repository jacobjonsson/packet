use js_ast_next::{
    array_expression::{ArrayExpression, ArrayExpressionElement},
    binding_identifier::BindingIdentifier,
    boolean_literal::BooleanLiteral,
    expression_statement::ExpressionStatement,
    lexical_declaration::LexicalDeclaration,
    numeric_literal::NumericLiteral,
    regexp_literal::RegexpLiteral,
    string_literal::StringLiteral,
    variable_statement::VariableStatement,
    Expression, Statement, TargetBindingPattern, AST,
};

pub struct Codegen {
    source: String,
    statement_start: usize,
}

impl Codegen {
    /// Creates a new codegen
    pub fn new() -> Codegen {
        Codegen {
            source: String::new(),
            statement_start: 0,
        }
    }

    /// Generates javascript code from an AST
    pub fn generate(&mut self, ast: AST) -> String {
        for statement in ast.statements {
            self.print_statement(&statement);
        }

        self.source.clone()
    }

    #[allow(dead_code)]
    fn print_newline(&mut self) {
        self.print("\n");
    }

    #[allow(dead_code)]
    fn print_semicolon_after_statement(&mut self) {
        self.print(";\n");
    }

    #[allow(dead_code)]
    fn print_space(&mut self) {
        self.print(" ");
    }

    fn print(&mut self, text: &str) {
        self.source.push_str(text);
    }

    /// Prints a statement
    fn print_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ExpressionStatement(e) => self.print_expression_statement(e),
            Statement::EmptyStatement(_) => todo!(),
            Statement::BreakStatement(_) => todo!(),
            Statement::BlockStatement(_) => todo!(),
            Statement::VariableStatement(v) => self.print_variable_statement(v),
            Statement::LexicalDeclaration(l) => self.print_lexical_declaration(l),
        }
    }

    /// Prints an expression statement
    fn print_expression_statement(&mut self, expression_statement: &ExpressionStatement) {
        self.statement_start = self.source.len();
        self.print_expression(&expression_statement.expression);
        self.print_semicolon_after_statement();
    }

    /// Prints a variable statement
    fn print_variable_statement(&mut self, variable_statement: &VariableStatement) {
        self.print("var ");
        for (idx, declaration) in variable_statement.declarations.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_target_binding_pattern(&declaration.binding);
            if let Some(initializer) = &declaration.initializer {
                self.print_space();
                self.print("=");
                self.print_space();
                self.print_expression(initializer);
            }
        }
        self.print_semicolon_after_statement();
    }

    /// Prints a lexical declaration
    fn print_lexical_declaration(&mut self, lexical_declaration: &LexicalDeclaration) {
        match lexical_declaration.is_const {
            true => self.print("const "),
            false => self.print("let "),
        };

        for (idx, declaration) in lexical_declaration.declarations.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_target_binding_pattern(&declaration.binding);
            if let Some(initializer) = &declaration.initializer {
                self.print_space();
                self.print("=");
                self.print_space();
                self.print_expression(initializer);
            }
        }
        self.print_semicolon_after_statement();
    }

    /// Prints a target binding pattern
    fn print_target_binding_pattern(&mut self, target_binding_pattern: &TargetBindingPattern) {
        match target_binding_pattern {
            TargetBindingPattern::BindingIdentifier(i) => self.print_binding_identifier(i),
        }
    }

    /// Prints a binding identifier
    fn print_binding_identifier(&mut self, binding_identifier: &BindingIdentifier) {
        self.print(&binding_identifier.name);
    }

    /// Prints an expression
    fn print_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::ArrayExpression(a) => self.print_array_expression(a),
            Expression::NumericLiteral(n) => self.print_numeric_expression(n),
            Expression::StringLiteral(s) => self.print_string_literal(s),
            Expression::RegexpLiteral(r) => self.print_regexp_literal(r),
            Expression::BooleanLiteral(b) => self.print_boolean_literal(b),
            _ => todo!(),
        }
    }

    /// Prints an array expression
    fn print_array_expression(&mut self, array_expression: &ArrayExpression) {
        self.print("[");
        for (idx, element) in array_expression.elements.iter().enumerate() {
            let is_last_element = idx == array_expression.elements.len() - 1;
            match element {
                ArrayExpressionElement::Hole(_) => {
                    self.print(",");
                    continue;
                }
                ArrayExpressionElement::Spread(s) => {
                    self.print("...");
                    self.print_expression(&s.argument);
                    if !is_last_element {
                        self.print(",");
                    }
                }
                ArrayExpressionElement::Expression(e) => {
                    self.print_expression(e);
                    if !is_last_element {
                        self.print(",");
                    }
                }
            }
        }
        self.print("]");
    }

    /// Prints a boolean expression
    fn print_boolean_literal(&mut self, boolean_expression: &BooleanLiteral) {
        match boolean_expression.value {
            true => self.print("true"),
            false => self.print("false"),
        };
    }

    /// Prints a numeric expression
    fn print_numeric_expression(&mut self, numeric_expression: &NumericLiteral) {
        self.print(&numeric_expression.value.to_string());
    }

    /// Prints a string expression
    fn print_string_literal(&mut self, string_literal: &StringLiteral) {
        self.print("\"");
        self.print(&string_literal.value);
        self.print("\"");
    }

    /// Prints a regexp expression
    fn print_regexp_literal(&mut self, regexp_literal: &RegexpLiteral) {
        self.print(&regexp_literal.value);
    }
}
