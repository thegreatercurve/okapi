use crate::parser::assert_parser_script_eq;

#[test]
fn lexical_declaration() {
    assert_parser_script_eq!(
        "let foo, bar;",
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"VariableDeclaration","start":0,"end":13,"declarations":[{"type":"VariableDeclarator","start":4,"end":7,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":null},{"type":"VariableDeclarator","start":9,"end":12,"id":{"type":"Identifier","start":9,"end":12,"name":"bar"},"init":null}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let {...foo} = null;"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":20,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"ObjectPattern","start":4,"end":12,"properties":[{"type":"RestElement","start":5,"end":11,"argument":{"type":"Identifier","start":8,"end":11,"name":"foo"}}]},"init":{"type":"Literal","start":15,"end":19,"value":null,"raw":"null"}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let { foo, bar } = {};"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"VariableDeclaration","start":0,"end":22,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"ObjectPattern","start":4,"end":16,"properties":[{"type":"Property","start":6,"end":9,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":6,"end":9,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":6,"end":9,"name":"foo"}},{"type":"Property","start":11,"end":14,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":11,"end":14,"name":"bar"},"kind":"init","value":{"type":"Identifier","start":11,"end":14,"name":"bar"}}]},"init":{"type":"ObjectExpression","start":19,"end":21,"properties":[]}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        "const foo = null;",
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"VariableDeclaration","start":0,"end":17,"declarations":[{"type":"VariableDeclarator","start":6,"end":16,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"init":{"type":"Literal","start":12,"end":16,"value":null,"raw":"null"}}],"kind":"const"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"const { foo: [bar], baz } = {};"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"VariableDeclaration","start":0,"end":31,"declarations":[{"type":"VariableDeclarator","start":6,"end":30,"id":{"type":"ObjectPattern","start":6,"end":25,"properties":[{"type":"Property","start":8,"end":18,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":8,"end":11,"name":"foo"},"value":{"type":"ArrayPattern","start":13,"end":18,"elements":[{"type":"Identifier","start":14,"end":17,"name":"bar"}]},"kind":"init"},{"type":"Property","start":20,"end":23,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":20,"end":23,"name":"baz"},"kind":"init","value":{"type":"Identifier","start":20,"end":23,"name":"baz"}}]},"init":{"type":"ObjectExpression","start":28,"end":30,"properties":[]}}],"kind":"const"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let foo = "bar", baz = null;"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":15,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":{"type":"Literal","start":10,"end":15,"value":"bar","raw":"\"bar\""}},{"type":"VariableDeclarator","start":17,"end":27,"id":{"type":"Identifier","start":17,"end":20,"name":"baz"},"init":{"type":"Literal","start":23,"end":27,"value":null,"raw":"null"}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let foo = {[computed]: "a"};"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":27,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":{"type":"ObjectExpression","start":10,"end":27,"properties":[{"type":"Property","start":11,"end":26,"method":false,"shorthand":false,"computed":true,"key":{"type":"Identifier","start":12,"end":20,"name":"computed"},"value":{"type":"Literal","start":23,"end":26,"value":"a","raw":"\"a\""},"kind":"init"}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}

#[test]
fn lexical_declaration_conditional_keyword_identifier() {
    assert_parser_script_eq!(
        r#"let get = foo.bar;"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"VariableDeclaration","start":0,"end":18,"kind":"let","declarations":[{"type":"VariableDeclarator","start":4,"end":17,"id":{"type":"Identifier","start":4,"end":7,"name":"get"},"init":{"type":"MemberExpression","start":10,"end":17,"object":{"type":"Identifier","start":10,"end":13,"name":"foo"},"property":{"type":"Identifier","start":14,"end":17,"name":"bar"},"computed":false,"optional":false}}]}],"sourceType":"script"}"#
    );
}
