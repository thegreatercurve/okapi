use crate::parser::common::assert_parse_module_eq;

#[test]
fn array_binding_rest() {
    assert_parse_module_eq!(
        r#"let [ ...abcd ] = a;"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":20,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"ArrayPattern","start":4,"end":15,"elements":[{"type":"RestElement","start":6,"end":13,"argument":{"type":"Identifier","start":9,"end":13,"name":"abcd"}}]},"init":{"type":"Identifier","start":18,"end":19,"name":"a"}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [ ...[x, y] ] = b;"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"VariableDeclaration","start":0,"end":22,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"ArrayPattern","start":4,"end":17,"elements":[{"type":"RestElement","start":6,"end":15,"argument":{"type":"ArrayPattern","start":9,"end":15,"elements":[{"type":"Identifier","start":10,"end":11,"name":"x"},{"type":"Identifier","start":13,"end":14,"name":"y"}]}}]},"init":{"type":"Identifier","start":20,"end":21,"name":"b"}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [ ...[ ...a ] ] = c;"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"VariableDeclaration","start":0,"end":24,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"ArrayPattern","start":4,"end":19,"elements":[{"type":"RestElement","start":6,"end":17,"argument":{"type":"ArrayPattern","start":9,"end":17,"elements":[{"type":"RestElement","start":11,"end":15,"argument":{"type":"Identifier","start":14,"end":15,"name":"a"}}]}}]},"init":{"type":"Identifier","start":22,"end":23,"name":"c"}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
