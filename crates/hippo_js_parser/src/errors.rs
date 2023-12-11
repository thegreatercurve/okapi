#[derive(Debug)]
pub enum ParserError {
    SyntaxError,

    InvalidIdentifierCharacter,

    InvalidEscapeSequence,
    InvalidEscapeSequenceCannotBeFormatted,
    InvalidHexadecimalEscapeSequence,
    InvalidUnicodeEscapeSequence,
    InvalidUnicodeCodePointEscapeSequence,
    InvalidOctalEscapeSequence,
    InvalidOctalEscapeSequenceNotAllowedInStrictMode,

    UnterminatedStringLiteral,
    ScannerError,
}
