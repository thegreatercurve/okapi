use crate::parser::common::assert_parse_module_eq;

#[test]
fn yield_expression() {
    assert_parse_module_eq!(
        r#"function* foo() {
            yield foo;
            yield* foo;
            yield;
            yield;
            yield;
          }"#,
        r#"{"type":"Program","start":0,"end":133,"body":[{"type":"FunctionDeclaration","start":0,"end":133,"id":{"type":"Identifier","start":10,"end":13,"name":"foo"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":133,"body":[{"type":"ExpressionStatement","start":30,"end":40,"expression":{"type":"YieldExpression","start":30,"end":39,"delegate":false,"argument":{"type":"Identifier","start":36,"end":39,"name":"foo"}}},{"type":"ExpressionStatement","start":53,"end":64,"expression":{"type":"YieldExpression","start":53,"end":63,"delegate":true,"argument":{"type":"Identifier","start":60,"end":63,"name":"foo"}}},{"type":"ExpressionStatement","start":77,"end":83,"expression":{"type":"YieldExpression","start":77,"end":82,"delegate":false,"argument":null}},{"type":"ExpressionStatement","start":96,"end":102,"expression":{"type":"YieldExpression","start":96,"end":101,"delegate":false,"argument":null}},{"type":"ExpressionStatement","start":115,"end":121,"expression":{"type":"YieldExpression","start":115,"end":120,"delegate":false,"argument":null}}]}}],"sourceType":"module"}"#
    );
}
