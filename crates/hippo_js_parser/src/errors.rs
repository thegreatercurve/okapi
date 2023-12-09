#[derive(Debug)]
pub enum ParserError {
    SyntaxError,

    InvalidIdentifierCharacter,
    InvalidHexadecimalEscapeSequence,
    InvalidUnicodeEscapeSequence,
    InvalidUnicodeCodePointEscapeSequence,
    InvalidOctalEscapeSequence,
    InvalidOctalEscapeSequenceNotAllowedInStrictMode,

    UnterminatedStringLiteral,
    ScannerError,
}
