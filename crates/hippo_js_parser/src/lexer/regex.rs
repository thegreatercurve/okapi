use crate::{Lexer, ParserError, Token, TokenKind};

use super::utils::{is_identifier_part, is_line_terminator};

// 12.9.5 Regular Expression Literals
// https://tc39.es/ecma262/#sec-literals-string-literals
impl<'a> Lexer<'a> {
    // https://tc39.es/ecma262/#prod-StringLiteral
    pub(crate) fn scan_regular_expression_literal(&mut self) -> Token {
        let start_index = self.read_index;

        self.read_char(); // Eat '/' char.

        let regular_expression_body = self.read_regular_expression_body();

        if regular_expression_body.is_err() {
            return Token::new(
                TokenKind::Illegal,
                start_index,
                self.read_index,
                Some(regular_expression_body.unwrap_err().to_string()),
            );
        }

        self.read_char(); // Eat '/' char.

        self.read_regular_expression_flags();

        let regular_expression_str = &self.source_str[start_index..self.read_index];

        Token::new(
            TokenKind::RegularExpressionLiteral,
            start_index,
            self.read_index,
            Some(regular_expression_str.to_string()),
        )
    }

    // https://tc39.es/ecma262/#prod-RegularExpressionBody
    fn read_regular_expression_body(&mut self) -> Result<(), ParserError> {
        let mut in_class = false;
        let mut in_backslash_sequence = false;

        while self.current_char() != '/' {
            let ch = self.current_char();

            match ch {
                _ if is_line_terminator(ch) => return Err(ParserError::UnterminatedRegExLiteral),
                '\\' => in_backslash_sequence = if in_backslash_sequence { false } else { true },
                '[' => in_class = true,
                ']' => in_class = false,
                _ => in_backslash_sequence = false,
            }

            self.read_char();
        }

        if (in_class && !in_backslash_sequence) || in_backslash_sequence {
            return Err(ParserError::UnterminatedRegExLiteral);
        }

        Ok(())
    }

    // https://tc39.es/ecma262/#prod-RegularExpressionFlags
    fn read_regular_expression_flags(&mut self) {
        while is_identifier_part(self.current_char()) {
            self.read_char();
        }
    }
}
