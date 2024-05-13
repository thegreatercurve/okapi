use crate::parser::assert_parser_eq;

#[test]
fn yield_expression() {
    assert_parser_eq!(
        r#"function* foo() {
    yield foo;
    yield* foo;
    yield;
    yield;
    yield;
}"#,
        r#"{"type":"Program","start":0,"end":83,"body":[{"type":"FunctionDeclaration","start":0,"end":83,"id":{"type":"Identifier","start":10,"end":13,"name":"foo"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":83,"body":[{"type":"ExpressionStatement","start":22,"end":32,"expression":{"type":"YieldExpression","start":22,"end":31,"delegate":false,"argument":{"type":"Identifier","start":28,"end":31,"name":"foo"}}},{"type":"ExpressionStatement","start":37,"end":48,"expression":{"type":"YieldExpression","start":37,"end":47,"delegate":true,"argument":{"type":"Identifier","start":44,"end":47,"name":"foo"}}},{"type":"ExpressionStatement","start":53,"end":59,"expression":{"type":"YieldExpression","start":53,"end":58,"delegate":false,"argument":null}},{"type":"ExpressionStatement","start":64,"end":70,"expression":{"type":"YieldExpression","start":64,"end":69,"delegate":false,"argument":null}},{"type":"ExpressionStatement","start":75,"end":81,"expression":{"type":"YieldExpression","start":75,"end":80,"delegate":false,"argument":null}}]}}],"sourceType":"module"}"#
    );
}
