use crate::parser::assert_parser_eq;

#[test]
fn postfix_expression() {
    assert_parser_eq!(
        r#"foo++"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"UpdateExpression","start":0,"end":5,"operator":"++","prefix":false,"argument":{"type":"Identifier","start":0,"end":3,"name":"foo"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo--"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"UpdateExpression","start":0,"end":5,"operator":"--","prefix":false,"argument":{"type":"Identifier","start":0,"end":3,"name":"foo"}}}],"sourceType":"module"}"#
    );
}
