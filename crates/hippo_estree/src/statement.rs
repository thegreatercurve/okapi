use serde::Serialize;

use crate::{Declaration, Expression, Identifier, Literal, Node, Pattern};

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Statement {
    Expression(ExpressionStatement),
    Directive(Directive),
    Block(BlockStatement),
    Empty(EmptyStatement),
    Debugger(DebuggerStatement),
    With(WithStatement),
    Return(ReturnStatement),
    Labeled(LabeledStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    If(IfStatement),
    Switch(SwitchStatement),
    SwitchCase(SwitchCase),
    Throw(ThrowStatement),
    Try(TryStatement),
    CatchClause(CatchClause),
    While(WhileStatement),
    DoWhile(DoWhileStatement),
    For(ForStatement),
    ForIn(ForInStatement),
    ForOf(ForOfStatement),

    Declaration(Declaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Directive {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Literal,
    pub directive: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BlockStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionBody {
    #[serde(flatten)]
    pub node: Node,
    pub body: FunctionBodyBody,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum FunctionBodyBody {
    Directive,
    Statement,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WithStatement {
    #[serde(flatten)]
    pub node: Node,
    pub object: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ReturnStatement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Option<Expression>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Identifier,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BreakStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ContinueStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct IfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchStatement {
    #[serde(flatten)]
    pub node: Node,
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchCase {
    #[serde(flatten)]
    pub node: Node,
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ThrowStatement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Expression,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TryStatement {
    #[serde(flatten)]
    pub node: Node,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CatchClause {
    #[serde(flatten)]
    pub node: Node,
    pub param: Pattern,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Box<Statement>,
    pub test: Expression,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForStatement {
    #[serde(flatten)]
    pub node: Node,
    pub init: ForStatementInit,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ForStatementInit {
    VariableDeclaration,
    Expression,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForInStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: ForInStatementLeft,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ForInStatementLeft {
    Directive,
    Statement,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: ForInStatementLeft,
    pub right: Expression,
    pub body: Box<Statement>,
    #[serde(alias = "await")]
    pub awaiting: bool,
}
