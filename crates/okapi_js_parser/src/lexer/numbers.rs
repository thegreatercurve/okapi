use super::char::LexerChar;

use crate::{Lexer, ParserError, Token, TokenKind, TokenValue};

const NUMERIC_LITERAL_SEPARATOR: char = '_';
const DECIMAL: char = '.';
const BIG_INT_SUFFIX: char = 'n';

#[derive(PartialEq)]
enum NumKind {
    Int,
    Decimal,
    Binary,
    Octal,
    Hexadecimal,
    LegacyOctal,
    PositiveExponent,
    NegativeExponent,
    BigInt,
}

fn is_decimal_literal_char(ch: char) -> bool {
    matches!(
        ch,
        '0'..='9' | DECIMAL | NUMERIC_LITERAL_SEPARATOR | 'e' | 'E'
    )
}

fn is_non_decimal_literal_char(ch: char, radix: u32) -> bool {
    matches!(ch, NUMERIC_LITERAL_SEPARATOR | _ if ch.is_digit(radix))
}

fn match_num_kind_to_start_index_offset(num_kind: &NumKind) -> usize {
    match num_kind {
        NumKind::Binary | NumKind::Octal | NumKind::Hexadecimal => 2,
        _ => 0,
    }
}

fn match_num_kind_to_radix(num_kind: &NumKind) -> u32 {
    match num_kind {
        NumKind::Binary => 2,
        NumKind::Octal | NumKind::LegacyOctal => 8,
        NumKind::Hexadecimal => 16,
        _ => 10,
    }
}

fn match_num_kind_to_parse_error(num_kind: &NumKind) -> ParserError {
    match num_kind {
        NumKind::Int | NumKind::Decimal => ParserError::InvalidDecimalLiteral,
        NumKind::Binary => ParserError::InvalidNonDecimalBinaryNumberLiteral,
        NumKind::Octal => ParserError::InvalidNonDecimalOctalNumberLiteral,
        NumKind::Hexadecimal => ParserError::InvalidNonDecimalHexadecimalNumberLiteral,
        NumKind::LegacyOctal => ParserError::InvalidLegacyOctalNumberLiteral,
        NumKind::PositiveExponent | NumKind::NegativeExponent => {
            ParserError::InvalidExponentPartNumberLiteral
        }
        NumKind::BigInt => ParserError::InvalidDecimalBigIntegerLiteral,
    }
}

// 12.9.3 Numeric Literals
// https://tc39.es/ecma262/#sec-literals-numeric-literals
impl Lexer {
    // https://tc39.es/ecma262/#prod-NumericLiteral
    pub(crate) fn scan_number_literal(&mut self) -> Result<Token, ParserError> {
        let start_index: usize = self.read_index;

        let num_kind = match (self.current_char(), self.peek_char()) {
            ('0', '.') | ('.', _) => self.read_decimal_literal(),
            ('1'..='9', _) => self.read_decimal_literal(),
            ('0', 'b' | 'B') => self.read_non_decimal_integer_literal(NumKind::Binary),
            ('0', 'o' | 'O') => self.read_non_decimal_integer_literal(NumKind::Octal),
            ('0', 'x' | 'X') => self.read_non_decimal_integer_literal(NumKind::Hexadecimal),
            ('0', peek_char) if peek_char.is_ascii_octaldigit() => {
                self.read_legacy_octal_integer_literal()
            }
            ('0', _) => self.read_decimal_literal(),
            (_, _) => Err(ParserError::SyntaxError),
        }?;

        match num_kind {
            NumKind::BigInt => Ok(Token::new(
                TokenKind::BigIntLiteral,
                start_index,
                self.read_index,
                self.line,
                self.column,
                TokenValue::BigInt(
                    self.chars[start_index..self.read_index]
                        .iter()
                        .collect::<String>(),
                ),
            )),
            _ => {
                let number_literal_f64 = self.parse_num_str_to_f64(
                    &num_kind,
                    start_index + match_num_kind_to_start_index_offset(&num_kind),
                )?;

                Ok(Token::new(
                    TokenKind::NumberLiteral,
                    start_index,
                    self.read_index,
                    self.line,
                    self.column,
                    TokenValue::Number {
                        raw: self.chars[start_index..self.read_index]
                            .iter()
                            .collect::<String>(),
                        value: number_literal_f64,
                    },
                ))
            }
        }
    }

    // https://tc39.es/ecma262/#prod-DecimalLiteral
    fn read_decimal_literal(&mut self) -> Result<NumKind, ParserError> {
        let mut num_kind = NumKind::Int;

        while is_decimal_literal_char(self.current_char()) && !self.is_end_of_file() {
            match self.current_char() {
                DECIMAL => {
                    self.read_char(); // Eat '.' char.

                    num_kind = match self.current_char() {
                        ch if !ch.is_ascii_digit() => {
                            return Err(ParserError::InvalidDecimalLiteral);
                        }
                        NUMERIC_LITERAL_SEPARATOR => {
                            return Err(ParserError::InvalidNumericSeparatorAtSibling);
                        }
                        _ => NumKind::Decimal,
                    }
                }
                NUMERIC_LITERAL_SEPARATOR => {
                    self.read_char(); // Eat '_' char.

                    match self.current_char() {
                        NUMERIC_LITERAL_SEPARATOR | DECIMAL => {
                            return Err(ParserError::InvalidNumericSeparatorAtSibling);
                        }

                        ch if !ch.is_ascii_digit() => {
                            return Err(ParserError::InvalidNumericSeparatorAtEnd);
                        }
                        _ => {}
                    }
                }
                'e' | 'E' => {
                    self.read_char(); // Eat 'e' or 'E' char.

                    num_kind = match self.current_char() {
                        '+' => NumKind::PositiveExponent,
                        '-' => NumKind::NegativeExponent,
                        ch if ch.is_ascii_digit() => NumKind::PositiveExponent,
                        _ => {
                            return Err(ParserError::InvalidExponentPartNumberLiteral);
                        }
                    }
                }
                _ => {}
            };

            self.read_char();
        }

        if self.current_char() == BIG_INT_SUFFIX && num_kind == NumKind::Int {
            self.read_char(); // Eat 'n' char.

            return Ok(NumKind::BigInt);
        }

        Ok(num_kind)
    }

    // https://tc39.es/ecma262/#prod-NonDecimalIntegerLiteral
    fn read_non_decimal_integer_literal(
        &mut self,
        num_kind: NumKind,
    ) -> Result<NumKind, ParserError> {
        self.read_char(); // Eat '0' char.
        self.read_char(); // Eat 'b', 'o' or 'x' char.

        while is_non_decimal_literal_char(self.current_char(), match_num_kind_to_radix(&num_kind)) {
            if self.current_char() == NUMERIC_LITERAL_SEPARATOR {
                self.read_char(); // Eat '_' char.

                match self.current_char() {
                    NUMERIC_LITERAL_SEPARATOR => {
                        return Err(ParserError::InvalidNumericSeparatorAtSibling);
                    }

                    ch if !ch.is_ascii_digit() => {
                        return Err(ParserError::InvalidNumericSeparatorAtEnd);
                    }
                    _ => {}
                }
            }

            self.read_char();
        }

        if self.current_char() == BIG_INT_SUFFIX && num_kind == NumKind::Int {
            self.read_char(); // Eat 'n' char.

            return Ok(NumKind::BigInt);
        }

        Ok(num_kind)
    }

    // https://tc39.es/ecma262/#prod-LegacyOctalIntegerLiteral
    fn read_legacy_octal_integer_literal(&mut self) -> Result<NumKind, ParserError> {
        if self.context.strict_mode {
            return Err(ParserError::InvalidLegacyOctalNumberLiteralNotAllowedInStrictMode);
        }

        self.read_char(); // Eat '0' char.

        while self.current_char().is_ascii_octaldigit() {
            self.read_char();
        }

        Ok(NumKind::LegacyOctal)
    }

    fn parse_num_str_to_f64(
        &mut self,
        num_kind: &NumKind,
        start_index: usize,
    ) -> Result<f64, ParserError> {
        let number_literal_str = &self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>()
            .replace(NUMERIC_LITERAL_SEPARATOR, "");

        let radix = match_num_kind_to_radix(num_kind);

        match num_kind {
            NumKind::Int
            | NumKind::Binary
            | NumKind::Octal
            | NumKind::Hexadecimal
            | NumKind::LegacyOctal => match u64::from_str_radix(number_literal_str, radix) {
                Ok(number_literal) => Ok(number_literal as f64),
                Err(_) => Err(match_num_kind_to_parse_error(num_kind)),
            },
            NumKind::Decimal | NumKind::PositiveExponent | NumKind::NegativeExponent => {
                number_literal_str
                    .parse()
                    .map_err(|_| match_num_kind_to_parse_error(num_kind))
            }
            NumKind::BigInt => Err(match_num_kind_to_parse_error(num_kind)),
        }
    }
}
