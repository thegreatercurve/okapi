#[derive(Debug)]
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

    ScannerError,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::SyntaxError => write!(f, "SyntaxError"),
            ParserError::InvalidIdentifierCharacter => write!(f, "InvalidIdentifierCharacter"),
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
                write!(f, "InvalidOctalEscapeSequence")
            }
            ParserError::InvalidLegacyOctalEscapeSequenceNotAllowedInStrictMode => {
                write!(f, "InvalidOctalEscapeSequenceNotAllowedInStrictMode")
            }
            ParserError::InvalidNonDecimalBinaryNumberLiteral => {
                write!(f, "InvalidNonDecimalBinaryNumberLiteral")
            }
            ParserError::InvalidNonDecimalOctalNumberLiteral => {
                write!(f, "InvalidNonDecimalOctalNumberLiteral")
            }
            ParserError::InvalidNonDecimalHexadecimalNumberLiteral => {
                write!(f, "InvalidNonDecimalHexadecimalNumberLiteral")
            }
            ParserError::UnterminatedStringLiteral => write!(f, "UnterminatedStringLiteral"),
            ParserError::ScannerError => write!(f, "ScannerError"),
            ParserError::InvalidDecimalLiteral => {
                write!(f, "InvalidDecimalLiteral")
            }

            ParserError::InvalidNumericSeparatorAtSibling => {
                write!(f, "InvalidNumericSeparatorAtSibling")
            }
            ParserError::InvalidNumericSeparatorAtEnd => {
                write!(f, "InvalidNumericSeparatorAtEnd")
            }
            ParserError::InvalidLegacyOctalNumberLiteral => {
                write!(f, "InvalidLegacyOctalNumberLiteral")
            }
            ParserError::InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode => {
                write!(f, "InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode")
            }

            ParserError::InvalidExponentPartNumberLiteral => todo!(),
            ParserError::InvalidDecimalBigIntegerLiteral => todo!(),
            ParserError::UnterminatedRegExLiteral => todo!(),
        }
    }
}
