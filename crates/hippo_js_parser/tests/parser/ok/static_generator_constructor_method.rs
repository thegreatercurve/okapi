use crate::parser::assert_parser_eq;

#[test]
fn static_generator_constructor_method() {
    assert_parser_eq!(
        r#"class A { static async *contructor() {} static *contructor() {} }"#,
        r#"{"type":"Program","start":0,"end":65,"body":[{"type":"ClassDeclaration","start":0,"end":65,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":65,"body":[{"type":"MethodDefinition","start":10,"end":39,"static":true,"computed":false,"key":{"type":"Identifier","start":24,"end":34,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":34,"end":39,"id":null,"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":37,"end":39,"body":[]}}},{"type":"MethodDefinition","start":40,"end":63,"static":true,"computed":false,"key":{"type":"Identifier","start":48,"end":58,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":58,"end":63,"id":null,"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":61,"end":63,"body":[]}}}]}}],"sourceType":"module"}"#
    );
}
