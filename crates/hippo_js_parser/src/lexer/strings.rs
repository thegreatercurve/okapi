use crate::{Lexer, ParserError, Token, TokenKind, TokenValue};

use super::char::{LexerChar, CR, LF};

enum SurrogatePair {
    LeadingInvalid(u32),
    LeadingValidMissingTrailing(u32),
    LeadingValidTrailingInvalid(u32, u32),
    AstralCodePoint(u32),
}

fn is_leading_surrogate(ch: u32) -> bool {
    matches!(ch, 55296..=56319) // 0xD800..=0xDBFF as u32
}

fn is_trailing_surrogate(ch: u32) -> bool {
    matches!(ch, 56320..=57343) // 0xDC00..=0xDFFF as u32
}

fn format_invalid_code_point_string(code_point_u32: u32) -> String {
    format!(r"\u{{{}}}", code_point_u32)
}

// 12.9.4 String Literals
// https://tc39.es/ecma262/#sec-literals-string-literals
impl Lexer {
    // https://tc39.es/ecma262/#prod-StringLiteral
    pub(crate) fn scan_string_literal(&mut self) -> Token {
        let start_index = self.read_index;

        let start_quote_character = self.current_char(); // '\'' | '"'

        let mut string_literal = String::new();

        self.read_char(); // Eat start quote char.

        while self.current_char() != start_quote_character {
            match self.current_char() {
                ch if ch == start_quote_character => break,
                '\\' => {
                    self.read_char(); // Eat '\' char.

                    match self.current_char() {
                        '\'' | '"' => {
                            string_literal.push(self.current_char());
                        }
                        'b' => string_literal.push('\u{0008}'),
                        'f' => string_literal.push('\u{000C}'),
                        'n' => string_literal.push('\n'),
                        'r' => string_literal.push('\r'),
                        't' => string_literal.push('\t'),
                        'v' => string_literal.push('\u{000B}'),
                        'x' => {
                            self.read_char(); // Eat 'x' char.

                            let escape_sequence_u32 =
                                self.read_hexadecimal_escape_sequence_u32().unwrap();

                            match char::from_u32(escape_sequence_u32) {
                                Some(ch) => {
                                    string_literal.push(ch);

                                    continue;
                                }
                                _ => {
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
                        'u' => {
                            self.read_char(); // Eat 'u' char.

                            let escape_sequence_u32 =
                                self.read_potential_unicode_or_code_point_surrogate_pairs();

                            match escape_sequence_u32 {
                                SurrogatePair::LeadingInvalid(code_point_u32) => {
                                    if let Some(ch) = char::from_u32(code_point_u32) {
                                        string_literal.push(ch);
                                    } else {
                                        string_literal.push_str(
                                            format_invalid_code_point_string(code_point_u32)
                                                .as_str(),
                                        );
                                    }

                                    continue;
                                }
                                SurrogatePair::LeadingValidMissingTrailing(code_point_u32)
                                | SurrogatePair::AstralCodePoint(code_point_u32) => {
                                    if let Some(ch) = char::from_u32(code_point_u32) {
                                        string_literal.push(ch);

                                        continue;
                                    }

                                    return Token::new(
                                        TokenKind::Illegal,
                                        start_index,
                                        self.read_index,
                                        self.line,
                                        self.column,
                                        TokenValue::Null,
                                    );
                                }
                                SurrogatePair::LeadingValidTrailingInvalid(
                                    leading_code_point_u32,
                                    trailing_code_point_u32,
                                ) => {
                                    for code_point_u32 in
                                        [leading_code_point_u32, trailing_code_point_u32]
                                    {
                                        if let Some(ch) = char::from_u32(code_point_u32) {
                                            string_literal.push(ch);
                                        } else {
                                            string_literal.push_str(
                                                format_invalid_code_point_string(code_point_u32)
                                                    .as_str(),
                                            );
                                        }
                                    }

                                    continue;
                                }
                            }
                        }
                        '0'..='7' => {
                            let escape_sequence_u32 = self.read_octal_escape_sequence().unwrap();

                            match char::from_u32(escape_sequence_u32) {
                                Some(ch) => {
                                    string_literal.push(ch);

                                    continue;
                                }
                                _ => {
                                    return Token::new(
                                        TokenKind::Illegal,
                                        start_index,
                                        self.read_index,
                                        self.line,
                                        self.column,
                                        TokenValue::Null,
                                    );
                                }
                            }
                        }
                        _ => string_literal.push(self.current_char()),
                    }
                }
                // Invalid line terminator chars.
                CR | LF => {
                    let error_str = ParserError::UnterminatedStringLiteral.to_string();

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
                _ => string_literal.push(self.current_char()),
            }

            self.read_char();
        }

        self.read_char(); // Eat end quote char.

        let raw_string_literal = self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>();

        Token::new(
            TokenKind::StringLiteral,
            start_index,
            self.read_index,
            self.line,
            self.column,
            TokenValue::String {
                raw: raw_string_literal,
                value: string_literal,
            },
        )
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    pub(crate) fn read_hexadecimal_escape_sequence_u32(&mut self) -> Result<u32, ParserError> {
        let start_index = self.read_index;

        for _ in 0..2 {
            if !self.current_char().is_ascii_hexdigit() {
                return Err(ParserError::InvalidHexadecimalEscapeSequence);
            }

            self.read_char();
        }

        let hex_str = &self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>();

        match u32::from_str_radix(hex_str, 16) {
            Ok(hex_u32) => Ok(hex_u32),
            Err(_) => Err(ParserError::InvalidHexadecimalEscapeSequence),
        }
    }

    // https://tc39.es/ecma262/#sec-ecmascript-language-types-string-type
    // Check for surrogate pairs before parsing unicode or code_point escape sequences.
    // "A sequence of two code units, where the first code unit c1 is a leading surrogate
    // and the second code unit c2 a trailing surrogate, is a surrogate pair and is interpreted
    // as a code point with the value (c1 - 0xD800) × 0x400 + (c2 - 0xDC00) + 0x10000."
    fn read_potential_unicode_or_code_point_surrogate_pairs(&mut self) -> SurrogatePair {
        let leading_surrogate = match self.current_char() {
            '{' => self.read_code_point_escape_sequence(),
            _ => self.read_unicode_escape_sequence(),
        }
        .unwrap();

        if is_leading_surrogate(leading_surrogate) {
            if self.current_char() != '\\' && self.peek_char() != 'u' {
                return SurrogatePair::LeadingValidMissingTrailing(leading_surrogate);
            }

            self.read_char(); // Eat '\' char.
            self.read_char(); // Eat 'u' char.

            let trailing_surrogate = match self.current_char() {
                '{' => self.read_code_point_escape_sequence(),
                _ => self.read_unicode_escape_sequence(),
            }
            .unwrap();

            if !is_trailing_surrogate(trailing_surrogate) {
                return SurrogatePair::LeadingValidTrailingInvalid(
                    leading_surrogate,
                    trailing_surrogate,
                );
            }

            let astral_code_point =
                (leading_surrogate - 55296) * 1024 + trailing_surrogate - 56320 + 65536;

            SurrogatePair::AstralCodePoint(astral_code_point)
        } else {
            SurrogatePair::LeadingInvalid(leading_surrogate)
        }
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    pub fn read_unicode_escape_sequence(&mut self) -> Result<u32, ParserError> {
        let start_index = self.read_index;

        for _ in 0..4 {
            if !self.current_char().is_ascii_hexdigit() {
                return Err(ParserError::InvalidUnicodeEscapeSequence);
            }

            self.read_char();
        }

        let unicode_str = &self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>();

        if unicode_str.len() < 4 {
            return Err(ParserError::InvalidUnicodeEscapeSequence);
        }

        if let Ok(unicode_u32) = u32::from_str_radix(unicode_str, 16) {
            // Check that it's inside of the range of the Basic Multilingual Plane.
            if (0x0000..=0xFFFF).contains(&unicode_u32) {
                return Ok(unicode_u32);
            }
        }

        Err(ParserError::InvalidUnicodeEscapeSequence)
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    fn read_code_point_escape_sequence(&mut self) -> Result<u32, ParserError> {
        self.read_char(); // Eat '{' char.

        let start_index = self.read_index;

        for _ in 0..6 {
            if self.current_char() == '}' {
                break;
            }

            if !self.current_char().is_ascii_hexdigit() {
                return Err(ParserError::InvalidUnicodeCodePointEscapeSequence);
            }

            self.read_char();
        }

        let code_point_str = &self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>();

        if code_point_str.len() < 4 {
            return Err(ParserError::InvalidUnicodeEscapeSequence);
        }

        if let Ok(code_point_value_u32) = u32::from_str_radix(code_point_str, 16) {
            if code_point_value_u32 < 0x10FFFF && self.current_char() == '}' {
                self.read_char(); // Eat '}' char.

                return Ok(code_point_value_u32);
            }
        }

        Err(ParserError::InvalidUnicodeCodePointEscapeSequence)
    }

    // https://tc39.es/ecma262/#prod-LegacyOctalEscapeSequence
    fn read_octal_escape_sequence(&mut self) -> Result<u32, ParserError> {
        if self.context.strict_mode {
            return Err(ParserError::InvalidLegacyOctalEscapeSequenceNotAllowedInStrictMode);
        }

        let start_index = self.read_index;

        self.read_char(); // Eat start 0..=7 char.

        match self.current_char() {
            '1'..='3' if self.current_char().is_ascii_octaldigit() => self.read_char(),
            '4'..='7'
                if self.current_char().is_ascii_octaldigit()
                    || self.peek_char().is_ascii_octaldigit() =>
            {
                self.read_char_nth(2)
            }
            _ => return Err(ParserError::InvalidLegacyOctalEscapeSequence),
        }

        self.read_char();

        let octal_str = &self.chars[start_index..self.read_index]
            .iter()
            .collect::<String>();

        match u32::from_str_radix(octal_str, 8) {
            Ok(octal_u32) => Ok(octal_u32),
            Err(_) => Err(ParserError::InvalidLegacyOctalEscapeSequence),
        }
    }
}
