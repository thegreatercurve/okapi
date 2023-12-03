use crate::{ExpressionData, Identifier, Pattern};

pub struct FunctionDeclaration {
    id: Identifier,
}

pub struct VariableDeclaration {
    declarations: Vec<VariableDeclarator>,
    kind: VariableKind,
}

pub struct VariableDeclarator {
    id: Pattern,
    init: Option<ExpressionData>,
}

enum VariableKind {
    Var,
    Let,
    Const,
}
