use crate::{parser::Config, tokens::Token, ParserError, TokenKind, TokenValue};

use super::char::LexerChar;

#[derive(Clone, Debug, PartialEq)]
pub enum GoalSymbol {
    InputElementRegExp,
    InputElementRegExpOrTemplateTail,
    InputElementDiv,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    pub config: Config,
    pub read_index: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) chars: Vec<char>,
    pub(crate) goal_symbol: GoalSymbol,
}

impl Lexer {
    pub fn new(input: &str, config: Config) -> Self {
        let mut lexer = Self::default();

        lexer.chars = input.chars().collect();
        lexer.config = config;

        Self::skip_comment_or_whitespace(&mut lexer, &mut false);

        lexer
    }

    fn default() -> Self {
        Self {
            config: Config::default(),
            read_index: 0,
            line: 1,
            column: 1,
            chars: Vec::new(),
            goal_symbol: GoalSymbol::InputElementDiv,
        }
    }

    pub(crate) fn read_char(&mut self) {
        if self.current_char().is_line_terminator() {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.read_index += 1;
    }

    pub(crate) fn read_char_nth(&mut self, offset: usize) {
        for _ in 0..offset {
            self.read_char();
        }
    }

    pub(crate) fn current_char(&mut self) -> char {
        if !self.is_end_of_file() {
            self.chars[self.read_index]
        } else {
            '\0'
        }
    }

    pub(crate) fn peek_char(&mut self) -> char {
        self.peek_char_nth(1)
    }

    pub(crate) fn peek_char_nth(&mut self, offset: usize) -> char {
        if self.read_index + offset < self.len() {
            self.chars[self.read_index + offset]
        } else {
            '\0'
        }
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_end_of_file(&mut self) -> bool {
        self.read_index >= self.len()
    }

    pub fn rewind_token(&mut self, read_index: usize, line: usize, column: usize) -> Token {
        self.read_index = read_index;
        self.line = line;
        self.column = column;

        self.scan_token()
    }

    pub fn next_token(&mut self) -> Token {
        self.scan_token()
    }

    fn scan_token(&mut self) -> Token {
        let start_index = self.read_index;
        let start_line = self.line;
        let start_column = self.column;

        let mut token = match self.current_char() {
            '#' => self.scan_private_identifier(),
            '0'..='9' => self.scan_number_literal(),
            '.' if self.peek_char().is_ascii_digit() => self.scan_number_literal(),
            '\'' | '"' => self.scan_string_literal(),
            '`' => self.scan_template_literal(),
            '}' if self.goal_symbol == GoalSymbol::InputElementRegExpOrTemplateTail => {
                self.scan_template_literal()
            }
            '/' if matches!(self.goal_symbol, GoalSymbol::InputElementRegExp) => {
                self.scan_regular_expression_literal()
            }
            ch if ch.is_punctuator_start() => self.scan_punctuator(),
            ch if ch.is_identifier_start() => self.scan_identifier_name_or_keyword(),
            _ if self.is_end_of_file() => {
                return Token::new(
                    TokenKind::EOF,
                    start_index,
                    self.read_index,
                    self.line,
                    self.column,
                    TokenValue::Null,
                );
            }
            _ => {
                self.read_char(); // Eat illegal char.

                let error_str = ParserError::SyntaxError.to_string();

                Token::new(
                    TokenKind::Illegal,
                    start_index,
                    self.read_index,
                    self.line,
                    self.column,
                    TokenValue::String {
                        raw: error_str.clone(),
                        value: error_str,
                    },
                )
            }
        };

        token.start = start_index;
        token.end = self.read_index;
        token.line = start_line;
        token.column = start_column;
        token.line_terminator = false;

        self.skip_comment_or_whitespace(&mut token.line_terminator);

        token
    }

    fn skip_comment_or_whitespace(&mut self, has_line_terminator: &mut bool) {
        while !self.is_end_of_file() {
            match self.current_char() {
                '/' => match self.peek_char() {
                    '/' => self.skip_single_line_comment(),
                    '*' => self.skip_multi_line_comment(),
                    _ => break,
                },
                ch if ch.is_js_whitespace() => {
                    self.read_char();
                }
                ch if ch.is_line_terminator() => {
                    *has_line_terminator = true;

                    self.read_char();
                }
                _ => break,
            }
        }
    }
}
