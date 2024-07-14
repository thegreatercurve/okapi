use crate::Lexer;

use super::char::LexerChar;

// 12.4 Comments
// https://tc39.es/ecma262/#sec-comments
impl Lexer {
    pub(crate) fn skip_single_line_comment(&mut self) {
        self.read_char(); // Eat '/' char.
        self.read_char(); // Eat '/' char.

        while !self.is_end_of_file() {
            match self.current_char() {
                ch if ch.is_line_terminator() => {
                    self.read_char(); // Eat line terminator.

                    break;
                }
                _ => self.read_char(),
            }
        }
    }

    pub(crate) fn skip_multi_line_comment(&mut self) {
        self.read_char(); // Eat '/' char.
        self.read_char(); // Eat '*' char.

        while !self.is_end_of_file() {
            match (self.current_char(), self.peek_char()) {
                ('*', '/') => {
                    self.read_char(); // Eat '*' char.
                    self.read_char(); // Eat '/' char.

                    break;
                }
                _ => self.read_char(),
            }
        }
    }
}
