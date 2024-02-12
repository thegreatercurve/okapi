use serde::Serialize;

use crate::{BlockStatement, ClassBody, Expression, Identifier, Node, Pattern};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Declaration {
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    #[serde(rename = "superClass")]
    pub super_class: Option<Expression>,
    pub body: ClassBody,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    pub expression: bool,
    pub generator: bool,
    #[serde(rename = "async")]
    pub asynchronous: bool,
    pub params: Vec<Pattern>,
    pub body: BlockStatement,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub node: Node,
    pub id: Pattern,
    pub init: Option<Expression>,
}
