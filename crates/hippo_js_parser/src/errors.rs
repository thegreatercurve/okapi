use crate::{TokenKind, TokenValue};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    SyntaxError,

    // Identifiers
    InvalidIdentifierCharacter,

    // String literals
    UnterminatedStringLiteral,
    InvalidEscapeSequence,
    InvalidEscapeSequenceCannotBeFormatted,
    InvalidHexadecimalEscapeSequence,
    InvalidUnicodeEscapeSequence,
    InvalidUnicodeCodePointEscapeSequence,
    InvalidLegacyOctalEscapeSequence,
    InvalidLegacyOctalEscapeSequenceNotAllowedInStrictMode,

    // Number literals
    InvalidDecimalLiteral,
    InvalidNonDecimalBinaryNumberLiteral,
    InvalidNonDecimalOctalNumberLiteral,
    InvalidNonDecimalHexadecimalNumberLiteral,
    InvalidLegacyOctalNumberLiteral,
    InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode,
    InvalidExponentPartNumberLiteral,

    InvalidNumericSeparatorAtSibling,
    InvalidNumericSeparatorAtEnd,

    // BigInt literals
    InvalidDecimalBigIntegerLiteral,

    // Regex literals
    UnterminatedRegExLiteral,
    InvalidRegexLiteralFirstChar,
    InvalidRegexBackslashSequence,
    InvalidRegexExpressionClass,

    // Object expressions
    InvalidPropertyKey,

    // Assignment patterns
    InvalidLeftHandSideExpression,

    // For statement
    InvalidAwaitForInStatement,

    // Conversion error
    InvalidExpressionToAssignmentExpressionConversion,
    InvalidExpressionToChainElementConversion,
    InvalidMethodDefinitionToPropertyConversion,
    InvalidExpressionToPatternConversion,
    InvalidExpressionToArrayPatternElementConversion,
    InvalidExpressionToLiteralConversion,

    UnexpectedToken(TokenKind, usize, usize),
    UnexpectedTokenValue(TokenKind, TokenValue, usize, usize),
    UnexpectedLineTerminator,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(token_kind, line, column) => {
                write!(
                    f,
                    "UnexpectedToken: `{:?}` at {:?}:{:?}",
                    token_kind, line, column
                )
            }
            ParserError::UnexpectedTokenValue(token_kind, token_value, line, column) => {
                write!(
                    f,
                    "UnexpectedTokenValue: `{:?}` of kind `{:?}` at {:?}:{:?}",
                    token_value, token_kind, line, column
                )
            }
            ParserError::SyntaxError => write!(f, "SyntaxError"),
            ParserError::InvalidIdentifierCharacter => write!(f, "InvalidIdentifierCharacter"),
            ParserError::UnterminatedStringLiteral => write!(f, "UnterminatedStringLiteral"),
            ParserError::InvalidEscapeSequence => write!(f, "InvalidEscapeSequence"),
            ParserError::InvalidEscapeSequenceCannotBeFormatted => {
                write!(f, "InvalidEscapeSequenceCannotBeFormatted")
            }
            ParserError::InvalidHexadecimalEscapeSequence => {
                write!(f, "InvalidHexadecimalEscapeSequence")
            }
            ParserError::InvalidUnicodeEscapeSequence => write!(f, "InvalidUnicodeEscapeSequence"),
            ParserError::InvalidUnicodeCodePointEscapeSequence => {
                write!(f, "InvalidUnicodeCodePointEscapeSequence")
            }
            ParserError::InvalidLegacyOctalEscapeSequence => {
                write!(f, "InvalidLegacyOctalEscapeSequence")
            }
            ParserError::InvalidLegacyOctalEscapeSequenceNotAllowedInStrictMode => {
                write!(f, "InvalidLegacyOctalEscapeSequenceNotAllowedInStrictMode")
            }
            ParserError::InvalidDecimalLiteral => write!(f, "InvalidDecimalLiteral"),
            ParserError::InvalidNonDecimalBinaryNumberLiteral => {
                write!(f, "InvalidNonDecimalBinaryNumberLiteral")
            }
            ParserError::InvalidNonDecimalOctalNumberLiteral => {
                write!(f, "InvalidNonDecimalOctalNumberLiteral")
            }
            ParserError::InvalidNonDecimalHexadecimalNumberLiteral => {
                write!(f, "InvalidNonDecimalHexadecimalNumberLiteral")
            }
            ParserError::InvalidLegacyOctalNumberLiteral => {
                write!(f, "InvalidLegacyOctalNumberLiteral")
            }
            ParserError::InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode => {
                write!(f, "InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode")
            }
            ParserError::InvalidExponentPartNumberLiteral => {
                write!(f, "InvalidExponentPartNumberLiteral")
            }
            ParserError::InvalidNumericSeparatorAtSibling => {
                write!(f, "InvalidNumericSeparatorAtSibling")
            }
            ParserError::InvalidNumericSeparatorAtEnd => write!(f, "InvalidNumericSeparatorAtEnd"),
            ParserError::InvalidDecimalBigIntegerLiteral => {
                write!(f, "InvalidDecimalBigIntegerLiteral")
            }
            ParserError::UnterminatedRegExLiteral => write!(f, "UnterminatedRegExLiteral"),
            ParserError::InvalidRegexLiteralFirstChar => write!(f, "InvalidRegexLiteralFirstChar"),
            ParserError::InvalidRegexBackslashSequence => {
                write!(f, "InvalidRegexBackslashSequence")
            }
            ParserError::InvalidRegexExpressionClass => write!(f, "InvalidRegexExpressionClass"),
            ParserError::InvalidPropertyKey => write!(f, "InvalidPropertyKey"),
            ParserError::InvalidLeftHandSideExpression => {
                write!(f, "InvalidLeftHandSideExpression")
            }
            ParserError::InvalidAwaitForInStatement => write!(f, "InvalidAwaitForInStatement"),
            ParserError::InvalidExpressionToAssignmentExpressionConversion => {
                write!(f, "InvalidExpressionToAssignmentExpressionConversion")
            }
            ParserError::InvalidExpressionToChainElementConversion => {
                write!(f, "InvalidExpressionToChainElementConversion")
            }
            ParserError::InvalidMethodDefinitionToPropertyConversion => {
                write!(f, "InvalidMethodDefinitionToPropertyConversion")
            }
            ParserError::InvalidExpressionToPatternConversion => {
                write!(f, "InvalidExpressionToPatternConversion")
            }
            ParserError::InvalidExpressionToArrayPatternElementConversion => {
                write!(f, "InvalidExpressionToArrayPatternElementConversion")
            }
            ParserError::InvalidExpressionToLiteralConversion => {
                write!(f, "InvalidExpressionToLiteralConversion")
            }
            ParserError::UnexpectedLineTerminator => write!(f, "UnexpectedLineTerminator"),
        }
    }
}
