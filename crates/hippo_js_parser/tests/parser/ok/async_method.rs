use crate::parser::assert_parser_eq;

#[test]
fn async_method() {
    assert_parser_eq!(
        r#"class foo { async foo() {} async *foo() {} }"#,
        r#"{"type":"Program","start":0,"end":44,"body":[{"type":"ClassDeclaration","start":0,"end":44,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":44,"body":[{"type":"MethodDefinition","start":12,"end":26,"static":false,"computed":false,"key":{"type":"Identifier","start":18,"end":21,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":21,"end":26,"id":null,"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":24,"end":26,"body":[]}}},{"type":"MethodDefinition","start":27,"end":42,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":37,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":37,"end":42,"id":null,"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":40,"end":42,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}
