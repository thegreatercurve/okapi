use crate::parser::assert_parser_eq;

#[test]
fn continue_statement() {
    assert_parser_eq!(
        r#"while (true) {
  while (true) {
    continue;
  }

  continue;
}"#,
        r#"{"type":"Program","start":0,"end":64,"body":[{"type":"WhileStatement","start":0,"end":64,"test":{"type":"Literal","start":7,"end":11,"value":true,"raw":"true"},"body":{"type":"BlockStatement","start":13,"end":64,"body":[{"type":"WhileStatement","start":17,"end":49,"test":{"type":"Literal","start":24,"end":28,"value":true,"raw":"true"},"body":{"type":"BlockStatement","start":30,"end":49,"body":[{"type":"ContinueStatement","start":36,"end":45,"label":null}]}},{"type":"ContinueStatement","start":53,"end":62,"label":null}]}}],"sourceType":"script"}"#
    );
}
