use crate::parser::assert_parser_script_eq;

#[test]
fn arrow_in_class_constructor() {
    assert_parser_script_eq!(
        r#"class Foo { 
    contructor() { 
        () => { 
            bar(); 
        }; 
        () => baz(); 
    } 
}"#,
        r#"{"type":"Program","start":0,"end":112,"body":[{"type":"ClassDeclaration","start":0,"end":112,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":112,"body":[{"type":"MethodDefinition","start":17,"end":109,"static":false,"computed":false,"key":{"type":"Identifier","start":17,"end":27,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":27,"end":109,"id":null,"params":[],"generator":false,"expression":false,"async":false,"body":{"type":"BlockStatement","start":30,"end":109,"body":[{"type":"ExpressionStatement","start":41,"end":80,"expression":{"type":"ArrowFunctionExpression","start":41,"end":79,"id":null,"params":[],"generator":false,"expression":false,"async":false,"body":{"type":"BlockStatement","start":47,"end":79,"body":[{"type":"ExpressionStatement","start":62,"end":68,"expression":{"type":"CallExpression","start":62,"end":67,"callee":{"type":"Identifier","start":62,"end":65,"name":"bar"},"arguments":[],"optional":false}}]}}},{"type":"ExpressionStatement","start":90,"end":102,"expression":{"type":"ArrowFunctionExpression","start":90,"end":101,"id":null,"params":[],"generator":false,"expression":true,"async":false,"body":{"type":"CallExpression","start":96,"end":101,"callee":{"type":"Identifier","start":96,"end":99,"name":"baz"},"arguments":[],"optional":false}}}]}}}]}}],"sourceType":"script"}"#
    );
}
