use crate::parser::common::assert_parse_module_eq;

#[test]
fn debugger_statement() {
    assert_parse_module_eq!(
        r#"debugger;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"DebuggerStatement","start":0,"end":9}],"sourceType":"module"}"#
    );
}
