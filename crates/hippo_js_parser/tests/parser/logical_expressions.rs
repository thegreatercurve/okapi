use crate::parser::common::assert_parser_eq;

#[test]
fn logical_expressions() {
    assert_parser_eq!(
        r#"foo ?? bar"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"LogicalExpression","start":0,"end":10,"left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"operator":"??","right":{"type":"Identifier","start":7,"end":10,"name":"bar"}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"a || b"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"LogicalExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"||","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"a && b"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"LogicalExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"&&","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"script"}"#
    );
}
