use crate::lexer::{assert_lexer_eq, utils::string_literal};

#[test]
fn strings_simple() {
    assert_lexer_eq!(
        r"'hello world'",
        vec![string_literal("hello world", r"'hello world'", 0, 13, 1, 1)]
    );
    assert_lexer_eq!(
        "'hello\\n\\tworld'",
        vec![string_literal(
            "hello\n\tworld",
            r"'hello\n\tworld'",
            0,
            16,
            1,
            1
        )]
    );
}

#[test]
fn strings_hexadecimal_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \x4A\x61vaScript'",
        vec![string_literal(
            r"hello JavaScript",
            r"'hello \x4A\x61vaScript'",
            0,
            24,
            1,
            1
        )]
    );
}

#[test]
fn strings_unicode_escape_sequence() {
    assert_lexer_eq!(
        r"'hello\u0020world'",
        vec![string_literal(
            r"hello world",
            r"'hello\u0020world'",
            0,
            18,
            1,
            1
        )]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs_valid() {
    assert_lexer_eq!(
        r"'\uD83D\uDE00'",
        vec![string_literal(r"😀", r"'\uD83D\uDE00'", 0, 14, 1, 1)]
    );
    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{DE04}\u{1F607}'",
        vec![string_literal(
            r"hello world😄😇",
            r"'hello\u0020world\u{D83D}\u{DE04}\u{1F607}'",
            0,
            43,
            1,
            1
        )]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs_invalid() {
    // Leading surrogate is invalid.
    assert_lexer_eq!(
        r"'hello world\u{1F607}\u{DE04}'",
        vec![string_literal(
            r"hello world😇\u{56836}",
            r"'hello world\u{1F607}\u{DE04}'",
            0,
            30,
            1,
            1
        )]
    );
    // Trailing surrogate is invalid.
    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{1F607}'",
        vec![string_literal(
            r"hello world\u{55357}😇",
            r"'hello\u0020world\u{D83D}\u{1F607}'",
            0,
            35,
            1,
            1
        )]
    );
    // Invalid leading surrogate is nested.
    assert_lexer_eq!(
        r"'hello world\u{1F607}\u{D83D}\u{1F607}'",
        vec![string_literal(
            r"hello world😇\u{55357}😇",
            r"'hello world\u{1F607}\u{D83D}\u{1F607}'",
            0,
            39,
            1,
            1
        )]
    );
    // Complex combination of valid and invalid surrogate pairs.
    assert_lexer_eq!(
        r"'hello world\u{D83D}\u{D83D}\u{D83D}\u{DE04}\u{1F607}\u{DE04}'",
        vec![string_literal(
            r"hello world\u{55357}\u{55357}😄😇\u{56836}",
            r"'hello world\u{D83D}\u{D83D}\u{D83D}\u{DE04}\u{1F607}\u{DE04}'",
            0,
            62,
            1,
            1
        )]
    );
}

#[test]
fn strings_code_points_escape_sequence() {
    assert_lexer_eq!(
        r"'hello world \u{1F607}\u{1F506}'",
        vec![string_literal(
            "hello world 😇🔆",
            r"'hello world \u{1F607}\u{1F506}'",
            0,
            32,
            1,
            1
        )]
    );
}

#[test]
fn strings_octal_legacy_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \127\117rld'",
        vec![string_literal(
            "hello WOrld",
            r"'hello \127\117rld'",
            0,
            19,
            1,
            1
        )]
    );
}

#[test]
fn strings_complex_graphemes() {
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910''10🎉""#,
        vec![string_literal(
            r"abcdefghijklmnopqrstuvwxyz🙂12345678910''10🎉",
            r#""abcdefghijklmnopqrstuvwxyz🙂12345678910''10🎉""#,
            0,
            45,
            1,
            1
        )]
    );
}

#[test]
fn strings_empty() {
    assert_lexer_eq!(r#""""#, vec![string_literal("", r#""""#, 0, 2, 1, 1)]);
    assert_lexer_eq!(r"''", vec![string_literal("", r"''", 0, 2, 1, 1)]);
}

#[test]
fn strings_non_english_chars() {
    assert_lexer_eq!(
        "'دیوانه'",
        vec![string_literal("دیوانه", r"'دیوانه'", 0, 8, 1, 1)]
    );
    assert_lexer_eq!("'a℮'", vec![string_literal("a℮", r"'a℮'", 0, 4, 1, 1)]);
    assert_lexer_eq!("'℘'", vec![string_literal("℘", r"'℘'", 0, 3, 1, 1)]);
    assert_lexer_eq!("'a᧚'", vec![string_literal("a᧚", r"'a᧚'", 0, 4, 1, 1)]);
    assert_lexer_eq!(
        "'б И Й К Л О Ф Ц Ш Э ж з'",
        vec![string_literal(
            "б И Й К Л О Ф Ц Ш Э ж з",
            "'б И Й К Л О Ф Ц Ш Э ж з'",
            0,
            25,
            1,
            1
        )]
    );
}

#[test]
fn strings_line_terminators() {
    assert_lexer_eq!(
        r#""new-line";

"#,
        vec![string_literal("new-line", r#""new-line""#, 0, 10, 1, 1)]
    );
}
