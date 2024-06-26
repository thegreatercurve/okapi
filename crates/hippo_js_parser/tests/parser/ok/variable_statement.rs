use crate::parser::assert_parser_script_eq;

#[test]
fn variable_statement() {
    assert_parser_script_eq!(
        "var let;",
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"VariableDeclaration","start":0,"end":8,"declarations":[{"type":"VariableDeclarator","start":4,"end":7,"id":{"type":"Identifier","start":4,"end":7,"name":"let"},"init":null}],"kind":"var"}],"sourceType":"script"}"#
    );
}
