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
    expression: Literal,
    directive: String,
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    body: Vec<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody {
    body: FunctionBodyBody,
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
    object: ExpressionData,
    body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    argument: Option<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub struct LabeledStatement {
    label: Identifier,
    body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct BreakStatement {
    label: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct ContinueStatement {
    label: Option<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    test: ExpressionData,
    consequent: Box<StatementData>,
    alternate: Option<Box<StatementData>>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchStatement {
    discriminant: ExpressionData,
    cases: Vec<SwitchCase>,
}

#[derive(Debug, PartialEq)]
pub struct SwitchCase {
    test: Option<ExpressionData>,
    consequent: Vec<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct ThrowStatement {
    argument: ExpressionData,
}

#[derive(Debug, PartialEq)]
pub struct TryStatement {
    block: BlockStatement,
    handler: Option<CatchClause>,
    finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialEq)]
pub struct CatchClause {
    param: Pattern,
    body: BlockStatement,
}

#[derive(Debug, PartialEq)]
pub struct WhileStatement {
    test: ExpressionData,
    body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub struct DoWhileStatement {
    body: Box<StatementData>,
    test: ExpressionData,
}

#[derive(Debug, PartialEq)]
pub struct ForStatement {
    init: ForStatementInit,
    test: Option<ExpressionData>,
    update: Option<ExpressionData>,
    body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub enum ForStatementInit {
    VariableDeclaration,
    Expression,
}

#[derive(Debug, PartialEq)]
pub struct ForInStatement {
    left: ForInStatementLeft,
    right: ExpressionData,
    body: Box<StatementData>,
}

#[derive(Debug, PartialEq)]
pub enum ForInStatementLeft {
    Directive,
    Statement,
}
