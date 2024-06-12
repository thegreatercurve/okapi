use crate::parser::assert_parser_eq;

#[test]
fn literals_number() {
    assert_parser_eq!(
        r#"5"#,
        r#"{"type":"Program","start":0,"end":1,"body":[{"type":"ExpressionStatement","start":0,"end":1,"expression":{"type":"Literal","start":0,"end":1,"value":5,"raw":"5"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"0, 0.0, 0e00"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"SequenceExpression","start":0,"end":12,"expressions":[{"type":"Literal","start":0,"end":1,"value":0,"raw":"0"},{"type":"Literal","start":3,"end":6,"value":0,"raw":"0.0"},{"type":"Literal","start":8,"end":12,"value":0,"raw":"0e00"}]}}],"sourceType":"script"}"#
    );
}

// #[test]
// fn literals_big_n() {
//     assert_parser_eq!(r#"5n"#, r#"undefined"#);
// }

#[test]
fn literals_string() {
    assert_parser_eq!(
        r#""foo""#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"value":"foo","raw":"\"foo\""},"directive":"foo"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"'bar'"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"value":"bar","raw":"'bar'"},"directive":"bar"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#""new-line";


"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"Literal","start":0,"end":10,"value":"new-line","raw":"\"new-line\""},"directive":"new-line"}],"sourceType":"script"}"#
    );
}

#[test]
fn literals_boolean() {
    assert_parser_eq!(
        r#"true"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Literal","start":0,"end":4,"value":true,"raw":"true"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"false"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"value":false,"raw":"false"}}],"sourceType":"script"}"#
    );
}

#[test]
fn literals_null() {
    assert_parser_eq!(
        r#"null"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"Literal","start":0,"end":4,"value":null,"raw":"null"}}],"sourceType":"script"}"#
    );
}

#[test]
fn literals_regex() {
    assert_parser_eq!(
        r#"/.*/g"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Literal","start":0,"end":5,"regex":{"pattern":".*","flags":"g"},"value":{},"raw":"/.*/g"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"/\/.*/g"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"Literal","start":0,"end":7,"regex":{"pattern":"\\/.*","flags":"g"},"value":{},"raw":"/\\/.*/g"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"/\[.*/g"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"Literal","start":0,"end":7,"regex":{"pattern":"\\[.*","flags":"g"},"value":{},"raw":"/\\[.*/g"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"/[\p{Control}\--[\t\n]]/g"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"ExpressionStatement","start":0,"end":25,"expression":{"type":"Literal","start":0,"end":25,"regex":{"pattern":"[\\p{Control}\\--[\\t\\n]]","flags":"g"},"value":{},"raw":"/[\\p{Control}\\--[\\t\\n]]/g"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"/^[a-z][^\s:/?#]$/i"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"ExpressionStatement","start":0,"end":19,"expression":{"type":"Literal","start":0,"end":19,"regex":{"pattern":"^[a-z][^\\s:/?#]$","flags":"i"},"value":{},"raw":"/^[a-z][^\\s:/?#]$/i"}}],"sourceType":"script"}"#
    );
}
