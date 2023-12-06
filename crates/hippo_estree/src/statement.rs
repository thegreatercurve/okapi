use serde::Serialize;

use crate::{DeclarationData, ExpressionData, Identifier, Literal, Node, Pattern};

#[derive(Debug, PartialEq, Serialize)]
pub enum StatementData {
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

    Declaration(DeclarationData),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub node: Node,
    pub expression: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Directive {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Literal,
    pub directive: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct BlockStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionBody {
    #[serde(flatten)]
    pub node: Node,
    pub body: FunctionBodyBody,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum FunctionBodyBody {
    Directive,
    Statement,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct WithStatement {
    #[serde(flatten)]
    pub node: Node,
    pub object: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ReturnStatement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Option<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Identifier,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct BreakStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ContinueStatement {
    #[serde(flatten)]
    pub node: Node,
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct IfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: ExpressionData,
    pub consequent: Box<StatementData>,
    pub alternate: Option<Box<StatementData>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SwitchStatement {
    #[serde(flatten)]
    pub node: Node,
    pub discriminant: ExpressionData,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SwitchCase {
    #[serde(flatten)]
    pub node: Node,
    pub test: Option<ExpressionData>,
    pub consequent: Vec<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ThrowStatement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TryStatement {
    #[serde(flatten)]
    pub node: Node,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct CatchClause {
    #[serde(flatten)]
    pub node: Node,
    pub param: Pattern,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct WhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub test: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub node: Node,
    pub body: Box<StatementData>,
    pub test: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ForStatement {
    #[serde(flatten)]
    pub node: Node,
    pub init: ForStatementInit,
    pub test: Option<ExpressionData>,
    pub update: Option<ExpressionData>,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ForStatementInit {
    VariableDeclaration,
    Expression,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ForInStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: ForInStatementLeft,
    pub right: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ForInStatementLeft {
    Directive,
    Statement,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: ForInStatementLeft,
    pub right: ExpressionData,
    pub body: Box<StatementData>,
}
