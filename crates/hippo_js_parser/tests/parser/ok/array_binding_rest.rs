use crate::parser::common::assert_parse_module_eq;

#[test]
fn array_binding_rest() {
    assert_parse_module_eq!(
        r#"let [ ...foo ] = bar;"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"VariableDeclaration","start":0,"end":21,"declarations":[{"type":"VariableDeclarator","start":4,"end":20,"id":{"type":"ArrayPattern","start":4,"end":14,"elements":[{"type":"RestElement","start":6,"end":12,"argument":{"type":"Identifier","start":9,"end":12,"name":"foo"}}]},"init":{"type":"Identifier","start":17,"end":20,"name":"bar"}}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let [ ...[foo, bar] ] = baz;"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":27,"id":{"type":"ArrayPattern","start":4,"end":21,"elements":[{"type":"RestElement","start":6,"end":19,"argument":{"type":"ArrayPattern","start":9,"end":19,"elements":[{"type":"Identifier","start":10,"end":13,"name":"foo"},{"type":"Identifier","start":15,"end":18,"name":"bar"}]}}]},"init":{"type":"Identifier","start":24,"end":27,"name":"baz"}}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let [ ...[ ...foo ] ] = bar;"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":27,"id":{"type":"ArrayPattern","start":4,"end":21,"elements":[{"type":"RestElement","start":6,"end":19,"argument":{"type":"ArrayPattern","start":9,"end":19,"elements":[{"type":"RestElement","start":11,"end":17,"argument":{"type":"Identifier","start":14,"end":17,"name":"foo"}}]}}]},"init":{"type":"Identifier","start":24,"end":27,"name":"bar"}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
