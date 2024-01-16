use crate::{FunctionBody, MetaProperty, Node, TaggedTemplateExpression, TemplateLiteral};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Expression {
    // ES5
    ThisExpression {
        #[serde(flatten)]
        node: Node,
    },
    ArrayExpression {
        #[serde(flatten)]
        node: Node,
        elements: Vec<Option<Box<MemberExpressionElements>>>,
    },
    ObjectExpression {
        #[serde(flatten)]
        node: Node,
        properties: Vec<ObjectExpressionProperties>,
    },
    FunctionExpression {
        #[serde(flatten)]
        node: Node,
    },
    UnaryExpression {
        #[serde(flatten)]
        node: Node,
        operator: UnaryOperator,
        prefix: bool,
        argument: Box<Expression>,
    },
    UpdateExpression {
        #[serde(flatten)]
        node: Node,
        operator: UpdateOperator,
        prefix: bool,
        argument: Box<Expression>,
    },
    BinaryExpression {
        #[serde(flatten)]
        node: Node,
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    AssignmentExpression {
        #[serde(flatten)]
        node: Node,
        operator: AssignmentOperator,
        left: AssignmentExpressionLeft,
        right: Box<Expression>,
    },
    LogicalExpression {
        #[serde(flatten)]
        node: Node,
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    MemberExpression {
        #[serde(flatten)]
        node: Node,
        object: Box<MemberExpressionObject>,
        property: Box<Expression>,
        computed: bool,
    },
    ConditionalExpression {
        #[serde(flatten)]
        node: Node,
        test: Box<Expression>,
        alternate: Box<Expression>,
        consequent: Box<Expression>,
    },
    CallExpression {
        #[serde(flatten)]
        node: Node,
        callee: Box<CallExpressionCallee>,
        arguments: Vec<CallExpressionArguments>,
    },
    NewExpression {
        #[serde(flatten)]
        node: Node,
        callee: Box<Expression>,
        arguments: Vec<NewExpressionArguments>,
    },
    SequenceExpression {
        #[serde(flatten)]
        node: Node,
        expressions: Vec<Expression>,
    },

    // ES2015
    ArrowFunctionExpression {
        #[serde(flatten)]
        node: Node,
        body: ArrowFunctionExpressionBody,
        expression: bool,
        generator: bool,
    },
    YieldExpression {
        #[serde(flatten)]
        node: Node,
        argument: Option<Box<Expression>>,
        delegate: bool,
    },
    TemplateLiteral(TemplateLiteral),
    TaggedTemplate(TaggedTemplateExpression),
    MetaProperty(MetaProperty),

    // ES2017
    AwaitExpression {
        #[serde(flatten)]
        node: Node,
        argument: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Serialize)]
pub enum MemberExpressionElements {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ObjectExpressionProperties {
    Property(Property),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    #[serde(flatten)]
    pub node: Node,
    pub key: PropertyKey,
    pub value: Box<Expression>,
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
pub enum UpdateOperator {
    PlusPlus,
    MinusMinus,
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
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum MemberExpressionObject {
    Expression(Expression),
    Super(Super),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum CallExpressionCallee {
    Expression(Expression),
    Super(Super),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum CallExpressionArguments {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum NewExpressionArguments {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Super {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SpreadElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Expression,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ArrowFunctionExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Box<Expression>),
}
