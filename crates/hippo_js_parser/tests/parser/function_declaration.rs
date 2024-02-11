use crate::parser::common::assert_parse_module_eq;

#[test]
fn function_declaration() {
    assert_parse_module_eq!(
        r#"function foo() {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"FunctionDeclaration","start":0,"end":17,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":17,"body":[]}}],"sourceType":"module"}"#
    );
}

#[test]
fn function_declaration_with_formal_parameters() {
    assert_parse_module_eq!(
        r#"function foo(bar, baz) {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"FunctionDeclaration","start":0,"end":25,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":13,"end":16,"name":"bar"},{"type":"Identifier","start":18,"end":21,"name":"baz"}],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}],"sourceType":"module"}"#
    );
}
