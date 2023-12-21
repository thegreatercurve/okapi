use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn keywords_and_identifiers() {
    assert_lexer_eq!(
        "const foo = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5),
            Token::identifier("foo".to_string(), 6, 9),
            Token::new(TokenKind::Assignment, 10, 11),
            Token::number_literal("1".to_string(), 12, 13),
            Token::new(TokenKind::Semicolon, 13, 14),
        ]
    );

    assert_lexer_eq!(
        r"while (foo) { 11; };",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::While), 0, 5),
            Token::new(TokenKind::LeftParenthesis, 6, 7),
            Token::identifier("foo".to_string(), 7, 10),
            Token::new(TokenKind::RightParenthesis, 10, 11),
            Token::new(TokenKind::LeftCurlyBrace, 12, 13),
            Token::number_literal("11".to_string(), 14, 16),
            Token::new(TokenKind::Semicolon, 16, 17),
            Token::new(TokenKind::RightCurlyBrace, 18, 19),
            Token::new(TokenKind::Semicolon, 19, 20),
        ]
    );

    assert_lexer_eq!(
        "while (foo) { 11; };",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::While), 0, 5),
            Token::new(TokenKind::LeftParenthesis, 6, 7),
            Token::identifier("foo".to_string(), 7, 10),
            Token::new(TokenKind::RightParenthesis, 10, 11),
            Token::new(TokenKind::LeftCurlyBrace, 12, 13),
            Token::number_literal("11".to_string(), 14, 16),
            Token::new(TokenKind::Semicolon, 16, 17),
            Token::new(TokenKind::RightCurlyBrace, 18, 19),
            Token::new(TokenKind::Semicolon, 19, 20),
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Let), 0, 3),
            Token::identifier("baz".to_string(), 4, 7),
            Token::new(TokenKind::Assignment, 8, 9),
            Token::number_literal("1".to_string(), 10, 11),
            Token::new(TokenKind::Semicolon, 11, 12),
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3),
            Token::identifier("baz".to_string(), 4, 7),
            Token::new(TokenKind::Assignment, 8, 9),
            Token::number_literal("1".to_string(), 10, 11),
            Token::new(TokenKind::Semicolon, 11, 12),
        ]
    );

    // TODO Handle surrogate unciode pairs in identifiers
    // assert_lexer_eq!(
    //     r#"var \u{0042}\u{0041}z = 1;"#,
    //     vec![
    //         Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3),
    //         Token::new("baz".to_string(), 4, 7),
    //         Token::new(TokenKind::Assignment, 8, 9),
    //         Token::new(TokenKind::NumberLiteral, 10, 11),
    //         Token::new(TokenKind::Semicolon, 11, 12),
    //     ]
    // );

    // Private identifiers
    assert_lexer_eq!(
        r"class Foo { #bar = 1; };",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Class), 0, 5),
            Token::identifier("Foo".to_string(), 6, 9),
            Token::new(TokenKind::LeftCurlyBrace, 10, 11),
            Token::identifier("#bar".to_string(), 12, 16),
            Token::new(TokenKind::Assignment, 17, 18),
            Token::number_literal("1".to_string(), 19, 20),
            Token::new(TokenKind::Semicolon, 20, 21),
            Token::new(TokenKind::RightCurlyBrace, 22, 23),
            Token::new(TokenKind::Semicolon, 23, 24),
        ]
    );
}
