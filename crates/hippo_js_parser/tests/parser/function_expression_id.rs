use crate::parser::common::assert_parse_module_eq;

#[test]
fn function_expression_id() {
    assert_parse_module_eq!(
        r#"// SCRIPT"#,
        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(function await() {});"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"FunctionExpression","start":1,"end":20,"id":{"type":"Identifier","start":10,"end":15,"name":"await"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":18,"end":20,"body":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(function yield() {});"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"FunctionExpression","start":1,"end":20,"id":{"type":"Identifier","start":10,"end":15,"name":"yield"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":18,"end":20,"body":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(async function yield() {});"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"ExpressionStatement","start":0,"end":28,"expression":{"type":"FunctionExpression","start":1,"end":26,"id":{"type":"Identifier","start":16,"end":21,"name":"yield"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":24,"end":26,"body":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(function* await() {})"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"FunctionExpression","start":1,"end":21,"id":{"type":"Identifier","start":11,"end":16,"name":"await"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":19,"end":21,"body":[]}}}],"sourceType":"module"}"#
    );
}
