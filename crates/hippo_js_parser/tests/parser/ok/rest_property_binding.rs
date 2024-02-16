use crate::parser::common::assert_parse_module_eq;

#[test]
fn rest_property_binding() {
    assert_parse_module_eq!(
        r#"let { ...abcd } = a;"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":20,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"ObjectPattern","start":4,"end":15,"properties":[{"type":"RestElement","start":6,"end":13,"argument":{"type":"Identifier","start":9,"end":13,"name":"abcd"}}]},"init":{"type":"Identifier","start":18,"end":19,"name":"a"}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let { b: { ...a } } = c;"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"VariableDeclaration","start":0,"end":24,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"ObjectPattern","start":4,"end":19,"properties":[{"type":"Property","start":6,"end":17,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":6,"end":7,"name":"b"},"value":{"type":"ObjectPattern","start":9,"end":17,"properties":[{"type":"RestElement","start":11,"end":15,"argument":{"type":"Identifier","start":14,"end":15,"name":"a"}}]},"kind":"init"}]},"init":{"type":"Identifier","start":22,"end":23,"name":"c"}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
