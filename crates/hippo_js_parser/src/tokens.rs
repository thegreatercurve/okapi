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

impl std::fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KeywordKind::Await => write!(f, "await"),
            KeywordKind::Break => write!(f, "break"),
            KeywordKind::Case => write!(f, "case"),
            KeywordKind::Catch => write!(f, "catch"),
            KeywordKind::Class => write!(f, "class"),
            KeywordKind::Const => write!(f, "const"),
            KeywordKind::Continue => write!(f, "continue"),
            KeywordKind::Debugger => write!(f, "debugger"),
            KeywordKind::Default => write!(f, "default"),
            KeywordKind::Delete => write!(f, "delete"),
            KeywordKind::Do => write!(f, "do"),
            KeywordKind::Else => write!(f, "else"),
            KeywordKind::Enum => write!(f, "enum"),
            KeywordKind::Export => write!(f, "export"),
            KeywordKind::Extends => write!(f, "extends"),
            KeywordKind::False => write!(f, "false"),
            KeywordKind::Finally => write!(f, "finally"),
            KeywordKind::For => write!(f, "for"),
            KeywordKind::Function => write!(f, "function"),
            KeywordKind::If => write!(f, "if"),
            KeywordKind::Import => write!(f, "import"),
            KeywordKind::In => write!(f, "in"),
            KeywordKind::Instanceof => write!(f, "instanceof"),
            KeywordKind::New => write!(f, "new"),
            KeywordKind::Null => write!(f, "null"),
            KeywordKind::Return => write!(f, "return"),
            KeywordKind::Super => write!(f, "super"),
            KeywordKind::Switch => write!(f, "switch"),
            KeywordKind::This => write!(f, "this"),
            KeywordKind::Throw => write!(f, "throw"),
            KeywordKind::True => write!(f, "true"),
            KeywordKind::Try => write!(f, "try"),
            KeywordKind::Typeof => write!(f, "typeof"),
            KeywordKind::Var => write!(f, "var"),
            KeywordKind::Void => write!(f, "void"),
            KeywordKind::While => write!(f, "while"),
            KeywordKind::With => write!(f, "with"),
            KeywordKind::Yield => write!(f, "yield"),
            KeywordKind::Let => write!(f, "let"),
            KeywordKind::Static => write!(f, "static"),
            KeywordKind::Implements => write!(f, "implements"),
            KeywordKind::Interface => write!(f, "interface"),
            KeywordKind::Package => write!(f, "package"),
            KeywordKind::Private => write!(f, "private"),
            KeywordKind::Protected => write!(f, "protected"),
            KeywordKind::Public => write!(f, "public"),
            KeywordKind::As => write!(f, "as"),
            KeywordKind::Async => write!(f, "async"),
            KeywordKind::From => write!(f, "from"),
            KeywordKind::Get => write!(f, "get"),
            KeywordKind::Of => write!(f, "of"),
            KeywordKind::Set => write!(f, "set"),
            KeywordKind::Target => write!(f, "target"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    String { raw: String, value: String },
    Number { raw: String, value: f64 },
    Boolean(bool),
    BigInt(String),
    RegularExpression { pattern: String, flags: String },
    Template { raw: String, cooked: String },
    Null,
}

impl From<TokenValue> for String {
    fn from(value: TokenValue) -> Self {
        match value {
            TokenValue::String { raw, .. } => raw,
            TokenValue::Number { raw, .. } => raw,
            TokenValue::Boolean(b) => b.to_string(),
            TokenValue::BigInt(s) => s,
            TokenValue::RegularExpression { pattern, .. } => pattern,
            TokenValue::Template { raw, .. } => raw,
            TokenValue::Null => "null".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub value: TokenValue,
    pub line_terminator: bool,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
        value: TokenValue,
    ) -> Self {
        Self {
            kind,
            start,
            end,
            line,
            column,
            value,
            line_terminator: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Keywords or Identifiers
    Keyword(KeywordKind),
    Identifier,
    PrivateIdentifier,

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
    TemplateNoSubstitution,
    TemplateHead,
    TemplateMiddle,
    TemplateTail,

    // Utility
    Illegal,
    EOF,
}

impl TokenKind {
    // 12.7 Names and Keywords
    // https://tc39.es/ecma262/#prod-PrivateIdentifier
    pub(crate) fn is_private_identifier(&self) -> bool {
        matches!(self, TokenKind::PrivateIdentifier)
    }

    pub(crate) fn is_identifier_name(&self) -> bool {
        matches!(self, TokenKind::Identifier | TokenKind::Keyword(_))
    }

    // 12.9.6 Template Literal Lexical Components
    pub(crate) fn is_template_start(&self) -> bool {
        matches!(
            self,
            TokenKind::TemplateNoSubstitution | TokenKind::TemplateHead
        )
    }

    pub(crate) fn is_template_part(&self) -> bool {
        matches!(
            self,
            TokenKind::TemplateNoSubstitution
                | TokenKind::TemplateHead
                | TokenKind::TemplateMiddle
                | TokenKind::TemplateTail
        )
    }

    // 13.1 Identifiers
    // https://tc39.es/ecma262/#prod-IdentifierReference
    pub(crate) fn is_identifier_reference(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Yield) | TokenKind::Keyword(KeywordKind::Await)
        ) || self.is_identifier()
    }

    // https://tc39.es/ecma262/#prod-ReservedWord
    pub(crate) fn is_reserved_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Await)
                | TokenKind::Keyword(KeywordKind::Break)
                | TokenKind::Keyword(KeywordKind::Case)
                | TokenKind::Keyword(KeywordKind::Catch)
                | TokenKind::Keyword(KeywordKind::Class)
                | TokenKind::Keyword(KeywordKind::Const)
                | TokenKind::Keyword(KeywordKind::Continue)
                | TokenKind::Keyword(KeywordKind::Debugger)
                | TokenKind::Keyword(KeywordKind::Default)
                | TokenKind::Keyword(KeywordKind::Delete)
                | TokenKind::Keyword(KeywordKind::Do)
                | TokenKind::Keyword(KeywordKind::Else)
                | TokenKind::Keyword(KeywordKind::Enum)
                | TokenKind::Keyword(KeywordKind::Export)
                | TokenKind::Keyword(KeywordKind::Extends)
                | TokenKind::Keyword(KeywordKind::False)
                | TokenKind::Keyword(KeywordKind::Finally)
                | TokenKind::Keyword(KeywordKind::For)
                | TokenKind::Keyword(KeywordKind::Function)
                | TokenKind::Keyword(KeywordKind::If)
                | TokenKind::Keyword(KeywordKind::Import)
                | TokenKind::Keyword(KeywordKind::In)
                | TokenKind::Keyword(KeywordKind::Instanceof)
                | TokenKind::Keyword(KeywordKind::New)
                | TokenKind::Keyword(KeywordKind::Null)
                | TokenKind::Keyword(KeywordKind::Return)
                | TokenKind::Keyword(KeywordKind::Super)
                | TokenKind::Keyword(KeywordKind::Switch)
                | TokenKind::Keyword(KeywordKind::This)
                | TokenKind::Keyword(KeywordKind::Throw)
                | TokenKind::Keyword(KeywordKind::True)
                | TokenKind::Keyword(KeywordKind::Try)
                | TokenKind::Keyword(KeywordKind::Typeof)
                | TokenKind::Keyword(KeywordKind::Var)
                | TokenKind::Keyword(KeywordKind::Void)
                | TokenKind::Keyword(KeywordKind::While)
                | TokenKind::Keyword(KeywordKind::With)
                | TokenKind::Keyword(KeywordKind::Yield)
        )
    }

    // 13.1 Identifiers
    // https://tc39.es/ecma262/#prod-BindingIdentifier
    pub(crate) fn is_binding_identifier(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Yield) | TokenKind::Keyword(KeywordKind::Await)
        ) || self.is_identifier()
    }

    // https://tc39.es/ecma262/#prod-Identifier
    pub(crate) fn is_identifier(&self) -> bool {
        self.is_identifier_name() && !self.is_reserved_keyword()
    }

    // 13.2.5 Property Accessors
    // https://tc39.es/ecma262/#prod-PropertyName
    pub(crate) fn is_property_name(&self) -> bool {
        matches!(
            self,
            TokenKind::StringLiteral | TokenKind::NumberLiteral | TokenKind::LeftSquareBracket
        ) || self.is_identifier_name()
    }

    // 13.4 Update Expressions
    // https://tc39.es/ecma262/#prod-UpdateExpression
    pub(crate) fn is_update_operator(&self) -> bool {
        matches!(self, TokenKind::Increment | TokenKind::Decrement)
    }

    //  13.5 Unary Operators
    // https://tc39.es/ecma262/#prod-UnaryExpression
    pub fn is_unary_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Delete)
                | TokenKind::Keyword(KeywordKind::Void)
                | TokenKind::Keyword(KeywordKind::Typeof)
                | TokenKind::Addition
                | TokenKind::Subtraction
                | TokenKind::BitwiseNot
                | TokenKind::LogicalNot
        )
    }

    // 13.6 Exponentiation Operators
    // https://tc39.es/ecma262/#prod-ExponentiationExpression

    // 13.7 Multiplicative Operators
    // https://tc39.es/ecma262/#prod-MultiplicativeExpression

    // 13.8 Additive Operators
    // https://tc39.es/ecma262/#prod-AdditiveExpression

    // 13.9 Bitwise Shift Operators
    // https://tc39.es/ecma262/#prod-ShiftExpression

    // 13.10 Relational Operators
    // https://tc39.es/ecma262/#prod-RelationalExpression

    // 13.11 Equality Operators
    // https://tc39.es/ecma262/#prod-EqualityExpression

    // 13.12 Binary Bitwise Operators
    // https://tc39.es/ecma262/#prod-BitwiseANDExpression
    // https://tc39.es/ecma262/#prod-BitwiseXORExpression
    // https://tc39.es/ecma262/#prod-BitwiseORExpression

    // 13.13 Binary Logical Operators
    // https://tc39.es/ecma262/#prod-LogicalANDExpression
    // https://tc39.es/ecma262/#prod-LogicalORExpression
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

    // 13.13 Binary Logical Operators
    // https://tc39.es/ecma262/#prod-LogicalORExpression
    pub(crate) fn is_logical_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::NullishCoalescing | TokenKind::LogicalAnd | TokenKind::LogicalOr
        )
    }

    // 13.15 Assignment Operators
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
            | TokenKind::Assignment
            // ES2021
            | TokenKind::LogicalAndAssignment
            | TokenKind::LogicalOrAssignment
            | TokenKind::NullishCoalescingAssignment
        )
    }

    // 13.15.5 Destructuring Assignment
    // https://tc39.es/ecma262/#prod-DestructuringAssignment
    pub(crate) fn is_assignment_pattern_start(&self) -> bool {
        matches!(
            self,
            TokenKind::LeftSquareBracket | TokenKind::LeftCurlyBrace
        )
    }

    // 14 ECMAScript Language: Statements and Declarations
    // https://tc39.es/ecma262/#prod-Declaration
    pub(crate) fn is_declaration_start(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Function) | TokenKind::Keyword(KeywordKind::Class)
        ) | self.is_lexical_declaration_start()
    }

    // 14 ECMAScript Language: Statements and Declarations
    // https://tc39.es/ecma262/#prod-HoistableDeclaration
    pub(crate) fn is_hoistable_declaration_start(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Function) | TokenKind::Keyword(KeywordKind::Async)
        )
    }

    // 14.3.1 Let and Const Declarations
    // https://tc39.es/ecma262/#prod-LexicalDeclaration
    pub(crate) fn is_lexical_declaration_start(&self) -> bool {
        matches!(
            self,
            TokenKind::Keyword(KeywordKind::Let) | TokenKind::Keyword(KeywordKind::Const)
        )
    }

    // 14.3.2 Variable Statement
    // https://tc39.es/ecma262/#prod-VariableStatement
    pub(crate) fn is_variable_declaration_start(&self) -> bool {
        matches!(self, TokenKind::Keyword(KeywordKind::Var))
    }

    // 14.3.3 Destructuring Binding Patterns
    // https://tc39.es/ecma262/#prod-BindingPattern
    pub(crate) fn is_binding_pattern_start(&self) -> bool {
        matches!(
            self,
            TokenKind::LeftSquareBracket | TokenKind::LeftCurlyBrace
        )
    }

    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassElementName
    pub(crate) fn is_class_declaration_start(&self) -> bool {
        matches!(self, TokenKind::Keyword(KeywordKind::Class))
    }

    pub(crate) fn is_class_element_name(&self) -> bool {
        matches!(self, TokenKind::PrivateIdentifier) || self.is_property_name()
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Addition => write!(f, "Addition"),
            TokenKind::Keyword(keyword) => keyword.to_string().fmt(f),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::PrivateIdentifier => write!(f, "PrivateIdentifier"),
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
            TokenKind::TemplateNoSubstitution => write!(f, "NoSubstitutionTemplate"),
            TokenKind::TemplateHead => write!(f, "TemplateHead"),
            TokenKind::TemplateMiddle => write!(f, "TemplateMiddle"),
            TokenKind::TemplateTail => write!(f, "TemplateTail"),
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}
