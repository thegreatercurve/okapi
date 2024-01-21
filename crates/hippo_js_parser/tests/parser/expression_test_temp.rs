use crate::parser::common::assert_parser_eq;

#[test]
fn expression_test_temp() {
    assert_parser_eq!(
        r#"delete foo.bar;"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"UnaryExpression","start":0,"end":14,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":7,"end":14,"object":{"type":"Identifier","start":7,"end":10,"name":"foo"},"property":{"type":"Identifier","start":11,"end":14,"name":"bar"},"computed":false,"optional":false}}}],"sourceType":"module"}"#
    );
}
