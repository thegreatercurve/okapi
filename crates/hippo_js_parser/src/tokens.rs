#[derive(Debug, PartialEq)]
pub enum KeywordValue {
    Const,
    Let,
    Var,
    Illegal,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords or Identifiers
    Keyword(KeywordValue),
    Identifier,

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
