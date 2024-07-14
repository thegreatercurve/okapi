use crate::parser::assert_parser_eq;

#[test]
fn decorator_class_declaration_top_level() {
    assert_parser_eq!(r#"@decorator"#, r#"undefined"#);
    assert_parser_eq!(
        r#"class Foo { }"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ClassDeclaration","start":0,"end":13,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":13,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"@first.field @second @(() => decorator)()"#,
        r#"undefined"#
    );

    assert_parser_eq!(
        r#"class Bar {}"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ClassDeclaration","start":0,"end":12,"id":{"type":"Identifier","start":6,"end":9,"name":"Bar"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":12,"body":[]}}],"sourceType":"script"}"#
    );
}
