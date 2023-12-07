use crate::{BaseNode, ExpressionData, FunctionExpression, Identifier};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Class {
    #[serde(flatten)]
    pub node: BaseNode,
    pub id: Option<Identifier>,
    #[serde(rename = "camelCase")]
    pub super_class: Option<ExpressionData>,
    pub body: ClassBody,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassBody {
    #[serde(flatten)]
    pub node: BaseNode,
    pub body: [MethodDefinition],
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MethodDefinition {
    #[serde(flatten)]
    pub node: BaseNode,
    pub key: ExpressionData,
    pub value: FunctionExpression,
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
    pub node: BaseNode,
    pub id: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ClassExpression {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct MetaProperty {
    #[serde(flatten)]
    pub node: BaseNode,
    pub meta: Identifier,
    pub property: Identifier,
}
