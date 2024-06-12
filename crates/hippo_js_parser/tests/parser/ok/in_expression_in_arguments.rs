use crate::parser::assert_parser_eq;

#[test]
fn in_expression_in_arguments() {
    assert_parser_eq!(
        r#"for (foo("call" in foo);;) {}"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"ForStatement","start":0,"end":29,"init":{"type":"CallExpression","start":5,"end":23,"callee":{"type":"Identifier","start":5,"end":8,"name":"foo"},"arguments":[{"type":"BinaryExpression","start":9,"end":22,"left":{"type":"Literal","start":9,"end":15,"value":"call","raw":"\"call\""},"operator":"in","right":{"type":"Identifier","start":19,"end":22,"name":"foo"}}],"optional":false},"test":null,"update":null,"body":{"type":"BlockStatement","start":27,"end":29,"body":[]}}],"sourceType":"script"}"#
    );
}
