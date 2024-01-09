use crate::parser::common::assert_parser_eq;

#[test]
fn debugger_statement() {
    assert_parser_eq!(
        r#"debugger;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"DebuggerStatement","start":0,"end":9}],"sourceType":"module"}"#
    );
}
