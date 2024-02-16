use crate::parser::common::assert_parse_module_eq;

#[test]
fn while_statement() {
    assert_parse_module_eq!(
        r#"while (true) {}"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"WhileStatement","start":0,"end":15,"test":{"type":"Literal","start":7,"end":11,"value":true,"raw":"true"},"body":{"type":"BlockStatement","start":13,"end":15,"body":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"while (5) {}"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"WhileStatement","start":0,"end":12,"test":{"type":"Literal","start":7,"end":8,"value":5.0,"raw":"5"},"body":{"type":"BlockStatement","start":10,"end":12,"body":[]}}],"sourceType":"module"}"#
    );
}
