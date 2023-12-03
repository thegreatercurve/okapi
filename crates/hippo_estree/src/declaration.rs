use crate::{ExpressionData, Identifier, Pattern};

#[derive(Debug, PartialEq)]
pub enum DeclarationData {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    id: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<ExpressionData>,
}

#[derive(Debug, PartialEq)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}
