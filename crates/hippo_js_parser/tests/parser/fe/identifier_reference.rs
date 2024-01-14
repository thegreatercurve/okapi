use crate::parser::common::assert_parser_eq;

#[test]
fn identifier_reference() {
    assert_parser_eq!(
        r#"// SCRIPT"#,
        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"foo;"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Identifier","start":0,"end":3,"name":"foo"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"yield;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":0,"end":5,"name":"yield"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"await;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":0,"end":5,"name":"await"}}],"sourceType":"script"}"#
    );
}
