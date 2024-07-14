use crate::parser::test_helper_macros::assert_parser_script_eq;

#[test]
fn debugger_statement() {
    assert_parser_script_eq!(
        r#"debugger;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"DebuggerStatement","start":0,"end":9}],"sourceType":"script"}"#
    );
}
