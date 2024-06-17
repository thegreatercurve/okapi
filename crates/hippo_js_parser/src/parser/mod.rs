pub(crate) use crate::{ast::*, Lexer, ParserError, TokenKind, TokenValue};

mod cursor;
mod directive;
mod expression;
mod functions_and_classes;
mod imports_and_modules;
mod statement;

pub use cursor::Cursor;

#[derive(Clone, Debug)]
pub struct Context {
    pub allow_in: bool,
    pub allow_yield: bool,
    pub allow_super: bool,
    pub in_optional_chain: bool,
    pub strict_mode: bool,
}

impl Context {
    pub fn default() -> Self {
        Self {
            allow_in: true,
            allow_yield: true,
            allow_super: false,
            in_optional_chain: false,
            strict_mode: false,
        }
    }
}

#[derive(Clone)]
pub struct Parser {
    pub cursor: Cursor,
    pub context: Context,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);

        let current_token = lexer.next_token();
        let next_token = lexer.clone().next_token();

        Self {
            cursor: Cursor::new(lexer, current_token, next_token),
            context: Context::default(),
        }
    }

    pub(crate) fn token_kind(&self) -> TokenKind {
        self.cursor.current_token_kind()
    }

    pub(crate) fn token_value(&self) -> TokenValue {
        self.cursor.current_token_value()
    }

    pub(crate) fn peek_token_kind(&self) -> TokenKind {
        self.cursor.peek_token_kind()
    }

    // pub(crate) fn peek_token_value(&self) -> TokenValue {
    //     self.cursor.peek_token_value()
    // }

    pub(crate) fn peek_nth_kind(&self, i: i32) -> TokenKind {
        self.cursor.peek_nth_kind(i)
    }

    pub(crate) fn has_previous_token_line_terminator(&self) -> bool {
        self.cursor.previous_token.line_terminator
    }

    pub(crate) fn has_current_token_line_terminator(&self) -> bool {
        self.cursor.current_token.line_terminator
    }

    pub(crate) fn has_peek_token_line_terminator(&self) -> bool {
        self.cursor.next_token.line_terminator
    }

    pub(crate) fn unexpected_current_token_kind(&mut self) -> ParserError {
        ParserError::UnexpectedToken(
            self.token_kind(),
            self.cursor.current_token.line,
            self.cursor.current_token.column,
        )
    }

    pub(crate) fn unexpected_current_token_value(&mut self) -> ParserError {
        ParserError::UnexpectedTokenValue(
            self.token_kind(),
            self.token_value(),
            self.cursor.current_token.line,
            self.cursor.current_token.column,
        )
    }

    pub fn parse_script_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_script();

        serde_json::to_string(&program.unwrap())
    }

    pub fn parse_module_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_module();

        serde_json::to_string(&program.unwrap())
    }

    pub(crate) fn start_node(&mut self) -> usize {
        self.cursor.current_token.start
    }

    pub fn end_node(&mut self, start_index: usize) -> Result<Node, ParserError> {
        let end_index = self.cursor.previous_token.end;

        Ok(Node::new(start_index, end_index))
    }

    pub(crate) fn advance_any(&mut self) {
        self.cursor.advance();
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<(), ParserError> {
        if self.token_kind() == token_kind {
            return Ok(());
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_and_advance(&mut self, token_kind: TokenKind) -> Result<(), ParserError> {
        self.expect(token_kind)?;

        self.advance_any();

        Ok(())
    }

    pub(crate) fn expect_one_of_and_advance(
        &mut self,
        token_kinds: Vec<TokenKind>,
    ) -> Result<(), ParserError> {
        for token_kind in token_kinds {
            if self.token_kind() == token_kind {
                self.advance_any();

                return Ok(());
            }
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_optional_semicolon_and_advance(&mut self) {
        if self.token_kind() == TokenKind::Semicolon {
            self.advance_any();
        }
    }
}
