use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn statement_without_whitespace() {
    assert_lexer_eq!(
        "const foo=\"hello\";",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5),
            Token::new(TokenKind::Identifier("foo".to_string()), 6, 9),
            Token::new(TokenKind::Assignment, 9, 10),
            Token::new(TokenKind::StringLiteral, 10, 17),
            Token::new(TokenKind::Semicolon, 17, 18),
        ]
    );
}
