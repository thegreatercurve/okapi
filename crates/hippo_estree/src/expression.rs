pub struct Expression {}

pub struct ThisExpression {}

pub struct ArrayExpression {
    elements: [Option<Expression>],
}

pub struct ObjectExpression {
    properties: [Property],
}

pub struct Property {
    key: PropertyKey,
    value: Expression,
    kind: ProprtyKind,
}

enum PropertyKey {
    Literal,
    Identifier,
}

enum ProprtyKind {
    Init,
    Get,
    Set,
}
pub struct FunctionExpression {}

pub struct UnaryExpression {
    operator: UnaryOperator,
    prefix: bool,
    argument: Expression,
}

enum UnaryOperator {
    Minus,
    Plus,
    Bang,
    Tilde,
    Typeof,
    Void,
    Delete,
}

pub struct UpdateExpression {
    operator: UpdateOperator,
    argument: Expression,
    prefix: bool,
}

enum UpdateOperator {
    PlusPlus,
    MinusMinus,
}

pub struct BinaryExpression {
    operator: BinaryOperator,
    left: Expression,
    right: Expression,
}

enum BinaryOperator {
    EqualEqual,
    NotEqual,
    EqualEqualEqual,
    NotEqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThanLessThan,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bar,
    Caret,
    Ampersand,
    In,
    Instanceof,
}

pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: AssignmentExpressionLeft,
    right: Expression,
}

enum AssignmentExpressionLeft {
    Pattern,
    Expression,
}

enum AssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    LessThanLessThanEqual,
    GreaterThanGreaterThanEqual,
    GreaterThanGreaterThanGreaterThanEqual,
    BarEqual,
    CaretEqual,
    AmpersandEqual,
}

pub struct LogicalExpression {
    operator: LogicalOperator,
    left: Expression,
    right: Expression,
}

enum LogicalOperator {
    Or,
    And,
}

pub struct MemberExpression {
    object: Expression,
    property: Expression,
    computed: bool,
}

pub struct ConditionalExpression {
    test: Expression,
    alternate: Expression,
    consequent: Expression,
}

pub struct CallExpression {
    callee: Expression,
    arguments: [Expression],
}

pub struct NewExpression {
    callee: Expression,
    arguments: [Expression],
}

pub struct SequenceExpression {
    expressions: [Expression],
}
