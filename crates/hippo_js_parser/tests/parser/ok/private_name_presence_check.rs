use crate::parser::assert_parser_eq;

#[test]
fn private_name_presence_check() {
    assert_parser_eq!(
        r#"class A { 
    #prop; 
    test() { 
        #prop in this; 
    } 
}"#,
        r#"{"type":"Program","start":0,"end":69,"body":[{"type":"ClassDeclaration","start":0,"end":69,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":69,"body":[{"type":"PropertyDefinition","start":15,"end":21,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":15,"end":20,"name":"prop"},"value":null},{"type":"MethodDefinition","start":27,"end":66,"static":false,"computed":false,"key":{"type":"Identifier","start":27,"end":31,"name":"test"},"kind":"method","value":{"type":"FunctionExpression","start":31,"end":66,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":34,"end":66,"body":[{"type":"ExpressionStatement","start":45,"end":59,"expression":{"type":"BinaryExpression","start":45,"end":58,"left":{"type":"PrivateIdentifier","start":45,"end":50,"name":"prop"},"operator":"in","right":{"type":"ThisExpression","start":54,"end":58}}}]}}}]}}],"sourceType":"script"}"#
    );
}
