use crate::{Expression, Identifier, Pattern};

pub struct FunctionDeclaration {
    id: Identifier,
}

pub struct VariableDeclaration {
    declarations: Vec<VariableDeclarator>,
    kind: VariableKind,
}

pub struct VariableDeclarator {
    id: Pattern,
    init: Option<Expression>,
}

enum VariableKind {
    Var,
    Let,
    Const,
}
