use hippo_js_parser::Token;

use crate::lexer::common::assert_lexer_eq;

#[test]
fn strings_simple() {
    assert_lexer_eq!(
        r"'hello world'",
        vec![Token::string_literal("hello world".to_string(), 0, 13)]
    );

    assert_lexer_eq!(
        r"'hello\n\tworld'",
        vec![Token::string_literal(r"hello\n\tworld".to_string(), 0, 16)]
    );
}

#[test]
fn strings_hexadecimal_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \x4A\x61vaScript'",
        vec![Token::string_literal(
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
        vec![Token::string_literal(r"hello world".to_string(), 0, 18)]
    );
}

#[test]
fn strings_escape_sequence_with_surrogate_pairs() {
    assert_lexer_eq!(
        r"'\uD83D\uDE00'",
        vec![Token::string_literal(r"😀".to_string(), 0, 14)]
    );

    assert_lexer_eq!(
        r"'hello\u0020world\u{D83D}\u{DE04}\u{1F607}'",
        vec![Token::string_literal(r"hello world😄😇".to_string(), 0, 43)]
    );

    // Second trailing surogate is invalid.
    assert_lexer_eq!(
        r"'hello\u0020world\u{DE04}\u{1F607}'",
        vec![Token::string_literal(r"hello world😄😇".to_string(), 0, 43)]
    );
}

#[test]
fn strings_code_points_escape_sequence() {
    assert_lexer_eq!(
        r"'hello world \u{1F607}\u{1F506}'",
        vec![Token::string_literal("hello world 😇🔆".to_string(), 0, 32)]
    );
}

#[test]
fn strings_octal_legacy_escape_sequence() {
    assert_lexer_eq!(
        r"'hello \127\117rld'",
        vec![Token::string_literal("hello WOrld".to_string(), 0, 19)]
    );
}

#[test]
fn strings_complex_graphemes() {
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉""#,
        vec![Token::string_literal(
            r"abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉".to_string(),
            0,
            46
        )]
    );
}

#[test]
fn strings_empty() {
    assert_lexer_eq!(r#""""#, vec![Token::string_literal("".to_string(), 0, 2)]);
    assert_lexer_eq!(r"''", vec![Token::string_literal("".to_string(), 0, 2)]);
}

#[test]
fn strings_non_english_chars() {
    assert_lexer_eq!(
        "'دیوانه'",
        vec![Token::string_literal("دیوانه".to_string(), 0, 8)]
    );
    assert_lexer_eq!("'a℮'", vec![Token::string_literal("a℮".to_string(), 0, 4)]);
    assert_lexer_eq!("'℘'", vec![Token::string_literal("℘".to_string(), 0, 3)]);
    assert_lexer_eq!("'a᧚'", vec![Token::string_literal("a᧚".to_string(), 0, 4)]);
    assert_lexer_eq!(
        "'б И Й К Л О Ф Ц Ш Э ж з'",
        vec![Token::string_literal(
            "б И Й К Л О Ф Ц Ш Э ж з".to_string(),
            0,
            25
        )]
    );
}
