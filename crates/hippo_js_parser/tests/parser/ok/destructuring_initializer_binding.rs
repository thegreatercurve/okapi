use crate::parser::common::assert_parse_module_eq;

#[test]
fn destructuring_initializer_binding() {
    assert_parse_module_eq!(
        r#"const { value, f = (value) => value } = item"#,
        r#"{"type":"Program","start":0,"end":44,"body":[{"type":"VariableDeclaration","start":0,"end":44,"declarations":[{"type":"VariableDeclarator","start":6,"end":44,"id":{"type":"ObjectPattern","start":6,"end":37,"properties":[{"type":"Property","start":8,"end":13,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":8,"end":13,"name":"value"},"kind":"init","value":{"type":"Identifier","start":8,"end":13,"name":"value"}},{"type":"Property","start":15,"end":35,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":15,"end":16,"name":"f"},"kind":"init","value":{"type":"AssignmentPattern","start":15,"end":35,"left":{"type":"Identifier","start":15,"end":16,"name":"f"},"right":{"type":"ArrowFunctionExpression","start":19,"end":35,"id":null,"expression":true,"generator":false,"async":false,"params":[{"type":"Identifier","start":20,"end":25,"name":"value"}],"body":{"type":"Identifier","start":30,"end":35,"name":"value"}}}}]},"init":{"type":"Identifier","start":40,"end":44,"name":"item"}}],"kind":"const"}],"sourceType":"module"}"#
    );
}
