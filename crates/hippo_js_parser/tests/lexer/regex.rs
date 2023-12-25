use crate::lexer::{common::assert_lexer_eq, utils::regular_expression_literal};

#[test]
fn regular_expression_simple() {
    assert_lexer_eq!("/123/", vec![regular_expression_literal("/123/", 0, 5)]);
    assert_lexer_eq!("/123/gu", vec![regular_expression_literal("/123/gu", 0, 7)]);
}
