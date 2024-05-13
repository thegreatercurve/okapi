use crate::parser::assert_parser_eq;

#[test]
fn getter_object_member() {
    assert_parser_eq!(
        r#"let a = {
    get foo() { return foo; },
    get bar() { return "bar"; },
    get ["a" + "b"]() { return "a" + "b"; },
    get 5() { return 5; },
    get() { return "This is a method and not a getter"; },
};"#,
        r#"{"type":"Program","start":0,"end":207,"body":[{"type":"VariableDeclaration","start":0,"end":207,"declarations":[{"type":"VariableDeclarator","start":4,"end":206,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":206,"properties":[{"type":"Property","start":14,"end":39,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":18,"end":21,"name":"foo"},"kind":"get","value":{"type":"FunctionExpression","start":21,"end":39,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":24,"end":39,"body":[{"type":"ReturnStatement","start":26,"end":37,"argument":{"type":"Identifier","start":33,"end":36,"name":"foo"}}]}}},{"type":"Property","start":45,"end":72,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":49,"end":52,"name":"bar"},"kind":"get","value":{"type":"FunctionExpression","start":52,"end":72,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":55,"end":72,"body":[{"type":"ReturnStatement","start":57,"end":70,"argument":{"type":"Literal","start":64,"end":69,"value":"bar","raw":"\"bar\""}}]}}},{"type":"Property","start":78,"end":117,"method":false,"shorthand":false,"computed":true,"key":{"type":"BinaryExpression","start":83,"end":92,"left":{"type":"Literal","start":83,"end":86,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":89,"end":92,"value":"b","raw":"\"b\""}},"kind":"get","value":{"type":"FunctionExpression","start":93,"end":117,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":96,"end":117,"body":[{"type":"ReturnStatement","start":98,"end":115,"argument":{"type":"BinaryExpression","start":105,"end":114,"left":{"type":"Literal","start":105,"end":108,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":111,"end":114,"value":"b","raw":"\"b\""}}}]}}},{"type":"Property","start":123,"end":144,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":127,"end":128,"value":5,"raw":"5"},"kind":"get","value":{"type":"FunctionExpression","start":128,"end":144,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":131,"end":144,"body":[{"type":"ReturnStatement","start":133,"end":142,"argument":{"type":"Literal","start":140,"end":141,"value":5,"raw":"5"}}]}}},{"type":"Property","start":150,"end":203,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":150,"end":153,"name":"get"},"kind":"init","value":{"type":"FunctionExpression","start":153,"end":203,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":156,"end":203,"body":[{"type":"ReturnStatement","start":158,"end":201,"argument":{"type":"Literal","start":165,"end":200,"value":"This is a method and not a getter","raw":"\"This is a method and not a getter\""}}]}}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
