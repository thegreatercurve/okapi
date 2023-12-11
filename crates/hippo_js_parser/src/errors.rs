#[derive(Debug)]
pub enum ParserError {
    SyntaxError,

    InvalidIdentifierCharacter,

    InvalidEscapeSequence,
    InvalidHexadecimalEscapeSequence,
    InvalidUnicodeEscapeSequence,
    InvalidUnicodeCodePointEscapeSequence,
    InvalidOctalEscapeSequence,
    InvalidOctalEscapeSequenceNotAllowedInStrictMode,

    UnterminatedStringLiteral,
    ScannerError,
}
