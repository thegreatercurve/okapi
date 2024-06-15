use crate::{GoalSymbol, Lexer, ParserError, Token, TokenKind, TokenValue};

use super::char::LexerChar;

// 12.9.6 Template Literal Lexical Components
// https://tc39.es/ecma262/#sec-template-literal-lexical-components
impl Lexer {
    // https://tc39.es/ecma262/#prod-Template
    pub(crate) fn scan_template_literal(&mut self) -> Token {
        let is_head = self.current_char() == '`';
        let mut is_tail = false;

        self.read_char(); // Eat '`' char.

        let start_index = self.read_index;
        let mut end_index = self.read_index;

        let mut cooked_string_literal = String::new();

        while self.current_char() != '\0' {
            match self.current_char() {
                '`' => {
                    self.goal_symbol = GoalSymbol::InputElementRegExp;

                    end_index = self.read_index;

                    self.read_char(); // Eat '`' char.

                    is_tail = true;

                    break;
                }
                '$' if self.peek_char() == '{' => {
                    self.goal_symbol = GoalSymbol::InputElementRegExpOrTemplateTail;

                    end_index = self.read_index;

                    self.read_char(); // Eat '$' char.
                    self.read_char(); // Eat '{' char.

                    is_tail = false;

                    break;
                }
                '\\' => {
                    self.read_char(); // Eat '\' char.

                    match self.current_char() {
                        '\\' | '`' | '$' => {
                            cooked_string_literal.push(self.current_char());

                            self.read_char() // Eat escape sequence or `$` char.
                        }
                        'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                            cooked_string_literal
                                .push_str(format!("\\{}", self.current_char()).as_str());

                            self.read_char() // Eat escape sequence char.
                        }
                        // TODO Add more here
                        'x' => {
                            self.read_char(); // Eat 'x' char.

                            let escape_sequence_u32 =
                                self.read_hexadecimal_escape_sequence_u32().unwrap();

                            match char::from_u32(escape_sequence_u32) {
                                Some(ch) => {
                                    cooked_string_literal.push(ch);

                                    continue;
                                }
                                None => {
                                    let error_str =
                                        ParserError::InvalidHexadecimalEscapeSequence.to_string();

                                    return Token::new(
                                        TokenKind::Illegal,
                                        start_index,
                                        self.read_index,
                                        self.line,
                                        self.column,
                                        TokenValue::String {
                                            raw: error_str.clone(),
                                            value: error_str,
                                        },
                                    );
                                }
                            }
                        }
                        ch if ch.is_line_terminator() => {
                            self.read_char(); // Eat line terminator char.

                            continue;
                        }
                        ch => cooked_string_literal.push(ch),
                    }
                }
                _ => {
                    cooked_string_literal.push(self.current_char());

                    self.read_char()
                }
            };
        }

        let raw_string_literal = self.chars[start_index..end_index]
            .iter()
            .collect::<String>();

        let token_kind = match (is_head, is_tail) {
            (true, true) => TokenKind::TemplateNoSubstitution,
            (true, false) => TokenKind::TemplateHead,
            (false, true) => TokenKind::TemplateTail,
            (false, false) => TokenKind::TemplateMiddle,
        };

        if is_tail {
            // Reset the goal symbol to `InputElementDiv` once the template literal is finished.
            self.goal_symbol = GoalSymbol::InputElementDiv;
        }

        Token::new(
            token_kind,
            start_index,
            end_index,
            self.line,
            self.column,
            TokenValue::Template {
                raw: raw_string_literal,
                cooked: cooked_string_literal,
            },
        )
    }
}
