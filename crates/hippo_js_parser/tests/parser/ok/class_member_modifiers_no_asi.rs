use crate::parser::assert_parser_eq;

#[test]
fn class_member_modifiers_no_asi() {
    assert_parser_eq!(
        r#"class A { static foo() {} }"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ClassDeclaration","start":0,"end":27,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":27,"body":[{"type":"MethodDefinition","start":10,"end":25,"static":true,"computed":false,"key":{"type":"Identifier","start":17,"end":20,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":20,"end":25,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}
