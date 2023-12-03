use crate::Literal;

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

    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub struct ThisExpression {}

#[derive(Debug, PartialEq)]
pub struct ArrayExpression {
    pub elements: Vec<Option<Box<ExpressionData>>>,
}

#[derive(Debug, PartialEq)]
pub struct ObjectExpression {
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub key: PropertyKey,
    pub value: Box<ExpressionData>,
    pub kind: ProprtyKind,
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
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<ExpressionData>,
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
    pub operator: UpdateOperator,
    pub argument: Box<ExpressionData>,
    pub prefix: bool,
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
    pub operator: AssignmentOperator,
    pub left: AssignmentExpressionLeft,
    pub right: Box<ExpressionData>,
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
    pub operator: LogicalOperator,
    pub left: Box<ExpressionData>,
    pub right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub struct MemberExpression {
    pub object: Box<ExpressionData>,
    pub property: Box<ExpressionData>,
    pub computed: bool,
}

#[derive(Debug, PartialEq)]
pub struct ConditionalExpression {
    pub test: Box<ExpressionData>,
    pub alternate: Box<ExpressionData>,
    pub consequent: Box<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub callee: Box<ExpressionData>,
    pub arguments: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct NewExpression {
    pub callee: Box<ExpressionData>,
    pub arguments: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct SequenceExpression {
    pub expressions: Vec<ExpressionData>,
}
