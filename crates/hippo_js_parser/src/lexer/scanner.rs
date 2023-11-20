use crate::{config::Config, tokens::KeywordKind, tokens::TokenType};
use hippo_unicode::{is_unicode_id_continue, is_unicode_id_start};

use super::char_utils::{is_line_terminator, is_numeric, is_whitespace};

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
        while is_whitespace(self.ch) || is_line_terminator(self.ch) {
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
            ch if self.is_part_of_identifier_or_keyword(ch) => self.scan_identifier_or_keyword(),
            ch if is_numeric(ch) => self.scan_number_literal(),
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
            '#' => {
                // https://tc39.es/ecma262/#prod-PrivateIdentifier
                // PrivateIdentifier ::
                //      # IdentifierName
                if self.is_start_of_identifier_or_keyword(self.ch) {
                    self.scan_identifier_or_keyword()
                } else {
                    TokenType::Illegal
                }
            }
            _ => TokenType::EOF,
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

// 12.7 Names and Keywords
// https://tc39.es/ecma262/#sec-names-and-keywords

impl<'a> Scanner<'a> {
    fn scan_identifier_or_keyword(&mut self) -> TokenType {
        let start_index = self.read_index;

        while self.is_part_of_identifier_or_keyword(self.ch) {
            self.read_char();
        }

        let keyword_or_identifer = &self.input[start_index..self.read_index];

        match keyword_or_identifer {
            "await" => TokenType::Keyword(KeywordKind::Await),
            "break" => TokenType::Keyword(KeywordKind::Break),
            "case" => TokenType::Keyword(KeywordKind::Case),
            "catch" => TokenType::Keyword(KeywordKind::Catch),
            "class" => TokenType::Keyword(KeywordKind::Class),
            "const" => TokenType::Keyword(KeywordKind::Const),
            "continue" => TokenType::Keyword(KeywordKind::Continue),
            "debugger" => TokenType::Keyword(KeywordKind::Debugger),
            "default" => TokenType::Keyword(KeywordKind::Default),
            "delete" => TokenType::Keyword(KeywordKind::Delete),
            "do" => TokenType::Keyword(KeywordKind::Do),
            "else" => TokenType::Keyword(KeywordKind::Else),
            "enum" => TokenType::Keyword(KeywordKind::Enum),
            "export" => TokenType::Keyword(KeywordKind::Export),
            "extends" => TokenType::Keyword(KeywordKind::Extends),
            "false" => TokenType::Keyword(KeywordKind::False),
            "finally" => TokenType::Keyword(KeywordKind::Finally),
            "for" => TokenType::Keyword(KeywordKind::For),
            "function" => TokenType::Keyword(KeywordKind::Function),
            "if" => TokenType::Keyword(KeywordKind::If),
            "import" => TokenType::Keyword(KeywordKind::Import),
            "in" => TokenType::Keyword(KeywordKind::In),
            "instanceof" => TokenType::Keyword(KeywordKind::Instanceof),
            "new" => TokenType::Keyword(KeywordKind::New),
            "null" => TokenType::Keyword(KeywordKind::Null),
            "return" => TokenType::Keyword(KeywordKind::Return),
            "super" => TokenType::Keyword(KeywordKind::Super),
            "switch" => TokenType::Keyword(KeywordKind::Switch),
            "this" => TokenType::Keyword(KeywordKind::This),
            "throw" => TokenType::Keyword(KeywordKind::Throw),
            "true" => TokenType::Keyword(KeywordKind::True),
            "try" => TokenType::Keyword(KeywordKind::Try),
            "typeof" => TokenType::Keyword(KeywordKind::Typeof),
            "var" => TokenType::Keyword(KeywordKind::Var),
            "void" => TokenType::Keyword(KeywordKind::Void),
            "while" => TokenType::Keyword(KeywordKind::While),
            "with" => TokenType::Keyword(KeywordKind::With),
            "yield" => TokenType::Keyword(KeywordKind::Yield),

            // Strict mode future reserved words
            "let" => TokenType::Keyword(KeywordKind::Let),
            "static" => TokenType::Keyword(KeywordKind::Static),
            "implements" => TokenType::Keyword(KeywordKind::Implements),
            "interface" => TokenType::Keyword(KeywordKind::Interface),
            "package" => TokenType::Keyword(KeywordKind::Package),
            "private" => TokenType::Keyword(KeywordKind::Private),
            "protected" => TokenType::Keyword(KeywordKind::Protected),
            "public" => TokenType::Keyword(KeywordKind::Public),

            _ => TokenType::Identifier,
        }
    }

    fn is_start_of_identifier_or_keyword(&self, ch: char) -> bool {
        if ch.is_ascii() {
            match ch {
                ch if ch.is_ascii_alphabetic() => true,
                '#' | '_' | '$' => true,
                _ => false,
            }
        } else {
            is_unicode_id_start(ch)
        }
    }

    fn is_part_of_identifier_or_keyword(&self, ch: char) -> bool {
        if ch.is_ascii() {
            match ch {
                ch if ch.is_ascii_alphanumeric() => true,
                '_' | '$' | '0'..='9' => true,
                _ => false,
            }
        } else {
            is_unicode_id_continue(ch)
        }
    }
}
