#![allow(dead_code)]
pub use declaration::*;
pub use expression::*;
pub use pattern::*;
pub use statement::*;

mod declaration;
mod expression;
mod pattern;
mod statement;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegExpLiteral),
}

#[derive(Debug, PartialEq)]
pub struct RegExpLiteral {
    regex: Regex,
}

#[derive(Debug, PartialEq)]
pub struct Regex {
    pub pattern: String,
    pub flags: String,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<ProgramBody>,
}

#[derive(Debug, PartialEq)]
pub enum ProgramBody {
    Directive(),
    Statement(StatementData),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
}
