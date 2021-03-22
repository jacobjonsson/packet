use js_ast::{expression::*, statement::*, Program};

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
                self.print_variable_declaration(v);
                self.print(";");
            }
            Statement::EmptyStatement(_) => self.print(";"),

            Statement::Return(r) => {
                self.print("return");
                if let Some(expression) = &r.expression {
                    self.print(" ");
                    self.print_expression(expression);
                }
                self.print(";");
            }

            Statement::Expression(e) => {
                self.statement_start = self.text.len();
                self.print_expression(&e.expression);
            }

            Statement::If(i) => self.print_if_statement(i),

            Statement::ContinueStatement(c) => {
                self.print("continue");
                if let Some(label) = &c.label {
                    self.print_space();
                    self.print_identifier(label);
                }
            }
            Statement::BreakStatement(b) => {
                self.print("break");
                if let Some(label) = &b.label {
                    self.print_space();
                    self.print_identifier(label);
                }
            }

            Statement::For(f) => {
                self.print("for");
                self.print_space();
                self.print("(");
                if let Some(init) = &f.init {
                    match init {
                        ForStatementInit::Expression(e) => self.print_expression(e),
                        ForStatementInit::VariableDeclaration(v) => {
                            self.print_variable_declaration(v);
                            self.print(";");
                        }
                        ForStatementInit::Pattern(p) => self.print_pattern(p),
                    }
                }
                // We currently auto print semicolons for variable declarations,
                // hence why we don't print anything here.
                self.print_space();
                if let Some(test) = &f.test {
                    self.print_expression(test);
                }
                self.print(";");
                self.print_space();
                if let Some(update) = &f.update {
                    self.print_expression(update);
                }
                self.print(")");
                self.print_space();
                self.print_statement(&f.body);
            }

            Statement::ForInStatement(f) => {
                self.print("for");
                self.print_space();
                self.print("(");
                match &f.left {
                    ForStatementInit::VariableDeclaration(v) => self.print_variable_declaration(v),
                    ForStatementInit::Pattern(p) => self.print_pattern(p),
                    ForStatementInit::Expression(e) => self.print_expression(e),
                };
                self.print(" in ");
                self.print_expression(&f.right);
                self.print(")");
                self.print_space();
                self.print_statement(&f.body);
            }

            Statement::ForOfStatement(f) => {
                self.print("for");
                self.print_space();
                self.print("(");
                match &f.left {
                    ForStatementInit::Expression(e) => self.print_expression(e),
                    ForStatementInit::VariableDeclaration(v) => self.print_variable_declaration(v),
                    ForStatementInit::Pattern(p) => self.print_pattern(p),
                };
                self.print(" of ");
                self.print_expression(&f.right);
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
                self.print_expression(&d.test);
                self.print(")");
            }

            Statement::WhileStatement(w) => {
                self.print("while");
                self.print_space();
                self.print("(");
                self.print_expression(&w.test);
                self.print(")");
                self.print_space();
                self.print_statement(&w.body);
            }

            Statement::SwitchStatement(s) => {
                self.print("switch");
                self.print_space();
                self.print("(");
                self.print_expression(&s.discriminant);
                self.print(")");

                self.print_space();
                self.print("{");
                if s.cases.len() == 0 {
                    self.print("}");
                    return;
                }
                self.print_space();
                let cases: Vec<&SwitchCase> = s.cases.iter().filter(|c| c.test != None).collect();
                for (idx, case) in cases.iter().enumerate() {
                    if idx != 0 {
                        self.print_space();
                    }
                    self.print("case ");
                    // Cases needs to have a test, only the default case is allowed to be none.
                    self.print_expression(case.test.as_ref().unwrap());
                    self.print(":");
                    self.print_space();
                    for consequent in &case.consequent {
                        self.print_statement(consequent.as_ref());
                    }
                }
                let default: Option<&SwitchCase> = s.cases.iter().find(|c| c.test == None);
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

            Statement::DebuggerStatement(_) => self.print("debugger"),

            Statement::LabeledStatement(l) => {
                self.print_identifier(&l.identifier);
                self.print(":");
                self.print_space();
                self.print_statement(&l.body);
            }

            Statement::ThrowStatement(t) => {
                self.print("throw ");
                self.print_expression(&t.argument);
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
                    self.print_pattern(&handler.param);
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
                self.print(";");
            }

            Statement::WithStatement(w) => {
                self.print("with");
                self.print_space();
                self.print("(");
                self.print_expression(&w.object);
                self.print(")");
                self.print_space();
                self.print_statement(&w.body);
            }

            Statement::Block(b) => self.print_block_statement(b),
            Statement::FunctionDeclaration(f) => self.print_function_declaration(f),

            // export * from "a";
            Statement::ExportAllDeclaration(e) => {
                self.print("export * from ");
                self.print_string_literal(&e.source);
                self.print(";");
            }

            // export {a}
            // export {a as b}
            // export function a() {}
            // export var a = 1;
            // export {a} from "b";
            // export {a as c} from "b";
            Statement::ExportNamedDeclaration(e) => {
                self.print("export");
                if let Some(declaration) = &e.declaration {
                    self.print(" ");
                    match declaration {
                        Declaration::FunctionDeclaration(f) => self.print_function_declaration(f),
                        Declaration::VariableDeclaration(v) => {
                            self.print_variable_declaration(v);
                            self.print(";")
                        }
                    }
                } else {
                    self.print_space();
                    self.print("{");
                    for (idx, specifier) in e.specifiers.iter().enumerate() {
                        if idx == 0 {
                            self.print_space();
                        }
                        if idx != 0 {
                            self.print(",");
                            self.print_space();
                        }

                        self.print_identifier(&specifier.local);
                        if specifier.local.name != specifier.exported.name {
                            self.print(" as ");
                            self.print_identifier(&specifier.exported);
                        }

                        if idx == e.specifiers.len() - 1 {
                            self.print_space();
                        }
                    }
                    self.print("}");

                    if let Some(source) = &e.source {
                        self.print_space();
                        self.print("from");
                        self.print_space();
                        self.print_string_literal(source);
                    }
                    self.print(";");
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
                        self.print_expression(exp);
                        self.print(";");
                    }
                    ExportDefaultDeclarationKind::AnonymousDefaultExportedFunctionDeclaration(
                        a,
                    ) => self.print_anonymous_default_exported_function_declaration(a),
                }
            }

            Statement::AnonymousDefaultExportedFunctionDeclaration(a) => {
                self.print_anonymous_default_exported_function_declaration(a)
            }
        };
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
            self.print_pattern(&declaration.id);
            if let Some(expression) = &declaration.init {
                self.print_space();
                self.print("=");
                self.print_space();
                self.print_expression(expression);
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
        self.print_expression(&if_statement.test);
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
        self.print("function ");
        self.print_identifier(&function_declaration.id);
        self.print("(");
        for (idx, argument) in function_declaration.parameters.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_pattern(argument);
        }
        self.print(")");
        self.print_space();
        self.print_block_statement(&function_declaration.body);
    }

    fn print_anonymous_default_exported_function_declaration(
        &mut self,
        function_declaration: &AnonymousDefaultExportedFunctionDeclaration,
    ) {
        self.print("function(");
        for (idx, argument) in function_declaration.parameters.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            self.print_pattern(argument);
        }
        self.print(")");
        self.print_space();
        self.print_block_statement(&function_declaration.body);
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
            Expression::ThisExpression(_) => self.print("this"),
            Expression::UpdateExpression(u) => self.print_update_expression(u),
            Expression::AssignmentExpression(a) => {
                match a.left.as_ref() {
                    AssignmentExpressionLeft::Expression(exp) => self.print_expression(exp),
                    AssignmentExpressionLeft::Pattern(p) => self.print_pattern(p),
                }
                self.print_space();
                match &a.operator {
                    AssignmentOperator::Equals => self.print("="),
                    AssignmentOperator::PlusEquals => self.print("+="),
                    AssignmentOperator::MinusEquals => self.print("-="),
                    AssignmentOperator::AsteriskEquals => self.print("*="),
                    AssignmentOperator::SlashEquals => self.print("/="),
                    AssignmentOperator::PercentEquals => self.print("%="),
                    AssignmentOperator::LessThanLessThanEquals => self.print("<<="),
                    AssignmentOperator::GreaterThanGreaterThanEquals => self.print(">>="),
                    AssignmentOperator::GreaterThanGreaterThanGreaterThanEquals => {
                        self.print(">>>=")
                    }
                    AssignmentOperator::BarEquals => self.print("|="),
                    AssignmentOperator::CaretEquals => self.print("^="),
                    AssignmentOperator::AmpersandEquals => self.print("&="),
                }
                self.print_space();
                self.print_expression(&a.right);
            }
            Expression::ArrayExpression(a) => {
                self.print("[");
                for (idx, element) in a.elements.iter().enumerate() {
                    let is_last_element = idx < a.elements.len() - 1;
                    match element {
                        Some(expression) => {
                            self.print_expression(expression);
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
            Expression::LogicalExpression(l) => {
                self.print_expression(&l.left);
                self.print_space();
                match &l.operator {
                    LogicalOperator::AmpersandAmpersand => self.print("&&"),
                    LogicalOperator::BarBar => self.print("||"),
                };
                self.print_space();
                self.print_expression(&l.right);
            }
            Expression::BinaryExpression(e) => {
                self.print_expression(e.left.as_ref());
                self.print_space();
                match e.operator {
                    BinaryOperator::Ampersand => self.print("&"),
                    BinaryOperator::EqualsEquals => self.print("=="),
                    BinaryOperator::EqualsEqualsEquals => self.print("==="),
                    BinaryOperator::ExclamationEquals => self.print("!="),
                    BinaryOperator::ExclamationEqualsEquals => self.print("!=="),
                    BinaryOperator::LessThan => self.print("<"),
                    BinaryOperator::LessThanLessThan => self.print("<<"),
                    BinaryOperator::LessThanEquals => self.print("<="),
                    BinaryOperator::GreaterThan => self.print(">"),
                    BinaryOperator::GreaterThanEquals => self.print(">="),
                    BinaryOperator::GreaterThanGreaterThan => self.print(">>"),
                    BinaryOperator::GreaterThanGreaterThanGreaterThan => self.print(">>>"),
                    BinaryOperator::Plus => self.print("+"),
                    BinaryOperator::Minus => self.print("-"),
                    BinaryOperator::Asterisk => self.print("*"),
                    BinaryOperator::Slash => self.print("/"),
                    BinaryOperator::Percent => self.print("%"),
                    BinaryOperator::Bar => self.print("|"),
                    BinaryOperator::Caret => self.print("^"),
                    BinaryOperator::In => self.print("in"),
                    BinaryOperator::Instanceof => self.print("instanceof"),
                }
                self.print_space();
                self.print_expression(e.right.as_ref());
            }

            Expression::PrefixExpression(e) => {
                self.print(&e.operator);
                self.print_expression(e.right.as_ref());
            }

            Expression::StringLiteral(e) => {
                self.print("\"");
                self.print(&e.value);
                self.print("\"");
            }

            Expression::CallExpression(c) => {
                self.print_expression(&c.callee);
                self.print("(");

                for (idx, argument) in c.arguments.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print_expression(&argument);
                }
                self.print(")");
            }

            Expression::FunctionExpression(f) => {
                let wrap = self.text.len() == self.statement_start;
                if wrap {
                    self.print("(");
                }
                self.print("function");
                if let Some(identifier) = &f.id {
                    self.print_space();
                    self.print_identifier(identifier);
                }
                self.print("(");
                for (idx, parameter) in f.parameters.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }

                    self.print_pattern(&parameter);
                }
                self.print(")");
                self.print_space();
                self.print_block_statement(&f.body);
                if wrap {
                    self.print(")");
                }
            }

            Expression::ConditionalExpression(c) => {
                self.print_expression(&c.test);
                self.print(" ? ");
                self.print_expression(&c.consequence);
                self.print(" : ");
                self.print_expression(&c.alternate);
            }

            Expression::NewExpression(n) => {
                self.print("new ");
                self.print_expression(&n.callee);
                self.print("(");
                for (idx, argument) in n.arguments.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print_expression(argument);
                }
                self.print(")");
            }

            Expression::MemberExpression(m) => {
                self.print_expression(&m.object);
                if m.computed {
                    self.print("[");
                } else {
                    self.print(".");
                }
                self.print_expression(&m.property);
                if m.computed {
                    self.print("]");
                }
            }

            Expression::ObjectExpression(o) => {
                self.print("{");
                for (idx, property) in o.properties.iter().enumerate() {
                    if idx == 0 {
                        self.print_space();
                    }
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    // { [a]: b }
                    // { "a": b, "c": d }
                    match &property.key {
                        PropertyKey::Identifier(i) => {
                            // [a]
                            if property.computed {
                                self.print("[");
                            }
                            self.print(&i.name);
                            if property.computed {
                                self.print("]");
                            }
                        }
                        PropertyKey::StringLiteral(s) => {
                            // "a"
                            self.print("\"");
                            self.print(&s.value);
                            self.print("\"");
                        }
                    }
                    self.print(":");
                    self.print_space();
                    self.print_expression(&property.value);

                    if idx == o.properties.len() - 1 {
                        self.print_space();
                    }
                }
                self.print("}");
            }
        }
    }

    fn print_update_expression(&mut self, update_expression: &UpdateExpression) {
        if update_expression.prefix {
            match update_expression.operator {
                UpdateOperator::Increment => self.print("++"),
                UpdateOperator::Decrement => self.print("--"),
            };
        }

        self.print_expression(&update_expression.argument);

        if update_expression.prefix == false {
            match update_expression.operator {
                UpdateOperator::Increment => self.print("++"),
                UpdateOperator::Decrement => self.print("--"),
            };
        }
    }

    fn print_identifier(&mut self, id: &Identifier) {
        self.print(&id.name);
    }

    fn print_property_key(&mut self, property_key: &PropertyKey) {
        match property_key {
            PropertyKey::StringLiteral(s) => self.print_string_literal(s),
            PropertyKey::Identifier(i) => self.print_identifier(i),
        };
    }

    fn print_object_pattern(&mut self, object_pattern: &ObjectPattern) {
        self.print("{");
        if object_pattern.properties.len() > 0 {
            self.print_space();
        }
        for (idx, property) in object_pattern.properties.iter().enumerate() {
            if idx != 0 {
                self.print(",");
                self.print_space();
            }
            match property {
                ObjectPatternProperty::AssignmentProperty(a) => {
                    self.print_property_key(&a.key);

                    // The only reason we don't call self.print_pattern here is because we need to insert : for some of the cases.
                    // a: b
                    // a: { b: c }
                    // a: [b]
                    // a = b
                    match a.value.as_ref() {
                        Pattern::Identifier(i) => {
                            self.print(":");
                            self.print_space();
                            self.print_identifier(i);
                        }
                        Pattern::ObjectPattern(o) => {
                            self.print(":");
                            self.print_space();
                            self.print_object_pattern(o);
                        }
                        Pattern::ArrayPattern(a) => {
                            self.print(":");
                            self.print_space();
                            self.print_array_pattern(a);
                        }
                        Pattern::AssignmentPattern(a) => {
                            self.print_space();
                            self.print_assignment_pattern(a);
                        }
                        Pattern::RestElement(_) => {
                            // This should be impossible, means the user entered: a: ...b which is not valid javascript.
                            panic!("Rest element as property value is not valid")
                        }
                    }
                }
                ObjectPatternProperty::RestElement(r) => {
                    self.print_rest_element(r);
                }
            }
        }
        if object_pattern.properties.len() > 0 {
            self.print_space();
        }
        self.print("}");
    }

    fn print_array_pattern(&mut self, array_pattern: &ArrayPattern) {
        self.print("[");
        for (idx, property) in array_pattern.properties.iter().enumerate() {
            let is_last_element = idx < array_pattern.properties.len() - 1;
            match property {
                Some(pattern) => {
                    self.print_pattern(pattern);
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

    fn print_rest_element(&mut self, rest_element: &RestElement) {
        self.print("...");
        self.print_pattern(&rest_element.argument);
    }

    fn print_assignment_pattern(&mut self, assignment_pattern: &AssignmentPattern) {
        self.print("=");
        self.print_space();
        self.print_expression(&assignment_pattern.right);
    }

    fn print_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Identifier(i) => self.print_identifier(i),
            Pattern::ObjectPattern(o) => self.print_object_pattern(o),
            Pattern::ArrayPattern(a) => self.print_array_pattern(a),
            Pattern::RestElement(r) => self.print_rest_element(r),
            Pattern::AssignmentPattern(a) => self.print_assignment_pattern(a),
        };
    }

    fn print_space(&mut self) {
        self.print(" ");
    }

    fn print(&mut self, text: &str) {
        self.text.push_str(text);
    }
}
