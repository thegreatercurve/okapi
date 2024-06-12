use crate::parser::assert_parser_script_eq;

#[test]
fn prefix() {
    assert_parser_script_eq!(
        r#"++foo"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"UpdateExpression","start":0,"end":5,"operator":"++","prefix":true,"argument":{"type":"Identifier","start":2,"end":5,"name":"foo"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"--foo"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"UpdateExpression","start":0,"end":5,"operator":"--","prefix":true,"argument":{"type":"Identifier","start":2,"end":5,"name":"foo"}}}],"sourceType":"script"}"#
    );
}
