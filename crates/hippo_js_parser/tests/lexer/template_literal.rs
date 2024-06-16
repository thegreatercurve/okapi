use hippo_js_parser::TokenKind;

use crate::lexer::{
    assert_lexer_eq, identifier, number_literal, punctuator, template_literal_head,
    template_literal_middle, template_literal_no_substitution, template_literal_tail,
};

#[test]
fn template_literal_simple() {
    assert_lexer_eq!(
        "`123`;",
        vec![template_literal_no_substitution("123", 0, 5, 1, 1)]
    );
    assert_lexer_eq!("``", vec![template_literal_no_substitution("", 0, 2, 1, 1)]);
}

#[test]
fn template_literal_with_multiple_parts() {
    assert_lexer_eq!(
        r"`123 ${foo}`;",
        vec![
            template_literal_head("123 ", 0, 7, 1, 1),
            identifier("foo", 7, 10, 1, 8),
            template_literal_tail("", 10, 12, 1, 11),
        ]
    );
    assert_lexer_eq!(
        r"`123 ${foo} bar`;",
        vec![
            template_literal_head("123 ", 0, 7, 1, 1),
            identifier("foo", 7, 10, 1, 8),
            template_literal_tail(" bar", 10, 16, 1, 11),
        ]
    );
    assert_lexer_eq!(
        r"`123 ${foo} bar ${456} baz`;",
        vec![
            template_literal_head("123 ", 0, 7, 1, 1),
            identifier("foo", 7, 10, 1, 8),
            template_literal_middle(" bar ", 10, 18, 1, 11),
            number_literal("456", 456.0, 18, 21, 1, 19),
            template_literal_tail(" baz", 21, 27, 1, 22),
        ]
    );
}

#[test]
fn template_literal_with_different_goal_symbols() {
    assert_lexer_eq!(
        r"`123 ${foo / 2} bar ${456} baz` 2 / 2;",
        vec![
            template_literal_head("123 ", 0, 7, 1, 1),
            identifier("foo", 7, 10, 1, 8),
            punctuator(TokenKind::Division, 11, 12, 1, 12),
            number_literal("2", 2.0, 13, 14, 1, 14),
            template_literal_middle(" bar ", 14, 22, 1, 15),
            number_literal("456", 456.0, 22, 25, 1, 23),
            template_literal_tail(" baz", 25, 31, 1, 26),
            number_literal("2", 2.0, 32, 33, 1, 33),
            punctuator(TokenKind::Division, 34, 35, 1, 35),
            number_literal("2", 2.0, 36, 37, 1, 37),
        ]
    );
}
