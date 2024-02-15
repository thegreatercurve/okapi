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
        r#"class Foo extends Bar { baz; bat = 1; #qux = 2; [quux] = 3 }"#,
        r#"{"type":"Program","start":0,"end":60,"body":[{"type":"ClassDeclaration","start":0,"end":60,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":60,"body":[{"type":"PropertyDefinition","start":24,"end":28,"static":false,"computed":false,"key":{"type":"Identifier","start":24,"end":27,"name":"baz"},"value":null},{"type":"PropertyDefinition","start":29,"end":37,"static":false,"computed":false,"key":{"type":"Identifier","start":29,"end":32,"name":"bat"},"value":{"type":"Literal","start":35,"end":36,"value":1.0,"raw":"1"}},{"type":"PropertyDefinition","start":38,"end":47,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":38,"end":42,"name":"qux"},"value":{"type":"Literal","start":45,"end":46,"value":2.0,"raw":"2"}},{"type":"PropertyDefinition","start":48,"end":58,"static":false,"computed":true,"key":{"type":"Identifier","start":49,"end":53,"name":"quux"},"value":{"type":"Literal","start":57,"end":58,"value":3.0,"raw":"3"}}]}}],"sourceType":"module"}"#
    );

    // assert_parse_module_eq!(
    //     r#"class Foo extends Bar { constructor() {} method() {} }"#,
    //     r#""#
    // );

    // assert_parse_module_eq!(
    //     r#"class Foo extends Bar { static baz; static method() {},  }"#,
    //     r#""#
    // );
}

#[test]
fn class_declaration_with_static_block() {
    assert_parse_module_eq!(
        r#"class Foo { static { 1 + 1; } }"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"ClassDeclaration","start":0,"end":31,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":31,"body":[{"type":"StaticBlock","start":12,"end":29,"body":[{"type":"ExpressionStatement","start":21,"end":27,"expression":{"type":"BinaryExpression","start":21,"end":26,"left":{"type":"Literal","start":21,"end":22,"value":1.0,"raw":"1"},"operator":"+","right":{"type":"Literal","start":25,"end":26,"value":1.0,"raw":"1"}}}]}]}}],"sourceType":"module"}"#
    );
}
