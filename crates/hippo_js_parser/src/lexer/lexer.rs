use crate::{parser::Config, tokens::Token, TokenKind};

use super::utils::{
    is_identifier_start, is_line_terminator, is_punctuator_start, is_regular_expression_first_char,
    is_whitespace,
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub config: Config,
    pub read_index: usize,
    pub source_str: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, config: Config) -> Self {
        Self {
            config: config,
            read_index: 0,
            source_str: input,
        }
    }

    pub(crate) fn read_char(&mut self) {
        self.read_char_nth(1);
    }

    pub(crate) fn read_char_nth(&mut self, offset: usize) {
        self.read_index += offset;
    }

    pub(crate) fn current_char(&mut self) -> char {
        self.source_str
            .chars()
            .nth(self.read_index)
            .unwrap_or(0 as char)
    }

    pub(crate) fn peek_char(&mut self) -> char {
        self.peek_char_nth(1)
    }

    pub(crate) fn peek_char_nth(&mut self, offset: usize) -> char {
        self.source_str
            .chars()
            .nth(self.read_index + offset)
            .unwrap_or(0 as char)
    }

    pub fn len(&self) -> usize {
        self.source_str.chars().count()
    }

    pub fn is_end_of_file(&mut self) -> bool {
        self.read_index >= self.len()
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.current_char()) || is_line_terminator(self.current_char()) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_end_of_file() {
            return Token::new(TokenKind::EOF, self.read_index, self.read_index, None);
        }

        let token = self.advance();

        token
    }

    fn advance(&mut self) -> Token {
        let start_index = self.read_index;

        let current_char = self.current_char();

        let mut token: Token = match current_char {
            '#' => self.scan_private_identifier(),
            '0'..='9' => self.scan_number_literal(),
            '.' if self.peek_char().is_ascii_digit() => self.scan_number_literal(),
            '\'' | '"' => self.scan_string_literal(),
            '/' if is_regular_expression_first_char(self.peek_char()) => {
                self.scan_regular_expression_literal()
            }
            _ if is_punctuator_start(current_char) => self.scan_punctuator(),
            _ if is_identifier_start(current_char) => self.scan_identifier_name_or_keyword(),
            _ => Token::new(TokenKind::Illegal, start_index, self.read_index, None),
        };

        let end_index = self.read_index;

        token.start = start_index;
        token.end = end_index;

        token
    }
}
