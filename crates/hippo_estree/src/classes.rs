use crate::{Expression, Identifier, Node};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Class {
    #[serde(flatten)]
    pub node: Node,
    pub id: Option<Identifier>,
    #[serde(rename = "camelCase")]
    pub super_class: Option<Expression>,
    pub body: ClassBody,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassBody {
    #[serde(flatten)]
    pub node: Node,
    pub body: [MethodDefinition],
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MethodDefinition {
    #[serde(flatten)]
    pub node: Node,
    pub key: Expression,
    // TODO Fix the types. Should be a function expression
    pub value: Expression,
    pub kind: MethodDefinitionKind,
    pub computed: bool,
    #[serde(alias = "static")]
    pub statc: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum MethodDefinitionKind {
    Constructor,
    Method,
    Get,
    Set,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub id: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassExpression {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MetaProperty {
    #[serde(flatten)]
    pub node: Node,
    pub meta: Identifier,
    pub property: Identifier,
}
