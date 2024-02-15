use crate::parser::common::assert_parse_module_eq;

#[test]
fn class_declaration() {
    assert_parse_module_eq!(
        r#"class Foo {}"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ClassDeclaration","start":0,"end":12,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":12,"body":[]}}],"sourceType":"module"}"#
    );
}

#[test]
fn class_declaration_with_class_heritage() {
    assert_parse_module_eq!(
        r#"class Foo extends Bar {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ClassDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":24,"body":[]}}],"sourceType":"module"}"#
    );
}

// #[test]
// fn class_declaration_with_complex_class_heritage() {
//     assert_parse_module_eq!(
//         r#"class foo extends foo.bar {}"#,
//         r#"{"type":"Program","start":0,"end":28,"body":[{"type":"ClassDeclaration","start":0,"end":28,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":{"type":"MemberExpression","start":18,"end":25,"object":{"type":"Identifier","start":18,"end":21,"name":"foo"},"property":{"type":"Identifier","start":22,"end":25,"name":"bar"},"computed":false,"optional":false},"body":{"type":"ClassBody","start":26,"end":28,"body":[]}}],"sourceType":"module"}"#
//     );
// }

#[test]
fn class_declaration_with_class_element_list() {
    assert_parse_module_eq!(
        r#"class Foo extends Bar { ;;; }"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"ClassDeclaration","start":0,"end":29,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":29,"body":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"class Foo extends Bar { 
    baz; bat = 1; 
    #qux = 2; 
    [quux] = 3; 
    static corge = "4";
}"#,
        r#"{"type":"Program","start":0,"end":101,"body":[{"type":"ClassDeclaration","start":0,"end":101,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":101,"body":[{"type":"PropertyDefinition","start":29,"end":33,"static":false,"computed":false,"key":{"type":"Identifier","start":29,"end":32,"name":"baz"},"value":null},{"type":"PropertyDefinition","start":34,"end":42,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":37,"name":"bat"},"value":{"type":"Literal","start":40,"end":41,"value":1.0,"raw":"1"}},{"type":"PropertyDefinition","start":48,"end":57,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":48,"end":52,"name":"qux"},"value":{"type":"Literal","start":55,"end":56,"value":2.0,"raw":"2"}},{"type":"PropertyDefinition","start":63,"end":74,"static":false,"computed":true,"key":{"type":"Identifier","start":64,"end":68,"name":"quux"},"value":{"type":"Literal","start":72,"end":73,"value":3.0,"raw":"3"}},{"type":"PropertyDefinition","start":80,"end":99,"static":true,"computed":false,"key":{"type":"Identifier","start":87,"end":92,"name":"corge"},"value":{"type":"Literal","start":95,"end":98,"value":"4","raw":"\"4\""}}]}}],"sourceType":"module"}"#
    );
}

#[test]
fn class_declaration_with_class_element_list_method_definitions() {
    assert_parse_module_eq!(
        r#"class Foo { constructor() {} bar() {} }"#,
        r#"{"type":"Program","start":0,"end":39,"body":[{"type":"ClassDeclaration","start":0,"end":39,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":39,"body":[{"type":"MethodDefinition","start":12,"end":28,"static":false,"computed":false,"key":{"type":"Identifier","start":12,"end":23,"name":"constructor"},"kind":"constructor","value":{"type":"FunctionExpression","start":23,"end":28,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":26,"end":28,"body":[]}}},{"type":"MethodDefinition","start":29,"end":37,"static":false,"computed":false,"key":{"type":"Identifier","start":29,"end":32,"name":"bar"},"kind":"method","value":{"type":"FunctionExpression","start":32,"end":37,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":35,"end":37,"body":[]}}}]}}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"class Foo { static method() {} }"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"ClassDeclaration","start":0,"end":32,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":32,"body":[{"type":"MethodDefinition","start":12,"end":30,"static":true,"computed":false,"key":{"type":"Identifier","start":19,"end":25,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":25,"end":30,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":28,"end":30,"body":[]}}}]}}],"sourceType":"module"}"#
    );
}

#[test]
fn class_declaration_with_static_block() {
    assert_parse_module_eq!(
        r#"class Foo { static { 1 + 1; } }"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"ClassDeclaration","start":0,"end":31,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":31,"body":[{"type":"StaticBlock","start":12,"end":29,"body":[{"type":"ExpressionStatement","start":21,"end":27,"expression":{"type":"BinaryExpression","start":21,"end":26,"left":{"type":"Literal","start":21,"end":22,"value":1.0,"raw":"1"},"operator":"+","right":{"type":"Literal","start":25,"end":26,"value":1.0,"raw":"1"}}}]}]}}],"sourceType":"module"}"#
    );
}
