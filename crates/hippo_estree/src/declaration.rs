use crate::{Expression, Identifier, Node};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Declaration {
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
    pub id: Identifier,
    pub init: Option<Expression>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}
