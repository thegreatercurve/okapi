use crate::parser::assert_parser_eq;

#[test]
fn array_element_in_expression() {
    assert_parser_eq!(
        r#"for(["a" in {}];;) {}"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ForStatement","start":0,"end":21,"init":{"type":"ArrayExpression","start":4,"end":15,"elements":[{"type":"BinaryExpression","start":5,"end":14,"left":{"type":"Literal","start":5,"end":8,"value":"a","raw":"\"a\""},"operator":"in","right":{"type":"ObjectExpression","start":12,"end":14,"properties":[]}}]},"test":null,"update":null,"body":{"type":"BlockStatement","start":19,"end":21,"body":[]}}],"sourceType":"module"}"#
    );
}
