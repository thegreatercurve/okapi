use crate::parser::common::assert_parser_eq;

#[test]
fn object_property_binding() {
    assert_parser_eq!(
        r#"let { foo: bar  } = {}"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"VariableDeclaration","start":0,"end":22,"declarations":[{"type":"VariableDeclarator","start":4,"end":22,"id":{"type":"ObjectPattern","start":4,"end":17,"properties":[{"type":"Property","start":6,"end":14,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":6,"end":9,"name":"foo"},"value":{"type":"Identifier","start":11,"end":14,"name":"bar"},"kind":"init"}]},"init":{"type":"ObjectExpression","start":20,"end":22,"properties":[]}}],"kind":"let"}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"let { foo: bar_bar = baz } = {}"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"VariableDeclaration","start":0,"end":31,"declarations":[{"type":"VariableDeclarator","start":4,"end":31,"id":{"type":"ObjectPattern","start":4,"end":26,"properties":[{"type":"Property","start":6,"end":24,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":6,"end":9,"name":"foo"},"value":{"type":"AssignmentPattern","start":11,"end":24,"left":{"type":"Identifier","start":11,"end":18,"name":"bar_bar"},"right":{"type":"Identifier","start":21,"end":24,"name":"baz"}},"kind":"init"}]},"init":{"type":"ObjectExpression","start":29,"end":31,"properties":[]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
