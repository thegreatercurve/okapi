use serde::Serialize;

use crate::{BaseNode, DeclarationData, ExpressionData, Identifier, Literal, Pattern};

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
#[serde(tag = "type")]
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub expression: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Directive {
    #[serde(flatten)]
    pub node: BaseNode,
    pub expression: Literal,
    pub directive: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BlockStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub body: Vec<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionBody {
    #[serde(flatten)]
    pub node: BaseNode,
    pub body: FunctionBodyBody,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum FunctionBodyBody {
    Directive,
    Statement,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WithStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub object: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ReturnStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub argument: Option<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub label: Identifier,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct BreakStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ContinueStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct IfStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub test: ExpressionData,
    pub consequent: Box<StatementData>,
    pub alternate: Option<Box<StatementData>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub discriminant: ExpressionData,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SwitchCase {
    #[serde(flatten)]
    pub node: BaseNode,
    pub test: Option<ExpressionData>,
    pub consequent: Vec<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ThrowStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub argument: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TryStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct CatchClause {
    #[serde(flatten)]
    pub node: BaseNode,
    pub param: Pattern,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct WhileStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub test: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub body: Box<StatementData>,
    pub test: ExpressionData,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForStatement {
    #[serde(flatten)]
    pub node: BaseNode,
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
#[serde(tag = "type")]
pub struct ForInStatement {
    #[serde(flatten)]
    pub node: BaseNode,
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
#[serde(tag = "type")]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub left: ForInStatementLeft,
    pub right: ExpressionData,
    pub body: Box<StatementData>,
}
