#[derive(Debug)]
pub enum ParserError {
    SyntaxError,

    InvalidIdentifierCharacter,

    // String literals
    UnterminatedStringLiteral,
    InvalidEscapeSequence,
    InvalidEscapeSequenceCannotBeFormatted,
    InvalidHexadecimalEscapeSequence,
    InvalidUnicodeEscapeSequence,
    InvalidUnicodeCodePointEscapeSequence,
    InvalidOctalEscapeSequence,
    InvalidOctalEscapeSequenceNotAllowedInStrictMode,

    // Number literals
    InvalidNumberLiteral,
    InvalidNonDecimalBinaryNumberLiteral,
    InvalidNonDecimalOctalNumberLiteral,
    InvalidNonDecimalHexadecimalNumberLiteral,

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
            ParserError::InvalidOctalEscapeSequence => write!(f, "InvalidOctalEscapeSequence"),
            ParserError::InvalidOctalEscapeSequenceNotAllowedInStrictMode => {
                write!(f, "InvalidOctalEscapeSequenceNotAllowedInStrictMode")
            }
            ParserError::InvalidNumberLiteral => {
                write!(f, "InvalidNonNumberLiteral")
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
        }
    }
}
