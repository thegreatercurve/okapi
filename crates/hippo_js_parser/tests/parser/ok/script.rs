use crate::parser::test_helper_macros::assert_parse_script_eq;

#[test]
fn script() {
    assert_parse_script_eq!(
        r#"let foo = "bar";"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"VariableDeclaration","start":0,"end":16,"declarations":[{"type":"VariableDeclarator","start":4,"end":15,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":{"type":"Literal","start":10,"end":15,"value":"bar","raw":"\"bar\""}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
