use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn whitespace_minified() {
    assert_lexer_eq!(
        "const foo=\"hello\";",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5),
            Token::identifier("foo".to_string(), 6, 9),
            Token::new(TokenKind::Assignment, 9, 10),
            Token::string_literal("hello".to_string(), 10, 17),
            Token::new(TokenKind::Semicolon, 17, 18),
        ]
    );
}
