use crate::parser::common::assert_parse_module_eq;

#[test]
fn object_binding() {
    assert_parse_module_eq!(
        r#"let {foo, bar} = {one: 1, 2: "two"};"#,
        r#"{"type":"Program","start":0,"end":36,"body":[{"type":"VariableDeclaration","start":0,"end":36,"declarations":[{"type":"VariableDeclarator","start":4,"end":35,"id":{"type":"ObjectPattern","start":4,"end":14,"properties":[{"type":"Property","start":5,"end":8,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":5,"end":8,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":5,"end":8,"name":"foo"}},{"type":"Property","start":10,"end":13,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":10,"end":13,"name":"bar"},"kind":"init","value":{"type":"Identifier","start":10,"end":13,"name":"bar"}}]},"init":{"type":"ObjectExpression","start":17,"end":35,"properties":[{"type":"Property","start":18,"end":24,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":18,"end":21,"name":"one"},"kind":"init","value":{"type":"Literal","start":23,"end":24,"value":1.0,"raw":"1"}},{"type":"Property","start":26,"end":34,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":26,"end":27,"value":2.0,"raw":"2"},"kind":"init","value":{"type":"Literal","start":29,"end":34,"value":"two","raw":"\"two\""}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let {foo = "default", bar} = {};"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"VariableDeclaration","start":0,"end":32,"declarations":[{"type":"VariableDeclarator","start":4,"end":31,"id":{"type":"ObjectPattern","start":4,"end":26,"properties":[{"type":"Property","start":5,"end":20,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":5,"end":8,"name":"foo"},"kind":"init","value":{"type":"AssignmentPattern","start":5,"end":20,"left":{"type":"Identifier","start":5,"end":8,"name":"foo"},"right":{"type":"Literal","start":11,"end":20,"value":"default","raw":"\"default\""}}},{"type":"Property","start":22,"end":25,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":22,"end":25,"name":"bar"},"kind":"init","value":{"type":"Identifier","start":22,"end":25,"name":"bar"}}]},"init":{"type":"ObjectExpression","start":29,"end":31,"properties":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn object_binding_with_rest_element() {
    assert_parse_module_eq!(
        r#"let {foo, ...bar} = {};"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"VariableDeclaration","start":0,"end":23,"declarations":[{"type":"VariableDeclarator","start":4,"end":22,"id":{"type":"ObjectPattern","start":4,"end":17,"properties":[{"type":"Property","start":5,"end":8,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":5,"end":8,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":5,"end":8,"name":"foo"}},{"type":"RestElement","start":10,"end":16,"argument":{"type":"Identifier","start":13,"end":16,"name":"bar"}}]},"init":{"type":"ObjectExpression","start":20,"end":22,"properties":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn object_binding_with_object_binding_pattern() {
    assert_parse_module_eq!(
        r#"let {foo, bar: { baz } } = {};"#,
        r#"{"type":"Program","start":0,"end":30,"body":[{"type":"VariableDeclaration","start":0,"end":30,"declarations":[{"type":"VariableDeclarator","start":4,"end":29,"id":{"type":"ObjectPattern","start":4,"end":24,"properties":[{"type":"Property","start":5,"end":8,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":5,"end":8,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":5,"end":8,"name":"foo"}},{"type":"Property","start":10,"end":22,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":10,"end":13,"name":"bar"},"kind":"init","value":{"type":"ObjectPattern","start":15,"end":22,"properties":[{"type":"Property","start":17,"end":20,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":17,"end":20,"name":"baz"},"kind":"init","value":{"type":"Identifier","start":17,"end":20,"name":"baz"}}]}}]},"init":{"type":"ObjectExpression","start":27,"end":29,"properties":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

// This should error.
// #[test]
// fn object_binding_with_array_binding_pattern() {
//     assert_parse_module_eq!(r#"let {foo, ...[bar] } = {one: 1, 2: "two"};"#, r#""#);
// }
