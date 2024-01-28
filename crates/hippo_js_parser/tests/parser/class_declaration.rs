use crate::parser::common::assert_parse_module_eq;

#[test]
fn class_declaration() {
    assert_parse_module_eq!(
        r#"class foo {}"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ClassDeclaration","start":0,"end":12,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":12,"body":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"class foo extends bar {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ClassDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"bar"},"body":{"type":"ClassBody","start":22,"end":24,"body":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"class foo extends foo.bar {}"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"ClassDeclaration","start":0,"end":28,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":{"type":"MemberExpression","start":18,"end":25,"object":{"type":"Identifier","start":18,"end":21,"name":"foo"},"property":{"type":"Identifier","start":22,"end":25,"name":"bar"},"computed":false,"optional":false},"body":{"type":"ClassBody","start":26,"end":28,"body":[]}}],"sourceType":"module"}"#
    );
}
