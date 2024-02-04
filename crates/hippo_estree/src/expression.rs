use crate::{
    ArrayPattern, FunctionBody, Identifier, Literal, MetaProperty, Node, ObjectPattern,
    TaggedTemplateExpression, TemplateLiteral,
};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),

    // ES5
    This(ThisExpression),
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

    // ES2015
    ArrowFunction(ArrowFunctionExpression),
    Yield(YieldExpression),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplate(TaggedTemplateExpression),
    MetaProperty(MetaProperty),

    // ES2017
    Await(AwaitExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ThisExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<Option<MemberExpressionElements>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum MemberExpressionElements {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectExpression {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectExpressionProperties>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum ObjectExpressionProperties {
    Property(Property),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    #[serde(flatten)]
    pub node: Node,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
    pub key: PropertyKey,
    pub kind: PropertyKind,
    pub value: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PropertyKey {
    Literal(Literal),
    Identifier(Identifier),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Bang,
    Tilde,
    Typeof,
    Void,
    Delete,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UpdateExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: UpdateOperator,
    pub argument: Box<Expression>,
    pub prefix: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum UpdateOperator {
    PlusPlus,
    MinusMinus,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BinaryExpression {
    #[serde(flatten)]
    pub node: Node,
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BinaryOperator {
    #[serde(rename = "==")]
    EqualEqual,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "===")]
    EqualEqualEqual,
    #[serde(rename = "!==")]
    NotEqualEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanEqual,
    #[serde(rename = "<<")]
    LessThanLessThan,
    #[serde(rename = ">>")]
    GreaterThanGreaterThan,
    #[serde(rename = ">>>")]
    GreaterThanGreaterThanGreaterThan,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "*")]
    Star,
    #[serde(rename = "/")]
    Slash,
    #[serde(rename = "%")]
    Percent,
    #[serde(rename = "|")]
    Bar,
    #[serde(rename = "^")]
    Caret,
    #[serde(rename = "&")]
    Ampersand,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "instanceof")]
    Instanceof,
    #[serde(rename = "**")]
    StarStar,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Box<AssignmentExpressionLeft>,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AssignmentExpressionLeft {
    ArrayPattern(ArrayPattern),
    ObjectPattern(ObjectPattern),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AssignExpressionLeftPattern {}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum AssignmentOperator {
    #[serde(rename = "=")]
    Assignment,
    #[serde(rename = "+=")]
    AdditionAssignment,
    #[serde(rename = "-=")]
    MinusAssignment,
    #[serde(rename = "*=")]
    MultiplyAssignment,
    #[serde(rename = "/=")]
    DivisionAssignment,
    #[serde(rename = "%=")]
    ModulusAssignment,
    #[serde(rename = "<<=")]
    LeftShiftAssignment,
    #[serde(rename = ">>=")]
    RightShiftAssignment,
    #[serde(rename = ">>>=")]
    UnsignedRightShiftAssignment,
    #[serde(rename = "|=")]
    BitwiseAndAssignment,
    #[serde(rename = "^=")]
    BitwiseOrAssignment,
    #[serde(rename = "&=")]
    BitwiseXorAssignment,
    // ES2016
    #[serde(rename = "**=")]
    ExponentiationAssignment,
    // ES2021
    #[serde(rename = "||=")]
    LogicalOrAssignment,
    #[serde(rename = "&&=")]
    LogicalAndAssignment,
    #[serde(rename = "??=")]
    NullishCoalescingAssignment,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LogicalExpression {
    #[serde(flatten)]
    pub node: Node,
    pub left: Box<Expression>,
    pub operator: LogicalOperator,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum LogicalOperator {
    #[serde(rename = "??")]
    NullishCoalescing,
    #[serde(rename = "||")]
    Or,
    #[serde(rename = "&&")]
    And,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MemberExpression {
    #[serde(flatten)]
    pub node: Node,
    pub object: Box<MemberExpressionObject>,
    pub property: Box<Expression>,
    pub computed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum MemberExpressionObject {
    Expression(Expression),
    Super(Super),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ConditionalExpression {
    #[serde(flatten)]
    pub node: Node,
    pub test: Box<Expression>,
    pub consequent: Box<Expression>,
    pub alternate: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CallExpression {
    #[serde(flatten)]
    pub node: Node,
    pub callee: Box<CallExpressionCallee>,
    pub arguments: Vec<CallExpressionArguments>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum CallExpressionCallee {
    Expression(Expression),
    Super(Super),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum CallExpressionArguments {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct NewExpression {
    #[serde(flatten)]
    pub node: Node,
    pub callee: Box<Expression>,
    pub arguments: Vec<NewExpressionArguments>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum NewExpressionArguments {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SequenceExpression {
    #[serde(flatten)]
    pub node: Node,
    pub expressions: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Super {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SpreadElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrowFunctionExpression {
    #[serde(flatten)]
    pub node: Node,
    pub body: ArrowFunctionExpressionBody,
    pub expression: bool,
    pub generator: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum ArrowFunctionExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Box<Expression>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct YieldExpression {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Option<Box<Expression>>,
    pub delegate: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AwaitExpression {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Box<Expression>,
}
