use crate::parser::common::assert_parser_eq;

#[test]
fn pattern_with_default_in_keyword() {
    assert_parser_eq!(
        r#"for ([a = "a" in {}] in []) {}"#,
        r#"{"type":"Program","start":0,"end":30,"body":[{"type":"ForInStatement","start":0,"end":30,"left":{"type":"ArrayPattern","start":5,"end":20,"elements":[{"type":"AssignmentPattern","start":6,"end":19,"left":{"type":"Identifier","start":6,"end":7,"name":"a"},"right":{"type":"BinaryExpression","start":10,"end":19,"left":{"type":"Literal","start":10,"end":13,"value":"a","raw":"\"a\""},"operator":"in","right":{"type":"ObjectExpression","start":17,"end":19,"properties":[]}}}]},"right":{"type":"ArrayExpression","start":24,"end":26,"elements":[]},"body":{"type":"BlockStatement","start":28,"end":30,"body":[]}}],"sourceType":"script"}"#
    );
}
