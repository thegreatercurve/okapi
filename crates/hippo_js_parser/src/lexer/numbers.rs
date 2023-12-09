use crate::{Lexer, TokenKind};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_number_literal(&mut self) -> TokenKind {
        // TODO
        self.read_char();

        TokenKind::NumberLiteral
    }
}
