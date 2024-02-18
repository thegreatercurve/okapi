use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct Node {
    #[serde(flatten)]
    pub loc: SourceLocation,
}

impl Node {
    pub fn new(start_column: usize, end_column: usize) -> Self {
        Self {
            loc: SourceLocation {
                start: start_column,
                end: end_column,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct SourceLocation {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArgumentListElement {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArrayExpressionElement {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArrayPatternElement {
    AssignmentPattern(AssignmentPattern),
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
    RestElement(RestElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArrowFunctionExpressionBody {
    BlockStatement(BlockStatement),
    Expression(Expression),
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
#[serde(untagged)]
pub enum AssignmentPatternLeft {
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BinaryExpressionLeft {
    Expression(Expression),
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
#[serde(untagged)]
pub enum BindingPattern {
    ArrayPattern(ArrayPattern),
    ObjectPattern,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BindingIdentifier {
    Identifier(Identifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CallExpressionCallee {
    Expression(Expression),
    ImportExpression(ImportExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CatchClauseParam {
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ChainElement {
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Class {
    ClassDeclaration(ClassDeclaration),
    ClassExpression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ClassBodyBody {
    MethodDefinition(MethodDefinition),
    PropertyDefinition(PropertyDefinition),
    StaticBlock(StaticBlock),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    ClassDeclaration(ClassDeclaration),
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportAllDeclarationExported {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportSpecifierExported {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportSpecifierLocal {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportableDefaultDeclaration {
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
    ClassDeclaration(ClassDeclaration),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportableNamedDeclaration {
    AsyncFunctionDeclaration(AsyncFunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportDeclaration {
    ExportAllDeclaration(ExportAllDeclaration),
    ExportDefaultDeclaration(ExportDefaultDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
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
#[serde(untagged)]
pub enum FunctionParameter {
    AssignmentPattern(AssignmentPattern),
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Function {
    FunctionDeclaration(FunctionDeclaration),
    FunctionExpression(FunctionExpression),
    ArrowFunctionExpression(ArrowFunctionExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportAttributeKey {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportDeclarationSpecifier {
    ImportDefaultSpecifier(ImportDefaultSpecifier),
    ImportNamespaceSpecifier(ImportNamespaceSpecifier),
    ImportSpecifier(ImportSpecifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportSpecifierImported {
    Identifier(Identifier),
    Literal(Literal),
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ImportSpecifierKind {
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
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
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegexLiteral),
}
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MemberExpressionProperty {
    Expression(Expression),
    PrivateIdentifier(PrivateIdentifier),
}
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MethodDefinitionKey {
    Expression(Expression),
    PrivateIdentifier(PrivateIdentifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObjectExpressionProperty {
    Property(Property),
    SpreadElement(SpreadElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObjectPatternProperty {
    Property(Property),
    RestElement(RestElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramSource {
    Script,
    Module,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Pattern {
    Expression(Expression),
    ObjectPattern(ObjectPattern),
    ArrayPattern(ArrayPattern),
    Identifier(Identifier),
    AssignmentPattern(AssignmentPattern),
    RestElement(RestElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PropertyKey {
    Identifier(Identifier),
    Literal(Literal),
    PrivateIdentifier(PrivateIdentifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PropertyValue {
    AssignmentPattern(AssignmentPattern),
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
    FunctionExpression(FunctionExpression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RestElementArgument {
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Statement {
    Block(BlockStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Debugger(DebuggerStatement),
    Declaration(Declaration),
    DoWhile(Box<DoWhileStatement>),
    Empty(EmptyStatement),
    Expression(ExpressionStatement),
    For(Box<ForStatement>),
    ForIn(Box<ForInStatement>),
    ForOf(Box<ForOfStatement>),
    If(Box<IfStatement>),
    Labeled(Box<LabeledStatement>),
    Return(ReturnStatement),
    StaticBlock(StaticBlock),
    Switch(SwitchStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
    While(Box<WhileStatement>),
    With(Box<WithStatement>),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum VariableDeclaratorId {
    BindingIdentifier(BindingIdentifier),
    BindingPattern(BindingPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<ArrayExpressionElement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<ArrayPatternElement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrowFunctionExpression {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<FunctionParameter>,
    pub body: Box<ArrowFunctionExpressionBody>,
    pub generator: bool,
    pub expression: bool,
    pub asynchronous: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub node: Node,
    pub left: AssignmentPatternLeft,
    pub right: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AsyncFunctionDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<FunctionParameter>,
    pub body: BlockStatement,
    pub generator: bool,
    pub expression: bool,
    pub asynchronous: bool,
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
pub struct BigIntLiteral {
    #[serde(flatten)]
    pub node: Node,
    pub value: Option<Box<BigIntLiteral>>,
    pub raw: String,
    pub bigint: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BinaryExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: BinaryOperator,
    pub left: Box<BinaryExpressionLeft>,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BlockStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BreakStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CallExpression {
    #[serde(flatten)]
    pub node: Node,
    pub callee: Box<CallExpressionCallee>,
    pub arguments: Vec<ArgumentListElement>,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CatchClause {
    #[serde(flatten)]
    pub node: Node,
    pub param: Option<CatchClauseParam>,
    pub body: BlockStatement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ChainExpression {
    #[serde(flatten)]
    pub node: Node,
    pub expression: ChainElement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassBody {
    #[serde(flatten)]
    pub node: Node,
    pub body: Box<ClassBodyBody>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    #[serde(rename = "superClass")]
    pub super_class: Option<Identifier>,
    pub body: ClassBody,
    pub decorators: Vec<Option<Decorator>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ClassDeclarationDecrorator {
    Decorator(Decorator),
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
    pub decorators: Vec<Option<Decorator>>,
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
pub struct ContinueStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Decorator {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Directive {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Expression,
    pub directive: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Statement,
    pub test: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportAllDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub exported: Option<ExportAllDeclarationExported>,
    pub source: Literal,
    pub assertions: Option<Vec<ImportAttribute>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportDefaultDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declaration: ExportableDefaultDeclaration,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportNamedDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declaration: Option<ExportableNamedDeclaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
    pub assertions: Option<Vec<ImportAttribute>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Box<ExportSpecifierLocal>,
    pub exported: Box<ExportSpecifierExported>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForInStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: Expression,
    pub right: Expression,
    pub body: Statement,
    pub each: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub is_await: bool,
    pub left: Expression,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForStatement {
    #[serde(flatten)]
    pub node: Node,
    pub init: Option<Expression>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    body: Statement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<FunctionParameter>,
    pub body: BlockStatement,
    pub generator: bool,
    pub expression: bool,
    pub asynchronous: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionExpression {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<FunctionParameter>,
    pub body: BlockStatement,
    pub generator: bool,
    pub expression: bool,
    pub asynchronous: bool,
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
pub struct IfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportAttribute {
    #[serde(flatten)]
    pub node: Node,
    pub key: ImportAttributeKey,
    pub value: Literal,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportExpression {
    #[serde(flatten)]
    pub node: Node,
    pub source: Expression,
    pub attributes: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub specifiers: Vec<ImportDeclarationSpecifier>,
    pub source: Literal,
    pub assertions: Option<Vec<ImportAttribute>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportDefaultSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Identifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportNamespaceSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Identifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportSpecifier {
    #[serde(rename = "type")]
    pub kind: ImportSpecifierKind,
    #[serde(flatten)]
    pub node: Node,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<ImportSpecifierImported>,
    pub local: Identifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Identifier,
    pub body: Statement,
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
#[serde(tag = "type")]
pub struct LogicalExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: LogicalOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MemberExpression {
    #[serde(flatten)]
    pub node: Node,
    pub computed: bool,
    pub object: Box<Expression>,
    pub property: Box<MemberExpressionProperty>,
    pub optional: bool,
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
pub struct MethodDefinition {
    #[serde(flatten)]
    pub node: Node,
    pub key: Option<MethodDefinitionKey>,
    pub computed: bool,
    pub value: Option<FunctionExpression>,
    pub kind: String,
    pub stattic: bool,
    pub decorators: Vec<Option<Decorator>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Module {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
    pub source_type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct NewExpression {
    #[serde(flatten)]
    pub node: Node,
    pub callee: Box<Expression>,
    pub arguments: Vec<ArgumentListElement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectExpression {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectExpressionProperty>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectPatternProperty>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct PrivateIdentifier {
    #[serde(flatten)]
    pub node: Node,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
    #[serde(rename = "sourceType")]
    pub source_type: ProgramSource,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Property {
    #[serde(flatten)]
    pub node: Node,
    pub key: PropertyKey,
    pub computed: bool,
    pub value: Option<PropertyValue>,
    pub kind: String,
    pub method: bool,
    pub shorthand: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct PropertyDefinition {
    #[serde(flatten)]
    pub node: Node,
    pub key: PropertyKey,
    pub computed: bool,
    pub value: Option<PropertyValue>,
    pub stattic: bool,
    pub decorators: Option<Vec<Decorator>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Regex;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RegexLiteral {
    #[serde(flatten)]
    pub node: Node,
    pub value: Regex,
    pub raw: String,
    pub regex: RegexLiteralRegex,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RegexLiteralRegex {
    #[serde(flatten)]
    pub node: Node,
    pub pattern: String,
    pub flags: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: RestElementArgument,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ReturnStatement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Script {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
    #[serde(rename = "sourceType")]
    pub source_type: ProgramSource,
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
pub struct SpreadElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct StaticBlock {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SuperExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchCase {
    #[serde(flatten)]
    pub node: Node,
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchStatement {
    #[serde(flatten)]
    pub node: Node,
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TaggedTemplateExpression {
    #[serde(flatten)]
    pub node: Node,
    pub tag: Expression,
    pub quasi: TemplateLiteral,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateElementValue {
    cooked: Option<String>,
    raw: String,
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
#[serde(tag = "type")]
pub struct TemplateLiteral {
    #[serde(flatten)]
    pub node: Node,
    pub quasis: Vec<TemplateElement>,
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
pub struct ThrowStatement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TryStatement {
    #[serde(flatten)]
    pub node: Node,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: String,
    pub argument: Box<Expression>,
    pub prefix: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct UpdateExpression {
    #[serde(flatten)]
    pub node: Node,
    pub operator: String,
    pub argument: Box<Expression>,
    pub prefix: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub node: Node,
    pub id: VariableDeclaratorId,
    pub init: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub body: Statement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WithStatement {
    #[serde(flatten)]
    pub node: Node,
    pub object: Expression,
    pub body: Statement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct YieldExpression {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Option<Box<Expression>>,
    pub delegate: bool,
}
