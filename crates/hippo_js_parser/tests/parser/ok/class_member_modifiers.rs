use crate::parser::assert_parser_eq;

#[test]
fn class_member_modifiers() {
    assert_parser_eq!(
        r#"class A { public() {} }"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"ClassDeclaration","start":0,"end":23,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":23,"body":[{"type":"MethodDefinition","start":10,"end":21,"static":false,"computed":false,"key":{"type":"Identifier","start":10,"end":16,"name":"public"},"kind":"method","value":{"type":"FunctionExpression","start":16,"end":21,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":19,"end":21,"body":[]}}}]}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"class A { static protected() {} }"#,
        r#"{"type":"Program","start":0,"end":33,"body":[{"type":"ClassDeclaration","start":0,"end":33,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":33,"body":[{"type":"MethodDefinition","start":10,"end":31,"static":true,"computed":false,"key":{"type":"Identifier","start":17,"end":26,"name":"protected"},"kind":"method","value":{"type":"FunctionExpression","start":26,"end":31,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}}]}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"class A { static; }"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"ClassDeclaration","start":0,"end":19,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":19,"body":[{"type":"PropertyDefinition","start":10,"end":17,"static":false,"computed":false,"key":{"type":"Identifier","start":10,"end":16,"name":"static"},"value":null}]}}],"sourceType":"module"}"#
    );
}
