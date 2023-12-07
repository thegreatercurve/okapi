use crate::{BaseNode, ExpressionData};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateLiteral {
    #[serde(flatten)]
    pub node: BaseNode,
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Box<ExpressionData>>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TaggedTemplateExpression {
    #[serde(flatten)]
    pub node: BaseNode,
    pub tag: Box<ExpressionData>,
    pub quasi: TemplateLiteral,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct TemplateElement {
    #[serde(flatten)]
    pub node: BaseNode,
    pub tail: bool,
    pub value: TemplateElementValue,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TemplateElementValue {
    pub cooked: Option<String>,
    pub raw: String,
}
