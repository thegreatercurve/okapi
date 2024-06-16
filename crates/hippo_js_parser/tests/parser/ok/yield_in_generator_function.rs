use crate::parser::assert_parser_script_eq;

#[test]
fn yield_in_generator_function() {
    assert_parser_script_eq!(
        r#"function* foo() { yield 10; }"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"FunctionDeclaration","start":0,"end":29,"id":{"type":"Identifier","start":10,"end":13,"name":"foo"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":29,"body":[{"type":"ExpressionStatement","start":18,"end":27,"expression":{"type":"YieldExpression","start":18,"end":26,"delegate":false,"argument":{"type":"Literal","start":24,"end":26,"value":10.0,"raw":"10"}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function* foo() { yield* bar; }"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"FunctionDeclaration","start":0,"end":31,"id":{"type":"Identifier","start":10,"end":13,"name":"foo"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":31,"body":[{"type":"ExpressionStatement","start":18,"end":29,"expression":{"type":"YieldExpression","start":18,"end":28,"delegate":true,"argument":{"type":"Identifier","start":25,"end":28,"name":"bar"}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function* foo() { yield; }"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"FunctionDeclaration","start":0,"end":26,"id":{"type":"Identifier","start":10,"end":13,"name":"foo"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":26,"body":[{"type":"ExpressionStatement","start":18,"end":24,"expression":{"type":"YieldExpression","start":18,"end":23,"delegate":false,"argument":null}}]}}],"sourceType":"script"}"#
    );
}
