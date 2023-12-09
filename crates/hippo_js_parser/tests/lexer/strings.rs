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
        "\"hellobtworld\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 16)]
    );
}

#[test]
fn strings_hexadecimal_escape_sequence() {
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
fn strings_unicode_escape_sequence() {
    assert_lexer_eq!(
        "\"hellou0020world\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
    assert_lexer_eq!(
        r#""hello\u0020world""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 18)]
    );
}

#[test]
fn strings_unicode_escape_sequence_with_surrogate_pairs() {
    assert_lexer_eq!(
        "\"hellou0020worlduD83DuDE00\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 30)]
    );
    assert_lexer_eq!(
        r#""hello\u0020world\uD83D\uDE00""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 30)]
    );
}

#[test]
fn strings_with_code_points() {
    assert_lexer_eq!(
        "\"hello world u{1F607}\"",
        vec![Token::new(TokenKind::StringLiteral, 0, 23)]
    );
    assert_lexer_eq!(
        r#""hello world \u{1F607}""#,
        vec![Token::new(TokenKind::StringLiteral, 0, 23)]
    );
}

#[test]
fn strings_with_complex_graphemes() {
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
fn strings_empty() {
    assert_lexer_eq!(r#""""#, vec![Token::new(TokenKind::StringLiteral, 0, 2)]);
    assert_lexer_eq!("\"\"", vec![Token::new(TokenKind::StringLiteral, 0, 2)]);

    assert_lexer_eq!(r#"''"#, vec![Token::new(TokenKind::StringLiteral, 0, 2)]);
    assert_lexer_eq!("''", vec![Token::new(TokenKind::StringLiteral, 0, 2)]);
}

#[test]
fn strings_non_english() {
    // assert_lexer_eq!("'دیوانه'", vec![Token::new(TokenKind::StringLiteral, 0, 6)]);
    // assert_lexer_eq!("a℮", vec![Token::new(TokenKind::StringLiteral, 0, 8)]);
    // assert_lexer_eq!("℘", vec![Token::new(TokenKind::StringLiteral, 0, 8)]);
    // assert_lexer_eq!("a᧚", vec![Token::new(TokenKind::StringLiteral, 0, 8)]);
    // assert_lexer_eq!(
    //     "б И Й К Л О Ф Ц Ш Э ж з",
    //     vec![Token::new(TokenKind::StringLiteral, 0, 22)]
    // );
}
