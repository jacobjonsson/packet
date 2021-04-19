use js_ast_next::{
    array_expression::{ArrayExpression, ArrayExpressionElement},
    array_hole::ArrayHole,
    binding_identifier::BindingIdentifier,
    boolean_literal::BooleanLiteral,
    expression_statement::ExpressionStatement,
    lexical_binding::LexicalBinding,
    lexical_declaration::LexicalDeclaration,
    numeric_literal::NumericLiteral,
    regexp_literal::RegexpLiteral,
    spread_element::SpreadElement,
    string_literal::StringLiteral,
    variable_declaration::VariableDeclaration,
    variable_statement::VariableStatement,
    Expression, Statement, TargetBindingPattern, AST,
};
use js_error::{JSError, JSErrorKind};
use js_lexer_next::{Lexer, Token};
use span::Span;

pub type ParserError<T> = Result<T, JSError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    /// Are we in strict mode
    strict: bool,
    /// Are we in a module
    module: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        Parser {
            lexer,
            strict: true,
            module: true,
        }
    }

    pub fn parse(&mut self) -> ParserError<AST> {
        let mut statements: Vec<Statement> = Vec::new();
        self.lexer.next()?;
        while self.lexer.token != Token::Eof {
            statements.push(self.parse_statement()?);
        }

        Ok(AST { statements })
    }

    /// Parses a statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-ecmascript-language-statements-and-declarations)
    fn parse_statement(&mut self) -> ParserError<Statement> {
        match self.lexer.token {
            Token::OpenBrace => self.parse_block_statement(),
            Token::Semicolon => self.parse_empty_statement(),
            Token::If => self.parse_if_statement(),
            Token::Do => self.parse_do_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            Token::Var => self
                .parse_variable_statement()
                .map(Statement::VariableStatement),
            Token::Const | Token::Let => self
                .parse_lexical_declaration()
                .map(Statement::LexicalDeclaration),
            Token::Continue => self.parse_continue_statement(),
            Token::Break => self.parse_break_statement(),
            Token::Return => self.parse_break_statement(),
            Token::Throw => self.parse_break_statement(),
            Token::Try => self.parse_try_statement(),
            Token::Debugger => self.parse_debugger_statement(),
            Token::Switch => self.parse_switch_statement(),
            Token::With => self.parse_with_statement(),
            Token::Function => self.parse_function_declaration(),
            _ => self
                .parse_expression_statement()
                .map(Statement::ExpressionStatement),
        }
    }

    /// Parses a block statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-block)
    fn parse_block_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses an empty statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-empty-statement)
    fn parse_empty_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses an if statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-if-statement)
    fn parse_if_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a do while statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-do-while-statement)
    fn parse_do_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a while statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-while-statement)
    fn parse_while_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a for statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-for-statement)
    fn parse_for_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a continue statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-continue-statement)
    fn parse_continue_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a break statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-break-statement)
    fn parse_break_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a try statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-try-statement)
    fn parse_try_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a debugger statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-debugger-statement)
    fn parse_debugger_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a switch statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-switch-statement)
    fn parse_switch_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a with statement (14.7.4)
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-with-statement)
    fn parse_with_statement(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a function declaration (14.7.4)
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-function-definitions)
    fn parse_function_declaration(&mut self) -> ParserError<Statement> {
        todo!()
    }

    /// Parses a variable statement (14.7.4)
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-variable-statement)
    fn parse_variable_statement(&mut self) -> ParserError<VariableStatement> {
        let start = self.lexer.token_start;
        self.lexer.next()?; // var
        let declarations = self.parse_variable_declaration_list()?;
        self.lexer.consume_optional(Token::Semicolon)?;
        let end = self.lexer.token_start;
        Ok(VariableStatement {
            declarations,
            span: Span::new(start, end),
        })
    }

    /// VariableDeclarationList :
    ///     VariableDeclaration
    ///     VariableDeclarationList `,` VariableDeclaration
    fn parse_variable_declaration_list(&mut self) -> ParserError<Vec<VariableDeclaration>> {
        let mut declarations: Vec<VariableDeclaration> = Vec::new();

        loop {
            declarations.push(self.parse_variable_declaration()?);
            if self.lexer.token != Token::Comma {
                break;
            }
            self.lexer.next()?;
        }

        Ok(declarations)
    }

    /// VariableDeclaration:
    ///     BindingIdentifier Option<Initializer>
    ///     BindingPattern Option<Initializer>
    fn parse_variable_declaration(&mut self) -> ParserError<VariableDeclaration> {
        let start = self.lexer.token_start;
        let binding = match self.lexer.token.is_keyword() {
            true => todo!(),
            false => self
                .parse_binding_identifier()
                .map(TargetBindingPattern::BindingIdentifier)?,
        };
        let initializer = match self.lexer.token {
            Token::Equals => {
                self.lexer.next()?;
                self.parse_expression().map(Some)?
            }
            _ => None,
        };
        let end = self.lexer.token_start;
        Ok(VariableDeclaration {
            span: Span::new(start, end),
            binding,
            initializer,
        })
    }

    /// Parses a binding identifier
    fn parse_binding_identifier(&mut self) -> ParserError<BindingIdentifier> {
        let name = self.lexer.token_text.to_string();

        if self.strict && self.lexer.token == Token::Yield {
            return Err(JSError::new(
                JSErrorKind::UnexpectedYieldAsBindingIdentifier,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        if self.module && self.lexer.token == Token::Await {
            return Err(JSError::new(
                JSErrorKind::UnexpectedAwaitAsBindingIdentifier,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        if self.lexer.token.is_keyword() {
            return Err(JSError::new(
                JSErrorKind::ExpectedBindingIdentifier,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        if self.strict && self.lexer.token.is_future_reserved() {
            return Err(JSError::new(
                JSErrorKind::StrictModeReserved,
                Span::new(self.lexer.token_start, self.lexer.token_end),
            ));
        }

        let start = self.lexer.token_start;
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(BindingIdentifier {
            name,
            span: Span::new(start, end),
        })
    }

    /// Parses a lexical declaration (14.7.4)
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-let-and-const-declarations)
    fn parse_lexical_declaration(&mut self) -> ParserError<LexicalDeclaration> {
        let start = self.lexer.token_start;
        let is_const = match self.lexer.token {
            Token::Const => true,
            _ => false,
        };
        self.lexer.next()?;
        let declarations = self.parse_lexical_binding_list(is_const)?;
        self.lexer.consume_optional(Token::Semicolon)?;
        let end = self.lexer.token_start;
        Ok(LexicalDeclaration {
            declarations,
            is_const,
            span: Span::new(start, end),
        })
    }

    /// Parses a lexical binding list
    fn parse_lexical_binding_list(&mut self, is_const: bool) -> ParserError<Vec<LexicalBinding>> {
        let mut declarations: Vec<LexicalBinding> = Vec::new();
        loop {
            declarations.push(self.parse_lexical_binding(is_const)?);
            if self.lexer.token != Token::Comma {
                break;
            }
            self.lexer.next()?;
        }
        Ok(declarations)
    }

    /// Parses a lexical binding
    ///
    fn parse_lexical_binding(&mut self, is_const: bool) -> ParserError<LexicalBinding> {
        let start = self.lexer.token_start;
        let binding = match self.lexer.token {
            Token::OpenBrace | Token::OpenBracket => todo!(),
            _ => self
                .parse_binding_identifier()
                .map(TargetBindingPattern::BindingIdentifier)?,
        };
        let initializer = match self.lexer.token {
            Token::Equals => {
                self.lexer.next()?;
                self.parse_expression().map(Some)?
            }
            _ => {
                if is_const {
                    return Err(JSError::new(
                        JSErrorKind::MissingConstInitializer,
                        Span::new(start, self.lexer.token_end),
                    ));
                } else {
                    None
                }
            }
        };
        let end = self.lexer.token_start;
        Ok(LexicalBinding {
            binding,
            initializer,
            span: Span::new(start, end),
        })
    }

    /// Parses an expression statement
    ///
    /// See [spec](https://tc39.es/ecma262/#sec-expression-statement)
    fn parse_expression_statement(&mut self) -> ParserError<ExpressionStatement> {
        let start = self.lexer.token_start;
        let expression = self.parse_expression()?;
        let end = self.lexer.token_start;
        Ok(ExpressionStatement {
            expression,
            span: Span::new(start, end),
        })
    }

    /// Parses an expression
    ///
    /// Expression parsing in packet is based the ideas brought
    /// forward by [Vaughan Pratt](https://tdop.github.io), so called
    /// pratt parser or top down operator precedence parsing.
    fn parse_expression(&mut self) -> ParserError<Expression> {
        let expr = self.parse_prefix()?;
        self.parse_suffix(expr)
    }

    /// Parses an expression in a prefix position
    fn parse_prefix(&mut self) -> ParserError<Expression> {
        match self.lexer.token {
            Token::Number => self.parse_numeric_literal().map(Expression::NumericLiteral),
            Token::OpenBracket => self
                .parse_array_expression()
                .map(Expression::ArrayExpression),
            Token::OpenParen => self.parse_paren_expression(),
            Token::Slash => self.parse_regexp_literal().map(Expression::RegexpLiteral),
            Token::String => self.parse_string_literal().map(Expression::StringLiteral),
            Token::True | Token::False => {
                self.parse_boolean_literal().map(Expression::BooleanLiteral)
            }
            _ => todo!(),
        }
    }

    /// Parses an paren expression
    fn parse_paren_expression(&mut self) -> ParserError<Expression> {
        self.lexer.next()?;
        let mut elements: Vec<Expression> = Vec::new();
        while self.lexer.token != Token::CloseParen {
            if self.lexer.token == Token::DotDotDot {
                todo!()
            }

            let element = self.parse_expression()?;
            elements.push(element);
        }
        self.lexer.consume(Token::CloseParen)?;

        // TODO: Parse arrow functions
        Ok(elements[0].clone())
    }

    /// Parses an array expression
    fn parse_array_expression(&mut self) -> ParserError<ArrayExpression> {
        let start = self.lexer.token_start;
        let mut elements: Vec<ArrayExpressionElement> = Vec::new();
        self.lexer.consume(Token::OpenBracket)?;
        while self.lexer.token != Token::CloseBracket {
            match self.lexer.token {
                // [,]
                Token::Comma => {
                    let element = self.parse_array_hole().map(ArrayExpressionElement::Hole)?;
                    elements.push(element);
                }
                // [...expression]
                Token::DotDotDot => {
                    let element = self
                        .parse_spread_expression()
                        .map(ArrayExpressionElement::Spread)?;
                    elements.push(element);
                    if self.lexer.token == Token::Comma {
                        self.lexer.next()?;
                    }
                }
                // Anything else
                _ => {
                    let element = self
                        .parse_expression()
                        .map(ArrayExpressionElement::Expression)?;
                    elements.push(element);
                    if self.lexer.token == Token::Comma {
                        self.lexer.next()?;
                    }
                }
            };
        }
        self.lexer.consume(Token::CloseBracket)?;
        let end = self.lexer.token_start;
        Ok(ArrayExpression {
            elements,
            span: Span::new(start, end),
        })
    }

    /// Parses an array hole
    fn parse_array_hole(&mut self) -> ParserError<ArrayHole> {
        let start = self.lexer.token_start;
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(ArrayHole {
            span: Span::new(start, end),
        })
    }

    /// Parses a spread expression
    fn parse_spread_expression(&mut self) -> ParserError<SpreadElement> {
        let start = self.lexer.token_start;
        self.lexer.next()?;
        let argument = self.parse_expression()?;
        let end = self.lexer.token_start;
        Ok(SpreadElement {
            argument,
            span: Span::new(start, end),
        })
    }

    /// Parses an numeric expression
    fn parse_numeric_literal(&mut self) -> ParserError<NumericLiteral> {
        let start = self.lexer.token_start;
        let value = self.lexer.token_number;
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(NumericLiteral {
            value,
            span: Span::new(start, end),
        })
    }

    /// Parses a string expression
    fn parse_string_literal(&mut self) -> ParserError<StringLiteral> {
        let start = self.lexer.token_start;
        let value = self.lexer.token_text.to_string();
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(StringLiteral {
            value,
            span: Span::new(start, end),
        })
    }

    /// Parses a boolean expression
    fn parse_boolean_literal(&mut self) -> ParserError<BooleanLiteral> {
        let start = self.lexer.token_start;
        let value = match self.lexer.token {
            Token::True => true,
            Token::False => false,
            _ => unreachable!(),
        };
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(BooleanLiteral {
            value,
            span: Span::new(start, end),
        })
    }

    /// Parses a regexp expression
    fn parse_regexp_literal(&mut self) -> ParserError<RegexpLiteral> {
        self.lexer.next_as_regexp()?;
        let start = self.lexer.token_start;
        let value = self.lexer.token_text.to_string();
        self.lexer.next()?;
        let end = self.lexer.token_start;
        Ok(RegexpLiteral {
            value,
            span: Span::new(start, end),
        })
    }

    /// Parses an expression in an infix or suffix position
    fn parse_suffix(&mut self, expr: Expression) -> ParserError<Expression> {
        Ok(expr)
    }
}
