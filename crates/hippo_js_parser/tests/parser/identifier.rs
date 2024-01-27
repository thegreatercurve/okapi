use crate::parser::common::assert_parse_script_eq;

#[test]
fn identifier() {
    assert_parse_script_eq!(
        r#"foo;"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Identifier","start":0,"end":3,"name":"foo"}}],"sourceType":"script"}"#
    );
    assert_parse_script_eq!(
        r#"let accessor = 5;"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"VariableDeclaration","start":0,"end":17,"declarations":[{"type":"VariableDeclarator","start":4,"end":16,"id":{"type":"Identifier","start":4,"end":12,"name":"accessor"},"init":{"type":"Literal","start":15,"end":16,"value":5,"raw":"5"}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
