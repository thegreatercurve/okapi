use crate::{errors::ParserError, Lexer, Token, TokenKind};

use super::utils::{CR, LF};

enum SurrogatePair {
    LeadingInvalid(u32),
    LeadingValidTrailingInvalid(u32, u32),
    AstralCodePoint(u32),
}

fn is_ascii_octaldigit(ch: char) -> bool {
    match ch {
        '0'..='7' => true,
        _ => false,
    }
}

fn is_high_surrogate(c: u32) -> bool {
    match c {
        55296..=56319 => true, // 0xD800..=0xDBFF as u32
        _ => false,
    }
}

fn is_low_surrogate(c: u32) -> bool {
    match c {
        56320..=57343 => true, // 0xDC00..=0xDFFF as u32,
        _ => false,
    }
}

impl<'a> Lexer<'a> {
    // https://tc39.es/ecma262/#sec-literals-string-literals
    // ```text
    // StringLiteral ::
    //   " DoubleStringCharactersopt "
    //   ' SingleStringCharactersopt '
    // DoubleStringCharacters ::
    //   DoubleStringCharacter DoubleStringCharactersopt
    // SingleStringCharacters ::
    //   SingleStringCharacter SingleStringCharactersopt
    // DoubleStringCharacter ::
    //   SourceCharacter but not one of " or \ or LineTerminator
    //   <LS>
    //   <PS>
    //   \ EscapeSequence
    //   LineContinuation
    // SingleStringCharacter ::
    //   SourceCharacter but not one of ' or \ or LineTerminator
    //   <LS>
    //   <PS>
    //   \ EscapeSequence
    //   LineContinuation
    // ```
    pub(crate) fn scan_string_literal(&mut self) -> Token {
        let start_quote_character = self.current_char(); // '\'' | '"'

        let mut string_literal = String::new();

        self.read_char(); // Eat start quote char.

        let mut current_char = self.current_char();

        while current_char != start_quote_character {
            current_char = self.current_char();

            match current_char {
                _ if current_char == start_quote_character => break,
                '\\' => {
                    self.read_char(); // Eat \ char.

                    current_char = self.current_char();

                    match current_char {
                        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                            string_literal.push_str(format!("\\{}", current_char).as_str());
                        }
                        'x' => {
                            self.read_char(); // Eat x char.

                            let escape_sequence_u32 =
                                self.read_hexadecimal_escape_sequence_u32().unwrap();

                            if let Some(ch) = char::from_u32(escape_sequence_u32) {
                                string_literal.push(ch);

                                continue;
                            } else {
                                return Token::default(TokenKind::Illegal);
                            }
                        }

                        'u' => {
                            self.read_char(); // Eat u char.

                            let escape_sequence_u32 =
                                self.read_potential_unicode_or_code_point_surrogate_pairs();

                            match escape_sequence_u32 {
                                SurrogatePair::LeadingInvalid(escape_sequence_u32) => {
                                    if let Some(ch) = char::from_u32(escape_sequence_u32) {
                                        string_literal.push(ch);

                                        continue;
                                    }

                                    return Token::default(TokenKind::Illegal);
                                }
                                SurrogatePair::AstralCodePoint(escape_sequence_u32) => {
                                    if let Some(ch) = char::from_u32(escape_sequence_u32) {
                                        string_literal.push(ch);

                                        continue;
                                    }

                                    return Token::default(TokenKind::Illegal);
                                }
                                SurrogatePair::LeadingValidTrailingInvalid(
                                    leading_escape_sequence_u32,
                                    trailing_escape_sequence_u32,
                                ) => {
                                    if let Some(ch) = char::from_u32(leading_escape_sequence_u32) {
                                        string_literal.push(ch);
                                    } else {
                                        let leading_code_point_str = format!(
                                            "\\u{{{}}}",
                                            &leading_escape_sequence_u32.to_string()
                                        );

                                        string_literal.push_str(leading_code_point_str.as_str());
                                    }

                                    if let Ok(ch) = char::try_from(trailing_escape_sequence_u32) {
                                        string_literal.push(ch);

                                        continue;
                                    } else {
                                        return Token::default(TokenKind::Illegal);
                                    }
                                }
                            }
                        }
                        '0'..='7' => {
                            let escape_sequence_u32 =
                                self.read_octal_escape_sequence_u32().unwrap();

                            if let Some(ch) = char::from_u32(escape_sequence_u32) {
                                string_literal.push(ch);

                                continue;
                            } else {
                                return Token::default(TokenKind::Illegal);
                            }
                        }
                        _ => {
                            string_literal.push(current_char);
                        }
                    }
                }
                // Invalid line terminator chars.
                CR | LF => {
                    self.errors.push(ParserError::UnterminatedStringLiteral);

                    return Token::default(TokenKind::Illegal);
                }
                _ => string_literal.push(current_char),
            }

            self.read_char();
        }

        self.read_char(); // Eat end quote char.

        Token::default_string_literal(string_literal)
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    // ```text
    // HexEscapeSequence ::
    //   x HexDigit HexDigit
    // ```
    fn read_hexadecimal_escape_sequence_u32(&mut self) -> Option<u32> {
        let start_index = self.read_index;

        for _ in 0..2 {
            if !self.current_char().is_ascii_hexdigit() {
                self.errors
                    .push(ParserError::InvalidHexadecimalEscapeSequence);

                return None;
            }

            self.read_char();
        }

        let hex_escape = &self.source_str[start_index..self.read_index];

        if let Ok(hex_value_u32) = u32::from_str_radix(hex_escape, 16) {
            return Some(hex_value_u32);
        }

        self.errors
            .push(ParserError::InvalidHexadecimalEscapeSequence);

        return None;
    }

    // https://tc39.es/ecma262
    // Check for surrogate pairs before parsing unicode or code_point escape sequences.
    // > A sequence of two code units, where the first code unit c1 is a leading surrogate
    // > and the second code unit c2 a trailing surrogate, is a surrogate pair and is interpreted
    // > as a code point with the value (c1 - 0xD800) × 0x400 + (c2 - 0xDC00) + 0x10000.
    fn read_potential_unicode_or_code_point_surrogate_pairs(&mut self) -> SurrogatePair {
        let leading_surrogate = if self.current_char() == '{' {
            self.read_code_point_escape_sequence_u32()
        } else {
            self.read_unicode_escape_sequence_u32()
        }
        .unwrap();

        if !is_high_surrogate(leading_surrogate)
            && self.current_char() != '\\'
            && self.peek_char() != 'u'
        {
            return SurrogatePair::LeadingInvalid(leading_surrogate);
        }

        self.read_char(); // Eat \ char.
        self.read_char(); // Eat u char.

        let trailing_surrogate = if self.current_char() == '{' {
            self.read_code_point_escape_sequence_u32()
        } else {
            self.read_unicode_escape_sequence_u32()
        }
        .unwrap();

        if !is_low_surrogate(trailing_surrogate) {
            return SurrogatePair::LeadingValidTrailingInvalid(
                leading_surrogate,
                trailing_surrogate,
            );
        }

        let astral_code_point =
            (leading_surrogate - 55296) * 1024 + trailing_surrogate - 56320 + 65536;

        SurrogatePair::AstralCodePoint(astral_code_point)
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u` Hex4Digits
    //
    // Hex4Digits ::
    //   HexDigit HexDigit HexDigit HexDigit
    // ```
    pub fn read_unicode_escape_sequence_u32(&mut self) -> Option<u32> {
        let start_index = self.read_index;

        for _ in 0..4 {
            if !self.current_char().is_ascii_hexdigit() {
                self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

                return None;
            }

            self.read_char();
        }

        let unicode_value = &self.source_str[start_index..self.read_index];

        if unicode_value.len() < 4 {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        if let Ok(unicode_value_u32) = u32::from_str_radix(unicode_value, 16) {
            // Check that it's inside of the range of the Basic Multilingual Plane.
            if (0x0000..=0xFFFF).contains(&unicode_value_u32) {
                return Some(unicode_value_u32);
            }
        }

        self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

        return None;
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u{` CodePoint `}`
    //
    // CodePoint ::
    //   HexDigits[~Sep] but only if MV of HexDigits ≤ 0x10FFFF
    // ```
    fn read_code_point_escape_sequence_u32(&mut self) -> Option<u32> {
        self.read_char(); // Eat { char.

        let start_index = self.read_index;

        for _ in 0..6 {
            if self.current_char() == '}' {
                break;
            }

            if !self.current_char().is_ascii_hexdigit() {
                self.errors
                    .push(ParserError::InvalidUnicodeCodePointEscapeSequence);

                return None;
            }

            self.read_char();
        }

        let code_point_value = &self.source_str[start_index..self.read_index];

        if code_point_value.len() < 4 {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        if let Ok(code_point_value_u32) = u32::from_str_radix(code_point_value, 16) {
            if code_point_value_u32 < 0x10FFFF && self.current_char() == '}' {
                self.read_char(); // Eat } char.

                return Some(code_point_value_u32);
            }
        }

        self.errors
            .push(ParserError::InvalidUnicodeCodePointEscapeSequence);

        None
    }

    // https://tc39.es/ecma262/#prod-LegacyOctalEscapeSequence
    // ```text
    // LegacyOctalEscapeSequence ::
    //   0 [lookahead ∈ { 8, 9 }]
    //   NonZeroOctalDigit [lookahead ∉ OctalDigit]
    //   ZeroToThree OctalDigit [lookahead ∉ OctalDigit]
    //   FourToSeven OctalDigit
    //   ZeroToThree OctalDigit OctalDigit
    // ```
    fn read_octal_escape_sequence_u32(&mut self) -> Option<u32> {
        if !self.config.strict_mode {
            self.errors
                .push(ParserError::InvalidOctalEscapeSequenceNotAllowedInStrictMode);

            return None;
        }

        let start_index = self.read_index;

        self.read_char(); // Eat start 0..=7 char.

        let current_char = self.current_char();
        let peek_char = self.peek_char();

        match current_char {
            '1'..='3' if is_ascii_octaldigit(current_char) => self.read_char(),
            '4'..='7' if is_ascii_octaldigit(current_char) || is_ascii_octaldigit(peek_char) => {
                self.read_char_nth(2)
            }
            _ => {
                self.errors.push(ParserError::InvalidOctalEscapeSequence);

                return None;
            }
        }

        self.read_char();

        let octal_value = &self.source_str[start_index..self.read_index];

        if let Ok(octal_value_u32) = u32::from_str_radix(octal_value, 8) {
            return Some(octal_value_u32);
        }

        self.errors.push(ParserError::InvalidOctalEscapeSequence);

        return None;
    }
}
