use hippo_js_parser::{Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn strings_simple() {
    assert_lexer_eq!(
        r#""hello world""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 13)]
    );
    assert_lexer_eq!(
        "\"hello world\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 13)]
    );

    assert_lexer_eq!(
        r#""hello\n\tworld""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 16)]
    );
    assert_lexer_eq!(
        "\"hello\\b\\tworld\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 16)]
    );
}

#[test]
fn string_hexadecimal_escape_sequence() {
    assert_lexer_eq!(
        "\"hello \x4A\x61vaScript\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
    assert_lexer_eq!(
        r#""hello \x4A\x61vaScript""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 24)]
    );
}

#[test]
fn string_unicode_escape_sequence() {
    assert_lexer_eq!(
        "\"hello\\u0020world\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
    assert_lexer_eq!(
        r#""hello\u0020world""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
}

#[test]
fn string_unicode_escape_sequence_with_surrogate_pairs() {
    assert_lexer_eq!(
        "\"hello\\u0020world\\uD83D\\uDE00\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 30)]
    );
    assert_lexer_eq!(
        r#""hello\u0020world\uD83D\uDE00""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 30)]
    );
}

#[test]
fn string_with_code_points() {
    assert_lexer_eq!(
        "\"hello world \\u{1F607}\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 23)]
    );
    assert_lexer_eq!(
        r#""hello world \u{1F607}""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 23)]
    );
}

#[test]
fn string_with_complex_graphemes() {
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 46)]
    );
    assert_lexer_eq!(
        "\"abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 45)]
    );
}

#[test]
fn empty_strings() {
    assert_lexer_eq!(r#""""#, vec![Token::new(TokenKind::StringLiteral, 0, 2)]);
    assert_lexer_eq!("\"\"", vec![Token::new(TokenKind::StringLiteral, 0, 2)]);

    assert_lexer_eq!(r#"''"#, vec![Token::new(TokenKind::StringLiteral, 0, 2)]);
    assert_lexer_eq!("''", vec![Token::new(TokenKind::StringLiteral, 0, 2)]);
}
