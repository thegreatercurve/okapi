use super::char::LexerChar;
use crate::{Lexer, ParserError, Token, TokenKind, TokenValue};

// 12.9.5 Regular Expression Literals
// https://tc39.es/ecma262/#sec-literals-regular-expression-literals
impl Lexer {
    // https://tc39.es/ecma262/#prod-RegularExpressionLiteral
    pub(crate) fn scan_regular_expression_literal(&mut self) -> Result<Token, ParserError> {
        let start_index = self.read_index;

        self.read_char(); // Eat '/' char.

        let regular_expression_body = self.read_regular_expression_body()?;

        self.read_char(); // Eat '/' char.

        let regular_expression_flags = self.read_regular_expression_flags();

        Ok(Token::new(
            TokenKind::RegularExpressionLiteral,
            start_index,
            self.read_index,
            self.line,
            self.column,
            TokenValue::RegularExpression {
                pattern: regular_expression_body.to_string(),
                flags: regular_expression_flags.to_string(),
            },
        ))
    }

    // https://tc39.es/ecma262/#prod-RegularExpressionBody
    fn read_regular_expression_body(&mut self) -> Result<String, ParserError> {
        let start_index = self.read_index;

        let mut in_class = false;
        let mut in_backslash_sequence = false;

        match self.current_char() {
            '*' | '/' => return Err(ParserError::InvalidRegexLiteralFirstChar),
            ch if ch.is_line_terminator() => return Err(ParserError::InvalidRegexLiteralFirstChar),
            '\\' => in_backslash_sequence = true,
            '[' => in_class = true,
            _ => {}
        }

        self.read_char(); // Eat first char.

        while self.current_char() != '/' || in_class || in_backslash_sequence {
            if self.current_char().is_line_terminator() {
                if in_class {
                    return Err(ParserError::InvalidRegexExpressionClass);
                } else if in_backslash_sequence {
                    return Err(ParserError::InvalidRegexBackslashSequence);
                }
            }

            match self.current_char() {
                ch if ch.is_line_terminator() => return Err(ParserError::UnterminatedRegExLiteral),
                '\\' => in_backslash_sequence = !in_backslash_sequence,
                '[' if !in_backslash_sequence => in_class = true,
                ']' if !in_backslash_sequence => in_class = false,
                ch if ch.is_line_terminator() => return Err(ParserError::UnterminatedRegExLiteral),
                _ => in_backslash_sequence = false,
            };

            self.read_char();
        }

        if in_class || in_backslash_sequence {
            return Err(ParserError::UnterminatedRegExLiteral);
        }

        Ok(self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>())
    }

    // https://tc39.es/ecma262/#prod-RegularExpressionFlags
    fn read_regular_expression_flags(&mut self) -> String {
        let start_index = self.read_index;

        while matches!(self.current_char(), 'd' | 'g' | 'i' | 'm' | 's' | 'u' | 'y') {
            self.read_char();
        }

        self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>()
    }
}
