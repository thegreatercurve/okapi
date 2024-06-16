use hippo_js_parser::{KeywordKind, Token, TokenKind, TokenValue};

use crate::lexer::{
    assert_lexer_eq,
    utils::{identifier, number_literal, punctuator},
};

#[test]
fn identifier_start() {
    assert_lexer_eq!("abc", vec![identifier("abc", 0, 3, 1, 1)]);
    assert_lexer_eq!("_123", vec![identifier("_123", 0, 4, 1, 1)]);
    assert_lexer_eq!("$123", vec![identifier("$123", 0, 4, 1, 1)]);
}

#[test]
fn identifier_part() {
    assert_lexer_eq!("abc__", vec![identifier("abc__", 0, 5, 1, 1)]);
    assert_lexer_eq!("__1", vec![identifier("__1", 0, 3, 1, 1)]);
    assert_lexer_eq!("$$$$", vec![identifier("$$$$", 0, 4, 1, 1)]);
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
                1,
                1,
                TokenValue::String {
                    raw: "const".to_string(),
                    value: "const".to_string()
                }
            ),
            identifier("foo", 6, 9, 1, 7),
            punctuator(TokenKind::Assignment, 10, 11, 1, 11),
            number_literal("1", 1.0, 12, 13, 1, 13),
            punctuator(TokenKind::Semicolon, 13, 14, 1, 14),
        ]
    );

    assert_lexer_eq!(
        r"while (foo) { 11; };",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::While),
                0,
                5,
                1,
                1,
                TokenValue::String {
                    raw: "while".to_string(),
                    value: "while".to_string()
                }
            ),
            punctuator(TokenKind::LeftParenthesis, 6, 7, 1, 7),
            identifier("foo", 7, 10, 1, 8),
            punctuator(TokenKind::RightParenthesis, 10, 11, 1, 11),
            punctuator(TokenKind::LeftCurlyBrace, 12, 13, 1, 13),
            number_literal("11", 11.0, 14, 16, 1, 15),
            punctuator(TokenKind::Semicolon, 16, 17, 1, 17),
            punctuator(TokenKind::RightCurlyBrace, 18, 19, 1, 19),
            punctuator(TokenKind::Semicolon, 19, 20, 1, 20),
        ]
    );

    assert_lexer_eq!(
        r"while (foo) { 11; };",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::While),
                0,
                5,
                1,
                1,
                TokenValue::String {
                    raw: "while".to_string(),
                    value: "while".to_string()
                }
            ),
            punctuator(TokenKind::LeftParenthesis, 6, 7, 1, 7),
            identifier("foo", 7, 10, 1, 8),
            punctuator(TokenKind::RightParenthesis, 10, 11, 1, 11),
            punctuator(TokenKind::LeftCurlyBrace, 12, 13, 1, 13),
            number_literal("11", 11.0, 14, 16, 1, 15),
            punctuator(TokenKind::Semicolon, 16, 17, 1, 17),
            punctuator(TokenKind::RightCurlyBrace, 18, 19, 1, 19),
            punctuator(TokenKind::Semicolon, 19, 20, 1, 20),
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::Let),
                0,
                3,
                1,
                1,
                TokenValue::String {
                    raw: "let".to_string(),
                    value: "let".to_string()
                }
            ),
            identifier("baz", 4, 7, 1, 5),
            punctuator(TokenKind::Assignment, 8, 9, 1, 9),
            number_literal("1", 1.0, 10, 11, 1, 11),
            punctuator(TokenKind::Semicolon, 11, 12, 1, 12),
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            Token::new(
                TokenKind::Keyword(KeywordKind::Var),
                0,
                3,
                1,
                1,
                TokenValue::String {
                    raw: "var".to_string(),
                    value: "var".to_string()
                }
            ),
            identifier("baz", 4, 7, 1, 5),
            punctuator(TokenKind::Assignment, 8, 9, 1, 9),
            number_literal("1", 1.0, 10, 11, 1, 11),
            punctuator(TokenKind::Semicolon, 11, 12, 1, 12),
        ]
    );

    // TODO Handle surrogate unciode pairs in identifiers
    // assert_lexer_eq!(
    //     r#"var \u{0042}\u{0041}z = 1;"#,
    //     vec![
    //         Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3, TokenValue::Null),
    //         punctuator("baz", 4, 7, 1, 7),
    //         punctuator(TokenKind::Assignment, 8, 9, 1, 9),
    //         punctuator(TokenKind::NumberLiteral, 10, 11, 1, 11),
    //         punctuator(TokenKind::Semicolon, 11, 12, 1, 12),
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
                1,
                1,
                TokenValue::String {
                    raw: "class".to_string(),
                    value: "class".to_string()
                },
            ),
            identifier("Foo", 6, 9, 1, 7),
            punctuator(TokenKind::LeftCurlyBrace, 10, 11, 1, 11),
            Token {
                kind: TokenKind::PrivateIdentifier,
                start: 12,
                end: 16,
                line: 1,
                column: 13,
                value: TokenValue::String {
                    raw: "bar".to_string(),
                    value: "bar".to_string()
                },
                line_terminator: false
            },
            punctuator(TokenKind::Assignment, 17, 18, 1, 18),
            number_literal("1", 1.0, 19, 20, 1, 20),
            punctuator(TokenKind::Semicolon, 20, 21, 1, 21),
            punctuator(TokenKind::RightCurlyBrace, 22, 23, 1, 23),
            punctuator(TokenKind::Semicolon, 23, 24, 1, 24),
        ]
    );
}
