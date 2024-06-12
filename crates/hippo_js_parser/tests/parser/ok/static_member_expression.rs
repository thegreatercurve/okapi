use crate::parser::assert_parser_eq;

#[test]
fn static_member_expression() {
    assert_parser_eq!(
        r#"foo.bar;"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":7,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":7,"name":"bar"},"computed":false,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"foo.await;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"MemberExpression","start":0,"end":9,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":9,"name":"await"},"computed":false,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"foo.yield;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"MemberExpression","start":0,"end":9,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":9,"name":"yield"},"computed":false,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"foo.for;"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":7,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":7,"name":"for"},"computed":false,"optional":false}}],"sourceType":"script"}"#
    );
}
