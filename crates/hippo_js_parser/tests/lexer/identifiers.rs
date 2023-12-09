use std::collections::VecDeque;

use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn keywords_and_identifiers() {
    assert_lexer_eq!(
        "const foo = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5),
            Token::new(TokenKind::Identifier("foo".to_string()), 6, 9),
            Token::new(TokenKind::Assignment, 10, 11),
            Token::new(TokenKind::NumberLiteral, 12, 13),
            Token::new(TokenKind::Semicolon, 14, 15),
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Let), 0, 3),
            Token::new(TokenKind::Identifier("baz".to_string()), 4, 7),
            Token::new(TokenKind::Assignment, 8, 9),
            Token::new(TokenKind::NumberLiteral, 10, 11),
            Token::new(TokenKind::Semicolon, 12, 13),
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Var), 0, 3),
            Token::new(TokenKind::Identifier("baz".to_string()), 4, 7),
            Token::new(TokenKind::Assignment, 8, 9),
            Token::new(TokenKind::NumberLiteral, 10, 11),
            Token::new(TokenKind::Semicolon, 12, 13),
        ]
    );

    // assert_lexer_eq!(
    //     r#"var \u{0042}\u{0041}z = 1;"#,
    //     vec![
    //         TokenType::Keyword(KeywordKind::Var),
    //         TokenType::Identifier("baz".to_string()),
    //         TokenType::Assignment,
    //         TokenType::NumberLiteral,
    //         TokenType::Semicolon,
    //     ]
    // );

    // Private identifiers
    assert_lexer_eq!(
        r#"class Foo { #bar = 1; };"#,
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Class), 0, 5),
            Token::new(TokenKind::Identifier("Foo".to_string()), 6, 9),
            Token::new(TokenKind::LeftCurlyBrace, 10, 11),
            Token::new(TokenKind::Identifier("#bar".to_string()), 12, 16),
            Token::new(TokenKind::Assignment, 17, 18),
            Token::new(TokenKind::NumberLiteral, 19, 20),
            Token::new(TokenKind::Semicolon, 20, 21),
            Token::new(TokenKind::RightCurlyBrace, 22, 23),
            Token::new(TokenKind::Semicolon, 23, 24),
        ]
    );

    // TODO Handle surrogate unciode pairs
}
