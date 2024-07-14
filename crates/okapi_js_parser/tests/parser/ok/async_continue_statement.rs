use crate::parser::assert_parser_script_eq;

#[test]
fn async_continue_statement() {
    assert_parser_script_eq!(
        r#"for(a of b) continue async;"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ForOfStatement","start":0,"end":27,"await":false,"left":{"type":"Identifier","start":4,"end":5,"name":"a"},"right":{"type":"Identifier","start":9,"end":10,"name":"b"},"body":{"type":"ContinueStatement","start":12,"end":27,"label":{"type":"Identifier","start":21,"end":26,"name":"async"}}}],"sourceType":"script"}"#
    );
}
