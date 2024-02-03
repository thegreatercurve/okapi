use crate::{Lexer, Token, TokenKind, TokenValue};

// 12.8 Punctuators
// https://tc39.es/ecma262/#sec-punctuators
impl<'a> Lexer<'a> {
    // https://tc39.es/ecma262/#prod-Punctuator
    pub(crate) fn scan_punctuator(&mut self) -> Token {
        let start_index = self.read_index;

        let token_kind = match self.current_char() {
            '{' => TokenKind::LeftCurlyBrace,
            '}' => TokenKind::RightCurlyBrace,
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            '[' => TokenKind::LeftSquareBracket,
            ']' => TokenKind::RightSquareBracket,
            '.' => {
                if self.peek_char() == '.' && self.peek_char_nth(1) == '.' {
                    self.read_char_nth(2);

                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '<' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '<' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::LeftShiftAssignment
                } else if peek_char == '<' {
                    self.read_char();

                    TokenKind::LeftShift
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::LessThanOrEqual
                } else {
                    TokenKind::LessThan
                }
            }
            '>' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);
                let peek_char_2 = self.peek_char_nth(3);

                if peek_char == '>' && peek_char_1 == '>' && peek_char_2 == '=' {
                    self.read_char_nth(3);

                    TokenKind::UnsignedRightShiftAssignment
                } else if peek_char == '>' && peek_char_1 == '>' {
                    self.read_char_nth(2);

                    TokenKind::UnsignedRightShift
                } else if peek_char == '>' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::RightShiftAssignment
                } else if peek_char == '>' {
                    self.read_char();

                    TokenKind::RightShift
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::GreaterThanOrEqual
                } else {
                    TokenKind::GreaterThan
                }
            }
            '=' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '=' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::StrictEqual
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::Equal
                } else if peek_char == '>' {
                    self.read_char();

                    TokenKind::ArrowFunction
                } else {
                    TokenKind::Assignment
                }
            }
            '!' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '=' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::StrictNotEqual
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::NotEqual
                } else {
                    TokenKind::LogicalNot
                }
            }
            '+' => {
                let peek_char = self.peek_char();

                if peek_char == '=' {
                    self.read_char();

                    TokenKind::AdditionAssignment
                } else if peek_char == '+' {
                    self.read_char();

                    TokenKind::Increment
                } else {
                    TokenKind::Addition
                }
            }
            '-' => {
                let peek_char = self.peek_char();

                if peek_char == '=' {
                    self.read_char();

                    TokenKind::MinusAssignment
                } else if peek_char == '-' {
                    self.read_char();

                    TokenKind::Decrement
                } else {
                    TokenKind::Subtraction
                }
            }
            '*' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '*' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::ExponentiationAssignment
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::MultiplyAssignment
                } else if peek_char == '*' {
                    self.read_char();

                    TokenKind::Exponentiation
                } else {
                    TokenKind::Multiplication
                }
            }
            '/' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenKind::DivisionAssignment
                } else {
                    TokenKind::Division
                }
            }
            '%' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenKind::ModulusAssignment
                } else {
                    TokenKind::Modulus
                }
            }
            '&' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '&' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::LogicalAndAssignment
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::BitwiseAndAssignment
                } else if peek_char == '&' {
                    self.read_char();

                    TokenKind::LogicalAnd
                } else {
                    TokenKind::BitwiseAnd
                }
            }
            '|' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '|' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::LogicalOrAssignment
                } else if peek_char == '=' {
                    self.read_char();

                    TokenKind::BitwiseOrAssignment
                } else if peek_char == '|' {
                    self.read_char();

                    TokenKind::LogicalOr
                } else {
                    TokenKind::BitwiseOr
                }
            }
            '^' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenKind::BitwiseXorAssignment
                } else {
                    TokenKind::BitwiseXor
                }
            }
            '~' => TokenKind::BitwiseNot,
            '?' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '?' && peek_char_1 == '=' {
                    self.read_char_nth(2);

                    TokenKind::NullishCoalescingAssignment
                } else if peek_char == '?' {
                    self.read_char();

                    TokenKind::NullishCoalescing
                } else if peek_char == '.' {
                    self.read_char();

                    TokenKind::OptionalChaining
                } else {
                    TokenKind::QuestionMark
                }
            }
            ':' => TokenKind::Colon,
            _ => TokenKind::Illegal,
        };

        self.read_char();

        Token::new(token_kind, start_index, self.read_index, TokenValue::Null)
    }
}
