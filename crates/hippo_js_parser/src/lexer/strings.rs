use crate::{errors::ParserError, Lexer, TokenKind};

use super::utils::{is_ascii_octaldigit, CR, LF};

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
    pub(crate) fn scan_string_literal(&mut self) -> TokenKind {
        let start_quote_character = self.current_char(); // '\'' | '"'

        self.read_char();

        while self.current_char() != start_quote_character {
            if self.current_char() == CR || self.current_char() == LF {
                self.errors.push(ParserError::UnterminatedStringLiteral);

                return TokenKind::Illegal;
            } else if self.current_char() == '\\' {
                let valid_escape_sequence = self.read_escape_sequence();

                if !valid_escape_sequence {
                    return TokenKind::Illegal;
                }
            }

            self.read_char();
        }

        self.read_char();

        TokenKind::StringLiteral
    }

    fn read_escape_sequence(&mut self) -> Option<String> {
        match self.current_char() {
            '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                self.read_char();

                true
            }
            'x' => self.read_hex_two_digits(),
            'u' => {
                if self.peek_char() == '{' {
                    self.read_unicode_escape_sequence()
                } else {
                    self.read_code_point_escape_sequence()
                }
            }
            '0'..='7' => self.read_octal_escape_sequence(),
            _ => false,
        }
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    // ```text
    // HexEscapeSequence ::
    //   x HexDigit HexDigit
    // ```
    fn read_hex_two_digits(&mut self) -> Option<String> {
        for _ in 0..2 {
            if !self.current_char().is_ascii_hexdigit() {
                self.errors
                    .push(ParserError::InvalidHexadecimalEscapeSequence);

                return None;
            }

            self.read_char();
        }

        true
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u` Hex4Digits
    //
    // Hex4Digits ::
    //   HexDigit HexDigit HexDigit HexDigit
    // ```
    pub fn read_unicode_escape_sequence(&mut self) -> Option<String> {
        let start_index = self.read_index;

        for _ in 0..4 {
            if !self.current_char().is_ascii_hexdigit() {
                self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

                return None;
            }

            self.read_char();
        }

        let hex_value = &self.source_str[start_index..self.read_index];

        if hex_value.len() < 4 {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        let hex_value_u32 = u32::from_str_radix(hex_value, 16).unwrap_or_else(|_| {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            // Next value up from 0xFFFF
            0x10000
        });

        // Check that it's not outside of range of the Basic Multilingual Plane.
        if !(0x0000..=0xFFFF).contains(&hex_value_u32) {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        true
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u{` CodePoint `}`
    //
    // CodePoint ::
    //   HexDigits[~Sep] but only if MV of HexDigits ≤ 0x10FFFF
    // ```
    fn read_code_point_escape_sequence(&mut self) -> Option<String> {
        let start_index = self.read_index;

        for _ in 0..6 {
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

        if self.peek_char() != '}' {
            self.errors
                .push(ParserError::InvalidUnicodeCodePointEscapeSequence);

            return None;
        }

        self.read_char();

        true
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
    fn read_octal_escape_sequence(&mut self) -> Option<String> {
        if !self.config.strict_mode {
            self.errors
                .push(ParserError::InvalidOctalEscapeSequenceNotAllowedInStrictMode);

            return None;
        }

        let mut max_length = 3;

        if ('4'..='7').contains(&self.current_char()) {
            max_length = 2;
        }

        while is_ascii_octaldigit(self.current_char()) && max_length > 0 {
            self.read_char();

            max_length -= 1;
        }

        true
    }

    fn read_surrogate_pair(&mut self) -> Option<String> {
        todo!("read_hex_escape_sequence")
    }
}
