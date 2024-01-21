use crate::parser::common::assert_parse_module_eq;

#[test]
fn do_while_stmt() {
    assert_parse_module_eq!(
        r#"do { } while (true)"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"DoWhileStatement","start":0,"end":19,"body":{"type":"BlockStatement","start":3,"end":6,"body":[]},"test":{"type":"Literal","start":14,"end":18,"value":true,"raw":"true"}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"do { throw Error("foo") } while (true)"#,
        r#"{"type":"Program","start":0,"end":38,"body":[{"type":"DoWhileStatement","start":0,"end":38,"body":{"type":"BlockStatement","start":3,"end":25,"body":[{"type":"ThrowStatement","start":5,"end":23,"argument":{"type":"CallExpression","start":11,"end":23,"callee":{"type":"Identifier","start":11,"end":16,"name":"Error"},"arguments":[{"type":"Literal","start":17,"end":22,"value":"foo","raw":"\"foo\""}],"optional":false}}]},"test":{"type":"Literal","start":33,"end":37,"value":true,"raw":"true"}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"do { break; } while (true)"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"DoWhileStatement","start":0,"end":26,"body":{"type":"BlockStatement","start":3,"end":13,"body":[{"type":"BreakStatement","start":5,"end":11,"label":null}]},"test":{"type":"Literal","start":21,"end":25,"value":true,"raw":"true"}}],"sourceType":"script"}"#
    );
}
