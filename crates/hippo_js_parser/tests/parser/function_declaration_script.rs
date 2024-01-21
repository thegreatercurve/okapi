use crate::parser::common::assert_parse_module_eq;

#[test]
fn function_declaration_script() {
    assert_parse_module_eq!(
        r#"// SCRIPT"#,
        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"function test(await) {}"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"FunctionDeclaration","start":0,"end":23,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":14,"end":19,"name":"await"}],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}}],"sourceType":"script"}"#
    );
}
