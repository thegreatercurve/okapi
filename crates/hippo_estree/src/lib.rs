pub use declaration::*;
pub use expression::*;
pub use pattern::*;
pub use statement::*;

mod declaration;
mod expression;
mod pattern;
mod statement;

pub struct Identifier {
    name: String,
}

pub struct Literal {
    value: LiteralValue,
}

enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegExpLiteral),
}

pub struct RegExpLiteral {
    regex: Regex,
}

struct Regex {
    pattern: String,
    flags: String,
}

pub struct Program {
    body: Vec<ProgramBody>,
}

enum ProgramBody {
    Directive,
    Statement,
}

pub struct Function {
    id: Option<Identifier>,
    params: Vec<Pattern>,
    body: FunctionBody,
}
