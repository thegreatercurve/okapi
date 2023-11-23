#[derive(Debug)]
pub enum ParserError {
    SyntaxError(String),
    IllegalCharacter,
    ScannerError,
}
