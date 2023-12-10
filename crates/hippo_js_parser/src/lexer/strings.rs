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

        let mut string_literal = String::new();

        self.read_char();

        while self.current_char() != start_quote_character {
            let current_char = self.current_char();

            if current_char == CR || current_char == LF {
                self.errors.push(ParserError::UnterminatedStringLiteral);

                return TokenKind::Illegal;
            } else if current_char == '\\' {
                let peek_char = self.peek_char();

                if is_escape_sequence_that_should_be_unescaped(peek_char) {
                    self.read_char(); // Eat \ char.
                } else {
                    match peek_char {
                        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => {
                            string_literal.push(current_char);
                            string_literal.push(peek_char);

                            self.read_char_nth(2);

                            continue;
                        }
                        _ => {
                            self.errors.push(ParserError::InvalidGeneralEscapeSequence);

                            return TokenKind::Illegal;
                        }
                    }
                }

                if let Some(unescaped_char) = self.read_escape_sequence() {
                    string_literal.push(unescaped_char);
                } else {
                    self.errors.push(ParserError::InvalidGeneralEscapeSequence);

                    return TokenKind::Illegal;
                }
            } else {
                string_literal.push(self.current_char());

                self.read_char();
            }
        }

        self.read_char();

        TokenKind::StringLiteral(string_literal)
    }

    fn read_escape_sequence(&mut self) -> Option<char> {
        let current_char = self.current_char();

        match current_char {
            'x' => {
                self.read_char(); // Eat x char.

                self.read_hexadecimal_escape_sequence()
            }
            'u' => {
                self.read_char(); // Eat u char.

                if self.current_char() == '{' {
                    self.read_char(); // Eat { char.

                    self.read_code_point_escape_sequence()
                } else {
                    self.read_unicode_escape_sequence()
                }
            }
            '0'..='7' => self.read_octal_escape_sequence(),
            _ => None,
        }
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    // ```text
    // HexEscapeSequence ::
    //   x HexDigit HexDigit
    // ```
    fn read_hexadecimal_escape_sequence(&mut self) -> Option<char> {
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

        let unescaped_char = unescape_unicode_escape_sequence(hex_escape);

        if unescaped_char.is_none() {
            self.errors
                .push(ParserError::InvalidHexadecimalEscapeSequence);

            return None;
        }

        unescaped_char
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u` Hex4Digits
    //
    // Hex4Digits ::
    //   HexDigit HexDigit HexDigit HexDigit
    // ```
    pub fn read_unicode_escape_sequence(&mut self) -> Option<char> {
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
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            // Next value up from 0xFFFF
            0x10000
        });

        // Check that it's not outside of range of the Basic Multilingual Plane.
        if !(0x0000..=0xFFFF).contains(&hex_value_u32) {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        let unescaped_char = unescape_unicode_escape_sequence(unicode_value);

        if unescaped_char.is_none() {
            self.errors.push(ParserError::InvalidUnicodeEscapeSequence);

            return None;
        }

        unescaped_char
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u{` CodePoint `}`
    //
    // CodePoint ::
    //   HexDigits[~Sep] but only if MV of HexDigits ≤ 0x10FFFF
    // ```
    fn read_code_point_escape_sequence(&mut self) -> Option<char> {
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

        let unescaped_char = unescape_unicode_escape_sequence(code_point_value);

        if unescaped_char.is_none() {
            self.errors
                .push(ParserError::InvalidUnicodeCodePointEscapeSequence);

            return None;
        }

        unescaped_char
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
    fn read_octal_escape_sequence(&mut self) -> Option<char> {
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

        Some('x')
    }
}

fn is_escape_sequence_that_should_be_unescaped(ch: char) -> bool {
    match ch {
        'x' | 'u' | '0'..='7' => true,
        _ => false,
    }
}

fn unescape_unicode_escape_sequence(unicode: &str) -> Option<char> {
    if let Ok(unicode_unescaped_32) = u32::from_str_radix(unicode, 16) {
        if let Some(ch) = std::char::from_u32(unicode_unescaped_32) {
            return Some(ch);
        }
    }

    None
}
