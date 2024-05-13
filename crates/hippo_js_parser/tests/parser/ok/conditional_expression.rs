use crate::parser::assert_parser_eq;

#[test]
fn conditional_expression() {
    assert_parser_eq!(
        r#"foo ? bar : baz"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"ConditionalExpression","start":0,"end":15,"test":{"type":"Identifier","start":0,"end":3,"name":"foo"},"consequent":{"type":"Identifier","start":6,"end":9,"name":"bar"},"alternate":{"type":"Identifier","start":12,"end":15,"name":"baz"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo ? bar : baz ? bar : baz"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ExpressionStatement","start":0,"end":27,"expression":{"type":"ConditionalExpression","start":0,"end":27,"test":{"type":"Identifier","start":0,"end":3,"name":"foo"},"consequent":{"type":"Identifier","start":6,"end":9,"name":"bar"},"alternate":{"type":"ConditionalExpression","start":12,"end":27,"test":{"type":"Identifier","start":12,"end":15,"name":"baz"},"consequent":{"type":"Identifier","start":18,"end":21,"name":"bar"},"alternate":{"type":"Identifier","start":24,"end":27,"name":"baz"}}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo !== bar ? baz.length / bat : 0"#,
        r#"{"type":"Program","start":0,"end":34,"body":[{"type":"ExpressionStatement","start":0,"end":34,"expression":{"type":"ConditionalExpression","start":0,"end":34,"test":{"type":"BinaryExpression","start":0,"end":11,"left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"operator":"!==","right":{"type":"Identifier","start":8,"end":11,"name":"bar"}},"consequent":{"type":"BinaryExpression","start":14,"end":30,"left":{"type":"MemberExpression","start":14,"end":24,"object":{"type":"Identifier","start":14,"end":17,"name":"baz"},"property":{"type":"Identifier","start":18,"end":24,"name":"length"},"computed":false,"optional":false},"operator":"/","right":{"type":"Identifier","start":27,"end":30,"name":"bat"}},"alternate":{"type":"Literal","start":33,"end":34,"value":0,"raw":"0"}}}],"sourceType":"module"}"#
    );
}
