
use crate::parser::assert_parser_script_eq;

#[test]
fn if_statement() {
    assert_parser_script_eq!(
        r#"if (true) {} else {}"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"IfStatement","start":0,"end":20,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"BlockStatement","start":10,"end":12,"body":[]},"alternate":{"type":"BlockStatement","start":18,"end":20,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"if (true) {}"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"IfStatement","start":0,"end":12,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"BlockStatement","start":10,"end":12,"body":[]},"alternate":null}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"if (true) false"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"IfStatement","start":0,"end":15,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"ExpressionStatement","start":10,"end":15,"expression":{"type":"Literal","start":10,"end":15,"value":false,"raw":"false"}},"alternate":null}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"if (bar) {} else if (true) {} else {}"#,
        r#"{"type":"Program","start":0,"end":37,"body":[{"type":"IfStatement","start":0,"end":37,"test":{"type":"Identifier","start":4,"end":7,"name":"bar"},"consequent":{"type":"BlockStatement","start":9,"end":11,"body":[]},"alternate":{"type":"IfStatement","start":17,"end":37,"test":{"type":"Literal","start":21,"end":25,"value":true,"raw":"true"},"consequent":{"type":"BlockStatement","start":27,"end":29,"body":[]},"alternate":{"type":"BlockStatement","start":35,"end":37,"body":[]}}}],"sourceType":"script"}"#
    );
}
