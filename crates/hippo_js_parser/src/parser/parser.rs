use crate::{Cursor, Lexer, ParserError, TokenKind};
use hippo_estree::*;

#[derive(Clone, Debug)]
pub struct Config {
    pub strict_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { strict_mode: true }
    }
}

#[derive(Clone)]
pub struct Parser<'a> {
    config: Config,
    pub cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let config = Config::default();

        let mut lexer = Lexer::new(input, config.clone());

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            config,
            cursor: Cursor::new(input, lexer, current_token, next_token),
        }
    }

    pub(crate) fn unexpected_current_token_kind(&self) -> ParserError {
        ParserError::UnexpectedToken(self.cursor.current_token_kind())
    }

    pub fn parse_script_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_script();

        serde_json::to_string(&program.unwrap())
    }

    pub fn parse_module_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_module();

        serde_json::to_string(&program.unwrap())
    }

    pub(crate) fn start_node(&mut self) {
        self.cursor
            .token_stack
            .push(self.cursor.current_token.clone())
    }

    pub fn end_node(&mut self) -> Result<Node, ParserError> {
        if self.cursor.token_stack.is_empty() {
            return Err(ParserError::UnexpectedEmptyNode);
        }

        let start = self.cursor.token_stack.pop().unwrap().start;
        let end: usize = self.cursor.previous_token.end;

        Ok(Node::new(start, end))
    }

    pub(crate) fn expect_and_advance(&mut self, token_kind: TokenKind) -> Result<(), ParserError> {
        if self.cursor.current_token_kind() == token_kind {
            self.cursor.advance();

            return Ok(());
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_one_of_and_advance(
        &mut self,
        token_kinds: Vec<TokenKind>,
    ) -> Result<(), ParserError> {
        for token_kind in token_kinds {
            if self.cursor.current_token_kind() == token_kind {
                self.cursor.advance();

                return Ok(());
            }
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_optional_semicolon_and_advance(&mut self) {
        if self.cursor.current_token_kind() == TokenKind::Semicolon {
            self.cursor.advance();
        }
    }

    // https://tc39.es/ecma262/#prod-Initializer
    fn parse_initializer(&mut self) -> Result<(), ParserError> {
        todo!("parse_initializer")
    }
}
