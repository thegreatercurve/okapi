use crate::lexer::char_extensions::CharExtensions;
use crate::{config::Config, tokens::TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    config: Config,
    input: &'a str,
    start_position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            config: Config::default(),
            input: &input,
            start_position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        lexer.read_char();

        return lexer;
    }

    fn next_char(&self) -> char {
        self.input
            .chars()
            .nth(self.read_position)
            .unwrap_or_else(|| {
                panic!(
                    "[Lexer Error]: Token out of range at index `{}`",
                    &self.read_position
                )
            })
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0 as char;
        } else {
            return self.next_char();
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        let token = match self.ch {
            ch if ch.is_start_of_identifier_or_keyword() => {
                self.scan_token_from_keyword_or_identifier()
            }
            ch if ch.is_string_literal() => self.scan_string_literal(),
            ch if ch.is_numeric() => self.scan_number_literal(),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            }
            _ => self.scan_token_from_single_char(),
        };

        self.read_char();

        token
    }

    fn scan_token_from_keyword_or_identifier(&mut self) -> TokenType {
        let keyword_or_identifer = self.read_word();

        match keyword_or_identifer {
            "const" => TokenType::Constant,
            "let" => TokenType::Let,
            "var" => TokenType::Variable,
            _ => TokenType::Name,
        }
    }

    fn scan_token_from_single_char(&mut self) -> TokenType {
        match self.ch {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            '<' => TokenType::LessThan,
            '>' => TokenType::GreaterThan,
            ';' => TokenType::SemiColon,
            _ => TokenType::EOF,
        }
    }

    // String literals

    fn scan_string_literal(&mut self) -> TokenType {
        // TODO Account for template literals.
        // TODO Account for escape characters better.

        while self.read_position < self.input.len() {
            self.read_char();

            if self.ch != '\\' && self.peek_char().is_string_literal() {
                break;
            }
        }

        self.read_char();

        TokenType::String
    }

    // Numbers

    fn scan_number_literal(&mut self) -> TokenType {
        // TODO Account for numbers starting with decimals ("".123").
        // TODO Account for numbers with underscore separators.
        // TODO Account for floats.

        while self.read_position < self.input.len() {
            self.read_char();

            if !self.peek_char().is_numeric() {
                break;
            }
        }

        TokenType::Number
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.next_char();
        }

        self.start_position = self.read_position;
        self.read_position += 1;
    }

    fn read_word(&mut self) -> &str {
        let start_position = self.start_position;

        while self.ch.is_within_identifier_or_keyword() {
            self.read_char();
        }

        &self.input[start_position..self.start_position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}