use crate::parser::assert_parser_script_eq;

#[test]
fn reparse_yield_as_identifier() {
    assert_parser_script_eq!(
        r#"function foo() { yield * bar; }"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"FunctionDeclaration","start":0,"end":31,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":31,"body":[{"type":"ExpressionStatement","start":17,"end":29,"expression":{"type":"BinaryExpression","start":17,"end":28,"left":{"type":"Identifier","start":17,"end":22,"name":"yield"},"operator":"*","right":{"type":"Identifier","start":25,"end":28,"name":"bar"}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function bar() { yield; }"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"FunctionDeclaration","start":0,"end":25,"id":{"type":"Identifier","start":9,"end":12,"name":"bar"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":25,"body":[{"type":"ExpressionStatement","start":17,"end":23,"expression":{"type":"Identifier","start":17,"end":22,"name":"yield"}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function baz() { yield; }"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"FunctionDeclaration","start":0,"end":25,"id":{"type":"Identifier","start":9,"end":12,"name":"baz"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":25,"body":[{"type":"ExpressionStatement","start":17,"end":23,"expression":{"type":"Identifier","start":17,"end":22,"name":"yield"}}]}}],"sourceType":"script"}"#
    );
}
