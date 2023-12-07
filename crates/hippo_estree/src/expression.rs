use crate::{
    BaseNode, FunctionBody, Literal, MetaProperty, TaggedTemplateExpression, TemplateLiteral,
};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum ExpressionData {
    // ES5
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

    // ES2015
    ArrowFunction(ArrowFunctionExpression),
    Yield(YieldExpression),
    TemplateLiteral(TemplateLiteral),
    TaggedTemplate(TaggedTemplateExpression),
    MetaProperty(MetaProperty),

    // ES2017
    Await(AwaitExpression),

    Literal(Literal),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ThisExpression {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub elements: Vec<Option<Box<MemberExpressionElements>>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum MemberExpressionElements {
    Expression(ExpressionData),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    #[serde(flatten)]
    pub node: BaseNode,
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
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub node: BaseNode,
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
    pub node: BaseNode,
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
    pub node: BaseNode,
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
    StarStar,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentExpression {
    #[serde(flatten)]
    pub node: BaseNode,
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
    StarStarEqual,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LogicalExpression {
    #[serde(flatten)]
    pub node: BaseNode,
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
    pub node: BaseNode,
    pub object: Box<MemberExpressionObject>,
    pub property: Box<ExpressionData>,
    pub computed: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum MemberExpressionObject {
    Expression(ExpressionData),
    Super(Super),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ConditionalExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub test: Box<ExpressionData>,
    pub alternate: Box<ExpressionData>,
    pub consequent: Box<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CallExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub callee: Box<CallExpressionCallee>,
    pub arguments: Vec<CallExpressionArguments>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum CallExpressionCallee {
    Expression(ExpressionData),
    Super(Super),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum CallExpressionArguments {
    Expression(ExpressionData),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct NewExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub callee: Box<ExpressionData>,
    pub arguments: Vec<NewExpressionArguments>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum NewExpressionArguments {
    Expression(ExpressionData),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SequenceExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub expressions: Vec<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Super {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SpreadElement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub argument: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrowFunctionExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub body: ArrowFunctionExpressionBody,
    pub expression: bool,
    pub generator: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ArrowFunctionExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Box<ExpressionData>),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct YieldExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub argument: Option<Box<ExpressionData>>,
    pub delegate: bool,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AwaitExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub argument: Box<ExpressionData>,
}
