use crate::Node;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Pattern {
    #[serde(flatten)]
    pub node: Node,
}
