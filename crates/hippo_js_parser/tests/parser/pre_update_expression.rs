use crate::parser::common::assert_parse_module_eq;

#[test]
fn pre_update_expression() {
    assert_parse_module_eq!(
        r#"++foo"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"UpdateExpression","start":0,"end":5,"operator":"++","prefix":true,"argument":{"type":"Identifier","start":2,"end":5,"name":"foo"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"--foo"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"UpdateExpression","start":0,"end":5,"operator":"--","prefix":true,"argument":{"type":"Identifier","start":2,"end":5,"name":"foo"}}}],"sourceType":"module"}"#
    );
}
