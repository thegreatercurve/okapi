use crate::parser::assert_parser_script_eq;

#[test]
fn class_declaration() {
    assert_parser_script_eq!(
        r#"class Foo {}"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ClassDeclaration","start":0,"end":12,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":12,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn class_declaration_with_class_heritage() {
    assert_parser_script_eq!(
        r#"class Foo extends Bar {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ClassDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn class_declaration_with_complex_class_heritage() {
    assert_parser_script_eq!(
        r#"class foo extends foo.bar {}"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"ClassDeclaration","start":0,"end":28,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":{"type":"MemberExpression","start":18,"end":25,"object":{"type":"Identifier","start":18,"end":21,"name":"foo"},"property":{"type":"Identifier","start":22,"end":25,"name":"bar"},"computed":false,"optional":false},"body":{"type":"ClassBody","start":26,"end":28,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn class_declaration_with_class_element_list_property_definitions() {
    assert_parser_script_eq!(
        r#"class Foo extends Bar { ;;; }"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"ClassDeclaration","start":0,"end":29,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":29,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class Foo extends Bar { 
    baz; bat = 1; 
    #qux = 2; 
    [quux] = 3; 
    static corge = "4";
}"#,
        r#"{"type":"Program","start":0,"end":101,"body":[{"type":"ClassDeclaration","start":0,"end":101,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":101,"body":[{"type":"PropertyDefinition","start":29,"end":33,"static":false,"computed":false,"key":{"type":"Identifier","start":29,"end":32,"name":"baz"},"value":null},{"type":"PropertyDefinition","start":34,"end":42,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":37,"name":"bat"},"value":{"type":"Literal","start":40,"end":41,"value":1,"raw":"1"}},{"type":"PropertyDefinition","start":48,"end":57,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":48,"end":52,"name":"qux"},"value":{"type":"Literal","start":55,"end":56,"value":2,"raw":"2"}},{"type":"PropertyDefinition","start":63,"end":74,"static":false,"computed":true,"key":{"type":"Identifier","start":64,"end":68,"name":"quux"},"value":{"type":"Literal","start":72,"end":73,"value":3,"raw":"3"}},{"type":"PropertyDefinition","start":80,"end":99,"static":true,"computed":false,"key":{"type":"Identifier","start":87,"end":92,"name":"corge"},"value":{"type":"Literal","start":95,"end":98,"value":"4","raw":"\"4\""}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn class_declaration_with_class_element_list_method_definitions() {
    assert_parser_script_eq!(
        r#"class Foo {
    constructor() {}
    bar() {}
}"#,
        r#"{"type":"Program","start":0,"end":47,"body":[{"type":"ClassDeclaration","start":0,"end":47,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":47,"body":[{"type":"MethodDefinition","start":16,"end":32,"static":false,"computed":false,"key":{"type":"Identifier","start":16,"end":27,"name":"constructor"},"kind":"constructor","value":{"type":"FunctionExpression","start":27,"end":32,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":30,"end":32,"body":[]}}},{"type":"MethodDefinition","start":37,"end":45,"static":false,"computed":false,"key":{"type":"Identifier","start":37,"end":40,"name":"bar"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":45,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":45,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn class_declaration_with_class_element_list_static_method_definitions() {
    assert_parser_script_eq!(
        r#"class foo {
    static foo(bar) {}
    static *foo() {}
    static async foo() {}
    static async *foo() {}
}"#,
        r#"{"type":"Program","start":0,"end":110,"body":[{"type":"ClassDeclaration","start":0,"end":110,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":110,"body":[{"type":"MethodDefinition","start":16,"end":34,"static":true,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":26,"end":34,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":27,"end":30,"name":"bar"}],"body":{"type":"BlockStatement","start":32,"end":34,"body":[]}}},{"type":"MethodDefinition","start":39,"end":55,"static":true,"computed":false,"key":{"type":"Identifier","start":47,"end":50,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":50,"end":55,"id":null,"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":53,"end":55,"body":[]}}},{"type":"MethodDefinition","start":60,"end":81,"static":true,"computed":false,"key":{"type":"Identifier","start":73,"end":76,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":76,"end":81,"id":null,"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":79,"end":81,"body":[]}}},{"type":"MethodDefinition","start":86,"end":108,"static":true,"computed":false,"key":{"type":"Identifier","start":100,"end":103,"name":"foo"},"kind":"method","value":{"type":"FunctionExpression","start":103,"end":108,"id":null,"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":106,"end":108,"body":[]}}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn class_declaration_with_static_block() {
    assert_parser_script_eq!(
        r#"class Foo { 
    static {
        1 + 1; 
    }
}"#,
        r#"{"type":"Program","start":0,"end":49,"body":[{"type":"ClassDeclaration","start":0,"end":49,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":49,"body":[{"type":"StaticBlock","start":17,"end":47,"body":[{"type":"ExpressionStatement","start":34,"end":40,"expression":{"type":"BinaryExpression","start":34,"end":39,"left":{"type":"Literal","start":34,"end":35,"value":1,"raw":"1"},"operator":"+","right":{"type":"Literal","start":38,"end":39,"value":1,"raw":"1"}}}]}]}}],"sourceType":"script"}"#
    );
}
