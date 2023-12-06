use serde::Serialize;

pub use declaration::*;
pub use expression::*;
pub use node::*;
pub use pattern::*;
pub use statement::*;

mod declaration;
mod expression;
mod node;
mod pattern;
mod statement;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Identifier {
    #[serde(flatten)]
    pub node: Node,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Literal {
    #[serde(flatten)]
    pub node: Node,
    pub value: LiteralValue,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegExpLiteral),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RegExpLiteral {
    pub regex: Regex,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Regex {
    #[serde(flatten)]
    pub node: Node,
    pub pattern: String,
    pub flags: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    #[serde(rename = "camelCase")]
    pub source_type: ProgramSourceTypes,
    pub body: Vec<ProgramBody>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ProgramSourceTypes {
    Script,
    Module,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ProgramBody {
    Directive,
    Statement(StatementData),
    ImportOrExportDeclaration(ImportOrExportDeclaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Function {
    #[serde(flatten)]
    pub generator: bool,
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
}
