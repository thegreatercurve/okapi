use crate::{BindingKind, Expression, Node, PropertyKind};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Pattern {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "Property")]
pub struct AssignmentProperty {
    #[serde(flatten)]
    pub node: Node,
    pub value: Pattern,
    pub kind: PropertyKind,
    pub method: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectPatternProperties>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ObjectPatternProperties {
    AssignmentProperty(AssignmentProperty),
    RestElement(RestElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<Option<Pattern>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: BindingKind,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub node: Node,
    pub left: Pattern,
    pub right: Expression,
}
