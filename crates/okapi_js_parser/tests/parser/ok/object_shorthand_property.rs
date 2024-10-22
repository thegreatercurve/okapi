use crate::parser::assert_parser_script_eq;

#[test]
fn object_shorthand_property() {
    assert_parser_script_eq!(
        r#"let { d = "default", e = call() } = c"#,
        r#"{"type":"Program","start":0,"end":37,"body":[{"type":"VariableDeclaration","start":0,"end":37,"declarations":[{"type":"VariableDeclarator","start":4,"end":37,"id":{"type":"ObjectPattern","start":4,"end":33,"properties":[{"type":"Property","start":6,"end":19,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":6,"end":7,"name":"d"},"kind":"init","value":{"type":"AssignmentPattern","start":6,"end":19,"left":{"type":"Identifier","start":6,"end":7,"name":"d"},"right":{"type":"Literal","start":10,"end":19,"value":"default","raw":"\"default\""}}},{"type":"Property","start":21,"end":31,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":21,"end":22,"name":"e"},"kind":"init","value":{"type":"AssignmentPattern","start":21,"end":31,"left":{"type":"Identifier","start":21,"end":22,"name":"e"},"right":{"type":"CallExpression","start":25,"end":31,"callee":{"type":"Identifier","start":25,"end":29,"name":"call"},"arguments":[],"optional":false}}}]},"init":{"type":"Identifier","start":36,"end":37,"name":"c"}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
