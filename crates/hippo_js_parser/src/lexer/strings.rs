use crate::{errors::ParserError, Lexer, Token, TokenKind};

use super::utils::{CR, LF};

enum SurrogatePair {
    LeadingInvalid(u32),
    LeadingValidMissingTrailing(u32),
    LeadingValidTrailingInvalid(u32, u32),
    AstralCodePoint(u32),
}

fn is_ascii_octaldigit(ch: char) -> bool {
    match ch {
        '0'..='7' => true,
        _ => false,
    }
}

fn is_leading_surrogate(c: u32) -> bool {
    match c {
        55296..=56319 => true, // 0xD800..=0xDBFF as u32
        _ => false,
    }
}

fn is_trailing_surrogate(c: u32) -> bool {
    match c {
        56320..=57343 => true, // 0xDC00..=0xDFFF as u32,
        _ => false,
    }
}

fn format_invalid_code_point_string(code_point_u32: u32) -> String {
    format!(r"\u{{{}}}", code_point_u32.to_string())
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
        let start_index = self.read_index;

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
                                return Token::new(
                                    TokenKind::Illegal,
                                    start_index,
                                    self.read_index,
                                    Some(ParserError::InvalidHexadecimalEscapeSequence.to_string()),
                                );
                            }
                        }

                        'u' => {
                            self.read_char(); // Eat u char.

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
                                        None,
                                    );
                                }
                                SurrogatePair::LeadingValidTrailingInvalid(
                                    leading_code_point_u32,
                                    trailing_code_point_u32,
                                ) => {
                                    for code_point_u32 in
                                        vec![leading_code_point_u32, trailing_code_point_u32]
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
                            let escape_sequence_u32 =
                                self.read_octal_escape_sequence_u32().unwrap();

                            if let Some(ch) = char::from_u32(escape_sequence_u32) {
                                string_literal.push(ch);

                                continue;
                            } else {
                                return Token::new(
                                    TokenKind::Illegal,
                                    start_index,
                                    self.read_index,
                                    None,
                                );
                            }
                        }
                        _ => {
                            string_literal.push(current_char);
                        }
                    }
                }
                // Invalid line terminator chars.
                CR | LF => {
                    return Token::new(
                        TokenKind::Illegal,
                        start_index,
                        self.read_index,
                        Some(ParserError::UnterminatedStringLiteral.to_string()),
                    );
                }
                _ => string_literal.push(current_char),
            }

            self.read_char();
        }

        self.read_char(); // Eat end quote char.

        Token::new(
            TokenKind::StringLiteral,
            start_index,
            self.read_index,
            Some(string_literal),
        )
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    // ```text
    // HexEscapeSequence ::
    //   x HexDigit HexDigit
    // ```
    fn read_hexadecimal_escape_sequence_u32(&mut self) -> Result<u32, ParserError> {
        let start_index = self.read_index;

        for _ in 0..2 {
            if !self.current_char().is_ascii_hexdigit() {
                return Err(ParserError::InvalidHexadecimalEscapeSequence);
            }

            self.read_char();
        }

        let hex_str = &self.source_str[start_index..self.read_index];

        if let Ok(hex_u32) = u32::from_str_radix(hex_str, 16) {
            return Ok(hex_u32);
        }

        Err(ParserError::InvalidHexadecimalEscapeSequence)
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

        if is_leading_surrogate(leading_surrogate) {
            if self.current_char() != '\\' && self.peek_char() != 'u' {
                return SurrogatePair::LeadingValidMissingTrailing(leading_surrogate);
            }

            self.read_char(); // Eat \ char.
            self.read_char(); // Eat u char.

            let trailing_surrogate = if self.current_char() == '{' {
                self.read_code_point_escape_sequence_u32()
            } else {
                self.read_unicode_escape_sequence_u32()
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
            return SurrogatePair::LeadingInvalid(leading_surrogate);
        }
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u` Hex4Digits
    //
    // Hex4Digits ::
    //   HexDigit HexDigit HexDigit HexDigit
    // ```
    pub fn read_unicode_escape_sequence_u32(&mut self) -> Result<u32, ParserError> {
        let start_index = self.read_index;

        for _ in 0..4 {
            if !self.current_char().is_ascii_hexdigit() {
                return Err(ParserError::InvalidUnicodeEscapeSequence);
            }

            self.read_char();
        }

        let unicode_str = &self.source_str[start_index..self.read_index];

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
    // ```text
    // UnicodeEscapeSequence ::
    //   `u{` CodePoint `}`
    //
    // CodePoint ::
    //   HexDigits[~Sep] but only if MV of HexDigits ≤ 0x10FFFF
    // ```
    fn read_code_point_escape_sequence_u32(&mut self) -> Result<u32, ParserError> {
        self.read_char(); // Eat { char.

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

        let code_point_str = &self.source_str[start_index..self.read_index];

        if code_point_str.len() < 4 {
            return Err(ParserError::InvalidUnicodeEscapeSequence);
        }

        if let Ok(code_point_value_u32) = u32::from_str_radix(code_point_str, 16) {
            if code_point_value_u32 < 0x10FFFF && self.current_char() == '}' {
                self.read_char(); // Eat } char.

                return Ok(code_point_value_u32);
            }
        }

        Err(ParserError::InvalidUnicodeCodePointEscapeSequence)
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
    fn read_octal_escape_sequence_u32(&mut self) -> Result<u32, ParserError> {
        if !self.config.strict_mode {
            return Err(ParserError::InvalidOctalEscapeSequenceNotAllowedInStrictMode);
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
            _ => return Err(ParserError::InvalidOctalEscapeSequence),
        }

        self.read_char();

        let octal_str = &self.source_str[start_index..self.read_index];

        if let Ok(octal_u32) = u32::from_str_radix(octal_str, 8) {
            return Ok(octal_u32);
        }

        Err(ParserError::InvalidOctalEscapeSequence)
    }
}
