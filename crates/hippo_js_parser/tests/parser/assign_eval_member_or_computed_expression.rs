use crate::parser::common::assert_parse_module_eq;

#[test]
fn assign_eval_member_or_computed_expression() {
    assert_parse_module_eq!(
        r#"eval.foo = 10"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"AssignmentExpression","start":0,"end":13,"operator":"=","left":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":4,"name":"eval"},"property":{"type":"Identifier","start":5,"end":8,"name":"foo"},"computed":false,"optional":false},"right":{"type":"Literal","start":11,"end":13,"value":10,"raw":"10"}}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"arguments[1] = "baz""#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"AssignmentExpression","start":0,"end":20,"operator":"=","left":{"type":"MemberExpression","start":0,"end":12,"object":{"type":"Identifier","start":0,"end":9,"name":"arguments"},"property":{"type":"Literal","start":10,"end":11,"value":1,"raw":"1"},"computed":true,"optional":false},"right":{"type":"Literal","start":15,"end":20,"value":"baz","raw":"\"baz\""}}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"eval[2] = "Chungking Express""#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"ExpressionStatement","start":0,"end":29,"expression":{"type":"AssignmentExpression","start":0,"end":29,"operator":"=","left":{"type":"MemberExpression","start":0,"end":7,"object":{"type":"Identifier","start":0,"end":4,"name":"eval"},"property":{"type":"Literal","start":5,"end":6,"value":2,"raw":"2"},"computed":true,"optional":false},"right":{"type":"Literal","start":10,"end":29,"value":"Chungking Express","raw":"\"Chungking Express\""}}}],"sourceType":"script"}"#
    );
}
