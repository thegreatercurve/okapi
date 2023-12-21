use crate::lexer::{common::assert_lexer_eq, utils::string_literal};

#[test]
fn strings_simple() {
    assert_lexer_eq!(r"'hello world'", vec![string_literal("hello world", 0, 13)]);
    assert_lexer_eq!(
        r"'hello\n\tworld'",
        vec![string_literal(r"hello\n\tworld", 0, 16)]
    );
}

#[test]
fn strings_hexadecimal_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \x4A\x61vaScript'",
        vec![string_literal(r"hello JavaScript", 0, 24)]
    );
}

#[test]
fn strings_unicode_escape_sequence() {
    assert_lexer_eq!(
        r"'hello\u0020world'",
        vec![string_literal(r"hello world", 0, 18)]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs_valid() {
    assert_lexer_eq!(r"'\uD83D\uDE00'", vec![string_literal(r"😀", 0, 14)]);
    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{DE04}\u{1F607}'",
        vec![string_literal(r"hello world😄😇", 0, 43)]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs_invalid() {
    // Leading surrogate is invalid.
    assert_lexer_eq!(
        r"'hello world\u{1F607}\u{DE04}'",
        vec![string_literal(r"hello world😇\u{56836}", 0, 30)]
    );
    // Trailing surrogate is invalid.
    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{1F607}'",
        vec![string_literal(r"hello world\u{55357}😇", 0, 35)]
    );
    // Invalid leading surrogate is nested.
    assert_lexer_eq!(
        r"'hello world\u{1F607}\u{D83D}\u{1F607}'",
        vec![string_literal(r"hello world😇\u{55357}😇", 0, 39)]
    );
    // Complex combination of valid and invalid surrogate pairs.
    assert_lexer_eq!(
        r"'hello world\u{D83D}\u{D83D}\u{D83D}\u{DE04}\u{1F607}\u{DE04}'",
        vec![string_literal(
            r"hello world\u{55357}\u{55357}😄😇\u{56836}",
            0,
            62
        )]
    );
}

#[test]
fn strings_code_points_escape_sequence() {
    assert_lexer_eq!(
        r"'hello world \u{1F607}\u{1F506}'",
        vec![string_literal("hello world 😇🔆", 0, 32)]
    );
}

#[test]
fn strings_octal_legacy_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \127\117rld'",
        vec![string_literal("hello WOrld", 0, 19)]
    );
}

#[test]
fn strings_complex_graphemes() {
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉""#,
        vec![string_literal(
            r"abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉",
            0,
            46
        )]
    );
}

#[test]
fn strings_empty() {
    assert_lexer_eq!(r#""""#, vec![string_literal("", 0, 2)]);
    assert_lexer_eq!(r"''", vec![string_literal("", 0, 2)]);
}

#[test]
fn strings_non_english_chars() {
    assert_lexer_eq!("'دیوانه'", vec![string_literal("دیوانه", 0, 8)]);
    assert_lexer_eq!("'a℮'", vec![string_literal("a℮", 0, 4)]);
    assert_lexer_eq!("'℘'", vec![string_literal("℘", 0, 3)]);
    assert_lexer_eq!("'a᧚'", vec![string_literal("a᧚", 0, 4)]);
    assert_lexer_eq!(
        "'б И Й К Л О Ф Ц Ш Э ж з'",
        vec![string_literal("б И Й К Л О Ф Ц Ш Э ж з", 0, 25)]
    );
}
