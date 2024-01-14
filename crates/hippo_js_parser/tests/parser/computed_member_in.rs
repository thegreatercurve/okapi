use crate::parser::common::assert_parser_eq;

#[test]
fn computed_member_in() {
    assert_parser_eq!(
        r#"for ({}["x" in {}];;) {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ForStatement","start":0,"end":24,"init":{"type":"MemberExpression","start":5,"end":18,"object":{"type":"ObjectExpression","start":5,"end":7,"properties":[]},"property":{"type":"BinaryExpression","start":8,"end":17,"left":{"type":"Literal","start":8,"end":11,"value":"x","raw":"\"x\""},"operator":"in","right":{"type":"ObjectExpression","start":15,"end":17,"properties":[]}},"computed":true,"optional":false},"test":null,"update":null,"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
    );
}
