use crate::{Lexer, Token, TokenKind, TokenValue};

#[derive(Clone)]
pub struct Cursor<'a> {
    pub(crate) current_token: Token,
    next_token: Token,
    pub(crate) previous_token: Token,
    source_str: &'a str,
    pub lexer: Lexer<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str, lexer: Lexer<'a>, current_token: Token, next_token: Token) -> Self {
        Self {
            previous_token: current_token.clone(),
            next_token,
            current_token,
            source_str: input,
            lexer,
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

    pub(crate) fn peek_token_value(&self) -> TokenValue {
        self.next_token.value.clone()
    }

    pub(crate) fn advance(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }
}
