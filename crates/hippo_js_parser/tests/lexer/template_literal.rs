use hippo_js_parser::{ParserError, TokenKind};

use crate::lexer::utils::{illegal, punctuator, regular_expression_literal};

#[test]
fn regular_expression_simple() {
    assert_lexer_eq!("/123/", vec![regular_expression_literal("/123/", 0, 5)]);
    assert_lexer_eq!("/123/gu", vec![regular_expression_literal("/123/gu", 0, 7)]);
}

#[test]
fn regular_expression_invalid() {
    assert_lexer_eq!(
        "/*",
        vec![
            illegal(ParserError::InvalidRegexLiteralFirstChar, 0, 1),
            punctuator(TokenKind::Multiplication, 1, 2),
        ]
    );
    assert_lexer_eq!(
        "/\\",
        vec![
            illegal(ParserError::InvalidRegexLiteralFirstChar, 0, 1),
            illegal(ParserError::SyntaxError, 1, 2),
        ]
    );
}
