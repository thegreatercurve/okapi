use crate::parser::common::assert_parse_module_eq;

#[test]
fn array_binding() {
    assert_parse_module_eq!(
        r#"let [,,foo] = [1, 2, 3];"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"VariableDeclaration","start":0,"end":24,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"ArrayPattern","start":4,"end":11,"elements":[null,null,{"type":"Identifier","start":7,"end":10,"name":"foo"}]},"init":{"type":"ArrayExpression","start":14,"end":23,"elements":[{"type":"Literal","start":15,"end":16,"value":1.0,"raw":"1"},{"type":"Literal","start":18,"end":19,"value":2.0,"raw":"2"},{"type":"Literal","start":21,"end":22,"value":3.0,"raw":"3"}]}}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let [foo, bar] = [1, 2, 3];"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"VariableDeclaration","start":0,"end":27,"declarations":[{"type":"VariableDeclarator","start":4,"end":26,"id":{"type":"ArrayPattern","start":4,"end":14,"elements":[{"type":"Identifier","start":5,"end":8,"name":"foo"},{"type":"Identifier","start":10,"end":13,"name":"bar"}]},"init":{"type":"ArrayExpression","start":17,"end":26,"elements":[{"type":"Literal","start":18,"end":19,"value":1.0,"raw":"1"},{"type":"Literal","start":21,"end":22,"value":2.0,"raw":"2"},{"type":"Literal","start":24,"end":25,"value":3.0,"raw":"3"}]}}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let [foo = "default", bar] = []"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"VariableDeclaration","start":0,"end":31,"declarations":[{"type":"VariableDeclarator","start":4,"end":31,"id":{"type":"ArrayPattern","start":4,"end":26,"elements":[{"type":"AssignmentPattern","start":5,"end":20,"left":{"type":"Identifier","start":5,"end":8,"name":"foo"},"right":{"type":"Literal","start":11,"end":20,"value":"default","raw":"\"default\""}},{"type":"Identifier","start":22,"end":25,"name":"bar"}]},"init":{"type":"ArrayExpression","start":29,"end":31,"elements":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn array_binding_with_rest_element() {
    assert_parse_module_eq!(
        r#"let [foo, ...bar] = [1];"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"VariableDeclaration","start":0,"end":24,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"ArrayPattern","start":4,"end":17,"elements":[{"type":"Identifier","start":5,"end":8,"name":"foo"},{"type":"RestElement","start":10,"end":16,"argument":{"type":"Identifier","start":13,"end":16,"name":"bar"}}]},"init":{"type":"ArrayExpression","start":20,"end":23,"elements":[{"type":"Literal","start":21,"end":22,"value":1.0,"raw":"1"}]}}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let [, f, ...rest] = []"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"VariableDeclaration","start":0,"end":23,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"ArrayPattern","start":4,"end":18,"elements":[null,{"type":"Identifier","start":7,"end":8,"name":"f"},{"type":"RestElement","start":10,"end":17,"argument":{"type":"Identifier","start":13,"end":17,"name":"rest"}}]},"init":{"type":"ArrayExpression","start":21,"end":23,"elements":[]}}],"kind":"let"}],"sourceType":"module"}"#
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

#[test]
fn array_binding_with_array_binding_pattern() {
    assert_parse_module_eq!(
        r#"let [foo, [bar]] = [1, 2, 3];"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"VariableDeclaration","start":0,"end":29,"declarations":[{"type":"VariableDeclarator","start":4,"end":28,"id":{"type":"ArrayPattern","start":4,"end":16,"elements":[{"type":"Identifier","start":5,"end":8,"name":"foo"},{"type":"ArrayPattern","start":10,"end":15,"elements":[{"type":"Identifier","start":11,"end":14,"name":"bar"}]}]},"init":{"type":"ArrayExpression","start":19,"end":28,"elements":[{"type":"Literal","start":20,"end":21,"value":1.0,"raw":"1"},{"type":"Literal","start":23,"end":24,"value":2.0,"raw":"2"},{"type":"Literal","start":26,"end":27,"value":3.0,"raw":"3"}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn array_binding_with_object_binding_pattern() {
    assert_parse_module_eq!(
        r#"let [[...rest2], { g }] = []"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":28,"id":{"type":"ArrayPattern","start":4,"end":23,"elements":[{"type":"ArrayPattern","start":5,"end":15,"elements":[{"type":"RestElement","start":6,"end":14,"argument":{"type":"Identifier","start":9,"end":14,"name":"rest2"}}]},{"type":"ObjectPattern","start":17,"end":22,"properties":[{"type":"Property","start":19,"end":20,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":19,"end":20,"name":"g"},"kind":"init","value":{"type":"Identifier","start":19,"end":20,"name":"g"}}]}]},"init":{"type":"ArrayExpression","start":26,"end":28,"elements":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
