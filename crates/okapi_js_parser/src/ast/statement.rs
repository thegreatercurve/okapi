use serde::Serialize;

use crate::ast::{Declaration, Expression, Identifier, Node, Pattern, VariableDeclaration};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Statement {
    Block(BlockStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Debugger(DebuggerStatement),
    Directive(DirectiveStatement),
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
    StaticBlock(StaticBlock),
    Switch(SwitchStatement),
    Throw(ThrowStatement),
    Try(TryStatement),
    While(WhileStatement),
    With(WithStatement),
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
    pub body: Vec<StatementListItem>,
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
#[serde(tag = "type", rename = "ExpressionStatement")]
pub struct DirectiveStatement {
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
#[serde(untagged)]
pub enum ForStatementInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForInStatement {
    #[serde(flatten)]
    pub node: Node,
    pub left: ForInStatementLeft,
    pub right: Expression,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ForInStatementLeft {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Pattern(Pattern),
}

impl From<ForStatementInit> for ForInStatementLeft {
    fn from(init: ForStatementInit) -> Self {
        match init {
            ForStatementInit::VariableDeclaration(declaration) => {
                ForInStatementLeft::VariableDeclaration(declaration)
            }
            ForStatementInit::Expression(expression) => ForInStatementLeft::Expression(expression),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ForOfStatement {
    #[serde(flatten)]
    pub node: Node,
    #[serde(rename = "await")]
    pub awaiting: bool,
    pub left: ForInStatementLeft,
    pub right: Expression,
    pub body: Box<Statement>,
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
    pub body: Box<Statement>,
    pub label: Identifier,
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
pub struct StaticBlock {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<StatementListItem>,
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
    pub consequent: Vec<Statement>,
    pub test: Option<Expression>,
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
    pub param: Option<Pattern>,
    pub body: BlockStatement,
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
    pub object: Expression,
    pub body: Box<Statement>,
}
