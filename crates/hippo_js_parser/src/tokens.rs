// 12.7.2 Keywords and Reserved Words
// https://tc39.es/ecma262/#sec-keywords-and-reserved-words
#[derive(Clone, Debug, PartialEq)]
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
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub value: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            start,
            end,
            value: None,
        }
    }

    pub fn default(kind: TokenKind) -> Self {
        Self {
            kind,
            start: 0,
            end: 0,
            value: None,
        }
    }

    pub fn identifier(value: String, start: usize, end: usize) -> Self {
        Self {
            kind: TokenKind::Identifier,
            start,
            end,
            value: Some(value),
        }
    }

    pub fn default_identifier(value: String) -> Self {
        Self {
            kind: TokenKind::Identifier,
            start: 0,
            end: 0,
            value: Some(value),
        }
    }

    pub fn default_string_literal(value: String) -> Self {
        Self {
            kind: TokenKind::StringLiteral,
            start: 0,
            end: 0,
            value: Some(value),
        }
    }

    pub fn string_literal(value: String, start: usize, end: usize) -> Self {
        Self {
            kind: TokenKind::StringLiteral,
            start,
            end,
            value: Some(value),
        }
    }

    pub fn default_number_literal(value: String) -> Self {
        Self {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 0,
            value: Some(value),
        }
    }

    pub fn number_literal(value: String, start: usize, end: usize) -> Self {
        Self {
            kind: TokenKind::NumberLiteral,
            start,
            end,
            value: Some(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Keywords or Identifiers
    Keyword(KeywordKind),
    Identifier,

    // Literals
    StringLiteral,
    NumberLiteral,

    // Punctuators
    OptionalChaining,
    LeftCurlyBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    Dot,
    Ellipsis,
    Semicolon,
    Comma,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    Addition,
    Subtraction,
    Multiplication,
    Modulus,
    Exponentiation,
    Increment,
    Decrement,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalNot,
    BitwiseNot,
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
    QuestionMark,
    Colon,
    Assignment,
    PlusAssignment,
    MinusAssignment,
    MultiplyAssignment,
    ModulusAssignment,
    ExponentiationAssignment,
    LeftShiftAssignment,
    RightShiftAssignment,
    UnsignedRightShiftAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    BitwiseXorAssignment,
    LogicalAndAssignment,
    LogicalOrAssignment,
    NullishCoalescingAssignment,
    ArrowFunction,
    Division,
    DivisionAssignment,
    RightCurlyBrace,

    // Utility
    Illegal,
    EOF,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Addition => write!(f, "Addition"),
            TokenKind::Keyword(_) => write!(f, "Keyword"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::StringLiteral => write!(f, "StringLiteral"),
            TokenKind::NumberLiteral => write!(f, "NumberLiteral"),
            TokenKind::OptionalChaining => write!(f, "OptionalChaining"),
            TokenKind::LeftCurlyBrace => write!(f, "LeftCurlyBrace"),
            TokenKind::LeftParenthesis => write!(f, "LeftParenthesis"),
            TokenKind::RightParenthesis => write!(f, "RightParenthesis"),
            TokenKind::LeftSquareBracket => write!(f, "LeftSquareBracket"),
            TokenKind::RightSquareBracket => write!(f, "RightSquareBracket"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Ellipsis => write!(f, "Ellipsis"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::LessThan => write!(f, "LessThan"),
            TokenKind::GreaterThan => write!(f, "GreaterThan"),
            TokenKind::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            TokenKind::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::NotEqual => write!(f, "NotEqual"),
            TokenKind::StrictEqual => write!(f, "StrictEqual"),
            TokenKind::StrictNotEqual => write!(f, "StrictNotEqual"),
            TokenKind::Subtraction => write!(f, "Subtraction"),
            TokenKind::Multiplication => write!(f, "Multiplication"),
            TokenKind::Modulus => write!(f, "Modulus"),
            TokenKind::Exponentiation => write!(f, "Exponentiation"),
            TokenKind::Increment => write!(f, "Increment"),
            TokenKind::Decrement => write!(f, "Decrement"),
            TokenKind::LeftShift => write!(f, "LeftShift"),
            TokenKind::RightShift => write!(f, "RightShift"),
            TokenKind::UnsignedRightShift => write!(f, "UnsignedRightShift"),
            TokenKind::BitwiseAnd => write!(f, "BitwiseAnd"),
            TokenKind::BitwiseOr => write!(f, "BitwiseOr"),
            TokenKind::BitwiseXor => write!(f, "BitwiseXor"),
            TokenKind::LogicalNot => write!(f, "LogicalNot"),
            TokenKind::BitwiseNot => write!(f, "BitwiseNot"),
            TokenKind::LogicalAnd => write!(f, "LogicalAnd"),
            TokenKind::LogicalOr => write!(f, "LogicalOr"),
            TokenKind::NullishCoalescing => write!(f, "NullishCoalescing"),
            TokenKind::QuestionMark => write!(f, "QuestionMark"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Assignment => write!(f, "Assignment"),
            TokenKind::PlusAssignment => write!(f, "PlusAssignment"),
            TokenKind::MinusAssignment => write!(f, "MinusAssignment"),
            TokenKind::MultiplyAssignment => write!(f, "MultiplyAssignment"),
            TokenKind::ModulusAssignment => write!(f, "ModulusAssignment"),
            TokenKind::ExponentiationAssignment => write!(f, "ExponentiationAssignment"),
            TokenKind::LeftShiftAssignment => write!(f, "LeftShiftAssignment"),
            TokenKind::RightShiftAssignment => write!(f, "RightShiftAssignment"),
            TokenKind::UnsignedRightShiftAssignment => write!(f, "UnsignedRightShiftAssignment"),
            TokenKind::BitwiseAndAssignment => write!(f, "BitwiseAndAssignment"),
            TokenKind::BitwiseOrAssignment => write!(f, "BitwiseOrAssignment"),
            TokenKind::BitwiseXorAssignment => write!(f, "BitwiseXorAssignment"),
            TokenKind::LogicalAndAssignment => write!(f, "LogicalAndAssignment"),
            TokenKind::LogicalOrAssignment => write!(f, "LogicalOrAssignment"),
            TokenKind::NullishCoalescingAssignment => write!(f, "NullishCoalescingAssignment"),
            TokenKind::ArrowFunction => write!(f, "ArrowFunction"),
            TokenKind::Division => write!(f, "Division"),
            TokenKind::DivisionAssignment => write!(f, "DivisionAssignment"),
            TokenKind::RightCurlyBrace => write!(f, "RightCurlyBrace"),
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

impl TokenKind {}

// impl TokenType {
//     pub fn to_str(&self) -> &str {
//         match self {
//             TokenType::OptionalChaining => "?.",
//             TokenType::LeftCurlyBrace => "{",
//             TokenType::LeftParenthesis => "(",
//             TokenType::RightParenthesis => ")",
//             TokenType::LeftSquareBracket => "[",
//             TokenType::RightSquareBracket => "]",
//             TokenType::Dot => ".",
//             TokenType::Ellipsis => "...",
//             TokenType::Semicolon => ";",
//             TokenType::Comma => ",",
//             TokenType::LessThan => "<",
//             TokenType::GreaterThan => ">",
//             TokenType::LessThanOrEqual => "<=",
//             TokenType::GreaterThanOrEqual => ">=",
//             TokenType::Equal => "==",
//             TokenType::NotEqual => "!=",
//             TokenType::StrictEqual => "===",
//             TokenType::StrictNotEqual => "!==",
//             TokenType::Addition => "+",
//             TokenType::Subtraction => "-",
//             TokenType::Multiplication => "*",
//             TokenType::Modulus => "%",
//             TokenType::Exponentiation => "**",
//             TokenType::Increment => "++",
//             TokenType::Decrement => "--",
//             TokenType::LeftShift => "<<",
//             TokenType::RightShift => ">>",
//             TokenType::UnsignedRightShift => ">>>",
//             TokenType::BitwiseAnd => "&",
//             TokenType::BitwiseOr => "|",
//             TokenType::BitwiseXor => "^",
//             TokenType::LogicalNot => "!",
//             TokenType::BitwiseNot => "~",
//             TokenType::LogicalAnd => "&&",
//             TokenType::LogicalOr => "||",
//             TokenType::NullishCoalescing => "??",
//             TokenType::QuestionMark => "?",
//             TokenType::Colon => ":",
//             TokenType::Assignment => "=",
//             TokenType::PlusAssignment => "+=",
//             TokenType::MinusAssignment => "-=",
//             TokenType::MultiplyAssignment => "*=",
//             TokenType::ModulusAssignment => "%=",
//             TokenType::ExponentiationAssignment => "**=",
//             TokenType::LeftShiftAssignment => "<<=",
//             TokenType::RightShiftAssignment => ">>=",
//             TokenType::UnsignedRightShiftAssignment => ">>>=",
//             TokenType::BitwiseAndAssignment => "&=",
//             TokenType::BitwiseOrAssignment => "|=",
//             TokenType::BitwiseXorAssignment => "^=",
//             TokenType::LogicalAndAssignment => "&&=",
//             TokenType::LogicalOrAssignment => "||=",
//             TokenType::NullishCoalescingAssignment => "??=",
//             TokenType::ArrowFunction => "=>",
//             TokenType::Division =>: "/",
//             TokenType::DivisionAssignment => "/=",
//             TokenType::RightCurlyBrace =>": "}"
//             _ => "",
//         }
//     }
// }
