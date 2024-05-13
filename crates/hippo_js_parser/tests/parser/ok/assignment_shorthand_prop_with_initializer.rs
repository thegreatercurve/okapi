use crate::parser::assert_parser_eq;

#[test]
fn assignment_shorthand_prop_with_initializer() {
    assert_parser_eq!(
        r#"for ({ arrow = () => {} } of [{}]) {}"#,
        r#"{"type":"Program","start":0,"end":37,"body":[{"type":"ForOfStatement","start":0,"end":37,"await":false,"left":{"type":"ObjectPattern","start":5,"end":25,"properties":[{"type":"Property","start":7,"end":23,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":7,"end":12,"name":"arrow"},"kind":"init","value":{"type":"AssignmentPattern","start":7,"end":23,"left":{"type":"Identifier","start":7,"end":12,"name":"arrow"},"right":{"type":"ArrowFunctionExpression","start":15,"end":23,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}}}}]},"right":{"type":"ArrayExpression","start":29,"end":33,"elements":[{"type":"ObjectExpression","start":30,"end":32,"properties":[]}]},"body":{"type":"BlockStatement","start":35,"end":37,"body":[]}}],"sourceType":"module"}"#
    );
}
