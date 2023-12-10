#[derive(Debug)]
pub enum ParserError {
    SyntaxError,

    InvalidIdentifierCharacter,

    InvalidGeneralEscapeSequence,
    InvalidHexadecimalEscapeSequence,
    InvalidUnicodeEscapeSequence,
    InvalidUnicodeCodePointEscapeSequence,
    InvalidOctalEscapeSequence,
    InvalidOctalEscapeSequenceNotAllowedInStrictMode,

    UnterminatedStringLiteral,
    ScannerError,
}
