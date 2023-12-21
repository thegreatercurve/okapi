use crate::{Lexer, ParserError, Token, TokenKind};

const NUMERIC_LITERAL_SEPARATOR: char = '_';

fn is_ascii_digit_or_separator(c: char) -> bool {
    c.is_ascii_digit() || c == NUMERIC_LITERAL_SEPARATOR
}

fn is_ascii_decimal_digit_or_separator(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == NUMERIC_LITERAL_SEPARATOR
}

fn is_radix_ascii_digit_or_separator(c: char, radix: u32) -> bool {
    (c.is_ascii_alphanumeric() && c.is_digit(radix)) || c == NUMERIC_LITERAL_SEPARATOR
}

fn is_ascii_octaldigit(ch: char) -> bool {
    match ch {
        '0'..='7' => true,
        _ => false,
    }
}

fn validate_numeric_literal_separators(number_literal: &str) -> Result<(), ParserError> {
    let sibling_separators = format!("{NUMERIC_LITERAL_SEPARATOR}{NUMERIC_LITERAL_SEPARATOR}",);

    if number_literal.contains(sibling_separators.as_str()) {
        return Err(ParserError::InvalidNumericSeparatorAtSibling);
    } else if number_literal.ends_with(NUMERIC_LITERAL_SEPARATOR) {
        return Err(ParserError::InvalidNumericSeparatorAtEnd);
    }

    Ok(())
}

impl<'a> Lexer<'a> {
    // 12.9.3 Numeric Literals
    // https://tc39.es/ecma262/#sec-literals-numeric-literals
    pub(crate) fn scan_number_literal(&mut self) -> Token {
        let start_index: usize = self.read_index;

        let current_char = self.current_char();
        let peek_char = self.peek_char();

        let number_literal_u64 = match (current_char, peek_char) {
            ('0', '.') | ('.', _) | ('1'..='9', _) => self.read_decimal_literal(),
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
            ('0', peek_char) if is_ascii_octaldigit(peek_char) => {
                self.read_legacy_octal_integer_literal()
            }
            ('0', _) => self.read_decimal_literal(),
            (_, _) => Err(ParserError::SyntaxError),
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

    fn read_decimal_literal(&mut self) -> Result<f64, ParserError> {
        let start_index: usize = self.read_index;

        let mut current_char = self.current_char();

        while is_ascii_decimal_digit_or_separator(current_char) {
            self.read_char();

            current_char = self.current_char();
        }

        let number_literal_str = &self.source_str[start_index..self.read_index];

        validate_numeric_literal_separators(number_literal_str)?;

        let number_literal_without_separators_str: String =
            number_literal_str.replace(NUMERIC_LITERAL_SEPARATOR, "");

        number_literal_without_separators_str
            .parse::<f64>()
            .map_err(|_| ParserError::InvalidIntegerLiteral)
    }

    fn read_non_decimal_integer_literal(
        &mut self,
        radix: u32,
        error: ParserError,
    ) -> Result<f64, ParserError> {
        self.read_char(); // Eat '0' char.
        self.read_char(); // Eat 'b', 'o' or 'x' char.

        let start_index: usize = self.read_index;

        let mut current_char = self.current_char();

        while is_radix_ascii_digit_or_separator(current_char, radix) {
            self.read_char();

            current_char = self.current_char();
        }

        let non_decimal_str = &self.source_str[start_index..self.read_index];

        validate_numeric_literal_separators(non_decimal_str)?;

        let non_decimal_without_separators_str: String =
            non_decimal_str.replace(NUMERIC_LITERAL_SEPARATOR, "");

        match u64::from_str_radix(non_decimal_without_separators_str.as_str(), radix) {
            Ok(non_decimal_u64) => Ok(non_decimal_u64 as f64),
            Err(_) => Err(error),
        }
    }

    fn read_legacy_octal_integer_literal(&mut self) -> Result<f64, ParserError> {
        if !self.config.strict_mode {
            return Err(ParserError::InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode);
        }

        let start_index: usize = self.read_index;

        self.read_char(); // Eat '0' char.

        let mut current_char = self.current_char();

        while is_ascii_octaldigit(current_char) {
            self.read_char();

            current_char = self.current_char();
        }

        let octal_str = &self.source_str[start_index..self.read_index];

        match u32::from_str_radix(octal_str, 8) {
            Ok(octal_u32) => Ok(octal_u32 as f64),
            Err(_) => Err(ParserError::InvalidLegacyOctalEscapeSequence),
        }
    }
}
