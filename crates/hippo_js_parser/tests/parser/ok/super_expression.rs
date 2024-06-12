use crate::parser::assert_parser_script_eq;

#[test]
fn super_expression() {
    assert_parser_script_eq!(
        r#"class Test extends B { 
    contructor() { 
        super(); 
    } 
    test() { 
        super.test(); 
        super[1];
    } 
}"#,
        r#"{"type":"Program","start":0,"end":132,"body":[{"type":"ClassDeclaration","start":0,"end":132,"id":{"type":"Identifier","start":6,"end":10,"name":"Test"},"superClass":{"type":"Identifier","start":19,"end":20,"name":"B"},"body":{"type":"ClassBody","start":21,"end":132,"body":[{"type":"MethodDefinition","start":28,"end":67,"static":false,"computed":false,"key":{"type":"Identifier","start":28,"end":38,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":38,"end":67,"id":null,"params":[],"generator":false,"expression":false,"async":false,"body":{"type":"BlockStatement","start":41,"end":67,"body":[{"type":"ExpressionStatement","start":52,"end":60,"expression":{"type":"CallExpression","start":52,"end":59,"callee":{"type":"Super","start":52,"end":57},"arguments":[],"optional":false}}]}}},{"type":"MethodDefinition","start":73,"end":129,"static":false,"computed":false,"key":{"type":"Identifier","start":73,"end":77,"name":"test"},"kind":"method","value":{"type":"FunctionExpression","start":77,"end":129,"id":null,"params":[],"generator":false,"expression":false,"async":false,"body":{"type":"BlockStatement","start":80,"end":129,"body":[{"type":"ExpressionStatement","start":91,"end":104,"expression":{"type":"CallExpression","start":91,"end":103,"callee":{"type":"MemberExpression","start":91,"end":101,"object":{"type":"Super","start":91,"end":96},"property":{"type":"Identifier","start":97,"end":101,"name":"test"},"computed":false,"optional":false},"arguments":[],"optional":false}},{"type":"ExpressionStatement","start":114,"end":123,"expression":{"type":"MemberExpression","start":114,"end":122,"object":{"type":"Super","start":114,"end":119},"property":{"type":"Literal","start":120,"end":121,"value":1,"raw":"1"},"computed":true,"optional":false}}]}}}]}}],"sourceType":"script"}"#
    );
}
