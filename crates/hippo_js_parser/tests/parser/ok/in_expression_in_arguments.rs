use crate::parser::common::assert_parse_module_eq;

#[test]
fn in_expression_in_arguments() {
    assert_parse_module_eq!(
        r#"function foo() {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"FunctionDeclaration","start":0,"end":17,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":17,"body":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"for (foo("call" in foo);;) {}"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"ForStatement","start":0,"end":29,"init":{"type":"CallExpression","start":5,"end":23,"callee":{"type":"Identifier","start":5,"end":8,"name":"foo"},"arguments":[{"type":"BinaryExpression","start":9,"end":22,"left":{"type":"Literal","start":9,"end":15,"value":"call","raw":"\"call\""},"operator":"in","right":{"type":"Identifier","start":19,"end":22,"name":"foo"}}],"optional":false},"test":null,"update":null,"body":{"type":"BlockStatement","start":27,"end":29,"body":[]}}],"sourceType":"module"}"#
    );
}
