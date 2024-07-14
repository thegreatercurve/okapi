use crate::parser::assert_parser_script_eq;

#[test]
fn reparse_await_as_identifier() {
    assert_parser_script_eq!(
        r#"function test() { a = await; }"#,
        r#"{"type":"Program","start":0,"end":30,"body":[{"type":"FunctionDeclaration","start":0,"end":30,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":30,"body":[{"type":"ExpressionStatement","start":18,"end":28,"expression":{"type":"AssignmentExpression","start":18,"end":27,"operator":"=","left":{"type":"Identifier","start":18,"end":19,"name":"a"},"right":{"type":"Identifier","start":22,"end":27,"name":"await"}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function test2() { return await; }"#,
        r#"{"type":"Program","start":0,"end":34,"body":[{"type":"FunctionDeclaration","start":0,"end":34,"id":{"type":"Identifier","start":9,"end":14,"name":"test2"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":34,"body":[{"type":"ReturnStatement","start":19,"end":32,"argument":{"type":"Identifier","start":26,"end":31,"name":"await"}}]}}],"sourceType":"script"}"#
    );
}
