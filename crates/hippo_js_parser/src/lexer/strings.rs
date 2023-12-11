use crate::{errors::ParserError, Lexer, Token, TokenKind};

use super::utils::{CR, LF};

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

            let potential_high_surrogate = current_char;

            match current_char {
                _ if current_char == start_quote_character => break,
                '\\' => {
                    self.read_char(); // Eat \ char.

                    current_char = self.current_char();

                    match current_char {
                        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                            string_literal.push('\\');
                            string_literal.push(current_char);
                        }
                        'x' | 'u' | '0'..='7' => {
                            if let Some(ch) = self.escape_sequence_to_char() {
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

    fn escape_sequence_to_char(&mut self) -> Option<char> {
        let current_char = self.current_char();

        let mut radix = 16;

        let code_str_option = match current_char {
            'x' => {
                self.read_char(); // Eat x char.

                self.read_hexadecimal_escape_sequence_str()
            }
            'u' => {
                self.read_char(); // Eat u char.

                if self.current_char() == '{' {
                    self.read_char(); // Eat { char.

                    self.read_code_point_escape_sequence_str()
                } else {
                    self.read_unicode_escape_sequence_str()
                }
            }
            '0'..='7' => {
                radix = 8;

                self.read_octal_escape_sequence_str()
            }
            _ => None,
        };

        if let Some(code_str) = code_str_option {
            if let Ok(unicode_unescaped_32) = u32::from_str_radix(code_str, radix) {
                if let Some(ch) = std::char::from_u32(unicode_unescaped_32) {
                    return Some(ch);
                }
            }

            return None;
        } else {
            self.errors
                .push(ParserError::InvalidEscapeSequenceCannotBeFormatted);

            return None;
        }
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    // ```text
    // HexEscapeSequence ::
    //   x HexDigit HexDigit
    // ```
    fn read_hexadecimal_escape_sequence_str(&mut self) -> Option<&str> {
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

        Some(hex_escape)
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u` Hex4Digits
    //
    // Hex4Digits ::
    //   HexDigit HexDigit HexDigit HexDigit
    // ```
    pub fn read_unicode_escape_sequence_str(&mut self) -> Option<&str> {
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

        let hex_value_u32 = u32::from_str_radix(unicode_value, 16).unwrap_or_else(|_| {
            // Next value up from 0xFFFF
            0x10000
        });

        // Check that it's not outside of range of the Basic Multilingual Plane.
        if !(0x0000..=0xFFFF).contains(&hex_value_u32) {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        Some(unicode_value)
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u{` CodePoint `}`
    //
    // CodePoint ::
    //   HexDigits[~Sep] but only if MV of HexDigits ≤ 0x10FFFF
    // ```
    fn read_code_point_escape_sequence_str(&mut self) -> Option<&str> {
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

        let code_point_value_u32 = u32::from_str_radix(code_point_value, 16).unwrap_or_else(|_| {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            // Next value up from 0x10FFFF
            0x110000
        });

        if code_point_value_u32 > 0x10FFFF {
            self.errors
                .push(ParserError::InvalidUnicodeCodePointEscapeSequence);

            return None;
        }

        if self.current_char() != '}' {
            self.errors
                .push(ParserError::InvalidUnicodeCodePointEscapeSequence);

            return None;
        }

        self.read_char(); // Eat } char.

        Some(code_point_value)
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
    fn read_octal_escape_sequence_str(&mut self) -> Option<&str> {
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

        Some(octal_value)
    }
}

pub fn is_ascii_octaldigit(ch: char) -> bool {
    match ch {
        '0'..='7' => true,
        _ => false,
    }
}
