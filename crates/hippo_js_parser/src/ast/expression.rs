use crate::{
    ast::{
        ArrayPattern, BlockStatement, FunctionParameter, Node, ObjectPattern, Pattern, StaticBlock,
    },
    ParserError,
};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BindingPattern {
    Array(ArrayPattern),
    Object(ObjectPattern),
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
    Chain(ChainExpression),
    Class(ClassExpression),
    Conditional(ConditionalExpression),
    Function(FunctionExpression),
    Identifier(Identifier),
    Import(ImportExpression),
    Literal(Literal),
    RegExpLiteral(RegExpLiteral),
    Logical(LogicalExpression),
    Member(MemberExpression),
    MetaProperty(MetaProperty),
    New(NewExpression),
    Object(ObjectExpression),
    Sequence(SequenceExpression),
    Super(SuperExpression),
    TaggedTemplate(TaggedTemplateExpression),
    TemplateLiteral(TemplateLiteral),
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
    pub id: Option<Identifier>,
    pub expression: bool,
    pub generator: bool,
    #[serde(rename = "async")]
    pub is_async: bool,
    pub params: Vec<Pattern>,
    pub body: ArrowFunctionExpressionBody,
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

impl TryFrom<Expression> for AssignmentExpression {
    type Error = ParserError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Assignment(assignment_expression) => Ok(assignment_expression),
            _ => Err(ParserError::InvalidExpressionToAssignmentExpressionConversion),
        }
    }
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
    #[serde(rename = "&=")]
    BitwiseAndAssignment,
    #[serde(rename = "|=")]
    BitwiseOrAssignment,
    #[serde(rename = "^=")]
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
    pub left: BinaryExpressionLeft,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BinaryExpressionLeft {
    Expression(Box<Expression>),
    PrivateIdentifier(PrivateIdentifier),
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
    pub callee: CallExpressionCallee,
    pub arguments: Vec<CallExpressionArgument>,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CallExpressionCallee {
    Expression(Box<Expression>),
    Import(ImportExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CallExpressionArgument {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ChainExpression {
    #[serde(flatten)]
    pub node: Node,
    pub expression: ChainElement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ChainElement {
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
}

impl TryFrom<Expression> for ChainElement {
    type Error = ParserError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Call(call_expression) => Ok(ChainElement::CallExpression(call_expression)),
            Expression::Member(member_expression) => {
                Ok(ChainElement::MemberExpression(member_expression))
            }
            _ => Err(ParserError::InvalidExpressionToChainElementConversion),
        }
    }
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
pub struct ImportExpression {
    #[serde(flatten)]
    pub node: Node,
    pub source: Box<Expression>,
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
    Regex {},
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "Literal")]
pub struct RegExpLiteral {
    #[serde(flatten)]
    pub node: Node,
    pub value: LiteralValue,
    pub raw: String,
    pub regex: Regex,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Regex {
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
    pub property: MemberExpressionProperty,
    pub computed: bool,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MemberExpressionProperty {
    Expression(Box<Expression>),
    PrivateIdentifier(PrivateIdentifier),
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

impl TryFrom<CallExpressionArgument> for NewExpressionArguments {
    type Error = ParserError;

    fn try_from(call_expression_arguments: CallExpressionArgument) -> Result<Self, Self::Error> {
        match call_expression_arguments {
            CallExpressionArgument::Expression(expression) => {
                Ok(NewExpressionArguments::Expression(expression))
            }
            CallExpressionArgument::SpreadElement(spread_element) => {
                Ok(NewExpressionArguments::SpreadElement(spread_element))
            }
        }
    }
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
    pub properties: Vec<ObjectExpressionProperty>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]

pub enum ObjectExpressionProperty {
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
    pub value: PropertyValue,
    pub kind: PropertyKind,
}

impl TryFrom<MethodDefinition> for Property {
    type Error = ParserError;

    fn try_from(method_definition: MethodDefinition) -> Result<Self, Self::Error> {
        let MethodDefinition {
            node,
            is_static: _,
            computed: is_computed,
            key,
            kind,
            value,
        } = method_definition;

        let is_method = kind == MethodDefinitionKind::Method;
        let is_shorthand = false;

        let property_key = if let Some(PropertyDefinitionKey::Expression(expression)) = key {
            expression
        } else {
            return Err(ParserError::InvalidMethodDefinitionToPropertyConversion);
        };

        let propery_value = match value {
            Some(function_expression) => {
                PropertyValue::Expression(Expression::Function(function_expression))
            }
            None => return Err(ParserError::InvalidMethodDefinitionToPropertyConversion),
        };

        let property_kind = match kind {
            MethodDefinitionKind::Method => PropertyKind::Init,
            MethodDefinitionKind::Get => PropertyKind::Get,
            MethodDefinitionKind::Set => PropertyKind::Set,
            MethodDefinitionKind::Constructor => PropertyKind::Init,
        };

        Ok(Property {
            node,
            method: is_method,
            shorthand: is_shorthand,
            computed: is_computed,
            key: property_key,
            kind: property_kind,
            value: propery_value,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

impl TryFrom<MethodDefinitionKind> for PropertyKind {
    type Error = ParserError;

    fn try_from(method_definition_kind: MethodDefinitionKind) -> Result<Self, Self::Error> {
        match method_definition_kind {
            MethodDefinitionKind::Method => Ok(PropertyKind::Init),
            MethodDefinitionKind::Get => Ok(PropertyKind::Get),
            MethodDefinitionKind::Set => Ok(PropertyKind::Set),
            MethodDefinitionKind::Constructor => Ok(PropertyKind::Init),
        }
    }
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
pub struct TaggedTemplateExpression {
    #[serde(flatten)]
    pub node: Node,
    pub tag: Box<Expression>,
    pub quasi: TemplateLiteral,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateElement {
    #[serde(flatten)]
    pub node: Node,
    pub value: TemplateElementValue,
    pub tail: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemplateElementValue {
    pub raw: String,
    pub cooked: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateLiteral {
    #[serde(flatten)]
    pub node: Node,
    pub expressions: Vec<Expression>,
    pub quasis: Vec<TemplateElement>,
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
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "!")]
    Bang,
    #[serde(rename = "~")]
    Tilde,
    #[serde(rename = "typeof")]
    Typeof,
    #[serde(rename = "void")]
    Void,
    #[serde(rename = "delete")]
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
#[serde(tag = "type", rename = "Super")]
pub struct SuperExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct YieldExpression {
    #[serde(flatten)]
    pub node: Node,
    pub delegate: bool,
    pub argument: Option<Box<Expression>>,
}
