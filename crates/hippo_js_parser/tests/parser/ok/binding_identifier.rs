use crate::parser::assert_parser_eq;

#[test]
fn binding_identifier() {
    assert_parser_eq!(
        r#"let a = "b";"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"VariableDeclaration","start":0,"end":12,"declarations":[{"type":"VariableDeclarator","start":4,"end":11,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"Literal","start":8,"end":11,"value":"b","raw":"\"b\""}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
