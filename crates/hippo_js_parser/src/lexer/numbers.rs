use crate::{errors::ParserError, Lexer, Token, TokenKind};

const NUMERIC_LITERAL_SEPARATOR: char = '_'; // TODO - Can't have similar siblings.

impl<'a> Lexer<'a> {
    pub(crate) fn scan_integer_literal(&mut self) -> Token {
        let start_index: usize = self.read_index;

        let mut current_char = self.current_char();

        while current_char.is_ascii_alphanumeric() || current_char == NUMERIC_LITERAL_SEPARATOR {
            self.read_char();

            current_char = self.current_char();
        }

        let number_literal_str = &self.source_str[start_index..self.read_index];

        Token::number_literal(number_literal_str.to_string(), start_index, self.read_index)
    }

    pub(crate) fn scan_decimal_number_literal(&mut self) -> Token {
        let start_index: usize = self.read_index;

        self.read_char(); // Eat '.' char.

        let mut current_char = self.current_char();

        while current_char.is_ascii_alphanumeric() || current_char == NUMERIC_LITERAL_SEPARATOR {
            self.read_char();

            current_char = self.current_char();
        }

        let number_literal_str = &self.source_str[start_index..self.read_index];

        Token::number_literal(number_literal_str.to_string(), start_index, self.read_index)
    }

    pub(crate) fn scan_non_decimal_integer_literal(&mut self) -> Token {
        let start_index: usize = self.read_index;

        self.read_char(); // Eat '0' char.

        let current_char = self.current_char();

        let radix_and_error = match current_char {
            'b' | 'B' => Some((2, ParserError::InvalidNonDecimalBinaryNumberLiteral)),
            'o' | 'O' => Some((8, ParserError::InvalidNonDecimalOctalNumberLiteral)),
            'x' | 'X' => Some((16, ParserError::InvalidNonDecimalHexadecimalNumberLiteral)),
            _ => None,
        };

        if let Some((radix, error)) = radix_and_error {
            let number_literal_u64 = self.read_non_decimal_integer_literal(radix, error);

            if let Some(number_literal_u64) = number_literal_u64 {
                return Token::number_literal(
                    number_literal_u64.to_string(),
                    start_index,
                    self.read_index,
                );
            }
        }

        self.errors
            .push(ParserError::InvalidNonDecimalNumberLiteral);

        Token::default(TokenKind::Illegal)
    }

    fn read_non_decimal_integer_literal(&mut self, radix: u32, error: ParserError) -> Option<u64> {
        self.read_char(); // Eat 'b', 'o' or 'x' char.

        let start_index = self.read_index;

        let mut current_char = self.current_char();

        while current_char.is_digit(radix) || current_char == NUMERIC_LITERAL_SEPARATOR {
            self.read_char();

            current_char = self.current_char();
        }

        let binary_str = &self.source_str[start_index..self.read_index];

        if let Ok(unicode_u64) = u64::from_str_radix(binary_str, radix) {
            return Some(unicode_u64);
        }

        self.errors.push(error);

        return None;
    }

    fn read_invalid_numeric_separator() {}
}
