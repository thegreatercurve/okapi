use crate::parser::assert_parser_script_eq;

#[test]
fn computed_member_name_in() {
    assert_parser_script_eq!(
        r#"for ({["x" in {}]: 3} ;;) {}"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"ForStatement","start":0,"end":28,"init":{"type":"ObjectExpression","start":5,"end":21,"properties":[{"type":"Property","start":6,"end":20,"method":false,"shorthand":false,"computed":true,"key":{"type":"BinaryExpression","start":7,"end":16,"left":{"type":"Literal","start":7,"end":10,"value":"x","raw":"\"x\""},"operator":"in","right":{"type":"ObjectExpression","start":14,"end":16,"properties":[]}},"value":{"type":"Literal","start":19,"end":20,"value":3,"raw":"3"},"kind":"init"}]},"test":null,"update":null,"body":{"type":"BlockStatement","start":26,"end":28,"body":[]}}],"sourceType":"script"}"#
    );
}
