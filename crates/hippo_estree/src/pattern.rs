use crate::Identifier;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Identifier(Identifier),
}
