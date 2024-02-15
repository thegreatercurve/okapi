use hippo_js_parser::{KeywordKind, Token, TokenKind, TokenValue};

use crate::lexer::{
    common::assert_lexer_eq,
    utils::{identifier, number_literal, punctuator},
};

#[test]
fn identifier_start() {
    assert_lexer_eq!("abc", vec![identifier("abc", 0, 3)]);
    assert_lexer_eq!("_123", vec![identifier("_123", 0, 4)]);
    assert_lexer_eq!("$123", vec![identifier("$123", 0, 4)]);
}

#[test]
fn identifier_part() {
    assert_lexer_eq!("abc__", vec![identifier("abc__", 0, 5)]);
    assert_lexer_eq!("__1", vec![identifier("__1", 0, 3)]);
    assert_lexer_eq!("$$$$", vec![identifier("$$$$", 0, 4)]);
}

#[test]
fn complex_keywords_and_identifiers() {
    assert_lexer_eq!(
        "const foo = 1;",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::Const),
                0,
                5,
                TokenValue::Null
            ),
            identifier("foo", 6, 9),
            punctuator(TokenKind::Assignment, 10, 11),
            number_literal("1", 1.0, 12, 13),
            punctuator(TokenKind::Semicolon, 13, 14),
        ]
    );

    assert_lexer_eq!(
        r"while (foo) { 11; };",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::While),
                0,
                5,
                TokenValue::Null
            ),
            punctuator(TokenKind::LeftParenthesis, 6, 7),
            identifier("foo", 7, 10),
            punctuator(TokenKind::RightParenthesis, 10, 11),
            punctuator(TokenKind::LeftCurlyBrace, 12, 13),
            number_literal("11", 11.0, 14, 16),
            punctuator(TokenKind::Semicolon, 16, 17),
            punctuator(TokenKind::RightCurlyBrace, 18, 19),
            punctuator(TokenKind::Semicolon, 19, 20),
        ]
    );

    assert_lexer_eq!(
        "while (foo) { 11; };",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::While),
                0,
                5,
                TokenValue::Null
            ),
            punctuator(TokenKind::LeftParenthesis, 6, 7),
            identifier("foo", 7, 10),
            punctuator(TokenKind::RightParenthesis, 10, 11),
            punctuator(TokenKind::LeftCurlyBrace, 12, 13),
            number_literal("11", 11.0, 14, 16),
            punctuator(TokenKind::Semicolon, 16, 17),
            punctuator(TokenKind::RightCurlyBrace, 18, 19),
            punctuator(TokenKind::Semicolon, 19, 20),
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Let), 0, 3, TokenValue::Null),
            identifier("baz", 4, 7),
            punctuator(TokenKind::Assignment, 8, 9),
            number_literal("1", 1.0, 10, 11),
            punctuator(TokenKind::Semicolon, 11, 12),
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3, TokenValue::Null),
            identifier("baz", 4, 7),
            punctuator(TokenKind::Assignment, 8, 9),
            number_literal("1", 1.0, 10, 11),
            punctuator(TokenKind::Semicolon, 11, 12),
        ]
    );

    // TODO Handle surrogate unciode pairs in identifiers
    // assert_lexer_eq!(
    //     r#"var \u{0042}\u{0041}z = 1;"#,
    //     vec![
    //         Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3, TokenValue::Null),
    //         punctuator("baz", 4, 7),
    //         punctuator(TokenKind::Assignment, 8, 9),
    //         punctuator(TokenKind::NumberLiteral, 10, 11),
    //         punctuator(TokenKind::Semicolon, 11, 12),
    //     ]
    // );

    // Private identifiers
    assert_lexer_eq!(
        r"class Foo { #bar = 1; };",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::Class),
                0,
                5,
                TokenValue::Null
            ),
            identifier("Foo", 6, 9),
            punctuator(TokenKind::LeftCurlyBrace, 10, 11),
            Token {
                kind: TokenKind::PrivateIdentifier,
                start: 12,
                end: 16,
                value: TokenValue::String("bar".to_string()),
            },
            punctuator(TokenKind::Assignment, 17, 18),
            number_literal("1", 1.0, 19, 20),
            punctuator(TokenKind::Semicolon, 20, 21),
            punctuator(TokenKind::RightCurlyBrace, 22, 23),
            punctuator(TokenKind::Semicolon, 23, 24),
        ]
    );
}
