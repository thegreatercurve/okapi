use crate::parser::common::assert_parser_eq;

#[test]
fn object_expr_ident_prop() {
    assert_parser_eq!(
        r#"({foo})"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"ObjectExpression","start":1,"end":6,"properties":[{"type":"Property","start":2,"end":5,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":2,"end":5,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":2,"end":5,"name":"foo"}}]}}],"sourceType":"script"}"#
    );
}
