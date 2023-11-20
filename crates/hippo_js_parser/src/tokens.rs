// 12.7.2 Keywords and Reserved Words
// https://tc39.es/ecma262/#sec-keywords-and-reserved-words
#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,

    // Strict mode future reserved words
    Let,
    Static,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords or Identifiers
    Keyword(KeywordKind),
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
