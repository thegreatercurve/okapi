use crate::parser::assert_parser_script_eq;

#[test]
fn this_expression() {
    assert_parser_script_eq!(
        r#"this"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"ThisExpression","start":0,"end":4}}],"sourceType":"script"}"#
    );
    assert_parser_script_eq!(
        r#"this.foo"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"ThisExpression","start":0,"end":4},"property":{"type":"Identifier","start":5,"end":8,"name":"foo"},"computed":false,"optional":false}}],"sourceType":"script"}"#
    );
}
