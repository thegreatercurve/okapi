use crate::parser::assert_parser_script_eq;

#[test]
fn arrow_expression_single_param() {
    assert_parser_script_eq!(
        r#"foo => {}"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"ArrowFunctionExpression","start":0,"end":9,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":0,"end":3,"name":"foo"}],"body":{"type":"BlockStatement","start":7,"end":9,"body":[]}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"yield => {}"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"ArrowFunctionExpression","start":0,"end":11,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":0,"end":5,"name":"yield"}],"body":{"type":"BlockStatement","start":9,"end":11,"body":[]}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"await => {}"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"ArrowFunctionExpression","start":0,"end":11,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":0,"end":5,"name":"await"}],"body":{"type":"BlockStatement","start":9,"end":11,"body":[]}}}],"sourceType":"script"}"#
    );
}
