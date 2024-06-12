


use crate::parser::assert_parser_script_eq;

#[test]
fn setter_object_member() {
    assert_parser_script_eq!(
        r#"let a = { 
    set foo(value) {}, 
    set bar(value) {}, 
    set ["a" + "b"](value) {}, 
    set 5(value) {}, 
    set() { return "Thi i a method and not a setter"; }, 
}"#,
        r#"{"type":"Program","start":0,"end":172,"body":[{"type":"VariableDeclaration","start":0,"end":172,"declarations":[{"type":"VariableDeclarator","start":4,"end":172,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":172,"properties":[{"type":"Property","start":15,"end":32,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":19,"end":22,"name":"foo"},"kind":"set","value":{"type":"FunctionExpression","start":22,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":23,"end":28,"name":"value"}],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}},{"type":"Property","start":39,"end":56,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":43,"end":46,"name":"bar"},"kind":"set","value":{"type":"FunctionExpression","start":46,"end":56,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":47,"end":52,"name":"value"}],"body":{"type":"BlockStatement","start":54,"end":56,"body":[]}}},{"type":"Property","start":63,"end":88,"method":false,"shorthand":false,"computed":true,"key":{"type":"BinaryExpression","start":68,"end":77,"left":{"type":"Literal","start":68,"end":71,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":74,"end":77,"value":"b","raw":"\"b\""}},"kind":"set","value":{"type":"FunctionExpression","start":78,"end":88,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":79,"end":84,"name":"value"}],"body":{"type":"BlockStatement","start":86,"end":88,"body":[]}}},{"type":"Property","start":95,"end":110,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":99,"end":100,"value":5,"raw":"5"},"kind":"set","value":{"type":"FunctionExpression","start":100,"end":110,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":101,"end":106,"name":"value"}],"body":{"type":"BlockStatement","start":108,"end":110,"body":[]}}},{"type":"Property","start":117,"end":168,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":117,"end":120,"name":"set"},"kind":"init","value":{"type":"FunctionExpression","start":120,"end":168,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":123,"end":168,"body":[{"type":"ReturnStatement","start":125,"end":166,"argument":{"type":"Literal","start":132,"end":165,"value":"Thi i a method and not a setter","raw":"\"Thi i a method and not a setter\""}}]}}}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
