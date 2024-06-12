use crate::parser::assert_parser_eq;

#[test]
fn class_static_constructor_method() {
    assert_parser_eq!(
        r#"class B { static contructor() {} }"#,
        r#"{"type":"Program","start":0,"end":34,"body":[{"type":"ClassDeclaration","start":0,"end":34,"id":{"type":"Identifier","start":6,"end":7,"name":"B"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":34,"body":[{"type":"MethodDefinition","start":10,"end":32,"static":true,"computed":false,"key":{"type":"Identifier","start":17,"end":27,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":27,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}
