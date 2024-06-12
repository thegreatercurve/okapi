use crate::parser::assert_parser_eq;

#[test]
fn throw_statement() {
    assert_parser_eq!(
        r#"throw new Error("foo");"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"ThrowStatement","start":0,"end":23,"argument":{"type":"NewExpression","start":6,"end":22,"callee":{"type":"Identifier","start":10,"end":15,"name":"Error"},"arguments":[{"type":"Literal","start":16,"end":21,"value":"foo","raw":"\"foo\""}]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"throw "foo""#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ThrowStatement","start":0,"end":11,"argument":{"type":"Literal","start":6,"end":11,"value":"foo","raw":"\"foo\""}}],"sourceType":"script"}"#
    );
}
