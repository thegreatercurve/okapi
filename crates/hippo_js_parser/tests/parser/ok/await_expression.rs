use crate::parser::assert_parser_eq;

#[test]
fn await_expression() {
    assert_parser_eq!(
        r#"async function test() { await inner(); (await inner()) + (await inner()); }"#,
        r#"{"type":"Program","start":0,"end":75,"body":[{"type":"FunctionDeclaration","start":0,"end":75,"id":{"type":"Identifier","start":15,"end":19,"name":"test"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":22,"end":75,"body":[{"type":"ExpressionStatement","start":24,"end":38,"expression":{"type":"AwaitExpression","start":24,"end":37,"argument":{"type":"CallExpression","start":30,"end":37,"callee":{"type":"Identifier","start":30,"end":35,"name":"inner"},"arguments":[],"optional":false}}},{"type":"ExpressionStatement","start":39,"end":73,"expression":{"type":"BinaryExpression","start":39,"end":72,"left":{"type":"AwaitExpression","start":40,"end":53,"argument":{"type":"CallExpression","start":46,"end":53,"callee":{"type":"Identifier","start":46,"end":51,"name":"inner"},"arguments":[],"optional":false}},"operator":"+","right":{"type":"AwaitExpression","start":58,"end":71,"argument":{"type":"CallExpression","start":64,"end":71,"callee":{"type":"Identifier","start":64,"end":69,"name":"inner"},"arguments":[],"optional":false}}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"async function inner() { return 4; }"#,
        r#"{"type":"Program","start":0,"end":36,"body":[{"type":"FunctionDeclaration","start":0,"end":36,"id":{"type":"Identifier","start":15,"end":20,"name":"inner"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":23,"end":36,"body":[{"type":"ReturnStatement","start":25,"end":34,"argument":{"type":"Literal","start":32,"end":33,"value":4,"raw":"4"}}]}}],"sourceType":"script"}"#
    );

    // assert_parser_eq!(r#"await test();"#, r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Identifier","start":0,"end":5,"name":"await"}},{"type":"ExpressionStatement","start":6,"end":13,"expression":{"type":"CallExpression","start":6,"end":12,"callee":{"type":"Identifier","start":6,"end":10,"name":"test"},"arguments":[],"optional":false}}],"sourceType":"script"}"#);
}
