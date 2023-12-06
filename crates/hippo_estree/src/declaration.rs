use crate::{ExpressionData, Identifier, Node, Pattern};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum DeclarationData {
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub node: Node,
    pub id: Pattern,
    pub init: Option<ExpressionData>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}
