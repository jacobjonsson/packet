use js_ast::{precedence::Precedence, *};
use js_lexer::{
    eat_token, expect_token, raw, scan_next_token, scan_regexp, scan_template_tail_or_middle, Lexer,
};
use js_token::Token;

/// Parses the given source into an AST.
pub fn parse(source: &str) -> AST {
    let lexer = js_lexer::create(source);
    let mut parser = create(lexer);
    let ast = parse_program(&mut parser);
    ast
}

pub struct ParserError(String);

pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    /// in statement are only allowed in certain expressions.
    allow_in: bool,
}

/// Creates a new parser instance
fn create<'a>(lexer: Lexer<'a>) -> Parser<'a> {
    Parser {
        allow_in: true,
        lexer,
    }
}

fn parse_program(parser: &mut Parser) -> AST {
    let mut statements = Vec::<Statement>::new();

    while &parser.lexer.token != &Token::EndOfFile {
        match parse_statement(parser) {
            Ok(s) => statements.push(s),
            Err(err) => panic!(err.0),
        }
    }

    AST { statements }
}

/// Consumes the next semicolon
fn consume_semicolon(parser: &mut Parser) {
    if parser.lexer.token == Token::Semicolon {
        scan_next_token(&mut parser.lexer);
    }
}

// Bindings
fn parse_binding(parser: &mut Parser) -> ParseResult<Binding> {
    match parser.lexer.token {
        Token::Identifier => parse_identifier(parser).map(Binding::Identifier),
        Token::OpenBrace => parse_object_binding(parser).map(Binding::Object),
        Token::OpenBracket => parse_array_binding(parser).map(Binding::Array),
        _ => todo!(),
    }
}

fn parse_object_binding(parser: &mut Parser) -> ParseResult<ObjectBinding> {
    scan_next_token(&mut parser.lexer);
    let mut properties: Vec<ObjectBindingPropertyKind> = Vec::new();
    while parser.lexer.token != Token::CloseBrace {
        match parser.lexer.token {
            // { ...a }
            Token::DotDotDot => {
                scan_next_token(&mut parser.lexer);
                // Note that the rest element inside of object has different constraints compared
                // to arrays, hence why we hand code the parsing of the rest element here instead of using
                // parse_rest_element_binding. The only node that is be a rest element inside an object
                // is an identifier, anything else is a syntax error.
                let identifier = parse_identifier(parser)?;
                properties.push(ObjectBindingPropertyKind::Rest(ObjectBindingPropertyRest {
                    key: identifier,
                }))
            }

            // { [a]: b }
            Token::OpenBracket => {
                scan_next_token(&mut parser.lexer);
                let key = parse_expression(parser, &Precedence::Comma)?;
                eat_token(&mut parser.lexer, Token::CloseBracket);
                eat_token(&mut parser.lexer, Token::Colon);
                let binding = parse_binding(parser)?;
                let initializer = parse_optional_initializer(parser)?;
                properties.push(ObjectBindingPropertyKind::Computed(
                    ObjectBindingPropertyComputed {
                        key,
                        binding,
                        initializer,
                    },
                ))
            }

            // Anything else: { a, a: b, "a": b, 2: b, null: b, undefined: b }
            _ => {
                let identifier = parse_literal_property_name(parser)?;
                // Means we've hit a shorthand property.
                if parser.lexer.token != Token::Colon {
                    let initializer = parse_optional_initializer(parser)?;
                    // We need to narrow the key type since only an identifier is allowed
                    let key = match identifier {
                        LiteralPropertyName::Identifier(i) => i,
                        _ => panic!("Only identifier is allowed as a shorthand property"),
                    };
                    properties.push(ObjectBindingPropertyKind::Shorthand(
                        ObjectBindingPropertyShorthand { initializer, key },
                    ));
                } else {
                    eat_token(&mut parser.lexer, Token::Colon);
                    let binding = parse_binding(parser)?;
                    let initializer = parse_optional_initializer(parser)?;
                    properties.push(ObjectBindingPropertyKind::Property(ObjectBindingProperty {
                        initializer,
                        key: identifier,
                        binding,
                    }));
                }
            }
        }

        if parser.lexer.token == Token::Comma {
            scan_next_token(&mut parser.lexer);
        }
    }
    eat_token(&mut parser.lexer, Token::CloseBrace);
    Ok(ObjectBinding { properties })
}

fn parse_array_binding(parser: &mut Parser) -> ParseResult<ArrayBinding> {
    scan_next_token(&mut parser.lexer);
    let mut items: Vec<Option<ArrayBindingItemKind>> = Vec::new();
    while parser.lexer.token != Token::CloseBracket {
        match parser.lexer.token {
            Token::DotDotDot => {
                items.push(
                    parse_rest_element(parser)
                        .map(ArrayBindingItemKind::Rest)
                        .map(Some)?,
                );
            }

            _ => {
                let binding = parse_binding(parser)?;
                let initializer = parse_optional_initializer(parser)?;

                items.push(Some(ArrayBindingItemKind::Item(ArrayBindingItem {
                    binding,
                    initializer,
                })));
            }
        };

        if parser.lexer.token == Token::Comma {
            scan_next_token(&mut parser.lexer);
        }
    }
    eat_token(&mut parser.lexer, Token::CloseBracket);
    Ok(ArrayBinding { items })
}

fn parse_rest_element(parser: &mut Parser) -> ParseResult<RestElement> {
    scan_next_token(&mut parser.lexer);
    let element = parse_binding(parser)?;
    Ok(RestElement { binding: element })
}

fn parse_optional_initializer(parser: &mut Parser) -> ParseResult<Option<Expression>> {
    if parser.lexer.token != Token::Equals {
        return Ok(None);
    }
    scan_next_token(&mut parser.lexer);
    parse_expression(parser, &Precedence::Comma).map(Some)
}

// Expressions
fn parse_expression(parser: &mut Parser, precedence: &Precedence) -> ParseResult<Expression> {
    let left = parse_prefix(parser)?;

    parse_suffix(parser, precedence, left)
}

fn parse_prefix(parser: &mut Parser) -> ParseResult<Expression> {
    match &parser.lexer.token {
        Token::Null => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::NullLiteral(NullLiteral {}))
        }

        Token::NumericLiteral => {
            let value = parser.lexer.number;
            scan_next_token(&mut parser.lexer);
            Ok(Expression::NumericLiteral(NumericLiteral { value }))
        }

        Token::BigIntegerLiteral => {
            let value = parser.lexer.identifier.clone();
            scan_next_token(&mut parser.lexer);
            Ok(Expression::BigIntLiteral(BigIntLiteral { value }))
        }

        Token::Slash | Token::SlashEquals => {
            scan_regexp(&mut parser.lexer);
            let value = raw(&mut parser.lexer).to_string();
            scan_next_token(&mut parser.lexer);
            Ok(Expression::RegexpLiteral(RegexpLiteral { value }))
        }

        Token::Identifier => {
            let identifier = parse_identifier(parser)?;

            // Arrow function
            if parser.lexer.token == Token::EqualsGreaterThan {
                scan_next_token(&mut parser.lexer);
                let body = match parser.lexer.token {
                    Token::OpenBrace => parse_block_statement(parser)
                        .map(ArrowFunctionExpressionBody::BlockStatement)?,
                    _ => parse_expression(parser, &Precedence::Comma)
                        .map(Box::new)
                        .map(ArrowFunctionExpressionBody::Expression)?,
                };

                return Ok(Expression::ArrowFunction(ArrowFunctionExpression {
                    body,
                    parameters: vec![ParameterKind::Parameter(Parameter {
                        binding: Binding::Identifier(identifier),
                        initializer: None,
                    })],
                }));
            }

            Ok(Expression::Identifier(identifier))
        }

        Token::StringLiteral | Token::TemplateNoSubstitutionLiteral => {
            parse_string_literal(parser).map(Expression::StringLiteral)
        }

        Token::TemplateHead => {
            let head = parser.lexer.identifier.clone();
            let mut parts: Vec<TemplateLiteralPart> = Vec::new();
            loop {
                scan_next_token(&mut parser.lexer);
                let expression = parse_expression(parser, &Precedence::Comma)?;
                scan_template_tail_or_middle(&mut parser.lexer);
                let text = parser.lexer.identifier.clone();
                parts.push(TemplateLiteralPart { expression, text });
                if parser.lexer.token == Token::TemplateTail {
                    scan_next_token(&mut parser.lexer);
                    break;
                }
            }
            Ok(Expression::TemplateLiteral(TemplateLiteral { head, parts }))
        }

        Token::Class => {
            scan_next_token(&mut parser.lexer);
            let identifier = match parser.lexer.token {
                Token::Identifier => parse_identifier(parser).map(Some)?,
                _ => None,
            };
            let extends = match parser.lexer.token {
                Token::Extends => {
                    scan_next_token(&mut parser.lexer);
                    parse_expression(parser, &Precedence::Comma)
                        .map(Box::new)
                        .map(Some)?
                }
                _ => None,
            };
            let body = parse_class_body(parser)?;
            Ok(Expression::Class(ClassExpression {
                body,
                extends,
                identifier,
            }))
        }

        // !a
        Token::Exclamation => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::LogicalNot,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // ~a
        Token::Tilde => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::BinaryNot,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // +a
        Token::Plus => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::Positive,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // ++a
        Token::PlusPlus => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Update(UpdateExpression {
                operator: UpdateExpressionOperator::PrefixIncrement,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // -a
        Token::Minus => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::Negative,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // --a
        Token::MinusMinus => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Update(UpdateExpression {
                operator: UpdateExpressionOperator::PrefixDecrement,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // typeof a
        Token::Typeof => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::Typeof,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // delete a
        Token::Delete => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::Delete,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // void a
        Token::Void => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Unary(UnaryExpression {
                operator: UnaryExpressionOperator::Void,
                argument: parse_expression(parser, &Precedence::Prefix).map(Box::new)?,
            }))
        }

        // true
        Token::True => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::BooleanLiteral(BooleanLiteral { value: true }))
        }

        // false
        Token::False => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::BooleanLiteral(BooleanLiteral { value: false }))
        }

        Token::OpenParen => parse_parenthesized_expression(parser).map(Ok)?,

        // Object expressions
        //
        // ({ a, b: c, [d]: e, ...f })
        Token::OpenBrace => {
            scan_next_token(&mut parser.lexer);
            let mut properties: Vec<ObjectExpressionPropertyKind> = Vec::new();
            while parser.lexer.token != Token::CloseBrace {
                if parser.lexer.token == Token::Comma {
                    scan_next_token(&mut parser.lexer);
                    continue;
                }

                // ...a
                if parser.lexer.token == Token::DotDotDot {
                    scan_next_token(&mut parser.lexer);
                    let element = parse_expression(parser, &Precedence::Comma)?;
                    properties.push(ObjectExpressionPropertyKind::Spread(SpreadElement {
                        element,
                    }));
                    continue;
                }

                // We store and optional identifier here because while parsing
                // get and set we might happen upon a get and set being used
                // as an identifier. If that is the case then we will need to
                // store it as an identifier so that we can reuse it later down
                // the chain. An alternative here could be to duplicate the logic
                // for both the get and set.
                //
                // get() {} | get: {} | get: [] | get: a
                //
                // set() {} | set: {} | set: [] | set a
                let mut identifier: Option<Identifier> = None;

                // get a() {} | get() {}
                if parser.lexer.token == Token::Identifier && parser.lexer.identifier == "get" {
                    scan_next_token(&mut parser.lexer);
                    // get a() {}
                    if parser.lexer.token == Token::Identifier {
                        let key = parse_literal_property_name(parser)?;
                        let parameters = parse_parameters(parser)?;
                        let body = parse_block_statement(parser)?;
                        properties.push(ObjectExpressionPropertyKind::MethodGet(
                            ObjectExpressionMethodGet {
                                key,
                                parameters,
                                body,
                            },
                        ));
                        continue;
                    }

                    // get [a]() {}
                    if parser.lexer.token == Token::OpenBracket {
                        scan_next_token(&mut parser.lexer);
                        let key = parse_expression(parser, &Precedence::Comma)?;
                        eat_token(&mut parser.lexer, Token::CloseBracket);
                        let parameters = parse_parameters(parser)?;
                        let body = parse_block_statement(parser)?;
                        properties.push(ObjectExpressionPropertyKind::MethodGetComputed(
                            ObjectExpressionMethodGetComputed {
                                key,
                                parameters,
                                body,
                            },
                        ));
                        continue;
                    }

                    // Means we parsed a get identifier instead of a get marker.
                    identifier = Some(Identifier {
                        name: String::from("get"),
                    });
                }

                // set a() {} | set() {}
                if parser.lexer.token == Token::Identifier && parser.lexer.identifier == "set" {
                    scan_next_token(&mut parser.lexer);
                    // set a() {}
                    if parser.lexer.token == Token::Identifier {
                        let key = parse_literal_property_name(parser)?;
                        let parameters = parse_parameters(parser)?;
                        let body = parse_block_statement(parser)?;
                        properties.push(ObjectExpressionPropertyKind::MethodSet(
                            ObjectExpressionMethodSet {
                                key,
                                parameters,
                                body,
                            },
                        ));
                        continue;
                    }

                    // set [a]() {}
                    if parser.lexer.token == Token::OpenBracket {
                        scan_next_token(&mut parser.lexer);
                        let key = parse_expression(parser, &Precedence::Comma)?;
                        eat_token(&mut parser.lexer, Token::CloseBracket);
                        let parameters = parse_parameters(parser)?;
                        let body = parse_block_statement(parser)?;
                        properties.push(ObjectExpressionPropertyKind::MethodSetComputed(
                            ObjectExpressionMethodSetComputed {
                                key,
                                parameters,
                                body,
                            },
                        ));
                        continue;
                    }

                    // Means we parsed a set identifier instead of a set marker.
                    identifier = Some(Identifier {
                        name: String::from("set"),
                    });
                }

                // Computed property
                //
                // { [a]: b }
                if parser.lexer.token == Token::OpenBracket {
                    scan_next_token(&mut parser.lexer);
                    let key = parse_expression(parser, &Precedence::Comma)?;
                    eat_token(&mut parser.lexer, Token::CloseBracket);

                    if parser.lexer.token == Token::Colon {
                        scan_next_token(&mut parser.lexer);
                        let value = parse_expression(parser, &Precedence::Comma)?;
                        properties.push(ObjectExpressionPropertyKind::Computed(
                            ObjectExpressionPropertyComputed { key, value },
                        ));
                    } else if parser.lexer.token == Token::OpenParen {
                        let parameters = parse_parameters(parser)?;
                        let body = parse_block_statement(parser)?;
                        properties.push(ObjectExpressionPropertyKind::MethodComputed(
                            ObjectExpressionMethodComputed {
                                key,
                                parameters,
                                body,
                            },
                        ));
                    }

                    continue;
                }

                let key: LiteralPropertyName;
                // If this is true then it means the we've already parsed
                // the identifier above and we do not need to parse it again.
                if let Some(ident) = identifier {
                    key = LiteralPropertyName::Identifier(ident);
                } else {
                    key = parse_literal_property_name(parser)?;
                }

                // a: b | "a": b | 1: b | undefined: b | null: b
                if parser.lexer.token == Token::Colon {
                    scan_next_token(&mut parser.lexer);
                    let value = parse_expression(parser, &Precedence::Comma)?;
                    properties.push(ObjectExpressionPropertyKind::Property(
                        ObjectExpressionProperty { key, value },
                    ));
                    continue;
                }

                if parser.lexer.token == Token::OpenParen {
                    let parameters = parse_parameters(parser)?;
                    let body = parse_block_statement(parser)?;
                    properties.push(ObjectExpressionPropertyKind::Method(
                        ObjectExpressionMethod {
                            key,
                            parameters,
                            body,
                        },
                    ));
                    continue;
                }

                // If we get all the way here then it means we've hit a shorthand property.
                // The key we defined above has to wide of a type so we need to narrow it
                // to only allow for identifiers. If it anything else we report a syntax error.
                let narrowed_key = match key {
                    LiteralPropertyName::Identifier(i) => i,
                    _ => {
                        panic!("Only identifiers are allowed to be used with the shorthand syntax")
                    }
                };
                properties.push(ObjectExpressionPropertyKind::Shorthand(
                    ObjectExpressionPropertyShorthand { key: narrowed_key },
                ));
            }
            eat_token(&mut parser.lexer, Token::CloseBrace);
            Ok(Expression::Object(ObjectExpression { properties }))
        }

        // Array expressions
        //
        // [a, b, c]
        Token::OpenBracket => {
            scan_next_token(&mut parser.lexer);
            let mut elements: Vec<Option<ArrayExpressionItem>> = Vec::new();
            while parser.lexer.token != Token::CloseBracket {
                match parser.lexer.token {
                    Token::Comma => elements.push(None),
                    Token::DotDotDot => {
                        scan_next_token(&mut parser.lexer);
                        let element = parse_expression(parser, &Precedence::Comma)?;
                        elements.push(Some(ArrayExpressionItem::Spread(SpreadElement { element })))
                    }
                    _ => {
                        let expression = parse_expression(parser, &Precedence::Comma)?;
                        elements.push(Some(ArrayExpressionItem::Expression(expression)));
                    }
                };

                if parser.lexer.token == Token::Comma {
                    scan_next_token(&mut parser.lexer);
                }
            }
            eat_token(&mut parser.lexer, Token::CloseBracket);
            Ok(Expression::Array(ArrayExpression { items: elements }))
        }

        // New expressions
        // new a()
        // new a.b.c()
        Token::New => {
            scan_next_token(&mut parser.lexer);
            let callee = Box::new(parse_expression(parser, &Precedence::Member)?);
            let mut arguments: Vec<ArgumentKind> = Vec::new();
            // The actual call expression in a new expression is optional.
            // This is valid: new a and is equivalent to new a()
            if parser.lexer.token == Token::OpenParen {
                arguments = parse_arguments(parser)?;
            }
            Ok(Expression::New(NewExpression { arguments, callee }))
        }

        // Function expression
        // let a = function b() {}
        // let a = function () {}
        Token::Function => {
            scan_next_token(&mut parser.lexer);
            let generator = match parser.lexer.token {
                Token::Asterisk => {
                    scan_next_token(&mut parser.lexer);
                    true
                }
                _ => false,
            };
            let identifier = match parser.lexer.token {
                Token::Identifier => parse_identifier(parser).map(Some)?,
                _ => None,
            };
            let parameters = parse_parameters(parser)?;
            let body = parse_block_statement(parser)?;
            Ok(Expression::Function(FunctionExpression {
                generator,
                parameters,
                body,
                identifier,
            }))
        }

        // this
        Token::This => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::This(ThisExpression {}))
        }

        // super
        Token::Super => {
            scan_next_token(&mut parser.lexer);
            Ok(Expression::Super(SuperExpression {}))
        }

        _ => todo!(),
    }
}

fn parse_suffix(
    parser: &mut Parser,
    precedence: &Precedence,
    left: Expression,
) -> ParseResult<Expression> {
    let mut expression = left;

    loop {
        match &parser.lexer.token {
            // a[b][c]
            Token::OpenBracket => {
                scan_next_token(&mut parser.lexer);
                let property = parse_expression(parser, &Precedence::Lowest).map(Box::new)?;
                eat_token(&mut parser.lexer, Token::CloseBracket);
                expression = Expression::Member(MemberExpression {
                    object: Box::new(expression),
                    computed: true,
                    property,
                })
            }

            // a.b.c
            Token::Dot => {
                scan_next_token(&mut parser.lexer);
                expression = Expression::Member(MemberExpression {
                    object: Box::new(expression),
                    computed: false,
                    property: parse_expression(parser, &Precedence::Member).map(Box::new)?,
                });
            }

            // a = 1
            Token::Equals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::Assign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a += 1
            Token::PlusEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::AdditionAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a -= 1
            Token::MinusEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::SubstitutionAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a *= 1
            Token::AsteriskEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::MultiplicationAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a /= 1
            Token::SlashEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::DivisionAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a %= 1
            Token::PercentEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::ModulusAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a **= 1
            Token::AsteriskAsteriskEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::ExponentiationAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a <<= 1
            Token::LessThanLessThanEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::LeftShiftAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a >>= 1
            Token::GreaterThanGreaterThanEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::RightShiftAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a >>>= 1
            Token::GreaterThanGreaterThanGreaterThanEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::UnsignedRightShiftAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a |= 1
            Token::BarEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::BitwiseOrAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a ^= 1
            Token::CaretEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::BitwiseXorAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a &= 1
            Token::AmpersandEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::BitwiseAndAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a ??= 1
            Token::QuestionQuestionEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::NullishCoalescingAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a ||= 1
            Token::BarBarEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::LogicalOrAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // a &&= 1
            Token::AmpersandAmpersandEquals => {
                if precedence >= &Precedence::Assign {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Assignment(AssignmentExpression {
                    left: match convert_expression_to_binding(parser, expression.clone()) {
                        Ok(b) => AssignmentExpressionLeft::Binding(b),
                        Err(_) => {
                            AssignmentExpressionLeft::Expression(Box::new(expression.clone()))
                        }
                    },
                    operator: AssignmentExpressionOperator::LogicalAndAssign,
                    right: Box::new(parse_expression(parser, &Precedence::Assign.lower())?),
                })
            }

            // 1 + 2
            Token::Plus => {
                if precedence >= &Precedence::Sum {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Addition,
                    right: Box::new(parse_expression(parser, &Precedence::Sum)?),
                });
            }

            // 1 - 2
            Token::Minus => {
                if precedence >= &Precedence::Sum {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Substitution,
                    right: Box::new(parse_expression(parser, &Precedence::Sum)?),
                });
            }

            // 1 % 2
            Token::Percent => {
                if precedence >= &Precedence::Product {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Modulus,
                    right: Box::new(parse_expression(parser, &Precedence::Product)?),
                });
            }

            // 1 / 2
            Token::Slash => {
                if precedence >= &Precedence::Product {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Division,
                    right: Box::new(parse_expression(parser, &Precedence::Product)?),
                });
            }

            // 1 * 2
            Token::Asterisk => {
                if precedence >= &Precedence::Product {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Multiplication,
                    right: Box::new(parse_expression(parser, &Precedence::Product)?),
                });
            }

            // 1 * 2
            Token::AsteriskAsterisk => {
                if precedence >= &Precedence::Product {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Exponentiation,
                    right: Box::new(parse_expression(parser, &Precedence::Product)?),
                });
            }

            // 1 < 2
            Token::LessThan => {
                if precedence >= &Precedence::Compare {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::LessThan,
                    right: Box::new(parse_expression(parser, &Precedence::Compare)?),
                });
            }

            // 1 <= 0
            Token::LessThanEquals => {
                if precedence >= &Precedence::Equals {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::LessThanEquals,
                    right: Box::new(parse_expression(parser, &Precedence::Compare)?),
                });
            }

            // 1 > 2
            Token::GreaterThan => {
                if precedence >= &Precedence::Compare {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::GreaterThan,
                    right: Box::new(parse_expression(parser, &Precedence::Compare)?),
                });
            }

            // 1 >= 0
            Token::GreaterThanEquals => {
                if precedence >= &Precedence::Equals {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::GreaterThanEquals,
                    right: Box::new(parse_expression(parser, &Precedence::Compare)?),
                });
            }

            // 1 | 2
            Token::Bar => {
                if precedence >= &Precedence::BitwiseOr {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::BitwiseOr,
                    right: Box::new(parse_expression(parser, &Precedence::BitwiseOr)?),
                });
            }

            // 1 & 2
            Token::Ampersand => {
                if precedence >= &Precedence::BitwiseAnd {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::BitwiseAnd,
                    right: Box::new(parse_expression(parser, &Precedence::BitwiseAnd)?),
                });
            }

            // 1 ^ 2
            Token::Caret => {
                if precedence >= &Precedence::BitwiseXor {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::BitwiseXor,
                    right: Box::new(parse_expression(parser, &Precedence::BitwiseXor)?),
                });
            }

            // 1 << 2
            Token::LessThanLessThan => {
                if precedence >= &Precedence::Shift {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::LeftShift,
                    right: Box::new(parse_expression(parser, &Precedence::Shift)?),
                });
            }

            // 1 >> 2
            Token::GreaterThanGreaterThan => {
                if precedence >= &Precedence::Shift {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::RightShift,
                    right: Box::new(parse_expression(parser, &Precedence::Shift)?),
                });
            }

            // 1 >>> 2
            Token::GreaterThanGreaterThanGreaterThan => {
                if precedence >= &Precedence::Shift {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::UnsignedRightShift,
                    right: Box::new(parse_expression(parser, &Precedence::Shift)?),
                });
            }

            // 1 == 1
            Token::EqualsEquals => {
                if precedence >= &Precedence::Equals {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::LooseEquals,
                    right: Box::new(parse_expression(parser, &Precedence::Equals)?),
                });
            }

            // 1 === 1
            Token::EqualsEqualsEquals => {
                if precedence >= &Precedence::Equals {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::StrictEquals,
                    right: Box::new(parse_expression(parser, &Precedence::Equals)?),
                });
            }

            // 1 != 2
            Token::ExclamationEquals => {
                if precedence >= &Precedence::Equals {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::LooseNotEquals,
                    right: Box::new(parse_expression(parser, &Precedence::Equals)?),
                });
            }

            // 1 !== 2
            Token::ExclamationEqualsEquals => {
                if precedence >= &Precedence::Equals {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::StrictNotEquals,
                    right: Box::new(parse_expression(parser, &Precedence::Equals)?),
                });
            }

            // a instanceof b
            Token::Instanceof => {
                if precedence >= &Precedence::Compare {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::Instanceof,
                    right: Box::new(parse_expression(parser, &Precedence::Compare)?),
                })
            }

            // a in b
            Token::In => {
                if precedence >= &Precedence::Compare || !parser.allow_in {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Binary(BinaryExpression {
                    left: Box::new(expression),
                    operator: BinaryExpressionOperator::In,
                    right: Box::new(parse_expression(parser, &Precedence::Compare)?),
                })
            }

            // a, b, c
            Token::Comma => {
                if precedence >= &Precedence::Comma {
                    return Ok(expression);
                }
                let mut expressions = vec![expression];
                while parser.lexer.token == Token::Comma {
                    scan_next_token(&mut parser.lexer);
                    expressions.push(parse_expression(parser, &Precedence::Comma)?);
                }
                expression = Expression::Sequence(SequenceExpression { expressions });
            }

            // Call expression
            Token::OpenParen => {
                if precedence >= &Precedence::Call {
                    return Ok(expression);
                }
                let arguments = parse_arguments(parser)?;
                expression = Expression::Call(CallExpression {
                    arguments,
                    callee: Box::new(expression),
                });
            }

            // Conditional (ternary)
            Token::Question => {
                if precedence >= &Precedence::Conditional {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                let consequence = parse_expression(parser, &Precedence::Comma)?;
                eat_token(&mut parser.lexer, Token::Colon);
                let alternate = parse_expression(parser, &Precedence::Comma)?;
                expression = Expression::Conditional(ConditionalExpression {
                    test: Box::new(expression),
                    consequence: Box::new(consequence),
                    alternate: Box::new(alternate),
                });
            }

            // 1++
            Token::PlusPlus => {
                if precedence >= &Precedence::Postfix {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Update(UpdateExpression {
                    operator: UpdateExpressionOperator::PostfixIncrement,
                    argument: Box::new(expression),
                });
            }

            // 1--
            Token::MinusMinus => {
                if precedence >= &Precedence::Postfix {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Update(UpdateExpression {
                    operator: UpdateExpressionOperator::PostfixDecrement,
                    argument: Box::new(expression),
                });
            }

            // a || b
            Token::BarBar => {
                if precedence >= &Precedence::LogicalOr {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Logical(LogicalExpression {
                    left: Box::new(expression),
                    operator: LogicalExpressionOperator::Or,
                    right: Box::new(parse_expression(parser, &Precedence::LogicalOr)?),
                });
            }

            // a && b
            Token::AmpersandAmpersand => {
                if precedence >= &Precedence::LogicalAnd {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Logical(LogicalExpression {
                    left: Box::new(expression),
                    operator: LogicalExpressionOperator::And,
                    right: Box::new(parse_expression(parser, &Precedence::LogicalAnd)?),
                });
            }

            // a ?? b
            Token::QuestionQuestion => {
                if precedence >= &Precedence::NullishCoalescing {
                    return Ok(expression);
                }
                scan_next_token(&mut parser.lexer);
                expression = Expression::Logical(LogicalExpression {
                    left: Box::new(expression),
                    operator: LogicalExpressionOperator::NullishCoalescing,
                    right: Box::new(parse_expression(parser, &Precedence::NullishCoalescing)?),
                });
            }

            _ => {
                return Ok(expression);
            }
        };
    }
}

fn parse_parameters(parser: &mut Parser) -> ParseResult<Vec<ParameterKind>> {
    eat_token(&mut parser.lexer, Token::OpenParen);
    let mut parameters: Vec<ParameterKind> = Vec::new();
    while parser.lexer.token != Token::CloseParen {
        if parser.lexer.token == Token::DotDotDot {
            scan_next_token(&mut parser.lexer);
            let element = parse_binding(parser)?;
            parameters.push(ParameterKind::Rest(RestElement { binding: element }));
            // TODO: A comma is not allowed after the rest element.
            continue;
        }

        let binding = parse_binding(parser)?;
        let initializer = parse_optional_initializer(parser)?;
        parameters.push(ParameterKind::Parameter(Parameter {
            binding,
            initializer,
        }));
        if parser.lexer.token == Token::Comma {
            scan_next_token(&mut parser.lexer);
        }
    }

    eat_token(&mut parser.lexer, Token::CloseParen);
    Ok(parameters)
}

// The content of a parenthesized expression can be parsed in different ways
// depending on the token following it. For example, (a, b, c) will result in a sequence
// expression while (a + b) * c will result in a binary expression. These are straightforward
// to parse since the content is still an expression. The interesting bit is when the subsequent
// token is "=>" because then the parenthesis is the parameters in an arrow function. The parameters
// in an arrow function is like all other functions an array of bindings, not expressions. Now,
// one could attempt to look ahead in the token stream and try to predict if the expression
// eventually will evaluate to an arrow function. But since a parenthesized expression might it self
// contain parenthesized expressions it is not as easy as just finding the next closing parenthesis and
// looking at the token following. The function attempting to look ahead would need to be more
// sophisticated than that. An alternative approach that other parsers take is to simply assume
// that the items inside the parenthesis are expressions and then once we know if it will be an arrow
// function or not, the attempt to convert the expressions into bindings. This is not the only place
// this happens, we do the same for assignment expressions. Not all expressions can be converted
// into bindings, for example there is no way to represent 3 + 3 as a binding is after all the binding
// between a variable and a value. If we happen upon an expression like this while converting, we will
// report it as a syntax error and assume that the user was attempting to write an arrow function.
// Note: Another possible solution to this problem could be to make use of a backtracking algorithm,
// but this is not something the lexer currently support.
fn parse_parenthesized_expression(parser: &mut Parser) -> ParseResult<Expression> {
    eat_token(&mut parser.lexer, Token::OpenParen);
    let mut expressions: Vec<Expression> = Vec::new();
    let mut rest_element: Option<RestElement> = None;
    while parser.lexer.token != Token::CloseParen {
        if parser.lexer.token == Token::DotDotDot {
            scan_next_token(&mut parser.lexer);
            rest_element = parse_binding(parser)
                .map(|binding| RestElement { binding })
                .map(Some)?;
        } else {
            parse_expression(parser, &Precedence::Comma)
                .map(|expression| expressions.push(expression))?;
        }
        if parser.lexer.token == Token::Comma {
            scan_next_token(&mut parser.lexer);
        }
    }
    eat_token(&mut parser.lexer, Token::CloseParen);

    // Arrow function
    if parser.lexer.token == Token::EqualsGreaterThan {
        scan_next_token(&mut parser.lexer);

        let mut parameters: Vec<ParameterKind> = Vec::new();
        for expression in expressions {
            let (binding, initializer) =
                convert_expression_to_binding_and_initializer(parser, expression)?;
            parameters.push(ParameterKind::Parameter(Parameter {
                binding,
                initializer,
            }));
        }

        if let Some(rest_elem) = rest_element {
            parameters.push(ParameterKind::Rest(rest_elem));
        }

        let body = match parser.lexer.token {
            Token::OpenBrace => {
                parse_block_statement(parser).map(ArrowFunctionExpressionBody::BlockStatement)?
            }
            _ => parse_expression(parser, &Precedence::Comma)
                .map(Box::new)
                .map(ArrowFunctionExpressionBody::Expression)?,
        };

        return Ok(Expression::ArrowFunction(ArrowFunctionExpression {
            body,
            parameters,
        }));
    }

    // Rest elements are only allowed as a parameters
    // and in bindings, this is a syntax error.
    if let Some(_) = rest_element {
        panic!("Rest elements are only allowed as bindings on parameters");
    }

    // A parenthesized expression
    if expressions.len() > 0 {
        return Ok(Expression::Sequence(SequenceExpression { expressions }));
    }

    // If we got all the way here, then it is not a an arrow function
    // but the user did neither have an any expressions inside the parenthesis.
    // This is a syntax error, we will report is as syntax error in the context of
    // an arrow function.
    panic!("Found a parenthesized expression with no expressions in it, this is a syntax error.")
}

// Sometimes we end up parsing an item as an expression but then a token downstream
// indicates that an expression actual needs to be a binding.
// For example: [a, b, c] is an array expression but [a, b, c] = d is an assignment expression.
// In assignment expression, the left hand side is no longer an expression but rather a binding.
// So in that case [a, b, c] will need to be converted from an array expression to an array binding.
// The same applies to arrow functions.
// (a, b, c) is a sequence expression but (a, b, c) => {} is an arrow function. And in arrow functions,
// the items between the parentheses is not expression but bindings.
//
// Not all expressions can be converted into bindings so this conversions might end up returning an error
// that we should present to the user as a syntax error.
fn convert_expression_to_binding_and_initializer(
    parser: &mut Parser,
    expression: Expression,
) -> ParseResult<(Binding, Option<Expression>)> {
    match &expression {
        Expression::Assignment(a) => match &a.left {
            AssignmentExpressionLeft::Binding(b) => Ok((b.clone(), Some(*a.right.clone()))),
            AssignmentExpressionLeft::Expression(e) => Ok((
                convert_expression_to_binding(parser, *e.clone())?,
                Some(*a.right.clone()),
            )),
        },
        _ => Ok((
            convert_expression_to_binding(parser, expression.clone())?,
            None,
        )),
    }
}

fn convert_expression_to_binding(
    parser: &mut Parser,
    expression: Expression,
) -> ParseResult<Binding> {
    match expression {
        Expression::Array(a) => {
            let mut items: Vec<Option<ArrayBindingItemKind>> = Vec::new();
            for item in a.items {
                if let Some(i) = item {
                    match i {
                        ArrayExpressionItem::Spread(s) => {
                            let binding = convert_expression_to_binding(parser, s.element)?;
                            items.push(Some(ArrayBindingItemKind::Rest(RestElement { binding })))
                        }

                        ArrayExpressionItem::Expression(e) => {
                            let (binding, initializer) =
                                convert_expression_to_binding_and_initializer(parser, e)?;

                            items.push(Some(ArrayBindingItemKind::Item(ArrayBindingItem {
                                binding,
                                initializer,
                            })));
                        }
                    }
                } else {
                    items.push(None)
                }
            }
            Ok(Binding::Array(ArrayBinding { items }))
        }

        Expression::Object(o) => {
            let mut properties: Vec<ObjectBindingPropertyKind> = Vec::new();
            for property in o.properties {
                match property {
                    // None of these are convertible
                    ObjectExpressionPropertyKind::MethodGet(_)
                    | ObjectExpressionPropertyKind::MethodGetComputed(_)
                    | ObjectExpressionPropertyKind::MethodSet(_)
                    | ObjectExpressionPropertyKind::MethodSetComputed(_)
                    | ObjectExpressionPropertyKind::Method(_)
                    | ObjectExpressionPropertyKind::MethodComputed(_) => {
                        return Err(ParserError("Not convertible".into()))
                    }

                    ObjectExpressionPropertyKind::Spread(s) => {
                        let key = match s.element {
                            Expression::Identifier(i) => i,
                            _ => return Err(ParserError("Not convertible".into())),
                        };

                        properties.push(ObjectBindingPropertyKind::Rest(
                            ObjectBindingPropertyRest { key },
                        ))
                    }

                    ObjectExpressionPropertyKind::Property(p) => {
                        let key = p.key;
                        let (binding, initializer) =
                            convert_expression_to_binding_and_initializer(parser, p.value)?;
                        properties.push(ObjectBindingPropertyKind::Property(
                            ObjectBindingProperty {
                                initializer,
                                binding,
                                key,
                            },
                        ));
                    }
                    ObjectExpressionPropertyKind::Shorthand(s) => {
                        let key = s.key;
                        properties.push(ObjectBindingPropertyKind::Shorthand(
                            ObjectBindingPropertyShorthand {
                                key,
                                initializer: None,
                            },
                        ));
                    }
                    ObjectExpressionPropertyKind::Computed(c) => {
                        let key = c.key;
                        let (binding, initializer) =
                            convert_expression_to_binding_and_initializer(parser, c.value)?;
                        properties.push(ObjectBindingPropertyKind::Computed(
                            ObjectBindingPropertyComputed {
                                initializer,
                                binding,
                                key,
                            },
                        ));
                    }
                }
            }
            Ok(Binding::Object(ObjectBinding { properties }))
        }

        Expression::Identifier(i) => Ok(Binding::Identifier(i)),

        _ => return Err(ParserError("Not convertible".into())),
    }
}

fn parse_class_body(parser: &mut Parser) -> ParseResult<Vec<ClassPropertyKind>> {
    eat_token(&mut parser.lexer, Token::OpenBrace);
    let mut properties: Vec<ClassPropertyKind> = Vec::new();
    while parser.lexer.token != Token::CloseBrace {
        if parser.lexer.token == Token::Semicolon {
            scan_next_token(&mut parser.lexer);
            continue;
        }

        // A class has a couple of markers, static, get and set that alter
        // the behavior of a property/method. However, those words
        // can also be used as the actual identifier as well.
        // This is the syntax of a marker get/set/static <identifier>() {}
        // But they can also be used get/set/static() {} and in this case
        // get isn't a marker but instead an identifier. To make this work
        // we store an early optional identifier that we populate if we git one
        // of those words but determine that isn't in a marker position.
        // Then we can skip parsing the identifier further down and reuse the already
        // parsed identifier. This is very similar to how we handle objects.
        let mut identifier: Option<Identifier> = None;

        let mut is_static = false;
        if parser.lexer.token == Token::Identifier && parser.lexer.identifier == "static" {
            scan_next_token(&mut parser.lexer);
            if parser.lexer.token == Token::Identifier {
                is_static = true;
            } else {
                identifier = Some(Identifier {
                    name: String::from("static"),
                });
            }
        }

        // Note: A constructor can't be a marker.
        if parser.lexer.token == Token::Identifier && parser.lexer.identifier == "constructor" {
            scan_next_token(&mut parser.lexer);
            let parameters = parse_parameters(parser)?;
            let body = parse_block_statement(parser)?;
            properties.push(ClassPropertyKind::Constructor(ClassConstructor {
                is_static,
                body,
                parameters,
            }));
            continue;
        }

        // get a() {} | get() {}
        if parser.lexer.token == Token::Identifier && parser.lexer.identifier == "get" {
            scan_next_token(&mut parser.lexer);

            if parser.lexer.token == Token::Identifier {
                let identifier = parse_literal_property_name(parser)?;
                let parameters = parse_parameters(parser)?;
                let body = parse_block_statement(parser)?;
                properties.push(ClassPropertyKind::MethodGet(ClassMethodGet {
                    is_static,
                    body,
                    identifier,
                    parameters,
                }));
                continue;
            }

            if parser.lexer.token == Token::OpenBracket {
                eat_token(&mut parser.lexer, Token::OpenBracket);
                let key = parse_expression(parser, &Precedence::Comma)?;
                eat_token(&mut parser.lexer, Token::CloseBracket);
                let parameters = parse_parameters(parser)?;
                let body = parse_block_statement(parser)?;
                properties.push(ClassPropertyKind::MethodGetComputed(
                    ClassMethodGetComputed {
                        is_static,
                        body,
                        key,
                        parameters,
                    },
                ));
                continue;
            }

            // Means get isn't used as a marker
            identifier = Some(Identifier {
                name: String::from("get"),
            });
        }

        // set a() {} | set() {}
        if parser.lexer.token == Token::Identifier && parser.lexer.identifier == "set" {
            scan_next_token(&mut parser.lexer);

            if parser.lexer.token == Token::Identifier {
                let identifier = parse_literal_property_name(parser)?;
                let parameters = parse_parameters(parser)?;
                let body = parse_block_statement(parser)?;
                properties.push(ClassPropertyKind::MethodSet(ClassMethodSet {
                    is_static,
                    body,
                    identifier,
                    parameters,
                }));
                continue;
            }

            if parser.lexer.token == Token::OpenBracket {
                eat_token(&mut parser.lexer, Token::OpenBracket);
                let key = parse_expression(parser, &Precedence::Comma)?;
                eat_token(&mut parser.lexer, Token::CloseBracket);
                let parameters = parse_parameters(parser)?;
                let body = parse_block_statement(parser)?;
                properties.push(ClassPropertyKind::MethodSetComputed(
                    ClassMethodSetComputed {
                        is_static,
                        body,
                        key,
                        parameters,
                    },
                ));
                continue;
            }

            // Means set isn't used as a marker
            identifier = Some(Identifier {
                name: String::from("set"),
            });
        }

        // Computed class method
        if parser.lexer.token == Token::OpenBracket {
            eat_token(&mut parser.lexer, Token::OpenBracket);
            let key = parse_expression(parser, &Precedence::Comma)?;
            eat_token(&mut parser.lexer, Token::CloseBracket);
            let parameters = parse_parameters(parser)?;
            let body = parse_block_statement(parser)?;
            properties.push(ClassPropertyKind::MethodComputed(ClassMethodComputed {
                body,
                is_static,
                key,
                parameters,
            }));
            continue;
        }

        let actual_identifier: LiteralPropertyName;
        if let Some(ident) = identifier {
            actual_identifier = LiteralPropertyName::Identifier(ident);
        } else {
            actual_identifier = parse_literal_property_name(parser)?;
        }
        let parameters = parse_parameters(parser)?;
        let body = parse_block_statement(parser)?;
        properties.push(ClassPropertyKind::Method(ClassMethod {
            body,
            identifier: actual_identifier,
            is_static,
            parameters,
        }));
    }

    eat_token(&mut parser.lexer, Token::CloseBrace);
    Ok(properties)
}

fn parse_arguments(parser: &mut Parser) -> ParseResult<Vec<ArgumentKind>> {
    eat_token(&mut parser.lexer, Token::OpenParen);
    let mut arguments: Vec<ArgumentKind> = Vec::new();
    while parser.lexer.token != Token::CloseParen {
        if parser.lexer.token == Token::DotDotDot {
            scan_next_token(&mut parser.lexer);
            let element = parse_expression(parser, &Precedence::Comma)?;
            arguments.push(ArgumentKind::Spread(SpreadElement { element }));
        } else {
            let expression = parse_expression(parser, &Precedence::Comma)?;
            arguments.push(ArgumentKind::Expression(expression));
        }

        if parser.lexer.token == Token::Comma {
            scan_next_token(&mut parser.lexer);
        }
    }
    eat_token(&mut parser.lexer, Token::CloseParen);
    Ok(arguments)
}

fn parse_identifier(parser: &mut Parser) -> ParseResult<Identifier> {
    expect_token(&mut parser.lexer, Token::Identifier);
    let identifier = Identifier {
        name: parser.lexer.identifier.clone(),
    };
    scan_next_token(&mut parser.lexer);
    Ok(identifier)
}

fn parse_string_literal(parser: &mut Parser) -> ParseResult<StringLiteral> {
    let string_literal = StringLiteral {
        value: parser.lexer.identifier.clone(),
    };
    scan_next_token(&mut parser.lexer);
    Ok(string_literal)
}

fn parse_literal_property_name(parser: &mut Parser) -> ParseResult<LiteralPropertyName> {
    match parser.lexer.token {
        Token::StringLiteral => {
            let string_literal = parse_string_literal(parser)?;
            Ok(LiteralPropertyName::String(string_literal))
        }

        Token::NumericLiteral => {
            let numeric_literal = NumericLiteral {
                value: parser.lexer.number.clone(),
            };
            scan_next_token(&mut parser.lexer);
            Ok(LiteralPropertyName::Numeric(numeric_literal))
        }

        Token::Identifier => {
            let identifier = parse_identifier(parser)?;
            Ok(LiteralPropertyName::Identifier(identifier))
        }

        Token::Null => {
            let identifier = Identifier {
                name: "null".into(),
            };
            scan_next_token(&mut parser.lexer);
            Ok(LiteralPropertyName::Identifier(identifier))
        }

        // Treat anything else as an identifier (null, undefined etc)
        _ => {
            let identifier = Identifier {
                name: parser.lexer.identifier.clone(),
            };
            scan_next_token(&mut parser.lexer);
            Ok(LiteralPropertyName::Identifier(identifier))
        }
    }
}

// Statements
fn parse_statement(parser: &mut Parser) -> ParseResult<Statement> {
    match &parser.lexer.token {
        Token::Const | Token::Var | Token::Let => {
            parse_variable_declaration(parser).map(Statement::VariableDeclaration)
        }

        Token::Import => {
            scan_next_token(&mut parser.lexer);

            if parser.lexer.token == Token::OpenParen {
                panic!("Import expressions are not yet supported");
            }

            if parser.lexer.token == Token::StringLiteral {
                let source = parse_string_literal(parser)?;
                consume_semicolon(parser);
                return Ok(Statement::ImportDeclaration(ImportDeclaration {
                    default: None,
                    namespace: None,
                    specifiers: Vec::new(),
                    source,
                }));
            }

            let mut default: Option<Identifier> = None;
            let mut namespace: Option<Identifier> = None;
            let mut specifiers: Vec<ImportDeclarationSpecifier> = Vec::new();

            match parser.lexer.token {
                // import * as a from "b"
                Token::Asterisk => {
                    scan_next_token(&mut parser.lexer); // *
                    scan_next_token(&mut parser.lexer); // as
                    namespace = parse_identifier(parser).map(Some)?;
                }

                // import {a, b} from "c"
                Token::OpenBrace => {
                    specifiers = parse_import_specifiers(parser)?;
                }

                // import a from "b"
                // import a, { b, c } from "d"
                // import a, * as b from "c"
                Token::Identifier => {
                    default = parse_identifier(parser).map(Some)?;
                    if parser.lexer.token == Token::Comma {
                        scan_next_token(&mut parser.lexer);
                        match parser.lexer.token {
                            Token::Asterisk => {
                                scan_next_token(&mut parser.lexer); // *
                                scan_next_token(&mut parser.lexer); // as
                                namespace = parse_identifier(parser).map(Some)?;
                            }

                            Token::OpenBrace => {
                                specifiers = parse_import_specifiers(parser)?;
                            }

                            _ => todo!(),
                        };
                    }
                }

                _ => todo!(),
            };

            eat_token(&mut parser.lexer, Token::From);
            let source = parse_string_literal(parser)?;
            consume_semicolon(parser);
            Ok(Statement::ImportDeclaration(ImportDeclaration {
                default,
                namespace,
                specifiers,
                source,
            }))
        }

        Token::Export => {
            scan_next_token(&mut parser.lexer);

            // export * from "a";
            if parser.lexer.token == Token::Asterisk {
                scan_next_token(&mut parser.lexer);
                eat_token(&mut parser.lexer, Token::From); // TODO: From is not a keyword but a contextual keyword.
                let source = parse_string_literal(parser)?;
                consume_semicolon(parser);
                return Ok(Statement::ExportAllDeclaration(ExportAllDeclaration {
                    source,
                }));
            }

            // export default
            if parser.lexer.token == Token::Default {
                scan_next_token(&mut parser.lexer);
                let declaration = match parser.lexer.token {
                    Token::Function => {
                        scan_next_token(&mut parser.lexer);
                        let generator = match parser.lexer.token {
                            Token::Asterisk => {
                                scan_next_token(&mut parser.lexer);
                                true
                            }
                            _ => false,
                        };
                        let identifier = match parser.lexer.token {
                            Token::Identifier => parse_identifier(parser).map(Some)?,
                            _ => None,
                        };
                        let parameters = parse_parameters(parser)?;
                        let body = parse_block_statement(parser)?;
                        if let Some(ident) = identifier {
                            ExportDefaultDeclarationKind::FunctionDeclaration(FunctionDeclaration {
                                generator,
                                identifier: ident,
                                parameters,
                                body,
                            })
                        } else {
                            ExportDefaultDeclarationKind::AnonymousDefaultExportedFunctionDeclaration(AnonymousDefaultExportedFunctionDeclaration {
                                    generator,
                                    body,
                                    parameters,
                                })
                        }
                    }

                    Token::Class => {
                        scan_next_token(&mut parser.lexer);
                        let identifier = match parser.lexer.token {
                            Token::Identifier => parse_identifier(parser).map(Some)?,
                            _ => None,
                        };
                        let extends = match parser.lexer.token {
                            Token::Extends => {
                                scan_next_token(&mut parser.lexer);
                                parse_expression(parser, &Precedence::Comma).map(Some)?
                            }
                            _ => None,
                        };
                        let body = parse_class_body(parser)?;
                        match identifier {
                                Some(ident) => ExportDefaultDeclarationKind::ClassDeclaration(ClassDeclaration {
                                    body,
                                    extends,
                                    identifier: ident
                                }),
                                None => ExportDefaultDeclarationKind::AnonymousDefaultExportedClassDeclaration(AnonymousDefaultExportedClassDeclaration {
                                    body,
                                    extends,
                                }),
                            }
                    }

                    _ => parse_expression(parser, &Precedence::Comma)
                        .map(ExportDefaultDeclarationKind::Expression)?,
                };
                return Ok(Statement::ExportDefaultDeclaration(
                    ExportDefaultDeclaration { declaration },
                ));
            }

            // Named export declaration
            match parser.lexer.token {
                // export function a() {}
                Token::Function => {
                    scan_next_token(&mut parser.lexer);
                    let generator = match parser.lexer.token {
                        Token::Asterisk => {
                            scan_next_token(&mut parser.lexer);
                            true
                        }
                        _ => false,
                    };
                    let identifier = parse_identifier(parser)?;
                    let parameters = parse_parameters(parser)?;
                    let body = parse_block_statement(parser)?;
                    Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                        declaration: ExportNamedDeclarationKind::FunctionDeclaration(
                            FunctionDeclaration {
                                generator,
                                parameters,
                                body,
                                identifier,
                            },
                        ),
                    }))
                }

                // export class A {}
                Token::Class => {
                    scan_next_token(&mut parser.lexer);
                    let identifier = parse_identifier(parser)?;
                    let extends = match parser.lexer.token {
                        Token::Extends => {
                            scan_next_token(&mut parser.lexer);
                            parse_expression(parser, &Precedence::Comma).map(Some)?
                        }
                        _ => None,
                    };
                    let body = parse_class_body(parser)?;
                    Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                        declaration: ExportNamedDeclarationKind::ClassDeclaration(
                            ClassDeclaration {
                                body,
                                extends,
                                identifier,
                            },
                        ),
                    }))
                }

                // export const a = 1;
                // export var a = 1;
                // export let a = 1;
                Token::Var | Token::Const | Token::Let => {
                    let declaration = parse_variable_declaration(parser)
                        .map(ExportNamedDeclarationKind::VariableDeclaration)?;
                    consume_semicolon(parser);
                    Ok(Statement::ExportNamedDeclaration(ExportNamedDeclaration {
                        declaration,
                    }))
                }

                // export { a, a as b }
                Token::OpenBrace => {
                    scan_next_token(&mut parser.lexer);
                    let mut specifiers: Vec<ExportNamedSpecifier> = Vec::new();
                    while parser.lexer.token != Token::CloseBrace {
                        // We don't call self.parse_identifier here because keywords
                        // are allowed as well. export { default as b } is valid.
                        let local = Identifier {
                            name: parser.lexer.identifier.clone(),
                        };
                        scan_next_token(&mut parser.lexer);
                        let mut exported: Option<Identifier> = None;
                        if parser.lexer.token == Token::As {
                            scan_next_token(&mut parser.lexer);
                            exported = parse_identifier(parser).map(Some)?;
                        }
                        if parser.lexer.token == Token::Comma {
                            scan_next_token(&mut parser.lexer);
                        }
                        specifiers.push(ExportNamedSpecifier {
                            exported: exported.unwrap_or_else(|| local.clone()),
                            local,
                        });
                        if parser.lexer.token == Token::Comma {
                            scan_next_token(&mut parser.lexer);
                        }
                    }
                    eat_token(&mut parser.lexer, Token::CloseBrace);
                    let mut source: Option<StringLiteral> = None;
                    if parser.lexer.token == Token::From {
                        scan_next_token(&mut parser.lexer);
                        source = parse_string_literal(parser).map(Some)?;
                    }
                    consume_semicolon(parser);
                    Ok(Statement::ExportNamedSpecifiers(ExportNamedSpecifiers {
                        specifiers,
                        source,
                    }))
                }

                _ => todo!(),
            }
        }

        Token::Function => {
            scan_next_token(&mut parser.lexer);
            let generator = match parser.lexer.token {
                Token::Asterisk => {
                    scan_next_token(&mut parser.lexer);
                    true
                }
                _ => false,
            };
            let identifier = parse_identifier(parser)?;
            let parameters = parse_parameters(parser)?;
            let body = parse_block_statement(parser)?;
            Ok(Statement::FunctionDeclaration(FunctionDeclaration {
                generator,
                identifier,
                body,
                parameters,
            }))
        }

        Token::Return => {
            scan_next_token(&mut parser.lexer);
            if parser.lexer.token == Token::Semicolon {
                scan_next_token(&mut parser.lexer);
                return Ok(Statement::ReturnStatement(ReturnStatement {
                    expression: None,
                }));
            }

            let expression = parse_expression(parser, &Precedence::Lowest)?;
            consume_semicolon(parser);
            Ok(Statement::ReturnStatement(ReturnStatement {
                expression: Some(expression),
            }))
        }

        Token::If => parse_if_statement(parser).map(Statement::IfStatement),

        Token::OpenBrace => parse_block_statement(parser).map(Statement::BlockStatement),

        Token::For => parse_for_statement(parser),

        Token::Continue => {
            scan_next_token(&mut parser.lexer);
            let mut label: Option<Identifier> = None;
            if parser.lexer.token == Token::Identifier {
                label = Some(parse_identifier(parser)?);
            }
            consume_semicolon(parser);
            Ok(Statement::ContinueStatement(ContinueStatement { label }))
        }

        Token::Break => {
            scan_next_token(&mut parser.lexer);
            let mut label: Option<Identifier> = None;
            if parser.lexer.token == Token::Identifier {
                label = Some(parse_identifier(parser)?);
            }
            consume_semicolon(parser);
            Ok(Statement::BreakStatement(BreakStatement { label }))
        }

        Token::Semicolon => {
            scan_next_token(&mut parser.lexer);
            Ok(Statement::EmptyStatement(EmptyStatement {}))
        }

        Token::Class => {
            scan_next_token(&mut parser.lexer);
            let identifier = parse_identifier(parser)?;
            let extends = match parser.lexer.token {
                Token::Extends => {
                    scan_next_token(&mut parser.lexer);
                    parse_expression(parser, &Precedence::Comma).map(Some)?
                }
                _ => None,
            };
            let body = parse_class_body(parser)?;
            Ok(Statement::ClassDeclaration(ClassDeclaration {
                body,
                extends,
                identifier,
            }))
        }

        Token::While => {
            scan_next_token(&mut parser.lexer);
            eat_token(&mut parser.lexer, Token::OpenParen);
            let test = parse_expression(parser, &Precedence::Lowest)?;
            eat_token(&mut parser.lexer, Token::CloseParen);
            let body = parse_statement(parser)?;
            Ok(Statement::WhileStatement(WhileStatement {
                body: Box::new(body),
                test,
            }))
        }

        Token::Do => {
            scan_next_token(&mut parser.lexer);
            let body = parse_statement(parser)?;
            eat_token(&mut parser.lexer, Token::While);
            eat_token(&mut parser.lexer, Token::OpenParen);
            let test = parse_expression(parser, &Precedence::Lowest)?;
            eat_token(&mut parser.lexer, Token::CloseParen);
            Ok(Statement::DoWhileStatement(DoWhileStatement {
                body: Box::new(body),
                test,
            }))
        }

        Token::Switch => {
            scan_next_token(&mut parser.lexer);
            eat_token(&mut parser.lexer, Token::OpenParen);
            let discriminant = parse_expression(parser, &Precedence::Lowest)?;
            eat_token(&mut parser.lexer, Token::CloseParen);
            eat_token(&mut parser.lexer, Token::OpenBrace);

            let mut cases: Vec<SwitchStatementCase> = Vec::new();
            let mut found_default = false;
            while parser.lexer.token != Token::CloseBrace {
                let mut test: Option<Expression> = None;
                let mut consequent: Vec<Box<Statement>> = Vec::new();

                if parser.lexer.token == Token::Default {
                    if found_default {
                        panic!("Multiple default clauses are not allowed");
                    }
                    scan_next_token(&mut parser.lexer);
                    eat_token(&mut parser.lexer, Token::Colon);
                    found_default = true;
                } else {
                    eat_token(&mut parser.lexer, Token::Case);
                    test = Some(parse_expression(parser, &Precedence::Lowest)?);
                    eat_token(&mut parser.lexer, Token::Colon);
                }

                'case_body: loop {
                    match &parser.lexer.token {
                        Token::CloseBrace | Token::Case | Token::Default => break 'case_body,
                        _ => consequent.push(Box::new(parse_statement(parser)?)),
                    };
                }

                cases.push(SwitchStatementCase { consequent, test })
            }
            eat_token(&mut parser.lexer, Token::CloseBrace);
            Ok(Statement::SwitchStatement(SwitchStatement {
                cases,
                discriminant,
            }))
        }

        Token::Debugger => {
            scan_next_token(&mut parser.lexer);
            Ok(Statement::DebuggerStatement(DebuggerStatement {}))
        }

        Token::With => {
            scan_next_token(&mut parser.lexer);
            eat_token(&mut parser.lexer, Token::OpenParen);
            let object = parse_expression(parser, &Precedence::Lowest)?;
            eat_token(&mut parser.lexer, Token::CloseParen);
            let body = parse_statement(parser)?;
            Ok(Statement::WithStatement(WithStatement {
                body: Box::new(body),
                object,
            }))
        }

        Token::Identifier => {
            let identifier = parse_identifier(parser)?;
            // Parse a labeled statement
            if parser.lexer.token == Token::Colon {
                scan_next_token(&mut parser.lexer);
                let body = parse_statement(parser)?;
                return Ok(Statement::LabeledStatement(LabeledStatement {
                    body: Box::new(body),
                    identifier,
                }));
            }

            // Arrow function
            if parser.lexer.token == Token::EqualsGreaterThan {
                scan_next_token(&mut parser.lexer);
                let body = match parser.lexer.token {
                    Token::OpenBrace => parse_block_statement(parser)
                        .map(ArrowFunctionExpressionBody::BlockStatement)?,
                    _ => parse_expression(parser, &Precedence::Comma)
                        .map(Box::new)
                        .map(ArrowFunctionExpressionBody::Expression)?,
                };
                return Ok(Statement::Expression(ExpressionStatement {
                    expression: Expression::ArrowFunction(ArrowFunctionExpression {
                        body,
                        parameters: vec![ParameterKind::Parameter(Parameter {
                            binding: Binding::Identifier(identifier),
                            initializer: None,
                        })],
                    }),
                }));
            }

            // Parse a normal expression
            let expression = parse_suffix(
                parser,
                &Precedence::Lowest,
                Expression::Identifier(identifier),
            )?;
            consume_semicolon(parser);
            return Ok(Statement::Expression(ExpressionStatement { expression }));
        }

        Token::Throw => {
            scan_next_token(&mut parser.lexer);
            let argument = parse_expression(parser, &Precedence::Lowest)?;
            Ok(Statement::ThrowStatement(ThrowStatement { argument }))
        }

        Token::Try => {
            scan_next_token(&mut parser.lexer);
            let block = parse_block_statement(parser)?;
            let mut handler: Option<CatchClause> = None;
            let mut finalizer: Option<BlockStatement> = None;
            // Either catch or finally must be present.
            if parser.lexer.token != Token::Catch && parser.lexer.token != Token::Finally {
                todo!();
            }
            if parser.lexer.token == Token::Catch {
                scan_next_token(&mut parser.lexer);
                eat_token(&mut parser.lexer, Token::OpenParen);
                let param = parse_binding(parser)?;
                eat_token(&mut parser.lexer, Token::CloseParen);
                let body = parse_block_statement(parser)?;
                handler = Some(CatchClause { body, param });
            }
            if parser.lexer.token == Token::Finally {
                scan_next_token(&mut parser.lexer);
                expect_token(&mut parser.lexer, Token::OpenBrace);
                finalizer = Some(parse_block_statement(parser)?);
            }

            Ok(Statement::TryStatement(TryStatement {
                block,
                handler,
                finalizer,
            }))
        }

        _ => {
            let expression = parse_expression(parser, &Precedence::Lowest)?;
            consume_semicolon(parser);

            Ok(Statement::Expression(ExpressionStatement { expression }))
        }
    }
}

fn parse_import_specifiers(parser: &mut Parser) -> ParseResult<Vec<ImportDeclarationSpecifier>> {
    eat_token(&mut parser.lexer, Token::OpenBrace);
    let mut specifiers: Vec<ImportDeclarationSpecifier> = Vec::new();
    while parser.lexer.token != Token::CloseBrace {
        let local = Identifier {
            name: parser.lexer.identifier.clone(),
        };
        scan_next_token(&mut parser.lexer);
        let imported = match parser.lexer.token {
            Token::As => {
                scan_next_token(&mut parser.lexer);
                parse_identifier(parser).map(Some)?
            }

            _ => None,
        };
        specifiers.push(ImportDeclarationSpecifier {
            imported: match imported {
                Some(i) => i,
                None => local.clone(),
            },
            local,
        });
        if parser.lexer.token == Token::Comma {
            scan_next_token(&mut parser.lexer);
        }
    }
    eat_token(&mut parser.lexer, Token::CloseBrace);
    Ok(specifiers)
}

/// Parses a block statement
///
/// {
///     statement1;
///     statement2;
/// }
fn parse_block_statement(parser: &mut Parser) -> ParseResult<BlockStatement> {
    eat_token(&mut parser.lexer, Token::OpenBrace);
    let mut statements: Vec<Statement> = Vec::new();
    while parser.lexer.token != Token::CloseBrace {
        statements.push(parse_statement(parser)?);
    }
    eat_token(&mut parser.lexer, Token::CloseBrace);
    Ok(BlockStatement { statements })
}

/// Parses an if statement
///
/// if (test) consequent else alternate
/// if (test) consequent else alternate
fn parse_if_statement(parser: &mut Parser) -> ParseResult<IfStatement> {
    scan_next_token(&mut parser.lexer); // if
    eat_token(&mut parser.lexer, Token::OpenParen);
    let test = parse_expression(parser, &Precedence::Lowest)?;
    eat_token(&mut parser.lexer, Token::CloseParen);

    let consequent = parse_statement(parser).map(Box::new)?;
    match consequent.as_ref() {
        Statement::FunctionDeclaration(_) => {
            panic!("Function declarations are not allowed to follow an if-statement in strict mode")
        }
        _ => {}
    };

    let mut alternate: Option<Box<Statement>> = None;
    if parser.lexer.token == Token::Else {
        scan_next_token(&mut parser.lexer);
        let tmp_alternate = parse_statement(parser)?;
        match &tmp_alternate {
            Statement::FunctionDeclaration(_) => panic!(
                "Function declarations are not allowed to follow an if-statement in strict mode"
            ),
            _ => {}
        };
        alternate = Some(Box::new(tmp_alternate));
    }

    Ok(IfStatement {
        alternate,
        consequent,
        test,
    })
}

/// Parses for statement
///
/// for (let a = 1; a < 10; a++) {}
/// for (let a in items) {}
/// for (let a of items) {}
fn parse_for_statement(parser: &mut Parser) -> ParseResult<Statement> {
    scan_next_token(&mut parser.lexer);

    if parser.lexer.token == Token::Await {
        panic!("\"for await\" syntax is not yet supported");
    }

    eat_token(&mut parser.lexer, Token::OpenParen);

    parser.allow_in = false;

    let init = match parser.lexer.token {
        Token::Const | Token::Let | Token::Var => parse_variable_declaration(parser)
            .map(Statement::VariableDeclaration)
            .map(Box::new)
            .map(Some)?,

        Token::Semicolon => {
            scan_next_token(&mut parser.lexer);
            None
        }

        _ => {
            let expression = parse_expression(parser, &Precedence::Lowest)
                .map(|expression| Statement::Expression(ExpressionStatement { expression }))
                .map(Box::new)
                .map(Some)?;
            consume_semicolon(parser);
            expression
        }
    };

    parser.allow_in = true;

    if parser.lexer.token == Token::Of {
        // TODO: We should check for declarations here and forbid them if they exist.
        scan_next_token(&mut parser.lexer);
        let right = parse_expression(parser, &Precedence::Lowest)?;
        eat_token(&mut parser.lexer, Token::CloseParen);
        let body = parse_statement(parser)?;
        if let Some(left) = init {
            return Ok(Statement::ForOfStatement(ForOfStatement {
                body: Box::new(body),
                left,
                right,
            }));
        } else {
            // This essentially means we've somehow reached something like
            // "for (in <expression>) {}"" which should be impossible to reach.
            todo!();
        }
    }

    if parser.lexer.token == Token::In {
        // TODO: We should check for declarations here and forbid them if they exist.
        scan_next_token(&mut parser.lexer);
        let right = parse_expression(parser, &Precedence::Lowest)?;
        eat_token(&mut parser.lexer, Token::CloseParen);
        let body = parse_statement(parser)?;
        if let Some(left) = init {
            return Ok(Statement::ForInStatement(ForInStatement {
                body: Box::new(body),
                left,
                right,
            }));
        } else {
            // This essentially means we've somehow reached something like
            // "for (in <expression>) {}"" which should be impossible to reach.
            todo!();
        }
    }

    let test = match parser.lexer.token {
        Token::Semicolon => {
            scan_next_token(&mut parser.lexer);
            None
        }
        _ => {
            let expression = parse_expression(parser, &Precedence::Lowest).map(Some)?;
            eat_token(&mut parser.lexer, Token::Semicolon);
            expression
        }
    };

    let update = match parser.lexer.token {
        Token::CloseParen => {
            scan_next_token(&mut parser.lexer);
            None
        }
        _ => {
            let expression = parse_expression(parser, &Precedence::Lowest).map(Some)?;
            eat_token(&mut parser.lexer, Token::CloseParen);
            expression
        }
    };

    let body = parse_statement(parser).map(Box::new)?;
    Ok(Statement::ForStatement(ForStatement {
        body,
        init,
        test,
        update,
    }))
}

/// Parses a variable declaration (var, const and let)
///
/// var a = 1;
/// var a = 1, b = 2;
/// var a;
fn parse_variable_declaration(parser: &mut Parser) -> ParseResult<VariableDeclaration> {
    let kind = match parser.lexer.token {
        Token::Const => VariableDeclarationKind::Const,
        Token::Let => VariableDeclarationKind::Let,
        Token::Var => VariableDeclarationKind::Var,
        _ => todo!(),
    };
    scan_next_token(&mut parser.lexer);

    let mut declarations: Vec<VariableDeclarator> = Vec::new();
    loop {
        let mut initializer: Option<Expression> = None;
        let binding = parse_binding(parser)?;
        if parser.lexer.token == Token::Equals {
            scan_next_token(&mut parser.lexer);
            initializer = parse_expression(parser, &Precedence::Assign).map(Some)?;
        }
        declarations.push(VariableDeclarator {
            binding,
            initializer,
        });
        if parser.lexer.token != Token::Comma {
            break;
        }
        scan_next_token(&mut parser.lexer);
    }

    consume_semicolon(parser);

    Ok(VariableDeclaration { declarations, kind })
}
