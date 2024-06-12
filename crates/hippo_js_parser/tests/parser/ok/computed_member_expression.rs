use crate::parser::assert_parser_script_eq;

#[test]
fn computed_member_expression() {
    assert_parser_script_eq!(
        r#"foo[bar]"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":7,"name":"bar"},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo[5 + 5]"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"MemberExpression","start":0,"end":10,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"BinaryExpression","start":4,"end":9,"left":{"type":"Literal","start":4,"end":5,"value":5,"raw":"5"},"operator":"+","right":{"type":"Literal","start":8,"end":9,"value":5,"raw":"5"}},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo["bar"]"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"MemberExpression","start":0,"end":10,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Literal","start":4,"end":9,"value":"bar","raw":"\"bar\""},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo[bar][baz]"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"MemberExpression","start":0,"end":13,"object":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":7,"name":"bar"},"computed":true,"optional":false},"property":{"type":"Identifier","start":9,"end":12,"name":"baz"},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo?.[bar]"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"ChainExpression","start":0,"end":10,"expression":{"type":"MemberExpression","start":0,"end":10,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":6,"end":9,"name":"bar"},"computed":true,"optional":true}}}],"sourceType":"script"}"#
    );
}
