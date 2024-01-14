use crate::parser::common::assert_parser_eq;

#[test]
fn literals() {
    assert_parser_eq!(
        r#"5"#,
        r#"{"type":"Program","start":0,"end":1,"body":[{"type":"ExpressionStatement","start":0,"end":1,"expression":{"type":"Literal","start":0,"end":1,"value":5,"raw":"5"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"true"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Literal","start":0,"end":4,"value":true,"raw":"true"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"false"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"value":false,"raw":"false"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(r#"5n"#, r#"undefined"#);
    assert_parser_eq!(
        r#""foo""#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"value":"foo","raw":"\"foo\""},"directive":"foo"}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"'bar'"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"value":"bar","raw":"'bar'"},"directive":"bar"}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"null"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Literal","start":0,"end":4,"value":null,"raw":"null"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(r#"0, 0.0, 0n, 0e00"#, r#"undefined"#);
    assert_parser_eq!(r#""test\"#, r#"undefined"#);
    assert_parser_eq!(r#"new-line";"#, r#"undefined"#);
    assert_parser_eq!(
        r#"/^[يفمئامئ‍ئاسۆند]/i; //regex with unicode"#,
        r#"{"type":"Program","start":0,"end":42,"body":[{"type":"ExpressionStatement","start":0,"end":21,"expression":{"type":"Literal","start":0,"end":20,"value":{},"raw":"/^[يفمئامئ‍ئاسۆند]/i","regex":{"pattern":"^[يفمئامئ‍ئاسۆند]","flags":"i"}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"/[\p{Control}--[\t\n]]/v;"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"ExpressionStatement","start":0,"end":25,"expression":{"type":"Literal","start":0,"end":24,"value":null,"raw":"/[\\p{Control}--[\\t\\n]]/v","regex":{"pattern":"[\\p{Control}--[\\t\\n]]","flags":"v"}}}],"sourceType":"script"}"#
    );
}