use crate::parser::common::assert_parser_eq;

#[test]
fn js_parenthesized_expression() {
    assert_parser_eq!(
        r#"((foo))"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"Identifier","start":2,"end":5,"name":"foo"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"(foo)"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Identifier","start":1,"end":4,"name":"foo"}}],"sourceType":"script"}"#
    );
}
