use crate::{Expression, Node};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Pattern {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "Property")]
pub struct AssignmentProperty {
    #[serde(flatten)]
    pub node: Node,
    pub value: Pattern,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectPatternProperties>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ObjectPatternProperties {
    AssignmentProperty(AssignmentProperty),
    RestElement(RestElement),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<Option<Pattern>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Pattern,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub node: Node,
    pub left: Pattern,
    pub right: Expression,
}
