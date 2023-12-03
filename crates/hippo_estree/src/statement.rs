use crate::{DeclarationData, ExpressionData, Identifier, Literal, Pattern};

#[derive(Debug, PartialEq)]
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

    Declaration(DeclarationData),
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: ExpressionData,
}

#[derive(Debug, PartialEq)]
pub struct Directive {
    pub expression: Literal,
    pub directive: String,
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody {
    pub body: FunctionBodyBody,
}

#[derive(Debug, PartialEq)]
pub enum FunctionBodyBody {
    Directive,
    Statement,
}

#[derive(Debug, PartialEq)]
pub struct EmptyStatement {}

#[derive(Debug, PartialEq)]
pub struct DebuggerStatement {}

#[derive(Debug, PartialEq)]
pub struct WithStatement {
    pub object: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub argument: Option<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct LabeledStatement {
    pub label: Identifier,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub test: ExpressionData,
    pub consequent: Box<StatementData>,
    pub alternate: Option<Box<StatementData>>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchStatement {
    pub discriminant: ExpressionData,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchCase {
    pub test: Option<ExpressionData>,
    pub consequent: Vec<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct ThrowStatement {
    pub argument: ExpressionData,
}

#[derive(Debug, PartialEq)]
pub struct TryStatement {
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialEq)]
pub struct CatchClause {
    pub param: Pattern,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    pub test: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct DoWhileStatement {
    pub body: Box<StatementData>,
    pub test: ExpressionData,
}

#[derive(Debug, PartialEq)]
pub struct ForStatement {
    pub init: ForStatementInit,
    pub test: Option<ExpressionData>,
    pub update: Option<ExpressionData>,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub enum ForStatementInit {
    VariableDeclaration,
    Expression,
}

#[derive(Debug, PartialEq)]
pub struct ForInStatement {
    pub left: ForInStatementLeft,
    pub right: ExpressionData,
    pub body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub enum ForInStatementLeft {
    Directive,
    Statement,
}
