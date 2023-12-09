use crate::{Expression, Node};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateLiteral {
    #[serde(flatten)]
    pub node: Node,
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Box<Expression>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TaggedTemplateExpression {
    #[serde(flatten)]
    pub node: Node,
    pub tag: Box<Expression>,
    pub quasi: TemplateLiteral,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateElement {
    #[serde(flatten)]
    pub node: Node,
    pub tail: bool,
    pub value: TemplateElementValue,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TemplateElementValue {
    pub cooked: Option<String>,
    pub raw: String,
}
