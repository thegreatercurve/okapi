use crate::lexer::char_extensions::CharExtensions;
use crate::{config::Config, tokens::KeywordValue, tokens::TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    config: Config,
    input: &'a str,
    read_index: usize,
    ch: char,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            config: Config::default(),
            input: &input,
            read_index: 0,
            ch: 0 as char,
        };

        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.read_index += 1;
    }

    fn peek_char(&self) -> char {
        self.input
            .chars()
            .nth(self.read_index + 1)
            .unwrap_or(0 as char)
    }

    pub fn is_end_of_file(&self) -> bool {
        self.read_index >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        if self.is_end_of_file() {
            return TokenType::EOF;
        }

        let token = self.scan_for_token_from_initial_character();

        self.read_char();

        token
    }

    fn scan_for_token_from_initial_character(&mut self) -> TokenType {
        let token = match self.ch {
            ch if ch.is_start_of_identifier_or_keyword() => self.scan_identifier_or_keyword(),
            ch if ch.is_numeric() => self.scan_number_literal(),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            _ => self.scan_token_from_single_char(),
        };

        return token;
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

    // https://tc39.github.io/ecma262/#sec-keywords
    fn scan_identifier_or_keyword(&mut self) -> TokenType {
        let start_index = self.read_index;

        while self.ch.is_within_identifier_or_keyword() {
            self.read_char();
        }

        let keyword_or_identifer = &self.input[start_index..self.read_index];

        match keyword_or_identifer {
            "if" => TokenType::Keyword(KeywordValue::Illegal),
            "in" => TokenType::Keyword(KeywordValue::Illegal),
            "do" => TokenType::Keyword(KeywordValue::Illegal),
            "var" => TokenType::Keyword(KeywordValue::Var),
            "for" => TokenType::Keyword(KeywordValue::Illegal),
            "new" => TokenType::Keyword(KeywordValue::Illegal),
            "try" => TokenType::Keyword(KeywordValue::Illegal),
            "let" => TokenType::Keyword(KeywordValue::Let),
            "this" => TokenType::Keyword(KeywordValue::Illegal),
            "else" => TokenType::Keyword(KeywordValue::Illegal),
            "case" => TokenType::Keyword(KeywordValue::Illegal),
            "void" => TokenType::Keyword(KeywordValue::Illegal),
            "with" => TokenType::Keyword(KeywordValue::Illegal),
            "enum" => TokenType::Keyword(KeywordValue::Illegal),
            "while" => TokenType::Keyword(KeywordValue::Illegal),
            "break" => TokenType::Keyword(KeywordValue::Illegal),
            "catch" => TokenType::Keyword(KeywordValue::Illegal),
            "throw" => TokenType::Keyword(KeywordValue::Illegal),
            "const" => TokenType::Keyword(KeywordValue::Const),
            "yield" => TokenType::Keyword(KeywordValue::Illegal),
            "class" => TokenType::Keyword(KeywordValue::Illegal),
            "super" => TokenType::Keyword(KeywordValue::Illegal),
            "return" => TokenType::Keyword(KeywordValue::Illegal),
            "typeof" => TokenType::Keyword(KeywordValue::Illegal),
            "delete" => TokenType::Keyword(KeywordValue::Illegal),
            "switch" => TokenType::Keyword(KeywordValue::Illegal),
            "export" => TokenType::Keyword(KeywordValue::Illegal),
            "import" => TokenType::Keyword(KeywordValue::Illegal),
            "default" => TokenType::Keyword(KeywordValue::Illegal),
            "finally" => TokenType::Keyword(KeywordValue::Illegal),
            "extends" => TokenType::Keyword(KeywordValue::Illegal),
            "function" => TokenType::Keyword(KeywordValue::Illegal),
            "continue" => TokenType::Keyword(KeywordValue::Illegal),
            "debugger" => TokenType::Keyword(KeywordValue::Illegal),
            "instanceof" => TokenType::Keyword(KeywordValue::Illegal),
            _ => TokenType::Identifier,
        }
    }

    // https://tc39.es/ecma262/#sec-literals-numeric-literals
    fn scan_number_literal(&mut self) -> TokenType {
        let start_index = self.read_index;

        // TODO Account for numbers starting with decimals ("".123").
        // TODO Account for numbers with underscore separators.
        // TODO Account for floats.

        while self.read_index < self.input.len() {
            self.read_char();

            if !self.peek_char().is_numeric() {
                break;
            }
        }

        TokenType::Number
    }
}
