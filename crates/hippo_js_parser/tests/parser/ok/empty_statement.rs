use crate::parser::common::assert_parse_module_eq;

#[test]
fn empty_statement() {
    assert_parse_module_eq!(
        r#";"#,
        r#"{"type":"Program","start":0,"end":1,"body":[{"type":"EmptyStatement","start":0,"end":1}],"sourceType":"module"}"#
    );
}
