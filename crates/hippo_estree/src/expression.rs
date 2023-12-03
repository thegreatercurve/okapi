#[derive(Debug, PartialEq)]
pub enum ExpressionData {
    This,
    Array(ArrayExpression),
    Object(ObjectExpression),
    Function(FunctionExpression),
    Unary(UnaryExpression),
    Update(UpdateExpression),
    Binary(BinaryExpression),
    Assignment(AssignmentExpression),
    Logical(LogicalExpression),
    Member(MemberExpression),
    Conditional(ConditionalExpression),
    Call(CallExpression),
    New(NewExpression),
    Sequence(SequenceExpression),
}

#[derive(Debug, PartialEq)]
pub struct ThisExpression {}

#[derive(Debug, PartialEq)]
pub struct ArrayExpression {
    elements: Vec<Option<Box<ExpressionData>>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpression {
    properties: Vec<Property>,
}

#[derive(Debug, PartialEq)]
pub struct Property {
    key: PropertyKey,
    value: Box<ExpressionData>,
    kind: ProprtyKind,
}

#[derive(Debug, PartialEq)]
pub enum PropertyKey {
    Literal,
    Identifier,
}

#[derive(Debug, PartialEq)]
pub enum ProprtyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {}

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    operator: UnaryOperator,
    prefix: bool,
    argument: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Bang,
    Tilde,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug, PartialEq)]
pub struct UpdateExpression {
    operator: UpdateOperator,
    argument: Box<ExpressionData>,
    prefix: bool,
}

#[derive(Debug, PartialEq)]
pub enum UpdateOperator {
    PlusPlus,
    MinusMinus,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<ExpressionData>,
    pub right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
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

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: AssignmentExpressionLeft,
    right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentExpressionLeft {
    Pattern,
    Expression,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
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

#[derive(Debug, PartialEq)]
pub struct LogicalExpression {
    operator: LogicalOperator,
    left: Box<ExpressionData>,
    right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub struct MemberExpression {
    object: Box<ExpressionData>,
    property: Box<ExpressionData>,
    computed: bool,
}

#[derive(Debug, PartialEq)]
pub struct ConditionalExpression {
    test: Box<ExpressionData>,
    alternate: Box<ExpressionData>,
    consequent: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    callee: Box<ExpressionData>,
    arguments: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct NewExpression {
    callee: Box<ExpressionData>,
    arguments: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct SequenceExpression {
    expressions: Vec<ExpressionData>,
}
