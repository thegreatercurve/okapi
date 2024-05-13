use crate::{Lexer, Token, TokenKind, TokenValue};

#[derive(Clone, Debug)]
pub struct Cursor {
    pub(crate) current_token: Token,
    pub(crate) lexer: Lexer,
    pub(crate) next_token: Token,
    pub(crate) previous_token: Token,
}

impl Cursor {
    pub fn new(lexer: Lexer, current_token: Token, next_token: Token) -> Self {
        Self {
            current_token: current_token.clone(),
            lexer,
            next_token,
            previous_token: current_token.clone(),
        }
    }

    pub(crate) fn current_token_kind(&self) -> TokenKind {
        self.current_token.kind.clone()
    }

    pub(crate) fn current_token_value(&self) -> TokenValue {
        self.current_token.value.clone()
    }

    pub(crate) fn peek_token_kind(&self) -> TokenKind {
        self.next_token.kind.clone()
    }

    pub(crate) fn peek_nth_kind(&self, i: i32) -> TokenKind {
        match i {
            0 => self.current_token_kind(),
            1 => self.peek_token_kind(),
            _ => {
                let mut lexer = self.lexer.clone();

                for _ in 0..i - 1 {
                    lexer.next_token();
                }

                lexer.next_token().kind
            }
        }
    }

    pub(crate) fn rewind(&mut self) {
        self.current_token = self.lexer.rewind_token(
            self.current_token.start,
            self.current_token.line,
            self.current_token.column,
        );
        self.next_token = self.lexer.clone().next_token();
    }

    pub(crate) fn advance(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.lexer.next_token();
        self.next_token = self.lexer.clone().next_token();
    }
}
