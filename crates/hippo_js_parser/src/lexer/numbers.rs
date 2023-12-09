use crate::{Lexer, TokenKind};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_number_literal(&mut self) -> TokenKind {
        while self.current_char().is_numeric() {
            self.read_char();
        }

        TokenKind::NumberLiteral
    }
}
