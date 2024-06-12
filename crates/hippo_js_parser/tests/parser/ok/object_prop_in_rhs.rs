use crate::parser::assert_parser_eq;

#[test]
fn object_prop_in_rhs() {
    assert_parser_eq!(
        r#"for ({ a: "x" in {} };;) {}"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ForStatement","start":0,"end":27,"init":{"type":"ObjectExpression","start":5,"end":21,"properties":[{"type":"Property","start":7,"end":19,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":7,"end":8,"name":"a"},"value":{"type":"BinaryExpression","start":10,"end":19,"left":{"type":"Literal","start":10,"end":13,"value":"x","raw":"\"x\""},"operator":"in","right":{"type":"ObjectExpression","start":17,"end":19,"properties":[]}},"kind":"init"}]},"test":null,"update":null,"body":{"type":"BlockStatement","start":25,"end":27,"body":[]}}],"sourceType":"script"}"#
    );
}
