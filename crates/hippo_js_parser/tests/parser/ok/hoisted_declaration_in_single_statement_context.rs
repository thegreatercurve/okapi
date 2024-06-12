use crate::parser::assert_parser_script_eq;

#[test]
fn hoisted_declaration_in_single_statement_context() {
    assert_parser_script_eq!(
        r#"if (true) var a;"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"IfStatement","start":0,"end":16,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"VariableDeclaration","start":10,"end":16,"declarations":[{"type":"VariableDeclarator","start":14,"end":15,"id":{"type":"Identifier","start":14,"end":15,"name":"a"},"init":null}],"kind":"var"},"alternate":null}],"sourceType":"script"}"#
    );
}
