pub fn is_string_literal(ch: char) -> bool {
    match ch {
        '\'' | '\"' => true,
        _ => false,
    }
}

// 12.1 Unicode Format-Control Characters
// https://tc39.es/ecma262/#sec-unicode-format-control-characters
//

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

const LF: char = '\u{000A}';
const CR: char = '\u{000D}';
const LS: char = '\u{2028}';
const PS: char = '\u{2029}';

pub fn is_line_terminator(ch: char) -> bool {
    match ch {
        LF | CR | LS | PS => true,
        _ => false,
    }
}
