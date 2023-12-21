use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::{
    common::assert_lexer_eq,
    utils::{identifier, string_literal},
};

#[test]
fn whitespace_minified() {
    assert_lexer_eq!(
        "const foo=\"hello\";",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5, None),
            identifier("foo", 6, 9),
            Token::new(TokenKind::Assignment, 9, 10, None),
            string_literal("hello", 10, 17),
            Token::new(TokenKind::Semicolon, 17, 18, None),
        ]
    );
}
