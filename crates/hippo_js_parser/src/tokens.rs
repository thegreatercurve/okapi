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

    // Strict mode future reserved words.
    Let,
    Static,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,

    // Appear as keywords within certain syntactic productions, at places where Identifier is not allowed.
    As,
    Async,
    From,
    Get,
    Of,
    Set,
    Target,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    String(String),
    Number { raw: String, value: f64 },
    Boolean(bool),
    BigInt(String),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize, value: TokenValue) -> Self {
        Self {
            kind,
            start,
            end,
            value,
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
    BigIntLiteral,
    RegularExpressionLiteral,

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
    AdditionAssignment,
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

    // Template Literals
    NoSubstitutionTemplate,
    TemplateHead,
    TemplateMiddle,
    TemplateTail,

    // Utility
    Illegal,
    EOF,
}

impl TokenKind {
    pub(crate) fn is_logical_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::NullishCoalescing | TokenKind::LogicalAnd | TokenKind::LogicalOr
        )
    }

    // https://tc39.es/ecma262/#prod-AssignmentOperator
    pub(crate) fn is_assignment_operator(&self) -> bool {
        matches!(
            self,
            |TokenKind::MultiplyAssignment| TokenKind::DivisionAssignment
                | TokenKind::ModulusAssignment
                | TokenKind::AdditionAssignment
                | TokenKind::MinusAssignment
                | TokenKind::LeftShiftAssignment
                | TokenKind::RightShiftAssignment
                | TokenKind::UnsignedRightShiftAssignment
                | TokenKind::BitwiseAndAssignment
                | TokenKind::BitwiseOrAssignment
                | TokenKind::BitwiseXorAssignment
                | TokenKind::ExponentiationAssignment
            
        )
    }

    pub(crate) fn is_binary_operator(&self) -> bool {
        matches!(self, |TokenKind::BitwiseOr| TokenKind::BitwiseXor
            | TokenKind::BitwiseAnd
            | TokenKind::Equal
            | TokenKind::NotEqual
            | TokenKind::StrictEqual
            | TokenKind::StrictNotEqual
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThanOrEqual
            | TokenKind::Keyword(KeywordKind::Instanceof)
            | TokenKind::Keyword(KeywordKind::In)
            | TokenKind::LeftShift
            | TokenKind::RightShift
            | TokenKind::UnsignedRightShift
            | TokenKind::Addition
            | TokenKind::Subtraction
            | TokenKind::Multiplication
            | TokenKind::Division
            | TokenKind::Modulus
            | TokenKind::Exponentiation)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Addition => write!(f, "Addition"),
            TokenKind::Keyword(_) => write!(f, "Keyword"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::StringLiteral => write!(f, "StringLiteral"),
            TokenKind::NumberLiteral => write!(f, "NumberLiteral"),
            TokenKind::BigIntLiteral => write!(f, "BigIntLiteral"),
            TokenKind::RegularExpressionLiteral => write!(f, "RegularExpressionLiteral"),
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
            TokenKind::AdditionAssignment => write!(f, "AdditionAssignment"),
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
            TokenKind::NoSubstitutionTemplate => write!(f, "NoSubstitutionTemplate"),
            TokenKind::TemplateHead => write!(f, "TemplateHead"),
            TokenKind::TemplateMiddle => write!(f, "TemplateMiddle"),
            TokenKind::TemplateTail => write!(f, "TemplateTail"),
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}
