use crate::parser::assert_parser_script_eq;

#[test]
fn bom_character() {
    assert_parser_script_eq!(
        r#"function foo ( ) {}"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"FunctionDeclaration","start":0,"end":19,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":19,"body":[]}}],"sourceType":"script"}"#
    );
}
