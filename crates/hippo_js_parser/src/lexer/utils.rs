// 12.1 Unicode Format-Control Characters
// https://tc39.es/ecma262/#sec-unicode-format-control-characters
//

use hippo_unicode::{is_unicode_id_continue, is_unicode_id_start};

const ZWNJ: char = '\u{200C}'; // Used in IdentifierPart
const ZWJ: char = '\u{200D}'; // Used in IdentifierPart
const ZWNBSP: char = '\u{FEFF}'; // Used in WhiteSpace

// 12.2 White Space
// https://tc39.es/ecma262/#sec-white-space
const TAB: char = '\u{0009}';
const VT: char = '\u{000B}';
const FF: char = '\u{000C}';
const SP: char = '\u{0020}';
const NBSP: char = '\u{00A0}';

pub fn is_whitespace(ch: char) -> bool {
    match ch {
        TAB | VT | FF | SP | NBSP | ZWNBSP => true,
        _ => false,
    }
}

// 12.3 Line Terminators
// https://tc39.es/ecma262/#sec-line-terminators

pub const LF: char = '\u{000A}';
pub const CR: char = '\u{000D}';
const LS: char = '\u{2028}';
const PS: char = '\u{2029}';

pub fn is_line_terminator(ch: char) -> bool {
    match ch {
        LF | CR | LS | PS => true,
        _ => false,
    }
}

pub fn is_identifier_start(ch: char) -> bool {
    if ch.is_ascii() {
        ch == '$' || ch == '_' || ch.is_ascii_alphabetic()
    } else {
        is_unicode_id_start(ch)
    }
}

pub fn is_identifier_part(ch: char) -> bool {
    if ch.is_ascii() {
        ch == '$' || ch == '_' || ch.is_ascii_alphanumeric()
    } else {
        is_unicode_id_continue(ch) || ch == ZWNJ || ch == ZWJ
    }
}

pub fn is_punctuator_start(ch: char) -> bool {
    match ch {
        '{' | '(' | ')' | '[' | ']' | '.' | ';' | ',' | '<' | '>' | '=' | '!' | '+' | '-' | '*'
        | '%' | '&' | '|' | '^' | '~' | '?' | ':' | '/' | '}' => true,
        _ => false,
    }
}

pub fn is_ascii_octaldigit(ch: char) -> bool {
    match ch {
        '0'..='7' => true,
        _ => false,
    }
}

pub fn is_regular_expression_first_char(ch: char) -> bool {
    println!(
        "is_regular_expression_first_char: {}",
        is_line_terminator(ch)
    );

    match ch {
        '*' | '\\' | '/' | '[' => false,
        _ if is_line_terminator(ch) => false,
        _ => true,
    }
}
