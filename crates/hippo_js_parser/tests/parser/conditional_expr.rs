use crate::parser::common::assert_parse_module_eq;

#[test]
fn conditional_expression() {
    assert_parse_module_eq!(
        r#"foo ? bar : baz"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"ConditionalExpression","start":0,"end":15,"test":{"type":"Identifier","start":0,"end":3,"name":"foo"},"consequent":{"type":"Identifier","start":6,"end":9,"name":"bar"},"alternate":{"type":"Identifier","start":12,"end":15,"name":"baz"}}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"foo ? bar : baz ? bar : baz"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ExpressionStatement","start":0,"end":27,"expression":{"type":"ConditionalExpression","start":0,"end":27,"test":{"type":"Identifier","start":0,"end":3,"name":"foo"},"consequent":{"type":"Identifier","start":6,"end":9,"name":"bar"},"alternate":{"type":"ConditionalExpression","start":12,"end":27,"test":{"type":"Identifier","start":12,"end":15,"name":"baz"},"consequent":{"type":"Identifier","start":18,"end":21,"name":"bar"},"alternate":{"type":"Identifier","start":24,"end":27,"name":"baz"}}}}],"sourceType":"script"}"#
    );
}
