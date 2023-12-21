use crate::lexer::{common::assert_lexer_eq, utils::create_string_literal};

#[test]
fn strings_simple() {
    assert_lexer_eq!(
        r"'hello world'",
        vec![create_string_literal("hello world".to_string(), 0, 13)]
    );

    assert_lexer_eq!(
        r"'hello\n\tworld'",
        vec![create_string_literal(r"hello\n\tworld".to_string(), 0, 16)]
    );
}

#[test]
fn strings_hexadecimal_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \x4A\x61vaScript'",
        vec![create_string_literal(
            r"hello JavaScript".to_string(),
            0,
            24
        )]
    );
}

#[test]
fn strings_unicode_escape_sequence() {
    assert_lexer_eq!(
        r"'hello\u0020world'",
        vec![create_string_literal(r"hello world".to_string(), 0, 18)]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs_valid() {
    assert_lexer_eq!(
        r"'\uD83D\uDE00'",
        vec![create_string_literal(r"😀".to_string(), 0, 14)]
    );

    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{DE04}\u{1F607}'",
        vec![create_string_literal(r"hello world😄😇".to_string(), 0, 43)]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs_invalid() {
    // Leading surrogate is invalid.
    assert_lexer_eq!(
        r"'hello world\u{1F607}\u{DE04}'",
        vec![create_string_literal(
            r"hello world😇\u{56836}".to_string(),
            0,
            30
        )]
    );

    // Trailing surrogate is invalid.
    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{1F607}'",
        vec![create_string_literal(
            r"hello world\u{55357}😇".to_string(),
            0,
            35
        )]
    );

    // Invalid leading surrogate is nested.
    assert_lexer_eq!(
        r"'hello world\u{1F607}\u{D83D}\u{1F607}'",
        vec![create_string_literal(
            r"hello world😇\u{55357}😇".to_string(),
            0,
            39
        )]
    );

    // Complex combination of valid and invalid surrogate pairs.
    assert_lexer_eq!(
        r"'hello world\u{D83D}\u{D83D}\u{D83D}\u{DE04}\u{1F607}\u{DE04}'",
        vec![create_string_literal(
            r"hello world\u{55357}\u{55357}😄😇\u{56836}".to_string(),
            0,
            62
        )]
    );
}

#[test]
fn strings_code_points_escape_sequence() {
    assert_lexer_eq!(
        r"'hello world \u{1F607}\u{1F506}'",
        vec![create_string_literal("hello world 😇🔆".to_string(), 0, 32)]
    );
}

#[test]
fn strings_octal_legacy_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \127\117rld'",
        vec![create_string_literal("hello WOrld".to_string(), 0, 19)]
    );
}

#[test]
fn strings_complex_graphemes() {
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉""#,
        vec![create_string_literal(
            r"abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉".to_string(),
            0,
            46
        )]
    );
}

#[test]
fn strings_empty() {
    assert_lexer_eq!(r#""""#, vec![create_string_literal("".to_string(), 0, 2)]);
    assert_lexer_eq!(r"''", vec![create_string_literal("".to_string(), 0, 2)]);
}

#[test]
fn strings_non_english_chars() {
    assert_lexer_eq!(
        "'دیوانه'",
        vec![create_string_literal("دیوانه".to_string(), 0, 8)]
    );
    assert_lexer_eq!("'a℮'", vec![create_string_literal("a℮".to_string(), 0, 4)]);
    assert_lexer_eq!("'℘'", vec![create_string_literal("℘".to_string(), 0, 3)]);
    assert_lexer_eq!("'a᧚'", vec![create_string_literal("a᧚".to_string(), 0, 4)]);
    assert_lexer_eq!(
        "'б И Й К Л О Ф Ц Ш Э ж з'",
        vec![create_string_literal(
            "б И Й К Л О Ф Ц Ш Э ж з".to_string(),
            0,
            25
        )]
    );
}
