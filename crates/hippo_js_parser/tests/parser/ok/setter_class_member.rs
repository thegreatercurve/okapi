use crate::parser::assert_parser_eq;

#[test]
fn setter_class_member() {
    assert_parser_eq!(
        r#"class Setter {
    set foo(a) {}
    set static(a) {} static
    set bar(a) {}
    set baz(a) {}
    set ["a" + "b"](a) {}
    set 5(a) {}
    set #private(a) {}
}"#,
        r#"{"type":"Program","start":0,"end":163,"body":[{"type":"ClassDeclaration","start":0,"end":163,"id":{"type":"Identifier","start":6,"end":12,"name":"Setter"},"superClass":null,"body":{"type":"ClassBody","start":13,"end":163,"body":[{"type":"MethodDefinition","start":19,"end":32,"static":false,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"foo"},"kind":"set","value":{"type":"FunctionExpression","start":26,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":27,"end":28,"name":"a"}],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}},{"type":"MethodDefinition","start":37,"end":53,"static":false,"computed":false,"key":{"type":"Identifier","start":41,"end":47,"name":"static"},"kind":"set","value":{"type":"FunctionExpression","start":47,"end":53,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":48,"end":49,"name":"a"}],"body":{"type":"BlockStatement","start":51,"end":53,"body":[]}}},{"type":"MethodDefinition","start":54,"end":78,"static":true,"computed":false,"key":{"type":"Identifier","start":69,"end":72,"name":"bar"},"kind":"set","value":{"type":"FunctionExpression","start":72,"end":78,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":73,"end":74,"name":"a"}],"body":{"type":"BlockStatement","start":76,"end":78,"body":[]}}},{"type":"MethodDefinition","start":83,"end":96,"static":false,"computed":false,"key":{"type":"Identifier","start":87,"end":90,"name":"baz"},"kind":"set","value":{"type":"FunctionExpression","start":90,"end":96,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":91,"end":92,"name":"a"}],"body":{"type":"BlockStatement","start":94,"end":96,"body":[]}}},{"type":"MethodDefinition","start":101,"end":122,"static":false,"computed":true,"key":{"type":"BinaryExpression","start":106,"end":115,"left":{"type":"Literal","start":106,"end":109,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":112,"end":115,"value":"b","raw":"\"b\""}},"kind":"set","value":{"type":"FunctionExpression","start":116,"end":122,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":117,"end":118,"name":"a"}],"body":{"type":"BlockStatement","start":120,"end":122,"body":[]}}},{"type":"MethodDefinition","start":127,"end":138,"static":false,"computed":false,"key":{"type":"Literal","start":131,"end":132,"value":5,"raw":"5"},"kind":"set","value":{"type":"FunctionExpression","start":132,"end":138,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":133,"end":134,"name":"a"}],"body":{"type":"BlockStatement","start":136,"end":138,"body":[]}}},{"type":"MethodDefinition","start":143,"end":161,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":147,"end":155,"name":"private"},"kind":"set","value":{"type":"FunctionExpression","start":155,"end":161,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":156,"end":157,"name":"a"}],"body":{"type":"BlockStatement","start":159,"end":161,"body":[]}}}]}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"class NotSetter { 
    set(a) {} 
    async set(a) {} 
    static set(a) {} 
}"#,
        r#"{"type":"Program","start":0,"end":78,"body":[{"type":"ClassDeclaration","start":0,"end":78,"id":{"type":"Identifier","start":6,"end":15,"name":"NotSetter"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":78,"body":[{"type":"MethodDefinition","start":23,"end":32,"static":false,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"set"},"kind":"method","value":{"type":"FunctionExpression","start":26,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":27,"end":28,"name":"a"}],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}},{"type":"MethodDefinition","start":38,"end":53,"static":false,"computed":false,"key":{"type":"Identifier","start":44,"end":47,"name":"set"},"kind":"method","value":{"type":"FunctionExpression","start":47,"end":53,"id":null,"expression":false,"generator":false,"async":true,"params":[{"type":"Identifier","start":48,"end":49,"name":"a"}],"body":{"type":"BlockStatement","start":51,"end":53,"body":[]}}},{"type":"MethodDefinition","start":59,"end":75,"static":true,"computed":false,"key":{"type":"Identifier","start":66,"end":69,"name":"set"},"kind":"method","value":{"type":"FunctionExpression","start":69,"end":75,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":70,"end":71,"name":"a"}],"body":{"type":"BlockStatement","start":73,"end":75,"body":[]}}}]}}],"sourceType":"module"}"#
    );
}
