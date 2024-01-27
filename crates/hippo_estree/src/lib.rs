use serde::Serialize;

pub use classes::*;
pub use declaration::*;
pub use expression::*;
pub use modules::*;
pub use node::*;
pub use pattern::*;
pub use statement::*;
pub use template_literals::*;

mod classes;
mod declaration;
mod expression;
mod modules;
mod node;
mod pattern;
mod statement;
mod template_literals;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Identifier {
    #[serde(flatten)]
    pub node: Node,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Literal {
    #[serde(flatten)]
    pub node: Node,
    pub value: LiteralValue,
    pub raw: LiteralValue,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegExpLiteral),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RegExpLiteral {
    pub regex: Regex,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Regex {
    #[serde(flatten)]
    pub node: Node,
    pub pattern: String,
    pub flags: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<ProgramBody>,
    pub source_type: ProgramSourceTypes,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramSourceTypes {
    Script,
    Module,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProgramBody {
    Directive(Directive),
    Statement(Statement),
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
    #[serde(alias = "async")]
    pub asynchronous: bool,
}
