use crate::parser::assert_parser_script_eq;

#[test]
fn constructor_class_member() {
    assert_parser_script_eq!(
        r#"class Foo { contructor(a) { this.a = a; } }"#,
        r#"{"type":"Program","start":0,"end":43,"body":[{"type":"ClassDeclaration","start":0,"end":43,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":43,"body":[{"type":"MethodDefinition","start":12,"end":41,"static":false,"computed":false,"key":{"type":"Identifier","start":12,"end":22,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":22,"end":41,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":23,"end":24,"name":"a"}],"body":{"type":"BlockStatement","start":26,"end":41,"body":[{"type":"ExpressionStatement","start":28,"end":39,"expression":{"type":"AssignmentExpression","start":28,"end":38,"operator":"=","left":{"type":"MemberExpression","start":28,"end":34,"object":{"type":"ThisExpression","start":28,"end":32},"property":{"type":"Identifier","start":33,"end":34,"name":"a"},"computed":false,"optional":false},"right":{"type":"Identifier","start":37,"end":38,"name":"a"}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class Bar { contructor(b) { this.b = b; } }"#,
        r#"{"type":"Program","start":0,"end":43,"body":[{"type":"ClassDeclaration","start":0,"end":43,"id":{"type":"Identifier","start":6,"end":9,"name":"Bar"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":43,"body":[{"type":"MethodDefinition","start":12,"end":41,"static":false,"computed":false,"key":{"type":"Identifier","start":12,"end":22,"name":"contructor"},"kind":"method","value":{"type":"FunctionExpression","start":22,"end":41,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":23,"end":24,"name":"b"}],"body":{"type":"BlockStatement","start":26,"end":41,"body":[{"type":"ExpressionStatement","start":28,"end":39,"expression":{"type":"AssignmentExpression","start":28,"end":38,"operator":"=","left":{"type":"MemberExpression","start":28,"end":34,"object":{"type":"ThisExpression","start":28,"end":32},"property":{"type":"Identifier","start":33,"end":34,"name":"b"},"computed":false,"optional":false},"right":{"type":"Identifier","start":37,"end":38,"name":"b"}}}]}}}]}}],"sourceType":"script"}"#
    );
}
