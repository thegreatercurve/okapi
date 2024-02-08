use serde::Serialize;

pub use expression_and_pattern::*;
pub use node::*;
pub use statement_and_declarations::*;

mod expression_and_pattern;
mod node;
mod statement_and_declarations;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<StatementListemItem>,
    pub source_type: Vec<ProgramSourceTypes>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramSourceTypes {
    Script,
    Module,
}
