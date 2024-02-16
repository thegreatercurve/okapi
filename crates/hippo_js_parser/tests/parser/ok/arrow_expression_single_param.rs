use crate::parser::common::assert_parse_module_eq;

#[test]
fn arrow_expression_single_param() {
    assert_parse_module_eq!(
        r#"foo => {}"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"ArrowFunctionExpression","start":0,"end":9,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":0,"end":3,"name":"foo"}],"body":{"type":"BlockStatement","start":7,"end":9,"body":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"yield => {}"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"ArrowFunctionExpression","start":0,"end":11,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":0,"end":5,"name":"yield"}],"body":{"type":"BlockStatement","start":9,"end":11,"body":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"await => {}"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"ArrowFunctionExpression","start":0,"end":11,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":0,"end":5,"name":"await"}],"body":{"type":"BlockStatement","start":9,"end":11,"body":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(r#"baz =>"#, r#"undefined"#);
    assert_parse_module_eq!(
        r#"{}"#,
        r#"{"type":"Program","start":0,"end":2,"body":[{"type":"BlockStatement","start":0,"end":2,"body":[]}],"sourceType":"module"}"#
    );
}
