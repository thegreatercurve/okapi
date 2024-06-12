use crate::parser::assert_parser_eq;

#[test]
fn class_expression() {
    assert_parser_eq!(
        r#"let a = class {};"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"VariableDeclaration","start":0,"end":17,"declarations":[{"type":"VariableDeclarator","start":4,"end":16,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ClassExpression","start":8,"end":16,"id":null,"superClass":null,"body":{"type":"ClassBody","start":14,"end":16,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );
}

#[test]
fn class_expression_with_method() {
    assert_parser_eq!(
        r#"let b = class foo {
  contructor() {}
};"#,
        r#"{"type":"Program","start":0,"end":40,"body":[{"type":"VariableDeclaration","start":0,"end":40,"declarations":[{"type":"VariableDeclarator","start":4,"end":39,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ClassExpression","start":8,"end":39,"id":{"type":"Identifier","start":14,"end":17,"name":"foo"},"superClass":null,"body":{"type":"ClassBody","start":18,"end":39,"body":[{"type":"MethodDefinition","start":22,"end":37,"static":false,"computed":false,"key":{"type":"Identifier","start":22,"end":32,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":32,"end":37,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":35,"end":37,"body":[]}}}]}}}],"kind":"let"}],"sourceType":"script"}"#
    );
}

#[test]
fn class_expression_as_accessor() {
    assert_parser_eq!(
        r#"foo[class {}];"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"MemberExpression","start":0,"end":13,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"ClassExpression","start":4,"end":12,"id":null,"superClass":null,"body":{"type":"ClassBody","start":10,"end":12,"body":[]}},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );
}
