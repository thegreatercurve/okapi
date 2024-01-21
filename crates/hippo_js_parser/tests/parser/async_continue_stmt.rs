use crate::parser::common::assert_parse_module_eq;

#[test]
fn async_continue_stmt() {
    assert_parse_module_eq!(
        r#"async: for(a of b) continue async;"#,
        r#"{"type":"Program","start":0,"end":34,"body":[{"type":"LabeledStatement","start":0,"end":34,"body":{"type":"ForOfStatement","start":7,"end":34,"await":false,"left":{"type":"Identifier","start":11,"end":12,"name":"a"},"right":{"type":"Identifier","start":16,"end":17,"name":"b"},"body":{"type":"ContinueStatement","start":19,"end":34,"label":{"type":"Identifier","start":28,"end":33,"name":"async"}}},"label":{"type":"Identifier","start":0,"end":5,"name":"async"}}],"sourceType":"script"}"#
    );
}
