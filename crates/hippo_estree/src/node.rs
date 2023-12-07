use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "Node")]
pub struct BaseNode {
    pub loc: Option<SourceLocation>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct SourceLocation {
    pub source: Option<String>,
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Position {
    pub line: usize,   // >= 1
    pub column: usize, // >= 0
}
