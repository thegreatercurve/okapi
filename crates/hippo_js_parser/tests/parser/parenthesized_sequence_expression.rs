use crate::parser::common::assert_parse_module_eq;

#[test]
fn parenthesized_sequence_expression() {
    assert_parse_module_eq!(
        r#"(a, b);"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"SequenceExpression","start":1,"end":5,"expressions":[{"type":"Identifier","start":1,"end":2,"name":"a"},{"type":"Identifier","start":4,"end":5,"name":"b"}]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(a, b, c);"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"SequenceExpression","start":1,"end":8,"expressions":[{"type":"Identifier","start":1,"end":2,"name":"a"},{"type":"Identifier","start":4,"end":5,"name":"b"},{"type":"Identifier","start":7,"end":8,"name":"c"}]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(a, b, c, d, e, f);"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"ExpressionStatement","start":0,"end":19,"expression":{"type":"SequenceExpression","start":1,"end":17,"expressions":[{"type":"Identifier","start":1,"end":2,"name":"a"},{"type":"Identifier","start":4,"end":5,"name":"b"},{"type":"Identifier","start":7,"end":8,"name":"c"},{"type":"Identifier","start":10,"end":11,"name":"d"},{"type":"Identifier","start":13,"end":14,"name":"e"},{"type":"Identifier","start":16,"end":17,"name":"f"}]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(a, b, c, d, e, f)"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"SequenceExpression","start":1,"end":17,"expressions":[{"type":"Identifier","start":1,"end":2,"name":"a"},{"type":"Identifier","start":4,"end":5,"name":"b"},{"type":"Identifier","start":7,"end":8,"name":"c"},{"type":"Identifier","start":10,"end":11,"name":"d"},{"type":"Identifier","start":13,"end":14,"name":"e"},{"type":"Identifier","start":16,"end":17,"name":"f"}]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(a, b, c)"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"SequenceExpression","start":1,"end":8,"expressions":[{"type":"Identifier","start":1,"end":2,"name":"a"},{"type":"Identifier","start":4,"end":5,"name":"b"},{"type":"Identifier","start":7,"end":8,"name":"c"}]}}],"sourceType":"module"}"#
    );
}
