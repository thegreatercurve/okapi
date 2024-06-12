use crate::parser::assert_parser_script_eq;

#[test]
fn getter_class_member() {
    assert_parser_script_eq!(
        r#"class Getter {
    get foo() {}
    get static() {} 
    static get bar() {}
    get baz() {}
    get ["a" + "b"]() {}
    get 5() {}
    get #private() {} 
}"#,
        r#"{"type":"Program","start":0,"end":158,"body":[{"type":"ClassDeclaration","start":0,"end":158,"id":{"type":"Identifier","start":6,"end":12,"name":"Getter"},"superClass":null,"body":{"type":"ClassBody","start":13,"end":158,"body":[{"type":"MethodDefinition","start":19,"end":31,"static":false,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"foo"},"kind":"get","value":{"type":"FunctionExpression","start":26,"end":31,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}},{"type":"MethodDefinition","start":36,"end":51,"static":false,"computed":false,"key":{"type":"Identifier","start":40,"end":46,"name":"static"},"kind":"get","value":{"type":"FunctionExpression","start":46,"end":51,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":49,"end":51,"body":[]}}},{"type":"MethodDefinition","start":57,"end":76,"static":true,"computed":false,"key":{"type":"Identifier","start":68,"end":71,"name":"bar"},"kind":"get","value":{"type":"FunctionExpression","start":71,"end":76,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":74,"end":76,"body":[]}}},{"type":"MethodDefinition","start":81,"end":93,"static":false,"computed":false,"key":{"type":"Identifier","start":85,"end":88,"name":"baz"},"kind":"get","value":{"type":"FunctionExpression","start":88,"end":93,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":91,"end":93,"body":[]}}},{"type":"MethodDefinition","start":98,"end":118,"static":false,"computed":true,"key":{"type":"BinaryExpression","start":103,"end":112,"left":{"type":"Literal","start":103,"end":106,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":109,"end":112,"value":"b","raw":"\"b\""}},"kind":"get","value":{"type":"FunctionExpression","start":113,"end":118,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":116,"end":118,"body":[]}}},{"type":"MethodDefinition","start":123,"end":133,"static":false,"computed":false,"key":{"type":"Literal","start":127,"end":128,"value":5,"raw":"5"},"kind":"get","value":{"type":"FunctionExpression","start":128,"end":133,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":131,"end":133,"body":[]}}},{"type":"MethodDefinition","start":138,"end":155,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":142,"end":150,"name":"private"},"kind":"get","value":{"type":"FunctionExpression","start":150,"end":155,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":153,"end":155,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn getter_class_member_get_as_property_name() {
    assert_parser_script_eq!(
        r#"class NotGetter {
    get() {}
    async get() {}
    static get() {}
}"#,
        r#"{"type":"Program","start":0,"end":71,"body":[{"type":"ClassDeclaration","start":0,"end":71,"id":{"type":"Identifier","start":6,"end":15,"name":"NotGetter"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":71,"body":[{"type":"MethodDefinition","start":22,"end":30,"static":false,"computed":false,"key":{"type":"Identifier","start":22,"end":25,"name":"get"},"kind":"method","value":{"type":"FunctionExpression","start":25,"end":30,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":28,"end":30,"body":[]}}},{"type":"MethodDefinition","start":35,"end":49,"static":false,"computed":false,"key":{"type":"Identifier","start":41,"end":44,"name":"get"},"kind":"method","value":{"type":"FunctionExpression","start":44,"end":49,"id":null,"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":47,"end":49,"body":[]}}},{"type":"MethodDefinition","start":54,"end":69,"static":true,"computed":false,"key":{"type":"Identifier","start":61,"end":64,"name":"get"},"kind":"method","value":{"type":"FunctionExpression","start":64,"end":69,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":67,"end":69,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}
