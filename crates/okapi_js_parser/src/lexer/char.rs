use okapi_unicode::{is_unicode_id_continue, is_unicode_id_start};

// 12.2 White Space
// https://tc39.es/ecma262/#sec-white-space
const TAB: char = '\u{0009}';
const VT: char = '\u{000B}';
const FF: char = '\u{000C}';
const SP: char = '\u{0020}';
const NBSP: char = '\u{00A0}';

// 12.1 Unicode Format-Control Characters
// https://tc39.es/ecma262/#sec-unicode-format-control-characters
const ZWNJ: char = '\u{200C}'; // Used in IdentifierPart
const ZWJ: char = '\u{200D}'; // Used in IdentifierPart
const ZWNBSP: char = '\u{FEFF}'; // Used in WhiteSpace

// 12.3 Line Terminators
// https://tc39.es/ecma262/#sec-line-terminators
pub const LF: char = '\u{000A}';
pub const CR: char = '\u{000D}';
const LS: char = '\u{2028}';
const PS: char = '\u{2029}';

pub(crate) trait LexerChar {
    fn is_js_whitespace(&self) -> bool;
    fn is_line_terminator(&self) -> bool;
    fn is_identifier_start(&self) -> bool;
    fn is_identifier_part(&self) -> bool;
    fn is_punctuator_start(&self) -> bool;
    fn is_ascii_octaldigit(&self) -> bool;
}

impl LexerChar for char {
    // 12.2 White Space
    // https://tc39.es/ecma262/#sec-white-space
    fn is_js_whitespace(&self) -> bool {
        matches!(self, &TAB | &VT | &FF | &SP | &NBSP | &ZWNBSP)
    }

    // 12.3 Line Terminators
    // https://tc39.es/ecma262/#sec-line-terminators
    fn is_line_terminator(&self) -> bool {
        matches!(self, &LF | &CR | &LS | &PS)
    }

    // 12.7 Names and Keywords
    // https://tc39.es/ecma262/#sec-names-and-keywords
    fn is_identifier_start(&self) -> bool {
        if self.is_ascii() {
            matches!(self, '$' | '_') || self.is_ascii_alphabetic()
        } else {
            is_unicode_id_start(self)
        }
    }

    fn is_identifier_part(&self) -> bool {
        if self.is_ascii() {
            matches!(self, '$' | &'_') || self.is_ascii_alphanumeric()
        } else {
            matches!(self, &ZWNJ | &ZWJ) || is_unicode_id_continue(self)
        }
    }

    // 12.8 Punctuators
    // https://tc39.es/ecma262/#sec-punctuators
    fn is_punctuator_start(&self) -> bool {
        matches!(
            self,
            '{' | &'('
                | &')'
                | &'['
                | &']'
                | &'.'
                | &';'
                | &','
                | &'<'
                | &'>'
                | &'='
                | &'!'
                | &'+'
                | &'-'
                | &'*'
                | &'%'
                | &'&'
                | &'|'
                | &'^'
                | &'~'
                | &'?'
                | &':'
                | &'/'
                | &'}'
        )
    }

    // 12.9.3 Numeric Literals
    // https://tc39.es/ecma262/#sec-literals-numeric-literals
    fn is_ascii_octaldigit(&self) -> bool {
        matches!(self, '0'..='7')
    }
}
