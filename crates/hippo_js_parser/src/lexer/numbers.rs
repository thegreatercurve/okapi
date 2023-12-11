use crate::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(crate) fn scan_number_literal(&mut self) -> Token {
        while self.current_char().is_numeric() {
            self.read_char();
        }

        Token::default(TokenKind::NumberLiteral)
    }
}