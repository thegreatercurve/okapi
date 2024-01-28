use crate::parser::common::assert_parse_module_eq;

#[test]
fn array_binding() {
    assert_parse_module_eq!(
        r#"let a = "b";"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"VariableDeclaration","start":0,"end":12,"declarations":[{"type":"VariableDeclarator","start":4,"end":11,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"Literal","start":8,"end":11,"value":"b","raw":"\"b\""}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [c, b] = [1, 2];"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":20,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"ArrayPattern","start":4,"end":10,"elements":[{"type":"Identifier","start":5,"end":6,"name":"c"},{"type":"Identifier","start":8,"end":9,"name":"b"}]},"init":{"type":"ArrayExpression","start":13,"end":19,"elements":[{"type":"Literal","start":14,"end":15,"value":1,"raw":"1"},{"type":"Literal","start":17,"end":18,"value":2,"raw":"2"}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [d, ...abcd] = [1];"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"VariableDeclaration","start":0,"end":23,"declarations":[{"type":"VariableDeclarator","start":4,"end":22,"id":{"type":"ArrayPattern","start":4,"end":16,"elements":[{"type":"Identifier","start":5,"end":6,"name":"d"},{"type":"RestElement","start":8,"end":15,"argument":{"type":"Identifier","start":11,"end":15,"name":"abcd"}}]},"init":{"type":"ArrayExpression","start":19,"end":22,"elements":[{"type":"Literal","start":20,"end":21,"value":1,"raw":"1"}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [e = "default", x] = []"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"VariableDeclaration","start":0,"end":27,"declarations":[{"type":"VariableDeclarator","start":4,"end":27,"id":{"type":"ArrayPattern","start":4,"end":22,"elements":[{"type":"AssignmentPattern","start":5,"end":18,"left":{"type":"Identifier","start":5,"end":6,"name":"e"},"right":{"type":"Literal","start":9,"end":18,"value":"default","raw":"\"default\""}},{"type":"Identifier","start":20,"end":21,"name":"x"}]},"init":{"type":"ArrayExpression","start":25,"end":27,"elements":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [, f, ...rest] = []"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"VariableDeclaration","start":0,"end":23,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"ArrayPattern","start":4,"end":18,"elements":[null,{"type":"Identifier","start":7,"end":8,"name":"f"},{"type":"RestElement","start":10,"end":17,"argument":{"type":"Identifier","start":13,"end":17,"name":"rest"}}]},"init":{"type":"ArrayExpression","start":21,"end":23,"elements":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let [[...rest2], { g }] = []"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":28,"id":{"type":"ArrayPattern","start":4,"end":23,"elements":[{"type":"ArrayPattern","start":5,"end":15,"elements":[{"type":"RestElement","start":6,"end":14,"argument":{"type":"Identifier","start":9,"end":14,"name":"rest2"}}]},{"type":"ObjectPattern","start":17,"end":22,"properties":[{"type":"Property","start":19,"end":20,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":19,"end":20,"name":"g"},"kind":"init","value":{"type":"Identifier","start":19,"end":20,"name":"g"}}]}]},"init":{"type":"ArrayExpression","start":26,"end":28,"elements":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
