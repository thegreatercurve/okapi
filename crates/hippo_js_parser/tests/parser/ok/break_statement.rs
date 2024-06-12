use crate::parser::assert_parser_eq;

#[test]
fn break_statement() {
    assert_parser_eq!(
        r"while (true) {
  break;
  {
    break;
  }
}",
        r#"{"type":"Program","start":0,"end":44,"body":[{"type":"WhileStatement","start":0,"end":44,"test":{"type":"Literal","start":7,"end":11,"value":true,"raw":"true"},"body":{"type":"BlockStatement","start":13,"end":44,"body":[{"type":"BreakStatement","start":17,"end":23,"label":null},{"type":"BlockStatement","start":26,"end":42,"body":[{"type":"BreakStatement","start":32,"end":38,"label":null}]}]}}],"sourceType":"script"}"#
    );
}
