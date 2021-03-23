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

            Statement::ReturnStatement(r) => {
                self.print("return");
                if let Some(expression) = &r.expression {
                    self.print(" ");
                    self.print_expression(expression, Precedence::Lowest);
                }
                self.print(";");
            }

            Statement::Expression(e) => {
                self.statement_start = self.text.len();
                self.print_expression(&e.expression, Precedence::Lowest);
            }

            Statement::IfStatement(i) => self.print_if_statement(i),

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
                let cases: Vec<&SwitchCase> = s.cases.iter().filter(|c| c.test != None).collect();
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
                self.print_expression(&t.argument, Precedence::Lowest);
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
                        self.print_expression(exp, Precedence::Comma);
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
            self.print_pattern(&declaration.id);
            if let Some(expression) = &declaration.init {
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

    fn print_expression(&mut self, expression: &Expression, precedence: Precedence) {
        match &expression {
            Expression::NullLiteral(_) => self.print("null"),
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
            Expression::RegexpLiteral(r) => {
                self.print(&r.value);
            }
            Expression::ThisExpression(_) => self.print("this"),
            Expression::ArrayExpression(a) => {
                self.print("[");
                for (idx, element) in a.elements.iter().enumerate() {
                    let is_last_element = idx < a.elements.len() - 1;
                    match element {
                        Some(expression) => {
                            self.print_expression(expression, Precedence::Comma);
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
            Expression::BinaryExpression(e) => {
                let entry = get_op_entry(&e.op_code);
                let wrap = precedence >= entry.precedence;
                if wrap {
                    self.print("(");
                }
                let mut left_precedence = entry.precedence.lower();
                let mut right_precedence = entry.precedence.lower();
                if e.op_code.is_right_associative() {
                    left_precedence = entry.precedence.clone();
                }
                if e.op_code.is_left_associative() {
                    right_precedence = entry.precedence.clone();
                }

                match &e.op_code {
                    // "??" can't directly contain "||" or "&&" without being wrapped in parentheses
                    OpCode::BinaryNullishCoalescing => {
                        match e.left.as_ref() {
                            Expression::BinaryExpression(ex) => {
                                if ex.op_code == OpCode::BinaryLogicalOr
                                    || ex.op_code == OpCode::BinaryLogicalAnd
                                {
                                    left_precedence = Precedence::Prefix;
                                }
                            }
                            _ => {}
                        };
                        match e.right.as_ref() {
                            Expression::BinaryExpression(ex) => {
                                if ex.op_code == OpCode::BinaryLogicalOr
                                    || ex.op_code == OpCode::BinaryLogicalAnd
                                {
                                    right_precedence = Precedence::Prefix;
                                }
                            }
                            _ => {}
                        };
                    }

                    // TODO: "**" can't contain certain unary expressions
                    // https://github.com/evanw/esbuild/blob/c8eb58f7fa9dd6f17a062f269a2262b42f282671/internal/js_printer/js_printer.go#L2015
                    _ => {}
                }

                self.print_expression(&e.left, left_precedence);

                if e.op_code != OpCode::BinaryComma {
                    self.print_space();
                }

                self.print(&entry.text);

                self.print_space();
                self.print_expression(&e.right, right_precedence);

                if wrap {
                    self.print(")");
                }
            }

            Expression::UnaryExpression(e) => {
                let entry = get_op_entry(&e.op_code);
                let wrap = precedence >= entry.precedence;
                if wrap {
                    self.print("(");
                }
                if !e.op_code.is_prefix() {
                    self.print_expression(&e.expression, Precedence::Postfix.lower());
                }
                if entry.is_keyword {
                    self.print(&entry.text);
                    self.print(" ");
                } else {
                    self.print(&entry.text);
                }
                if e.op_code.is_prefix() {
                    self.print_expression(&e.expression, Precedence::Prefix.lower());
                }
                if wrap {
                    self.print(")");
                }
            }

            Expression::StringLiteral(e) => {
                self.print("\"");
                self.print(&e.value);
                self.print("\"");
            }

            Expression::CallExpression(c) => {
                self.print_expression(&c.callee, Precedence::Postfix);
                self.print("(");

                for (idx, argument) in c.arguments.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print_expression(&argument, Precedence::Comma);
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
                self.print_expression(&c.test, Precedence::Lowest);
                self.print(" ? ");
                self.print_expression(&c.consequence, Precedence::Lowest);
                self.print(" : ");
                self.print_expression(&c.alternate, Precedence::Lowest);
            }

            Expression::NewExpression(n) => {
                self.print("new ");
                self.print_expression(&n.callee, Precedence::New);
                self.print("(");
                for (idx, argument) in n.arguments.iter().enumerate() {
                    if idx != 0 {
                        self.print(",");
                        self.print_space();
                    }
                    self.print_expression(argument, Precedence::Comma);
                }
                self.print(")");
            }

            Expression::MemberExpression(m) => {
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
                    self.print_expression(&property.value, Precedence::Comma);

                    if idx == o.properties.len() - 1 {
                        self.print_space();
                    }
                }
                self.print("}");
            }
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
        self.print_expression(&assignment_pattern.right, Precedence::Comma);
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
