use crate::parser::common::assert_parse_module_eq;

#[test]
fn labeled_statement() {
    assert_parse_module_eq!(
        r#"label1: 1"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"LabeledStatement","start":0,"end":9,"body":{"type":"ExpressionStatement","start":8,"end":9,"expression":{"type":"Literal","start":8,"end":9,"value":1,"raw":"1"}},"label":{"type":"Identifier","start":0,"end":6,"name":"label1"}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"label1: 1"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"LabeledStatement","start":0,"end":9,"body":{"type":"ExpressionStatement","start":8,"end":9,"expression":{"type":"Literal","start":8,"end":9,"value":1,"raw":"1"}},"label":{"type":"Identifier","start":0,"end":6,"name":"label1"}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"label2: 2"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"LabeledStatement","start":0,"end":9,"body":{"type":"ExpressionStatement","start":8,"end":9,"expression":{"type":"Literal","start":8,"end":9,"value":2,"raw":"2"}},"label":{"type":"Identifier","start":0,"end":6,"name":"label2"}}],"sourceType":"module"}"#
    );
}
