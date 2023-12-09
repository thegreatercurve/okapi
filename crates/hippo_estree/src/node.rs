use serde::Serialize;

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename = "Node")]
pub struct Node {
    pub loc: SourceLocation,
}

impl Node {
    pub fn new(start_column: usize, end_column: usize) -> Self {
        Self {
            loc: SourceLocation {
                start: start_column,
                end: end_column,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct SourceLocation {
    pub start: usize,
    pub end: usize,
}

// #[derive(Debug, PartialEq, Serialize)]
// #[serde(tag = "type")]
// pub struct SourceLocation {
//     pub source: Option<String>,
//     pub start: Position,
//     pub end: Position,
// }

// #[derive(Debug, PartialEq, Serialize)]
// #[serde(tag = "type")]
// pub struct Position {
//     pub line: usize,   // >= 1
//     pub column: usize, // >= 0
// }
