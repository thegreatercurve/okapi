use crate::parser::common::assert_parser_eq;

#[test]
fn expression_test_temp() {
    assert_parser_eq!(
        r#"delete asdf++;"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"UnaryExpression","start":0,"end":13,"operator":"delete","prefix":true,"argument":{"type":"UpdateExpression","start":7,"end":13,"operator":"++","prefix":false,"argument":{"type":"Identifier","start":7,"end":11,"name":"asdf"}}}}],"sourceType":"module"}"#
    );
}
