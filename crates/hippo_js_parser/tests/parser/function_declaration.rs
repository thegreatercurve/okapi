use crate::parser::common::assert_parse_module_eq;

#[test]
fn function_declaration() {
    assert_parse_module_eq!(
        r#"function foo() {}"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"FunctionDeclaration","start":0,"end":18,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":18,"body":[]}}],"sourceType":"module"}"#
    );
}
