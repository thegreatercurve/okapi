use crate::parser::common::assert_parse_module_eq;

#[test]
fn arrow_expression_in_alternate() {
    assert_parse_module_eq!(
        r#"a ? (b) : a => {};"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"ConditionalExpression","start":0,"end":17,"test":{"type":"Identifier","start":0,"end":1,"name":"a"},"consequent":{"type":"Identifier","start":5,"end":6,"name":"b"},"alternate":{"type":"ArrowFunctionExpression","start":10,"end":17,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":10,"end":11,"name":"a"}],"body":{"type":"BlockStatement","start":15,"end":17,"body":[]}}}}],"sourceType":"script"}"#
    );
}
