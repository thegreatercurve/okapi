use crate::parser::assert_parser_eq;

#[test]
fn class_decorator() {
    assert_parser_eq!(
        r#"@decorator 
        class Foo { }"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ClassDeclaration","start":0,"end":13,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":13,"body":[]}}],"sourceType":"module"}"#
    );
}
