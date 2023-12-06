use crate::{Literal, Node};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ThisExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<Option<Box<ExpressionData>>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectExpression {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    #[serde(flatten)]
    pub node: Node,
    pub key: PropertyKey,
    pub value: Box<ExpressionData>,
    pub kind: ProprtyKind,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PropertyKey {
    Literal,
    Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ProprtyKind {
    Init,
    Get,
    Set,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Bang,
    Tilde,
    Typeof,
    Void,
    Delete,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UpdateExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: UpdateOperator,
    pub argument: Box<ExpressionData>,
    pub prefix: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum UpdateOperator {
    PlusPlus,
    MinusMinus,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BinaryExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: BinaryOperator,
    pub left: Box<ExpressionData>,
    pub right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: AssignmentExpressionLeft,
    pub right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum AssignmentExpressionLeft {
    Pattern,
    Expression,
}

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LogicalExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: LogicalOperator,
    pub left: Box<ExpressionData>,
    pub right: Box<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MemberExpression {
    #[serde(flatten)]
    pub node: Node,
    pub object: Box<ExpressionData>,
    pub property: Box<ExpressionData>,
    pub computed: bool,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ConditionalExpression {
    #[serde(flatten)]
    pub node: Node,
    pub test: Box<ExpressionData>,
    pub alternate: Box<ExpressionData>,
    pub consequent: Box<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CallExpression {
    #[serde(flatten)]
    pub node: Node,
    pub callee: Box<ExpressionData>,
    pub arguments: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct NewExpression {
    #[serde(flatten)]
    pub node: Node,
    pub callee: Box<ExpressionData>,
    pub arguments: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SequenceExpression {
    #[serde(flatten)]
    pub node: Node,
    pub expressions: Vec<ExpressionData>,
}
