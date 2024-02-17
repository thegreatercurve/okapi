use crate::{
    ArrayPattern, BlockStatement, FunctionParameter, Node, ObjectPattern, Pattern, StaticBlock,
};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BindingPattern {
    ArrayPattern(ArrayPattern),
    ObjectPattern(ObjectPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Expression {
    Array(ArrayExpression),
    ArrowFunction(ArrowFunctionExpression),
    Assignment(AssignmentExpression),
    Await(AwaitExpression),
    Binary(BinaryExpression),
    Call(CallExpression),
    Class(ClassExpression),
    Conditional(ConditionalExpression),
    Function(FunctionExpression),
    Identifier(Identifier),
    Literal(Literal),
    Logical(LogicalExpression),
    Member(MemberExpression),
    MetaProperty(MetaProperty),
    New(NewExpression),
    Object(ObjectExpression),
    Sequence(SequenceExpression),
    Super(SuperExpression),
    TaggedTemplate(String),
    TemplateLiteral(String),
    This(ThisExpression),
    Unary(UnaryExpression),
    Update(UpdateExpression),
    Yield(YieldExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<Option<ArrayExpressionElement>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum ArrayExpressionElement {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrowFunctionExpression {
    #[serde(flatten)]
    pub node: Node,
    pub params: Vec<Pattern>,
    pub body: ArrowFunctionExpressionBody,
    pub expression: bool,
    pub generator: bool,
    #[serde(rename = "async")]
    pub is_async: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArrowFunctionExpressionBody {
    BlockStatement(BlockStatement),
    Expression(Box<Expression>),
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
    Expression(Expression),
    Pattern(Pattern),
}

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
pub struct AwaitExpression {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Box<Expression>,
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
    Import(Import),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Import {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CallExpressionArguments {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassExpression {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    #[serde(rename = "superClass")]
    pub super_class: Option<Identifier>,
    pub body: ClassBody,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassBody {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<ClassBodyBody>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ClassBodyBody {
    MethodDefinition(MethodDefinition),
    PropertyDefinition(PropertyDefinition),
    StaticBlock(StaticBlock),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct PropertyDefinition {
    #[serde(flatten)]
    pub node: Node,
    #[serde(rename = "static")]
    pub is_static: bool,
    pub computed: bool,
    pub key: Option<PropertyDefinitionKey>,
    pub value: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PropertyDefinitionKey {
    Expression(Expression),
    PrivateIdentifier(PrivateIdentifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MethodDefinition {
    #[serde(flatten)]
    pub node: Node,
    #[serde(rename = "static")]
    pub is_static: bool,
    pub computed: bool,
    pub key: Option<PropertyDefinitionKey>,
    pub kind: MethodDefinitionKind,
    pub value: Option<FunctionExpression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MethodDefinitionKind {
    Constructor,
    Method,
    Get,
    Set,
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
pub struct FunctionExpression {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub expression: bool,
    pub generator: bool,
    #[serde(rename = "async")]
    pub is_async: bool,
    pub params: Vec<FunctionParameter>,
    pub body: BlockStatement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Identifier {
    #[serde(flatten)]
    pub node: Node,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct PrivateIdentifier {
    #[serde(flatten)]
    pub node: Node,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Literal {
    #[serde(flatten)]
    pub node: Node,
    pub value: LiteralValue,
    pub raw: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegExpLiteral),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RegExpLiteral {
    pub regex: Regex,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Regex {
    #[serde(flatten)]
    pub node: Node,
    pub pattern: String,
    pub flags: String,
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
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub computed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MetaProperty {
    #[serde(flatten)]
    pub node: Node,
    pub meta: Identifier,
    pub property: Identifier,
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
pub struct SpreadElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Expression,
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
    pub key: Expression,
    pub kind: PropertyKind,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Expression(Expression),
    Pattern(Pattern),
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
pub struct ThisExpression {
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
    pub prefix: bool,
    pub argument: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum UpdateOperator {
    #[serde(rename = "++")]
    PlusPlus,
    #[serde(rename = "--")]
    MinusMinus,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SuperExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct YieldExpression {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Option<Box<Expression>>,
    pub delegate: bool,
}
