use crate::parser::assert_parser_eq;

#[test]
fn object_expr_async_method() {
    assert_parser_eq!(
        r#"let a = { async foo() {}, async *foo() {} };"#,
        r#"{"type":"Program","start":0,"end":44,"body":[{"type":"VariableDeclaration","start":0,"end":44,"declarations":[{"type":"VariableDeclarator","start":4,"end":43,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":43,"properties":[{"type":"Property","start":10,"end":24,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":16,"end":19,"name":"foo"},"kind":"init","value":{"type":"FunctionExpression","start":19,"end":24,"id":null,"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}},{"type":"Property","start":26,"end":41,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":33,"end":36,"name":"foo"},"kind":"init","value":{"type":"FunctionExpression","start":36,"end":41,"id":null,"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":39,"end":41,"body":[]}}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
