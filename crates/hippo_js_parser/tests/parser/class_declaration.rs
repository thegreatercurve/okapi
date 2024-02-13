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
fn class_declaration_with_property_definitions() {}

#[test]
fn class_declaration_with_class_element_list() {
    assert_parse_module_eq!(r#"class Foo extends Bar { ;;; }"#, r#""#);
    assert_parse_module_eq!(r#"class Foo extends Bar { baz; bat = 1; }"#, r#""#);

    assert_parse_module_eq!(
        r#"class Foo extends Bar { constructor() {} method() {} }"#,
        r#""#
    );

    assert_parse_module_eq!(
        r#"class Foo extends Bar { static baz; static method() {},  }"#,
        r#""#
    );
}
