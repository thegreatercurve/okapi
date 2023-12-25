use hippo_js_parser::{KeywordKind, Token, TokenKind};

use crate::lexer::{
    common::assert_lexer_eq,
    utils::{identifier, punctuator, string_literal},
};

#[test]
fn whitespace_minified() {
    assert_lexer_eq!(
        "const foo=\"hello\";",
        vec![
            Token::new(TokenKind::Keyword(KeywordKind::Const), 0, 5, None),
            identifier("foo", 6, 9),
            punctuator(TokenKind::Assignment, 9, 10),
            string_literal("hello", 10, 17),
            punctuator(TokenKind::Semicolon, 17, 18),
        ]
    );
}
