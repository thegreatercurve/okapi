use crate::parser::assert_parser_script_eq;

#[test]
fn labeled_statement() {
    assert_parser_script_eq!(
        r#"label1: 1"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"LabeledStatement","start":0,"end":9,"body":{"type":"ExpressionStatement","start":8,"end":9,"expression":{"type":"Literal","start":8,"end":9,"value":1.0,"raw":"1"}},"label":{"type":"Identifier","start":0,"end":6,"name":"label1"}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"label1: 1"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"LabeledStatement","start":0,"end":9,"body":{"type":"ExpressionStatement","start":8,"end":9,"expression":{"type":"Literal","start":8,"end":9,"value":1.0,"raw":"1"}},"label":{"type":"Identifier","start":0,"end":6,"name":"label1"}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"label2: 2"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"LabeledStatement","start":0,"end":9,"body":{"type":"ExpressionStatement","start":8,"end":9,"expression":{"type":"Literal","start":8,"end":9,"value":2.0,"raw":"2"}},"label":{"type":"Identifier","start":0,"end":6,"name":"label2"}}],"sourceType":"script"}"#
    );
}
