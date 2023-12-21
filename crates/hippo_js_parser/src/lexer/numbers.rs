use crate::{errors::ParserError, Lexer, Token, TokenKind};

const NUMERIC_LITERAL_SEPARATOR: char = '_'; // TODO - Can't have similar siblings.

fn is_ascii_digit_or_separator(c: char) -> bool {
    c.is_ascii_digit() || c == NUMERIC_LITERAL_SEPARATOR
}

impl<'a> Lexer<'a> {
    pub(crate) fn scan_number_literal(&mut self) -> Token {
        let start_index: usize = self.read_index;

        let current_char = self.current_char();
        let peek_char = self.peek_char();

        let number_literal_u64 = match (current_char, peek_char) {
            ('0', '.') => {
                self.read_char(); // Eat '0' char.

                self.read_decimal_number_literal()
            }
            ('.', _) => self.read_decimal_number_literal(),
            ('0', 'b' | 'B') => self.read_non_decimal_integer_literal(
                2,
                ParserError::InvalidNonDecimalBinaryNumberLiteral,
            ),
            ('0', 'o' | 'O') => self.read_non_decimal_integer_literal(
                8,
                ParserError::InvalidNonDecimalOctalNumberLiteral,
            ),
            ('0', 'x' | 'X') => self.read_non_decimal_integer_literal(
                16,
                ParserError::InvalidNonDecimalHexadecimalNumberLiteral,
            ),
            ('1'..='9', _) => self.read_integer_literal(),
            (_, _) => Err(ParserError::InvalidNumberLiteral),
        };

        match number_literal_u64 {
            Ok(number_literal) => Token::new(
                TokenKind::NumberLiteral,
                start_index,
                self.read_index,
                Some(number_literal.to_string()),
            ),
            Err(error) => Token::new(
                TokenKind::Illegal,
                start_index,
                self.read_index,
                Some(error.to_string()),
            ),
        }
    }

    fn read_integer_literal(&mut self) -> Result<u64, ParserError> {
        let start_index: usize = self.read_index;

        let mut current_char = self.current_char();

        while is_ascii_digit_or_separator(current_char) {
            self.read_char();

            current_char = self.current_char();
        }

        let number_literal_str = &self.source_str[start_index..self.read_index];

        u64::from_str_radix(number_literal_str, 10).map_err(|_| ParserError::InvalidNumberLiteral)
    }

    fn read_decimal_number_literal(&mut self) -> Result<u64, ParserError> {
        let start_index: usize = self.read_index;

        self.read_char(); // Eat '.' char.

        let mut current_char = self.current_char();

        while is_ascii_digit_or_separator(current_char) || current_char == '.' {
            self.read_char();

            current_char = self.current_char();
        }

        let number_literal_str = &self.source_str[start_index..self.read_index];

        Ok(number_literal_str.parse::<u64>().unwrap())
    }

    fn read_non_decimal_integer_literal(
        &mut self,
        radix: u32,
        error: ParserError,
    ) -> Result<u64, ParserError> {
        self.read_char(); // Eat '0' char.
        self.read_char(); // Eat 'b', 'o' or 'x' char.

        let start_index: usize = self.read_index;

        let mut current_char = self.current_char();

        while is_ascii_digit_or_separator(current_char) {
            self.read_char();

            current_char = self.current_char();
        }

        let binary_str = &self.source_str[start_index..self.read_index];

        if let Ok(unicode_u64) = u64::from_str_radix(binary_str, radix) {
            return Ok(unicode_u64);
        }

        Err(error)
    }

    fn read_invalid_numeric_separator() {}
}
