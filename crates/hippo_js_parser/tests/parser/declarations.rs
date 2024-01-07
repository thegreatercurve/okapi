use crate::parser::common::assert_parser_eq;

#[test]
fn single_binding_identifier() {
    assert_parser_eq!(
        "let hello;",
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"VariableDeclaration","start":0,"end":10,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":9,"name":"hello"},"init":null}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        "var hello;",
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"VariableDeclaration","start":0,"end":10,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":9,"name":"hello"},"init":null}],"kind":"var"}],"sourceType":"module"}"#
    );
}

#[test]
fn multiple_binding_identifiers() {
    assert_parser_eq!(
        "let hello, world;",
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"VariableDeclaration","start":0,"end":17,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":9,"name":"hello"},"init":null},{"type":"VariableDeclarator","start":11,"end":16,"id":{"type":"Identifier","start":11,"end":16,"name":"world"},"init":null}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        "var hello, world;",
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"VariableDeclaration","start":0,"end":17,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":9,"name":"hello"},"init":null},{"type":"VariableDeclarator","start":11,"end":16,"id":{"type":"Identifier","start":11,"end":16,"name":"world"},"init":null}],"kind":"var"}],"sourceType":"module"}"#
    );
}

#[test]
fn simple_object_binding_pattern() {
    assert_parser_eq!(
        r"const {} = {}",
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"VariableDeclaration","start":0,"end":13,"declarations":[{"type":"VariableDeclarator","start":6,"end":13,"id":{"type":"ObjectPattern","start":6,"end":8,"properties":[]},"init":{"type":"ObjectExpression","start":11,"end":13,"properties":[]}}],"kind":"const"}],"sourceType":"module"}"#
    );
}
