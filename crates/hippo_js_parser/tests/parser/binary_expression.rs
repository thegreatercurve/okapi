use crate::parser::common::assert_parse_module_eq;

#[test]
fn binary_expression() {
    assert_parse_module_eq!(
        r#"4 + 4;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":4.0,"raw":"4"},"operator":"+","right":{"type":"Literal","start":4,"end":5,"value":4.0,"raw":"4"}}}],"sourceType":"module"}"#
    );
}
