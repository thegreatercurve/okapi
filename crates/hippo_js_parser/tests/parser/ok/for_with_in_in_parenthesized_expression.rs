use crate::parser::assert_parser_eq;

#[test]
fn for_with_in_in_parenthesized_expression() {
    assert_parser_eq!(
        r#"for((true,"selectionStart"in true);;) {}"#,
        r#"{"type":"Program","start":0,"end":40,"body":[{"type":"ForStatement","start":0,"end":40,"init":{"type":"SequenceExpression","start":5,"end":33,"expressions":[{"type":"Literal","start":5,"end":9,"value":true,"raw":"true"},{"type":"BinaryExpression","start":10,"end":33,"left":{"type":"Literal","start":10,"end":26,"value":"selectionStart","raw":"\"selectionStart\""},"operator":"in","right":{"type":"Literal","start":29,"end":33,"value":true,"raw":"true"}}]},"test":null,"update":null,"body":{"type":"BlockStatement","start":38,"end":40,"body":[]}}],"sourceType":"module"}"#
    );
}
