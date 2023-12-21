use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::{
    common::assert_lexer_eq,
    utils::{identifier, number_literal},
};

#[test]
fn identifier_start() {
    assert_lexer_eq!("abc", vec![identifier("abc".to_string(), 0, 3)]);
    assert_lexer_eq!("_123", vec![identifier("_123".to_string(), 0, 4)]);
    assert_lexer_eq!("$123", vec![identifier("$123".to_string(), 0, 4)]);
}

#[test]
fn identifier_part() {
    assert_lexer_eq!("abc__", vec![identifier("abc__".to_string(), 0, 5)]);
    assert_lexer_eq!("__1", vec![identifier("__1".to_string(), 0, 3)]);
    assert_lexer_eq!("$$$$", vec![identifier("$$$$".to_string(), 0, 4)]);
}

#[test]
fn compelx_keywords_and_identifiers() {
    assert_lexer_eq!(
        "const foo = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5, None),
            identifier("foo".to_string(), 6, 9),
            Token::new(TokenKind::Assignment, 10, 11, None),
            number_literal("1".to_string(), 12, 13),
            Token::new(TokenKind::Semicolon, 13, 14, None),
        ]
    );

    assert_lexer_eq!(
        r"while (foo) { 11; };",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::While), 0, 5, None),
            Token::new(TokenKind::LeftParenthesis, 6, 7, None),
            identifier("foo".to_string(), 7, 10),
            Token::new(TokenKind::RightParenthesis, 10, 11, None),
            Token::new(TokenKind::LeftCurlyBrace, 12, 13, None),
            number_literal("11".to_string(), 14, 16),
            Token::new(TokenKind::Semicolon, 16, 17, None),
            Token::new(TokenKind::RightCurlyBrace, 18, 19, None),
            Token::new(TokenKind::Semicolon, 19, 20, None),
        ]
    );

    assert_lexer_eq!(
        "while (foo) { 11; };",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::While), 0, 5, None),
            Token::new(TokenKind::LeftParenthesis, 6, 7, None),
            identifier("foo".to_string(), 7, 10),
            Token::new(TokenKind::RightParenthesis, 10, 11, None),
            Token::new(TokenKind::LeftCurlyBrace, 12, 13, None),
            number_literal("11".to_string(), 14, 16),
            Token::new(TokenKind::Semicolon, 16, 17, None),
            Token::new(TokenKind::RightCurlyBrace, 18, 19, None),
            Token::new(TokenKind::Semicolon, 19, 20, None),
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Let), 0, 3, None),
            identifier("baz".to_string(), 4, 7),
            Token::new(TokenKind::Assignment, 8, 9, None),
            number_literal("1".to_string(), 10, 11),
            Token::new(TokenKind::Semicolon, 11, 12, None),
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3, None),
            identifier("baz".to_string(), 4, 7),
            Token::new(TokenKind::Assignment, 8, 9, None),
            number_literal("1".to_string(), 10, 11),
            Token::new(TokenKind::Semicolon, 11, 12, None),
        ]
    );

    // TODO Handle surrogate unciode pairs in identifiers
    // assert_lexer_eq!(
    //     r#"var \u{0042}\u{0041}z = 1;"#,
    //     vec![
    //         Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3, None),
    //         Token::new("baz".to_string(), 4, 7, None),
    //         Token::new(TokenKind::Assignment, 8, 9, None),
    //         Token::new(TokenKind::NumberLiteral, 10, 11, None),
    //         Token::new(TokenKind::Semicolon, 11, 12, None),
    //     ]
    // );

    // Private identifiers
    assert_lexer_eq!(
        r"class Foo { #bar = 1; };",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Class), 0, 5, None),
            identifier("Foo".to_string(), 6, 9),
            Token::new(TokenKind::LeftCurlyBrace, 10, 11, None),
            identifier("#bar".to_string(), 12, 16),
            Token::new(TokenKind::Assignment, 17, 18, None),
            number_literal("1".to_string(), 19, 20),
            Token::new(TokenKind::Semicolon, 20, 21, None),
            Token::new(TokenKind::RightCurlyBrace, 22, 23, None),
            Token::new(TokenKind::Semicolon, 23, 24, None),
        ]
    );
}
