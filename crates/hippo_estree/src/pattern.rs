use crate::{BaseNode, ExpressionData};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Pattern {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "Property")]
pub struct AssignmentProperty {
    #[serde(flatten)]
    pub node: BaseNode,
    pub value: Pattern,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub node: BaseNode,
    pub properties: Vec<AssignmentProperty>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub node: BaseNode,
    pub elements: Vec<Option<Pattern>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub argument: Pattern,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub node: BaseNode,
    pub left: Pattern,
    pub right: ExpressionData,
}
