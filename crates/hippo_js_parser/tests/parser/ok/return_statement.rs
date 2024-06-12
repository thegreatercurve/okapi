use crate::parser::assert_parser_eq;

#[test]
fn return_statement() {
    assert_parser_eq!(
        r#"() => { return; }"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"ArrowFunctionExpression","start":0,"end":17,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":6,"end":17,"body":[{"type":"ReturnStatement","start":8,"end":15,"argument":null}]}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"() => { return 1 + 1; }"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"ExpressionStatement","start":0,"end":23,"expression":{"type":"ArrowFunctionExpression","start":0,"end":23,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":6,"end":23,"body":[{"type":"ReturnStatement","start":8,"end":21,"argument":{"type":"BinaryExpression","start":15,"end":20,"left":{"type":"Literal","start":15,"end":16,"value":1,"raw":"1"},"operator":"+","right":{"type":"Literal","start":19,"end":20,"value":1,"raw":"1"}}}]}}}],"sourceType":"script"}"#
    );
}
