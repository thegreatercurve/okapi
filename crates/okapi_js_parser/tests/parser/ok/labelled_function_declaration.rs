use crate::parser::assert_parser_script_eq;

#[test]
fn labelled_function_declaration() {
    assert_parser_script_eq!(
        r#"label1: function a() {}"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"LabeledStatement","start":0,"end":23,"body":{"type":"FunctionDeclaration","start":8,"end":23,"id":{"type":"Identifier","start":17,"end":18,"name":"a"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}},"label":{"type":"Identifier","start":0,"end":6,"name":"label1"}}],"sourceType":"script"}"#
    );
}
