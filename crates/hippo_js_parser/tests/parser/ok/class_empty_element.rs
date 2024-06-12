use crate::parser::assert_parser_eq;

#[test]
fn class_empty_element() {
    assert_parser_eq!(
        r#"class foo { get foo() {} }"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"ClassDeclaration","start":0,"end":26,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":26,"body":[{"type":"MethodDefinition","start":12,"end":24,"static":false,"computed":false,"key":{"type":"Identifier","start":16,"end":19,"name":"foo"},"kind":"get","value":{"type":"FunctionExpression","start":19,"end":24,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}
