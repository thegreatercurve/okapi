use crate::{Lexer, Token, TokenKind};

// 12.9.6 Template Literal Lexical Components
// https://tc39.es/ecma262/#sec-template-literal-lexical-components
impl<'a> Lexer<'a> {
    // https://tc39.es/ecma262/#prod-Template
    pub(crate) fn scan_template_literal(&mut self) -> Token {
        let start_index = self.read_index;

        self.read_char(); // Eat '`' char.

        let mut current_char = self.current_char();

        while current_char != '`' {
            match current_char {
                '$' if self.peek_char() == '{' => {
                    self.read_char(); // Eat '$' char.
                    self.read_char(); // Eat '{' char.
                }
                '\\' => {
                    self.read_char(); // Eat '\' char.
                    self.read_char(); // Eat next char.
                }
                _ => {
                    self.read_char();
                }
            }

            current_char = self.current_char();
        }

        Token::new(
            TokenKind::NoSubstitutionTemplate,
            start_index,
            self.read_index,
            Some("".to_string()),
        )
    }
}
