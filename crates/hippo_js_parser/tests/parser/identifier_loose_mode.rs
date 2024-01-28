use crate::parser::common::assert_parse_module_eq;

#[test]
fn identifier_loose_mode() {
    assert_parse_module_eq!(
        r#"// SCRIPT"#,
        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"foo;"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Identifier","start":0,"end":3,"name":"foo"}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"yield;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":0,"end":5,"name":"yield"}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"await;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":0,"end":5,"name":"await"}}],"sourceType":"module"}"#
    );
}
