use crate::parser::common::assert_parser_eq;

#[test]
fn array_expression() {
    assert_parser_eq!(
        r#"[foo, bar];"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"ArrayExpression","start":0,"end":10,"elements":[{"type":"Identifier","start":1,"end":4,"name":"foo"},{"type":"Identifier","start":6,"end":9,"name":"bar"}]}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"[foo];"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"ArrayExpression","start":0,"end":5,"elements":[{"type":"Identifier","start":1,"end":4,"name":"foo"}]}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"[,foo];"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"ArrayExpression","start":0,"end":6,"elements":[null,{"type":"Identifier","start":2,"end":5,"name":"foo"}]}}],"sourceType":"script"}"#
    );
    // assert_parser_eq!(
    //     r#"[foo,];"#,
    //     r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"ArrayExpression","start":0,"end":6,"elements":[{"type":"Identifier","start":1,"end":4,"name":"foo"}]}}],"sourceType":"script"}"#
    // );
    // assert_parser_eq!(
    //     r#"[,,,,,foo,,,,];"#,
    //     r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"ArrayExpression","start":0,"end":14,"elements":[null,null,null,null,null,{"type":"Identifier","start":6,"end":9,"name":"foo"},null,null,null]}}],"sourceType":"script"}"#
    // );
    // assert_parser_eq!(
    //     r#"[...a, ...b];"#,
    //     r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"ArrayExpression","start":0,"end":12,"elements":[{"type":"SpreadElement","start":1,"end":5,"argument":{"type":"Identifier","start":4,"end":5,"name":"a"}},{"type":"SpreadElement","start":7,"end":11,"argument":{"type":"Identifier","start":10,"end":11,"name":"b"}}]}}],"sourceType":"script"}"#
    // );
}
