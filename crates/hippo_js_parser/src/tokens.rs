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
    Identifier(String),

    // Literals
    String,
    Number,

    // Punctuators
    LeftCurlyBrace,
    RightCurlyBrace,
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

    // Utility
    Illegal,
    EOF,
}

// impl TokenType {
//     pub fn to_str(&self) -> &str {
//         match self {
//             TokenType::LeftCurlyBrace => "{",
//             TokenType::RightCurlyBrace => "}",
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
//             _ => "",
//         }
//     }
// }
