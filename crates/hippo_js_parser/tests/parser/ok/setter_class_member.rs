use crate::parser::assert_parser_script_eq;

#[test]
fn setter_class_member() {
    assert_parser_script_eq!(
        r#"class Setter {
    set foo(a) {}
    set static(a) {} 
    static set bar(a) {}
    set baz(a) {}
    set ["a" + "b"](a) {}
    set 5(a) {}
    set #private(a) {}
}"#,
        r#"{"type":"Program","start":0,"end":164,"body":[{"type":"ClassDeclaration","start":0,"end":164,"id":{"type":"Identifier","start":6,"end":12,"name":"Setter"},"superClass":null,"body":{"type":"ClassBody","start":13,"end":164,"body":[{"type":"MethodDefinition","start":19,"end":32,"static":false,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"foo"},"kind":"set","value":{"type":"FunctionExpression","start":26,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":27,"end":28,"name":"a"}],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}},{"type":"MethodDefinition","start":37,"end":53,"static":false,"computed":false,"key":{"type":"Identifier","start":41,"end":47,"name":"static"},"kind":"set","value":{"type":"FunctionExpression","start":47,"end":53,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":48,"end":49,"name":"a"}],"body":{"type":"BlockStatement","start":51,"end":53,"body":[]}}},{"type":"MethodDefinition","start":59,"end":79,"static":true,"computed":false,"key":{"type":"Identifier","start":70,"end":73,"name":"bar"},"kind":"set","value":{"type":"FunctionExpression","start":73,"end":79,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":74,"end":75,"name":"a"}],"body":{"type":"BlockStatement","start":77,"end":79,"body":[]}}},{"type":"MethodDefinition","start":84,"end":97,"static":false,"computed":false,"key":{"type":"Identifier","start":88,"end":91,"name":"baz"},"kind":"set","value":{"type":"FunctionExpression","start":91,"end":97,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":92,"end":93,"name":"a"}],"body":{"type":"BlockStatement","start":95,"end":97,"body":[]}}},{"type":"MethodDefinition","start":102,"end":123,"static":false,"computed":true,"key":{"type":"BinaryExpression","start":107,"end":116,"left":{"type":"Literal","start":107,"end":110,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":113,"end":116,"value":"b","raw":"\"b\""}},"kind":"set","value":{"type":"FunctionExpression","start":117,"end":123,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":118,"end":119,"name":"a"}],"body":{"type":"BlockStatement","start":121,"end":123,"body":[]}}},{"type":"MethodDefinition","start":128,"end":139,"static":false,"computed":false,"key":{"type":"Literal","start":132,"end":133,"value":5,"raw":"5"},"kind":"set","value":{"type":"FunctionExpression","start":133,"end":139,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":134,"end":135,"name":"a"}],"body":{"type":"BlockStatement","start":137,"end":139,"body":[]}}},{"type":"MethodDefinition","start":144,"end":162,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":148,"end":156,"name":"private"},"kind":"set","value":{"type":"FunctionExpression","start":156,"end":162,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":157,"end":158,"name":"a"}],"body":{"type":"BlockStatement","start":160,"end":162,"body":[]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class NotSetter { 
    set(a) {} 
    async set(a) {} 
    static set(a) {} 
}"#,
        r#"{"type":"Program","start":0,"end":78,"body":[{"type":"ClassDeclaration","start":0,"end":78,"id":{"type":"Identifier","start":6,"end":15,"name":"NotSetter"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":78,"body":[{"type":"MethodDefinition","start":23,"end":32,"static":false,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"set"},"kind":"method","value":{"type":"FunctionExpression","start":26,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":27,"end":28,"name":"a"}],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}},{"type":"MethodDefinition","start":38,"end":53,"static":false,"computed":false,"key":{"type":"Identifier","start":44,"end":47,"name":"set"},"kind":"method","value":{"type":"FunctionExpression","start":47,"end":53,"id":null,"expression":false,"generator":false,"async":true,"params":[{"type":"Identifier","start":48,"end":49,"name":"a"}],"body":{"type":"BlockStatement","start":51,"end":53,"body":[]}}},{"type":"MethodDefinition","start":59,"end":75,"static":true,"computed":false,"key":{"type":"Identifier","start":66,"end":69,"name":"set"},"kind":"method","value":{"type":"FunctionExpression","start":69,"end":75,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":70,"end":71,"name":"a"}],"body":{"type":"BlockStatement","start":73,"end":75,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}
