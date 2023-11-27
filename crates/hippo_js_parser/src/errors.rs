#[derive(Debug)]
pub enum ParserError {
    SyntaxError,

    IllegalIdentifierCharacter,
    IllegalHexadecimalEscapeSequence,
    IllegalUnicodeEscapeSequence,
    IllegalUnicodeCodePointEscapeSequence,

    UnterminatedStringLiteral,
    ScannerError,
}
