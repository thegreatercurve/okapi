use crate::{Expression, Identifier, Literal, Pattern};

pub struct Statement {}

pub struct ExpressionStatement {
    expression: Expression,
}

pub struct Directive {
    expression: Literal,
    directive: String,
}

pub struct BlockStatement {
    body: Vec<Statement>,
}

pub struct FunctionBody {
    body: FunctionBodyBody,
}

pub enum FunctionBodyBody {
    Directive,
    Statement,
}

pub struct EmptyStatement {}

pub struct DebuggerStatement {}

pub struct WithStatement {
    object: Expression,
    body: Statement,
}

pub struct ReturnStatement {
    argument: Option<Expression>,
}

pub struct LabeledStatement {
    label: Identifier,
    body: Statement,
}

pub struct BreakStatement {
    label: Option<Identifier>,
}

pub struct ContinueStatement {
    label: Option<Identifier>,
}

pub struct IfStatement {
    test: Expression,
    consequent: Statement,
    alternate: Option<Statement>,
}

pub struct SwitchStatement {
    discriminant: Expression,
    cases: Vec<SwitchCase>,
}

pub struct SwitchCase {
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

pub struct ThrowStatement {
    argument: Expression,
}

pub struct TryStatement {
    block: BlockStatement,
    handler: Option<CatchClause>,
    finalizer: Option<BlockStatement>,
}

pub struct CatchClause {
    param: Pattern,
    body: BlockStatement,
}

pub struct WhileStatement {
    test: Expression,
    body: Statement,
}

pub struct DoWhileStatement {
    body: Statement,
    test: Expression,
}

pub struct ForStatement {
    init: ForStatementInit,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Statement,
}

pub enum ForStatementInit {
    VariableDeclaration,
    Expression,
}

pub struct ForInStatement {
    left: ForInStatementLeft,
    right: Expression,
    body: Statement,
}

pub enum ForInStatementLeft {
    Directive,
    Statement,
}
