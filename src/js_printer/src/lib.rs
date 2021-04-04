use js_ast::{precedence::*, *};

pub struct Printer {
    text: String,
    statement_start: usize,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            text: String::new(),
            statement_start: 0,
        }
    }

    pub fn print_program(&mut self, program: &AST) -> String {
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
                self.print_variable_declaration(v);
                self.print_semicolon_after_statement();
            }

            Statement::EmptyStatement(_) => self.print(";"),

            Statement::ClassDeclaration(c) => {
                self.print("class ");
                self.print_identifier(&c.identifier);
                self.print_space();
                if let Some(super_class) = &c.extends {
                    self.print("extends ");
                    self.print_expression(super_class, Precedence::Comma);
                    self.print_space();
                }
                self.print_class_body(&c.body);
            }

            Statement::ReturnStatement(r) => {
                self.print("return");
                if let Some(expression) = &r.expression {
                    self.print(" ");
                    self.print_expression(expression, Precedence::Lowest);
                }
                self.print_semicolon_after_statement();
            }

            Statement::Expression(e) => {
                self.statement_start = self.text.len();
                self.print_expression(&e.expression, Precedence::Lowest);
                self.print_semicolon_after_statement();
            }

            Statement::IfStatement(i) => self.print_if_statement(i),

            Statement::ContinueStatement(c) => {
                self.print("continue");
                if let Some(label) = &c.label {
                    self.print_space();
                    self.print_identifier(label);
                }
                self.print_semicolon_after_statement();
            }
            Statement::BreakStatement(b) => {
                self.print("break");
                if let Some(label) = &b.label {
                    self.print_space();
                    self.print_identifier(label);
                }
                self.print_semicolon_after_statement();
            }

            Statement::ForStatement(f) => {
                self.print("for");
                self.print_space();
                self.print("(");
                if let Some(init) = &f.init {
                    self.print_for_loop_init(init);
                }
                self.print(";");
                self.print_space();
                if let Some(test) = &f.test {
                    self.print_expression(test, Precedence::Lowest);
                }
                self.print(";");
                self.print_space();
                if let Some(update) = &f.update {
                    self.print_expression(update, Precedence::Lowest);
                }
                self.print(")");
                self.print_space();
                self.print_statement(&f.body);
            }

            Statement::ForInStatement(f) => {
                self.print("for");
                self.print_space();
                self.print("(");
                self.print_for_loop_init(&f.left);
                self.print(" in ");
                self.print_expression(&f.right, Precedence::Lowest);
                self.print(")");
                self.print_space();
                self.print_statement(&f.body);
            }

            Statement::ForOfStatement(f) => {
                self.print("for");
                self.print_space();
                self.print("(");
                self.print_for_loop_init(&f.left);
                self.print(" of ");
                self.print_expression(&f.right, Precedence::Lowest);
                self.print(")");
                self.print_space();
                self.print_statement(&f.body);
            }

            Statement::DoWhileStatement(d) => {
                self.print("do");
                self.print_space();
                self.print_statement(&d.body);
                self.print_space();
                self.print("while");
                self.print_space();
                self.print("(");
                self.print_expression(&d.test, Precedence::Lowest);
                self.print(")");
                self.print_semicolon_after_statement();
            }

            Statement::WhileStatement(w) => {
                self.print("while");
                self.print_space();
                self.print("(");
                self.print_expression(&w.test, Precedence::Lowest);
                self.print(")");
                self.print_space();
                self.print_statement(&w.body);
            }

            Statement::SwitchStatement(s) => {
                self.print("switch");
                self.print_space();
                self.print("(");
                self.print_expression(&s.discriminant, Precedence::Lowest);
                self.print(")");

                self.print_space();
                self.print("{");
                if s.cases.len() == 0 {
                    self.print("}");
                    return;
                }
                self.print_space();
                let cases: Vec<&SwitchStatementCase> =
                    s.cases.iter().filter(|c| c.test != None).collect();
                for (idx, case) in cases.iter().enumerate() {
                    if idx != 0 {
                        self.print_space();
                    }
                    self.print("case ");
                    // Cases needs to have a test, only the default case is allowed to be none.
                    self.print_expression(case.test.as_ref().unwrap(), Precedence::LogicalAnd);
                    self.print(":");
                    self.print_space();
                    for consequent in &case.consequent {
                        self.print_statement(consequent.as_ref());
                    }
                }
                let default: Option<&SwitchStatementCase> = s.cases.iter().find(|c| c.test == None);
                if let Some(case) = default {
                    if cases.len() > 0 {
                        self.print_space();
                    }
                    self.print("default:");
                    self.print_space();
                    for consequent in &case.consequent {
                        self.print_statement(consequent.as_ref());
                    }
                }
                self.print_space();
                self.print("}");
            }

            Statement::DebuggerStatement(_) => {
                self.print("debugger");
                self.print_semicolon_after_statement();
            }

            Statement::LabeledStatement(l) => {
                self.print_identifier(&l.identifier);
                self.print(":");
                self.print_space();
                self.print_statement(&l.body);
            }

            Statement::ThrowStatement(t) => {
                self.print("throw ");
                self.print_expression(&t.argument, Precedence::Lowest);
                self.print_semicolon_after_statement();
            }

            Statement::TryStatement(t) => {
                self.print("try");
                self.print_space();
                self.print_block_statement(&t.block);
                if let Some(handler) = &t.handler {
                    self.print_space();
                    self.print("catch");
                    self.print_space();
                    self.print("(");
                    self.print_binding(&handler.param);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&handler.body);
                }
                if let Some(finalizer) = &t.finalizer {
                    self.print_space();
                    self.print("finally");
                    self.print_space();
                    self.print_block_statement(finalizer);
                }
            }

            Statement::ImportDeclaration(i) => {
                self.print("import");
                self.print_space();

                if let Some(default) = &i.default {
                    self.print_identifier(default);
                }

                if let Some(namespace) = &i.namespace {
                    if let Some(_) = &i.default {
                        self.print(",");
                        self.print_space();
                    }
                    self.print("*");
                    self.print_space();
                    self.print("as ");
                    self.print_identifier(namespace);
                } else if i.specifiers.len() > 0 {
                    if let Some(_) = &i.default {
                        self.print(",");
                        self.print_space();
                    }

                    self.print("{");
                    self.print_space();
                    for (idx, specifier) in i.specifiers.iter().enumerate() {
                        if idx != 0 {
                            self.print(",");
                            self.print_space();
                        }
                        self.print_identifier(&specifier.local);
                        if specifier.local.name != specifier.imported.name {
                            self.print(" as ");
                            self.print_identifier(&specifier.imported);
                        }
                    }
                    self.print_space();
                    self.print("}");
                }

                // Only print the from if one of the following is true
                if i.default != None || i.namespace != None || i.specifiers.len() != 0 {
                    self.print(" ");
                    self.print("from");
                    self.print_space();
                }

                self.print("\"");
                self.print(&i.source.value);
                self.print("\"");
                self.print_semicolon_after_statement();
            }

            Statement::WithStatement(w) => {
                self.print("with");
                self.print_space();
                self.print("(");
                self.print_expression(&w.object, Precedence::Lowest);
                self.print(")");
                self.print_space();
                self.print_statement(&w.body);
            }

            Statement::BlockStatement(b) => self.print_block_statement(b),
            Statement::FunctionDeclaration(f) => self.print_function_declaration(f),

            // export * from "a";
            Statement::ExportAllDeclaration(e) => {
                self.print("export * from ");
                self.print_string_literal(&e.source);
                self.print_semicolon_after_statement();
            }

            // export {a}
            // export {a as b}
            // export function a() {}
            // export var a = 1;
            // export {a} from "b";
            // export {a as c} from "b";
            Statement::ExportNamedDeclaration(e) => {
                self.print("export");
                self.print(" ");
                match &e.declaration {
                    ExportNamedDeclarationKind::FunctionDeclaration(f) => {
                        self.print_function_declaration(f)
                    }

                    ExportNamedDeclarationKind::VariableDeclaration(v) => {
                        self.print_variable_declaration(v);
                        self.print_semicolon_after_statement();
                    }
                    ExportNamedDeclarationKind::ClassDeclaration(c) => {
                        self.print("class ");
                        self.print_identifier(&c.identifier);
                        self.print_space();
                        if let Some(super_class) = &c.extends {
                            self.print("extends ");
                            self.print_expression(super_class, Precedence::Comma);
                            self.print_space();
                        }
                        self.print_class_body(&c.body);
                    }
                }
            }

            // export default 3 + 3
            // export default function a() {}
            // export default function() {}
            // export default {}
            Statement::ExportDefaultDeclaration(e) => {
                self.print("export default ");
                match &e.declaration {
                    ExportDefaultDeclarationKind::FunctionDeclaration(f) => {
                        self.print_function_declaration(f);
                    }

                    ExportDefaultDeclarationKind::Expression(exp) => {
                        self.print_expression(exp, Precedence::Comma);
                        self.print_semicolon_after_statement();
                    }

                    ExportDefaultDeclarationKind::AnonymousDefaultExportedFunctionDeclaration(
                        a,
                    ) => {
                        self.print("function");
                        if a.generator {
                            self.print("*");
                        }
                        self.print("(");
                        self.print_parameters(&a.parameters);
                        self.print(")");
                        self.print_space();
                        self.print_block_statement(&a.body);
                    }

                    ExportDefaultDeclarationKind::AnonymousDefaultExportedClassDeclaration(c) => {
                        self.print("class");
                        self.print_space();
                        if let Some(super_class) = &c.extends {
                            self.print("extends ");
                            self.print_expression(super_class, Precedence::Comma);
                            self.print_space();
                        }
                        self.print_class_body(&c.body);
                    }

                    ExportDefaultDeclarationKind::ClassDeclaration(c) => {
                        self.print("class ");
                        self.print_identifier(&c.identifier);
                        self.print_space();
                        if let Some(super_class) = &c.extends {
                            self.print("extends ");
                            self.print_expression(super_class, Precedence::Comma);
                            self.print_space();
                        }
                        self.print_class_body(&c.body);
                    }
                }
            }

            Statement::ExportNamedSpecifiers(e) => {
                self.print("export");
                self.print_space();
                self.print("{");
                if e.specifiers.len() > 0 {
                    self.print_space();
                }
                for (idx, specifier) in e.specifiers.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print_identifier(&specifier.local);
                    if specifier.local != specifier.exported {
                        self.print(" as ");
                        self.print_identifier(&specifier.exported);
                    }
                }
                if e.specifiers.len() > 0 {
                    self.print_space();
                }
                self.print("}");
                if let Some(source) = &e.source {
                    self.print_space();
                    self.print("from");
                    self.print_space();
                    self.print_string_literal(source);
                }
                self.print_semicolon_after_statement();
            }
        };
    }

    fn print_literal_property_name(&mut self, literal_property_name: &LiteralPropertyName) {
        match literal_property_name {
            LiteralPropertyName::Identifier(i) => self.print_identifier(i),
            LiteralPropertyName::String(s) => self.print_string_literal(s),
            LiteralPropertyName::Numeric(n) => self.print_numeric_literal(n),
        }
    }

    fn print_computed_property_name(&mut self, expression: &Expression) {
        self.print("[");
        self.print_expression(expression, Precedence::Comma);
        self.print("]");
    }

    fn print_for_loop_init(&mut self, init: &Statement) {
        match init {
            Statement::Expression(exp) => {
                self.print_expression(&exp.expression, Precedence::Lowest)
            }
            Statement::VariableDeclaration(v) => self.print_variable_declaration(v),
            _ => panic!("Internal server error"),
        }
    }

    fn print_string_literal(&mut self, string_literal: &StringLiteral) {
        self.print(&format!("\"{}\"", string_literal.value));
    }

    fn print_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) {
        match variable_declaration.kind {
            VariableDeclarationKind::Const => {
                self.print_declaration_statement("const", &variable_declaration.declarations)
            }
            VariableDeclarationKind::Let => {
                self.print_declaration_statement("let", &variable_declaration.declarations)
            }
            VariableDeclarationKind::Var => {
                self.print_declaration_statement("var", &variable_declaration.declarations)
            }
        };
    }

    fn print_declaration_statement(
        &mut self,
        keyword: &str,
        declarations: &Vec<VariableDeclarator>,
    ) {
        self.print(keyword);
        self.print_space();
        // TODO: We currently only handle one declaration.
        for (idx, declaration) in declarations.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_binding(&declaration.binding);
            if let Some(expression) = &declaration.initializer {
                self.print_space();
                self.print("=");
                self.print_space();
                self.print_expression(expression, Precedence::Comma);
            }
        }
    }

    fn print_block_statement(&mut self, block_statement: &BlockStatement) {
        if block_statement.statements.len() == 0 {
            self.print("{}");
            return;
        }

        self.print("{");
        self.print_space();
        for statement in &block_statement.statements {
            self.print_statement(statement);
        }
        self.print_space();
        self.print("}");
    }

    fn print_if_statement(&mut self, if_statement: &IfStatement) {
        self.print("if");
        self.print_space();
        self.print("(");
        self.print_expression(&if_statement.test, Precedence::Lowest);
        self.print(")");
        self.print_space();
        self.print_statement(&if_statement.consequent);
        if let Some(alternate) = &if_statement.alternate {
            self.print_space();
            self.print("else");
            self.print_space();
            self.print_statement(alternate);
        }
    }

    fn print_function_declaration(&mut self, function_declaration: &FunctionDeclaration) {
        self.print("function");
        if function_declaration.generator {
            self.print("*");
        }
        self.print(" ");
        self.print_identifier(&function_declaration.identifier);
        self.print("(");
        self.print_parameters(&function_declaration.parameters);
        self.print(")");
        self.print_space();
        self.print_block_statement(&function_declaration.body);
    }

    fn print_expression(&mut self, expression: &Expression, precedence: Precedence) {
        match &expression {
            Expression::NullLiteral(_) => self.print("null"),

            Expression::BooleanLiteral(e) => {
                match e.value {
                    true => self.print("true"),
                    false => self.print("false"),
                };
            }

            Expression::BigIntLiteral(b) => {
                self.print(&b.value);
                self.print("n");
            }

            Expression::Class(c) => {
                self.print("class");
                if let Some(id) = &c.identifier {
                    self.print(" ");
                    self.print_identifier(id);
                    self.print_space();
                } else {
                    self.print_space();
                }
                if let Some(super_class) = &c.extends {
                    self.print("extends ");
                    self.print_expression(super_class, Precedence::Comma);
                    self.print_space();
                }
                self.print_class_body(&c.body);
            }

            Expression::Identifier(e) => {
                self.print(&e.name);
            }

            Expression::NumericLiteral(e) => {
                self.print(&e.value.to_string());
            }

            Expression::RegexpLiteral(r) => {
                self.print(&r.value);
            }

            Expression::This(_) => self.print("this"),

            Expression::Super(_) => self.print("super"),

            Expression::Array(a) => {
                self.print("[");
                for (idx, element) in a.items.iter().enumerate() {
                    let is_last_element = idx < a.items.len() - 1;
                    match element {
                        Some(item) => {
                            match item {
                                ArrayExpressionItem::Spread(s) => self.print_spread_element(s),
                                ArrayExpressionItem::Expression(e) => {
                                    self.print_expression(e, Precedence::Comma)
                                }
                            }

                            if is_last_element {
                                self.print(",");
                            }
                        }
                        None => {
                            self.print(",");
                        }
                    }

                    // Do not print spaces for the last element
                    if is_last_element {
                        self.print_space();
                    }
                }
                self.print("]");
            }

            Expression::Binary(e) => {
                let operator_precedence = e.operator.precedence();
                let wrap = precedence >= operator_precedence;
                if wrap {
                    self.print("(");
                }

                let left_precedence = match e.operator.is_right_associative() {
                    true => operator_precedence.clone(),
                    false => operator_precedence.lower(),
                };
                let right_precedence = match e.operator.is_left_associative() {
                    true => operator_precedence.clone(),
                    false => operator_precedence.lower(),
                };

                self.print_expression(&e.left, left_precedence);
                if e.operator == BinaryExpressionOperator::In
                    || e.operator == BinaryExpressionOperator::Instanceof
                {
                    self.print(" ");
                } else {
                    self.print_space();
                }
                match &e.operator {
                    BinaryExpressionOperator::Addition => self.print("+"),
                    BinaryExpressionOperator::Substitution => self.print("-"),
                    BinaryExpressionOperator::Multiplication => self.print("*"),
                    BinaryExpressionOperator::Division => self.print("/"),
                    BinaryExpressionOperator::Modulus => self.print("%"),
                    BinaryExpressionOperator::Exponentiation => self.print("**"),
                    BinaryExpressionOperator::LessThan => self.print("<"),
                    BinaryExpressionOperator::LessThanEquals => self.print("<="),
                    BinaryExpressionOperator::GreaterThan => self.print(">"),
                    BinaryExpressionOperator::GreaterThanEquals => self.print(">="),
                    BinaryExpressionOperator::In => self.print("in"),
                    BinaryExpressionOperator::Instanceof => self.print("instanceof"),
                    BinaryExpressionOperator::LeftShift => self.print("<<"),
                    BinaryExpressionOperator::RightShift => self.print(">>"),
                    BinaryExpressionOperator::UnsignedRightShift => self.print(">>>"),
                    BinaryExpressionOperator::LooseEquals => self.print("=="),
                    BinaryExpressionOperator::LooseNotEquals => self.print("!="),
                    BinaryExpressionOperator::StrictEquals => self.print("==="),
                    BinaryExpressionOperator::StrictNotEquals => self.print("!=="),
                    BinaryExpressionOperator::NullishCoalescing => self.print("??"),
                    BinaryExpressionOperator::BitwiseOr => self.print("|"),
                    BinaryExpressionOperator::BitwiseAnd => self.print("&"),
                    BinaryExpressionOperator::BitwiseXor => self.print("^"),
                };
                if e.operator == BinaryExpressionOperator::In
                    || e.operator == BinaryExpressionOperator::Instanceof
                {
                    self.print(" ");
                } else {
                    self.print_space();
                }
                self.print_expression(&e.right, right_precedence);
                if wrap {
                    self.print(")");
                }
            }

            Expression::Unary(e) => {
                let operator_precedence = e.operator.precedence();
                let wrap = precedence >= operator_precedence;
                if wrap {
                    self.print("(");
                }
                match &e.operator {
                    UnaryExpressionOperator::Positive => self.print("+"),
                    UnaryExpressionOperator::Negative => self.print("-"),
                    UnaryExpressionOperator::BinaryNot => self.print("~"),
                    UnaryExpressionOperator::LogicalNot => self.print("!"),
                    UnaryExpressionOperator::Void => self.print("void "),
                    UnaryExpressionOperator::Typeof => self.print("typeof "),
                    UnaryExpressionOperator::Delete => self.print("delete "),
                };
                self.print_expression(&e.argument, operator_precedence.lower());
                if wrap {
                    self.print(")");
                }
            }

            Expression::Logical(l) => {
                let operator_precedence = l.operator.precedence();
                let wrap = precedence >= operator_precedence;
                if wrap {
                    self.print("(");
                }
                self.print_expression(&l.left, operator_precedence.lower());
                self.print_space();
                match &l.operator {
                    LogicalExpressionOperator::Or => self.print("||"),
                    LogicalExpressionOperator::And => self.print("&&"),
                    LogicalExpressionOperator::NullishCoalescing => self.print("??"),
                }
                self.print_space();
                self.print_expression(&l.right, operator_precedence.clone());
                if wrap {
                    self.print(")");
                }
            }

            Expression::StringLiteral(e) => {
                self.print("\"");
                self.print(&e.value);
                self.print("\"");
            }

            Expression::Call(c) => {
                self.print_expression(&c.callee, Precedence::Postfix);
                self.print("(");

                for (idx, argument) in c.arguments.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }

                    match &argument {
                        ArgumentKind::Expression(e) => self.print_expression(e, Precedence::Comma),
                        ArgumentKind::Spread(s) => self.print_spread_element(s),
                    }
                }
                self.print(")");
            }

            Expression::Function(f) => {
                let wrap = self.text.len() == self.statement_start;
                if wrap {
                    self.print("(");
                }
                self.print("function");
                if f.generator {
                    self.print("*");
                }
                if let Some(identifier) = &f.identifier {
                    self.print_space();
                    self.print_identifier(identifier);
                }
                self.print("(");
                self.print_parameters(&f.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&f.body);
                if wrap {
                    self.print(")");
                }
            }

            Expression::Conditional(c) => {
                let wrap = precedence >= Precedence::Conditional;
                if wrap {
                    self.print("(");
                }
                self.print_expression(&c.test, Precedence::Conditional);
                self.print(" ? ");
                self.print_expression(&c.consequence, Precedence::Yield);
                self.print(" : ");
                self.print_expression(&c.alternate, Precedence::Yield);
                if wrap {
                    self.print(")");
                }
            }

            Expression::New(n) => {
                self.print("new ");
                self.print_expression(&n.callee, Precedence::New);
                self.print("(");
                for (idx, argument) in n.arguments.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    match &argument {
                        ArgumentKind::Expression(e) => self.print_expression(e, Precedence::Comma),
                        ArgumentKind::Spread(s) => self.print_spread_element(s),
                    }
                }
                self.print(")");
            }

            Expression::Member(m) => {
                self.print_expression(&m.object, Precedence::Postfix);
                if m.computed {
                    self.print("[");
                } else {
                    self.print(".");
                }
                self.print_expression(&m.property, Precedence::Lowest);
                if m.computed {
                    self.print("]");
                }
            }

            Expression::Object(o) => {
                let wrap = self.text.len() == self.statement_start;
                if wrap {
                    self.print("(");
                }
                self.print("{");
                for (idx, property) in o.properties.iter().enumerate() {
                    if idx == 0 {
                        self.print_space();
                    }
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print_object_expression_property(property);

                    if idx == o.properties.len() - 1 {
                        self.print_space();
                    }
                }
                self.print("}");
                if wrap {
                    self.print(")");
                }
            }

            Expression::Assignment(a) => {
                match &a.left {
                    AssignmentExpressionLeft::Binding(b) => self.print_binding(b),
                    AssignmentExpressionLeft::Expression(e) => {
                        self.print_expression(e, Precedence::Comma)
                    }
                };
                self.print_space();
                match a.operator {
                    AssignmentExpressionOperator::Assign => self.print("="),
                    AssignmentExpressionOperator::AdditionAssign => self.print("+="),
                    AssignmentExpressionOperator::SubstitutionAssign => self.print("-="),
                    AssignmentExpressionOperator::MultiplicationAssign => self.print("*="),
                    AssignmentExpressionOperator::DivisionAssign => self.print("/="),
                    AssignmentExpressionOperator::ModulusAssign => self.print("%="),
                    AssignmentExpressionOperator::ExponentiationAssign => self.print("**="),
                    AssignmentExpressionOperator::LeftShiftAssign => self.print("<<="),
                    AssignmentExpressionOperator::RightShiftAssign => self.print(">>="),
                    AssignmentExpressionOperator::UnsignedRightShiftAssign => self.print(">>>="),
                    AssignmentExpressionOperator::BitwiseOrAssign => self.print("|="),
                    AssignmentExpressionOperator::BitwiseAndAssign => self.print("&="),
                    AssignmentExpressionOperator::BitwiseXorAssign => self.print("^="),
                    AssignmentExpressionOperator::NullishCoalescingAssign => self.print("??="),
                    AssignmentExpressionOperator::LogicalOrAssign => self.print("||="),
                    AssignmentExpressionOperator::LogicalAndAssign => self.print("&&="),
                }
                self.print_space();
                self.print_expression(&a.right, Precedence::Assign.lower());
            }

            Expression::ArrowFunction(a) => {
                self.print("(");
                self.print_parameters(&a.parameters);
                self.print(")");
                self.print_space();
                self.print("=>");
                self.print_space();
                match &a.body {
                    ArrowFunctionExpressionBody::BlockStatement(b) => self.print_block_statement(b),
                    ArrowFunctionExpressionBody::Expression(e) => {
                        self.print_expression(e, Precedence::Comma)
                    }
                }
            }

            Expression::Sequence(s) => {
                let wrap = precedence >= Precedence::Comma;
                if wrap {
                    self.print("(");
                }
                for (idx, expression) in s.expressions.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }

                    self.print_expression(&expression, Precedence::Comma);
                }
                if wrap {
                    self.print(")");
                }
            }

            Expression::Update(u) => {
                match &u.operator {
                    UpdateExpressionOperator::PrefixIncrement => self.print("++"),
                    UpdateExpressionOperator::PrefixDecrement => self.print("--"),
                    _ => {}
                };
                self.print_expression(&u.argument, Precedence::Prefix);
                match &u.operator {
                    UpdateExpressionOperator::PostfixIncrement => self.print("++"),
                    UpdateExpressionOperator::PostfixDecrement => self.print("--"),
                    _ => {}
                }
            }

            Expression::TemplateLiteral(t) => {
                self.print("`");
                self.print(&t.head);
                for part in &t.parts {
                    self.print("${");
                    self.print_expression(&part.expression, Precedence::Comma);
                    self.print("}");
                    self.print(&part.text);
                }
                self.print("`");
            }
        }
    }

    fn print_class_body(&mut self, properties: &Vec<ClassPropertyKind>) {
        if properties.len() == 0 {
            self.print("{}");
            return;
        }
        self.print("{");
        self.print_space();
        for (idx, item) in properties.iter().enumerate() {
            if idx != 0 {
                self.print_newline();
            }

            match item {
                ClassPropertyKind::Constructor(c) => {
                    self.print("constructor(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
                ClassPropertyKind::Method(c) => {
                    self.print_literal_property_name(&c.identifier);
                    self.print("(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
                ClassPropertyKind::MethodComputed(c) => {
                    self.print_computed_property_name(&c.key);
                    self.print("(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
                ClassPropertyKind::MethodGet(c) => {
                    self.print("get ");
                    self.print_literal_property_name(&c.identifier);
                    self.print("(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
                ClassPropertyKind::MethodGetComputed(c) => {
                    self.print("get");
                    self.print_space();
                    self.print_computed_property_name(&c.key);
                    self.print("(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
                ClassPropertyKind::MethodSet(c) => {
                    self.print("set ");
                    self.print_literal_property_name(&c.identifier);
                    self.print("(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
                ClassPropertyKind::MethodSetComputed(c) => {
                    self.print("set");
                    self.print_space();
                    self.print_computed_property_name(&c.key);
                    self.print("(");
                    self.print_parameters(&c.parameters);
                    self.print(")");
                    self.print_space();
                    self.print_block_statement(&c.body);
                }
            }
        }

        self.print_space();
        self.print("}");
    }

    fn print_parameters(&mut self, parameters: &Vec<ParameterKind>) {
        for (idx, parameter) in parameters.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_parameter(parameter);
        }
    }

    fn print_parameter(&mut self, parameter: &ParameterKind) {
        match &parameter {
            ParameterKind::Parameter(p) => {
                self.print_binding(&p.binding);
                if let Some(initializer) = &p.initializer {
                    self.print_space();
                    self.print("=");
                    self.print_space();
                    self.print_expression(initializer, Precedence::Comma);
                }
            }
            ParameterKind::Rest(r) => {
                self.print("...");
                self.print_binding(&r.binding);
            }
        }
    }

    fn print_spread_element(&mut self, spread_expression: &SpreadElement) {
        self.print("...");
        self.print_expression(&spread_expression.element, Precedence::Comma);
    }

    fn print_object_expression_property(&mut self, property: &ObjectExpressionPropertyKind) {
        match property {
            ObjectExpressionPropertyKind::Spread(s) => self.print_spread_element(s),
            ObjectExpressionPropertyKind::Property(p) => {
                self.print_literal_property_name(&p.key);
                self.print(":");
                self.print_space();
                self.print_expression(&p.value, Precedence::Comma);
            }
            ObjectExpressionPropertyKind::Shorthand(p) => {
                self.print_identifier(&p.key);
            }
            ObjectExpressionPropertyKind::Computed(p) => {
                self.print_computed_property_name(&p.key);
                self.print(":");
                self.print_space();
                self.print_expression(&p.value, Precedence::Comma);
            }
            ObjectExpressionPropertyKind::Method(m) => {
                self.print_literal_property_name(&m.key);
                self.print("(");
                self.print_parameters(&m.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&m.body);
            }
            ObjectExpressionPropertyKind::MethodComputed(m) => {
                self.print_computed_property_name(&m.key);
                self.print("(");
                self.print_parameters(&m.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&m.body);
            }
            ObjectExpressionPropertyKind::MethodGet(m) => {
                self.print("get ");
                self.print_literal_property_name(&m.key);
                self.print("(");
                self.print_parameters(&m.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&m.body);
            }
            ObjectExpressionPropertyKind::MethodGetComputed(m) => {
                self.print("get ");
                self.print_computed_property_name(&m.key);
                self.print("(");
                self.print_parameters(&m.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&m.body);
            }
            ObjectExpressionPropertyKind::MethodSet(m) => {
                self.print("set ");
                self.print_literal_property_name(&m.key);
                self.print("(");
                self.print_parameters(&m.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&m.body);
            }
            ObjectExpressionPropertyKind::MethodSetComputed(m) => {
                self.print("set ");
                self.print_computed_property_name(&m.key);
                self.print("(");
                self.print_parameters(&m.parameters);
                self.print(")");
                self.print_space();
                self.print_block_statement(&m.body);
            }
        }
    }

    fn print_identifier(&mut self, id: &Identifier) {
        self.print(&id.name);
    }

    fn print_binding(&mut self, binding: &Binding) {
        match binding {
            Binding::Identifier(i) => self.print_identifier(i),
            Binding::Object(o) => self.print_object_binding(o),
            Binding::Array(a) => self.print_array_binding(a),
        };
    }

    fn print_object_binding(&mut self, object_binding: &ObjectBinding) {
        if object_binding.properties.len() == 0 {
            self.print("{}");
            return;
        }

        self.print("{");
        self.print_space();
        for (idx, property) in object_binding.properties.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_newline();
            }

            match &property {
                ObjectBindingPropertyKind::Property(o) => {
                    self.print_literal_property_name(&o.key);
                    self.print(":");
                    self.print_space();
                    self.print_binding(&o.binding);
                    if let Some(initializer) = &o.initializer {
                        self.print_space();
                        self.print("=");
                        self.print_space();
                        self.print_expression(initializer, Precedence::Comma);
                    }
                }
                ObjectBindingPropertyKind::Shorthand(o) => {
                    self.print_identifier(&o.key);
                    if let Some(initializer) = &o.initializer {
                        self.print_space();
                        self.print("=");
                        self.print_space();
                        self.print_expression(initializer, Precedence::Comma);
                    }
                }
                ObjectBindingPropertyKind::Computed(o) => {
                    self.print_computed_property_name(&o.key);
                    self.print(":");
                    self.print_space();
                    self.print_binding(&o.binding);
                    if let Some(initializer) = &o.initializer {
                        self.print_space();
                        self.print("=");
                        self.print_space();
                        self.print_expression(initializer, Precedence::Comma);
                    }
                }
                ObjectBindingPropertyKind::Rest(o) => {
                    self.print("...");
                    self.print_identifier(&o.key);
                }
            }
        }

        self.print_space();
        self.print("}");
    }

    fn print_array_binding(&mut self, array_binding: &ArrayBinding) {
        if array_binding.items.len() == 0 {
            self.print("[]");
        } else {
            self.print("[");
            for (idx, item) in array_binding.items.iter().enumerate() {
                if idx != 0 {
                    self.print(",");
                    self.print_space();
                }

                if let Some(i) = &item {
                    match i {
                        ArrayBindingItemKind::Item(i) => {
                            self.print_binding(&i.binding);
                            if let Some(initializer) = &i.initializer {
                                self.print_space();
                                self.print("=");
                                self.print_space();
                                self.print_expression(initializer, Precedence::Comma);
                            }
                        }
                        ArrayBindingItemKind::Rest(r) => {
                            self.print("...");
                            self.print_binding(&r.binding);
                        }
                    }
                } else {
                    self.print(",");
                }
            }
            self.print("]");
        }
    }

    fn print_numeric_literal(&mut self, numeric_literal: &NumericLiteral) {
        self.print(&format!("{}", numeric_literal.value));
    }

    fn print_newline(&mut self) {
        self.print("\n");
    }

    fn print_semicolon_after_statement(&mut self) {
        self.print(";\n");
    }

    fn print_space(&mut self) {
        self.print(" ");
    }

    fn print(&mut self, text: &str) {
        self.text.push_str(text);
    }
}
