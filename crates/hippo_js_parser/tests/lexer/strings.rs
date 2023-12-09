use std::collections::VecDeque;

use hippo_js_parser::{Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn strings() {
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

    // Hexadecimal escape sequence
    assert_lexer_eq!(
        "\"hello \x4A\x61vaScript\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
    assert_lexer_eq!(
        r#""hello \x4A\x61vaScript""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 24)]
    );

    // Unicode escape sequence
    assert_lexer_eq!(
        "\"hello\\u0020world\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
    assert_lexer_eq!(
        r#""hello\u0020world""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );

    // Unicode escape sequence with surrogate pairs
    assert_lexer_eq!(
        "\"hello\\u0020world\\uD83D\\uDE00\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 30)]
    );
    assert_lexer_eq!(
        r#""hello\u0020world\uD83D\uDE00""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 30)]
    );

    // Code point escape sequence with surrogate pairs
    assert_lexer_eq!(
        "\"hello world \\u{1F607}\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 23)]
    );
    assert_lexer_eq!(
        r#""hello world \u{1F607}""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 23)]
    );

    // Unescaped string with complex graphemes.
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 46)]
    );
    assert_lexer_eq!(
        "\"abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 45)]
    );
}
