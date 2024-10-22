use crate::parser::assert_parser_script_eq;

#[test]
fn labelled_statement_in_single_statement_context() {
    assert_parser_script_eq!(
        r#"if (true) label1: var a = 10;"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"IfStatement","start":0,"end":29,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"LabeledStatement","start":10,"end":29,"body":{"type":"VariableDeclaration","start":18,"end":29,"declarations":[{"type":"VariableDeclarator","start":22,"end":28,"id":{"type":"Identifier","start":22,"end":23,"name":"a"},"init":{"type":"Literal","start":26,"end":28,"value":10.0,"raw":"10"}}],"kind":"var"},"label":{"type":"Identifier","start":10,"end":16,"name":"label1"}},"alternate":null}],"sourceType":"script"}"#
    );
}
