use crate::{class::ClassExpression, statement::BlockStatement};

/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table
/// https://github.com/evanw/esbuild/blob/51b785f89933426afe675b4e633cf531d5a9890d/internal/js_ast/js_ast.go#L29
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Comma,
    Spread,
    Yield,
    Assign,
    Conditional,
    NullishCoalescing,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equals,
    Compare,
    Shift,
    Add,
    Multiply,
    Exponentiation,
    Prefix,
    Postfix,
    New,
    Call,
    Member,
}

impl Precedence {
    pub fn raise(&self) -> Precedence {
        match self {
            Precedence::Lowest => Precedence::Comma,
            Precedence::Comma => Precedence::Spread,
            Precedence::Spread => Precedence::Yield,
            Precedence::Yield => Precedence::Assign,
            Precedence::Assign => Precedence::Conditional,
            Precedence::Conditional => Precedence::NullishCoalescing,
            Precedence::NullishCoalescing => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::BitwiseOr,
            Precedence::BitwiseOr => Precedence::BitwiseXor,
            Precedence::BitwiseXor => Precedence::BitwiseAnd,
            Precedence::BitwiseAnd => Precedence::Equals,
            Precedence::Equals => Precedence::Compare,
            Precedence::Compare => Precedence::Shift,
            Precedence::Shift => Precedence::Add,
            Precedence::Add => Precedence::Multiply,
            Precedence::Multiply => Precedence::Exponentiation,
            Precedence::Exponentiation => Precedence::Prefix,
            Precedence::Prefix => Precedence::Postfix,
            Precedence::Postfix => Precedence::New,
            Precedence::New => Precedence::Call,
            Precedence::Call => Precedence::Member,
            Precedence::Member => Precedence::Member,
        }
    }

    pub fn lower(&self) -> Precedence {
        match self {
            Precedence::Lowest => Precedence::Lowest,
            Precedence::Comma => Precedence::Lowest,
            Precedence::Spread => Precedence::Comma,
            Precedence::Yield => Precedence::Spread,
            Precedence::Assign => Precedence::Yield,
            Precedence::Conditional => Precedence::Assign,
            Precedence::NullishCoalescing => Precedence::Conditional,
            Precedence::LogicalOr => Precedence::NullishCoalescing,
            Precedence::LogicalAnd => Precedence::LogicalOr,
            Precedence::BitwiseOr => Precedence::LogicalAnd,
            Precedence::BitwiseXor => Precedence::BitwiseOr,
            Precedence::BitwiseAnd => Precedence::BitwiseXor,
            Precedence::Equals => Precedence::BitwiseAnd,
            Precedence::Compare => Precedence::Equals,
            Precedence::Shift => Precedence::Compare,
            Precedence::Add => Precedence::Shift,
            Precedence::Multiply => Precedence::Add,
            Precedence::Exponentiation => Precedence::Multiply,
            Precedence::Prefix => Precedence::Exponentiation,
            Precedence::Postfix => Precedence::Prefix,
            Precedence::New => Precedence::Postfix,
            Precedence::Call => Precedence::New,
            Precedence::Member => Precedence::Call,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OpCode {
    // Prefix
    UnaryPositive,
    UnaryNegative,
    UnaryBinaryNot,
    UnaryLogicalNot,
    UnaryVoid,
    UnaryTypeof,
    UnaryDelete,

    // Prefix update
    UnaryPrefixDecrement,
    UnaryPrefixIncrement,

    // Postfix update
    UnaryPostfixDecrement,
    UnaryPostfixIncrement,

    // Left-associative
    BinaryAddition,
    BinarySubstitution,
    BinaryMultiply,
    BinaryDivision,
    BinaryReminder,
    BinaryPower,
    BinaryLessThan,
    BinaryLessThanEquals,
    BinaryGreaterThan,
    BinaryGreaterThanEquals,
    BinaryIn,
    BinaryInstanceof,
    BinaryLeftShift,
    BinaryRightShift,
    BinaryUnsignedRightShift,
    BinaryLooseEquals,
    BinaryLooseNotEquals,
    BinaryStrictEquals,
    BinaryStrictNotEquals,
    BinaryNullishCoalescing,
    BinaryLogicalOr,
    BinaryLogicalAnd,
    BinaryBitwiseOr,
    BinaryBitwiseAnd,
    BinaryBitwiseXor,

    // Non-associative
    BinaryComma,

    // Right-associative
    BinaryAssign,
    BinaryAdditionAssign,
    BinarySubstitutionAssign,
    BinaryMultiplyAssign,
    BinaryDivisionAssign,
    BinaryReminderAssign,
    BinaryPowerAssign,
    BinaryLeftShiftAssign,
    BinaryRightShiftAssign,
    BinaryUnsignedRightShiftAssign,
    BinaryBitwiseOrAssign,
    BinaryBitwiseAndAssign,
    BinaryBitwiseXorAssign,
    BinaryNullishCoalescingAssign,
    BinaryLogicalOrAssign,
    BinaryLogicalAndAssign,
}

impl OpCode {
    pub fn is_left_associative(&self) -> bool {
        *self >= OpCode::BinaryAddition && *self < OpCode::BinaryComma
    }

    pub fn is_right_associative(&self) -> bool {
        *self >= OpCode::BinaryAssign || *self == OpCode::BinaryPower
    }

    pub fn is_prefix(&self) -> bool {
        *self < OpCode::UnaryPostfixDecrement
    }
}

pub struct OpEntry {
    pub text: String,
    pub precedence: Precedence,
    pub is_keyword: bool,
}

pub fn get_op_entry(op_code: &OpCode) -> OpEntry {
    match op_code {
        // Prefix
        OpCode::UnaryPositive => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "+".into(),
        },
        OpCode::UnaryNegative => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "-".into(),
        },
        OpCode::UnaryBinaryNot => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "~".into(),
        },
        OpCode::UnaryLogicalNot => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "!".into(),
        },
        OpCode::UnaryVoid => OpEntry {
            is_keyword: true,
            precedence: Precedence::Prefix,
            text: "void".into(),
        },
        OpCode::UnaryTypeof => OpEntry {
            is_keyword: true,
            precedence: Precedence::Prefix,
            text: "typeof".into(),
        },
        OpCode::UnaryDelete => OpEntry {
            is_keyword: true,
            precedence: Precedence::Prefix,
            text: "delete".into(),
        },

        // Prefix Update
        OpCode::UnaryPrefixDecrement => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "--".into(),
        },
        OpCode::UnaryPrefixIncrement => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "++".into(),
        },

        // Postfix
        OpCode::UnaryPostfixDecrement => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "--".into(),
        },
        OpCode::UnaryPostfixIncrement => OpEntry {
            is_keyword: false,
            precedence: Precedence::Prefix,
            text: "++".into(),
        },

        // Left-associative
        OpCode::BinaryAddition => OpEntry {
            is_keyword: false,
            precedence: Precedence::Add,
            text: "+".into(),
        },
        OpCode::BinarySubstitution => OpEntry {
            is_keyword: false,
            precedence: Precedence::Add,
            text: "-".into(),
        },
        OpCode::BinaryMultiply => OpEntry {
            is_keyword: false,
            precedence: Precedence::Multiply,
            text: "*".into(),
        },
        OpCode::BinaryDivision => OpEntry {
            is_keyword: false,
            precedence: Precedence::Multiply,
            text: "/".into(),
        },
        OpCode::BinaryReminder => OpEntry {
            is_keyword: false,
            precedence: Precedence::Multiply,
            text: "%".into(),
        },
        OpCode::BinaryPower => OpEntry {
            is_keyword: false,
            precedence: Precedence::Exponentiation,
            text: "**".into(),
        },
        OpCode::BinaryLessThan => OpEntry {
            is_keyword: false,
            precedence: Precedence::Compare,
            text: "<".into(),
        },
        OpCode::BinaryLessThanEquals => OpEntry {
            is_keyword: false,
            precedence: Precedence::Compare,
            text: "<=".into(),
        },
        OpCode::BinaryGreaterThan => OpEntry {
            is_keyword: false,
            precedence: Precedence::Compare,
            text: ">".into(),
        },
        OpCode::BinaryGreaterThanEquals => OpEntry {
            is_keyword: false,
            precedence: Precedence::Compare,
            text: ">=".into(),
        },
        OpCode::BinaryIn => OpEntry {
            is_keyword: false,
            precedence: Precedence::Compare,
            text: "in".into(),
        },
        OpCode::BinaryInstanceof => OpEntry {
            is_keyword: false,
            precedence: Precedence::Compare,
            text: "instanceof".into(),
        },
        OpCode::BinaryLeftShift => OpEntry {
            is_keyword: false,
            precedence: Precedence::Shift,
            text: "<<".into(),
        },
        OpCode::BinaryRightShift => OpEntry {
            is_keyword: false,
            precedence: Precedence::Shift,
            text: ">>".into(),
        },
        OpCode::BinaryUnsignedRightShift => OpEntry {
            is_keyword: false,
            precedence: Precedence::Shift,
            text: ">>>".into(),
        },
        OpCode::BinaryLooseEquals => OpEntry {
            is_keyword: false,
            precedence: Precedence::Equals,
            text: "==".into(),
        },
        OpCode::BinaryLooseNotEquals => OpEntry {
            is_keyword: false,
            precedence: Precedence::Equals,
            text: "!=".into(),
        },
        OpCode::BinaryStrictEquals => OpEntry {
            is_keyword: false,
            precedence: Precedence::Equals,
            text: "===".into(),
        },
        OpCode::BinaryStrictNotEquals => OpEntry {
            is_keyword: false,
            precedence: Precedence::Equals,
            text: "!==".into(),
        },
        OpCode::BinaryNullishCoalescing => OpEntry {
            is_keyword: false,
            precedence: Precedence::NullishCoalescing,
            text: "??".into(),
        },
        OpCode::BinaryLogicalOr => OpEntry {
            is_keyword: false,
            precedence: Precedence::LogicalOr,
            text: "||".into(),
        },
        OpCode::BinaryLogicalAnd => OpEntry {
            is_keyword: false,
            precedence: Precedence::LogicalAnd,
            text: "&&".into(),
        },
        OpCode::BinaryBitwiseOr => OpEntry {
            is_keyword: false,
            precedence: Precedence::BitwiseOr,
            text: "|".into(),
        },
        OpCode::BinaryBitwiseAnd => OpEntry {
            is_keyword: false,
            precedence: Precedence::BitwiseAnd,
            text: "&".into(),
        },
        OpCode::BinaryBitwiseXor => OpEntry {
            is_keyword: false,
            precedence: Precedence::BitwiseXor,
            text: "^".into(),
        },

        // Non-associative
        OpCode::BinaryComma => OpEntry {
            is_keyword: false,
            precedence: Precedence::Comma,
            text: ",".into(),
        },

        // Right-associative
        OpCode::BinaryAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "=".into(),
        },
        OpCode::BinaryAdditionAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "+=".into(),
        },
        OpCode::BinarySubstitutionAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "-=".into(),
        },
        OpCode::BinaryMultiplyAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "*=".into(),
        },
        OpCode::BinaryDivisionAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "/=".into(),
        },
        OpCode::BinaryReminderAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "%=".into(),
        },
        OpCode::BinaryPowerAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "**=".into(),
        },
        OpCode::BinaryLeftShiftAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "<<=".into(),
        },
        OpCode::BinaryRightShiftAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: ">>=".into(),
        },
        OpCode::BinaryUnsignedRightShiftAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: ">>>=".into(),
        },
        OpCode::BinaryBitwiseOrAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "|=".into(),
        },
        OpCode::BinaryBitwiseAndAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "&=".into(),
        },
        OpCode::BinaryBitwiseXorAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "^=".into(),
        },
        OpCode::BinaryNullishCoalescingAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "??=".into(),
        },
        OpCode::BinaryLogicalOrAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "||=".into(),
        },
        OpCode::BinaryLogicalAndAssign => OpEntry {
            is_keyword: false,
            precedence: Precedence::Assign,
            text: "&&=".into(),
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BinaryExpression(BinaryExpression),
    BooleanExpression(BooleanExpression),
    FunctionExpression(FunctionExpression),
    StringLiteral(StringLiteral),
    RegexpLiteral(RegexpLiteral),
    NullLiteral(NullLiteral),
    CallExpression(CallExpression),
    ConditionalExpression(ConditionalExpression),
    UnaryExpression(UnaryExpression),
    ThisExpression(ThisExpression),
    ArrayExpression(ArrayExpression),
    ObjectExpression(ObjectExpression),
    NewExpression(NewExpression),
    MemberExpression(MemberExpression),
    ClassExpression(ClassExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NewExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpression {}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayExpression {
    pub elements: Vec<Option<Box<Expression>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectExpressionPropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpressionProperty {
    pub is_computed: bool,
    pub is_method: bool,
    pub key: Option<Expression>,
    pub value: Expression,
    pub kind: ObjectExpressionPropertyKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectExpressionProperty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegexpLiteral {
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NullLiteral {}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub value: f64,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BooleanExpression {
    pub value: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub id: Option<Identifier>,
    pub parameters: Vec<Pattern>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub test: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternate: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub op_code: OpCode,
    pub expression: Box<Expression>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub op_code: OpCode,
    pub right: Box<Expression>,
}

/* -------------------------------------------------------------------------- */
/*                                  Patterns                                  */
/* -------------------------------------------------------------------------- */

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Identifier(Identifier),
    ObjectPattern(ObjectPattern),
    ArrayPattern(ArrayPattern),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectPatternProperty {
    pub is_computed: bool,
    pub is_rest: bool,
    pub key: Option<Expression>,
    pub value: Pattern,
    pub default_value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectPattern {
    pub properties: Vec<ObjectPatternProperty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayPatternItem {
    pub is_rest: bool,
    pub value: Pattern,
    pub default_value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayPattern {
    pub properties: Vec<Option<ArrayPatternItem>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RestElement {
    pub argument: Box<Pattern>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentPattern {
    pub right: Box<Expression>,
}
