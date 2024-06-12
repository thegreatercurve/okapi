use crate::parser::assert_parser_script_eq;

#[test]
fn empty_statement() {
    assert_parser_script_eq!(
        r#";"#,
        r#"{"type":"Program","start":0,"end":1,"body":[{"type":"EmptyStatement","start":0,"end":1}],"sourceType":"script"}"#
    );
}
