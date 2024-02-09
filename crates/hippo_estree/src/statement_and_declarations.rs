use serde::Serialize;

use crate::{
    ArrayPattern, AssignmentPattern, BindingPattern, ClassBody, Expression, Identifier, Node,
    ObjectPattern,
};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Statement {
    Block(BlockStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Debugger(DebuggerStatement),
    Declaration(Declaration),
    DoWhile(DoWhileStatement),
    Empty(EmptyStatement),
    Expression(ExpressionStatement),
    For(ForStatement),
    ForIn(ForInStatement),
    ForOf(ForOfStatement),
    If(IfStatement),
    Labeled(LabeledStatement),
    Return(ReturnStatement),
    StaticBlock(StaticBlockStatement),
    Switch(SwitchStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
    While(WhileStatement),
    With(WithStatement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Declaration {
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StatementListItem {
    Declaration(Declaration),
    Statement(Statement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BlockStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Box<StatementListItem>>,
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
pub struct ClassDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    #[serde(alias = "superClass")]
    pub super_class: Option<Expression>,
    pub body: ClassBody,
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
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Box<Statement>,
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
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Expression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directive: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForStatement {
    #[serde(flatten)]
    pub node: Node,
    pub init: Option<ForStatementInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ForStatementInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForInStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: Expression,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: Expression,
    pub right: Expression,
    pub body: Box<Statement>,
    #[serde(alias = "await")]
    pub awaiting: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<FunctionParameter>,
    pub body: BlockStatement,
    pub generate: bool,
    #[serde(alias = "async")]
    pub asynchronous: bool,
    pub expression: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum FunctionParameter {
    AssignmentPattern(AssignmentPattern),
    BindingPattern(BindingPattern),
    Identifier(Identifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct IfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Identifier,
    pub body: Box<Statement>,
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
pub struct StaticBlockStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Box<StatementListItem>>,
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
pub struct SwitchCase {
    #[serde(flatten)]
    pub node: Node,
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
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
pub struct CatchClause {
    #[serde(flatten)]
    pub node: Node,
    pub param: CatchClauseParameter,
    pub body: BlockStatement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum CatchClauseParameter {
    BindingPattern(BindingPattern),
    Identifier(Identifier),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub node: Node,
    pub id: VariableDeclaratorBindingKind,
    pub init: Option<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum VariableDeclaratorBindingKind {
    Identifier(Identifier),
    ObjectPattern(ObjectPattern),
    ArrayPattern(ArrayPattern),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WithStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub body: Box<Statement>,
}
