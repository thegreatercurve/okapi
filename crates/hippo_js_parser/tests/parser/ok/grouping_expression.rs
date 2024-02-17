use crate::parser::common::assert_parse_module_eq;

#[test]
fn grouping_expression() {
    assert_parse_module_eq!(
        r#"((foo));"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"Identifier","start":2,"end":5,"name":"foo"}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(foo);"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":1,"end":4,"name":"foo"}}],"sourceType":"module"}"#
    );
}