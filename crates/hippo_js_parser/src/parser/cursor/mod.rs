use crate::{Lexer, Token, TokenKind, TokenValue};

#[derive(Clone)]
pub struct Cursor<'a> {
    pub(crate) current_token: Token,
    pub(crate) lexer: Lexer<'a>,
    next_token: Token,
    source_str: &'a str,
    pub(crate) token_stack: Vec<Token>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str, lexer: Lexer<'a>, current_token: Token, next_token: Token) -> Self {
        Self {
            next_token,
            current_token,
            source_str: input,
            token_stack: Vec::new(),
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
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }
}
