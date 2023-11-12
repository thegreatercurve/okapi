#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    Constant,
    Let,
    Variable,

    // Identifiers
    Name,

    // Literals
    String,
    Number,

    // Operators
    Assign,
    Bang,
    Plus,
    Minus,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

    // Delimiters
    SemiColon,

    // Utility
    Illegal,
    EOF,
}
