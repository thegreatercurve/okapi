#[derive(Debug)]
pub enum ParserError {
    SyntaxError(String),
    IllegalCharacter,
    UnterminatedStringLiteral,
    ScannerError,
}
