use hippo_js_parser::{KeywordKind, Token, TokenKind, TokenValue};

use crate::lexer::{
    assert_lexer_eq,
    utils::{identifier, punctuator, string_literal},
};

#[test]
fn whitespace_minified() {
    assert_lexer_eq!(
        "const foo=\"hello\";",
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
            punctuator(TokenKind::Assignment, 9, 10, 1, 10),
            string_literal("hello", "\"hello\"", 10, 17, 1, 11),
            punctuator(TokenKind::Semicolon, 17, 18, 1, 18),
        ]
    );
}
