use crate::parser::assert_parser_eq;

#[test]
fn assignment_target() {
    assert_parser_eq!(
        r#"foo += bar = b ??= 3;"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ExpressionStatement","start":0,"end":21,"expression":{"type":"AssignmentExpression","start":0,"end":20,"operator":"+=","left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"right":{"type":"AssignmentExpression","start":7,"end":20,"operator":"=","left":{"type":"Identifier","start":7,"end":10,"name":"bar"},"right":{"type":"AssignmentExpression","start":13,"end":20,"operator":"??=","left":{"type":"Identifier","start":13,"end":14,"name":"b"},"right":{"type":"Literal","start":19,"end":20,"value":3,"raw":"3"}}}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"a.foo -= bar;"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"AssignmentExpression","start":0,"end":12,"operator":"-=","left":{"type":"MemberExpression","start":0,"end":5,"object":{"type":"Identifier","start":0,"end":1,"name":"a"},"property":{"type":"Identifier","start":2,"end":5,"name":"foo"},"computed":false,"optional":false},"right":{"type":"Identifier","start":9,"end":12,"name":"bar"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"(foo = bar);"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"AssignmentExpression","start":1,"end":10,"operator":"=","left":{"type":"Identifier","start":1,"end":4,"name":"foo"},"right":{"type":"Identifier","start":7,"end":10,"name":"bar"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"(((foo))) = bar;"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"ExpressionStatement","start":0,"end":16,"expression":{"type":"AssignmentExpression","start":0,"end":15,"operator":"=","left":{"type":"Identifier","start":3,"end":6,"name":"foo"},"right":{"type":"Identifier","start":12,"end":15,"name":"bar"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"a["test"] = bar;"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"ExpressionStatement","start":0,"end":16,"expression":{"type":"AssignmentExpression","start":0,"end":15,"operator":"=","left":{"type":"MemberExpression","start":0,"end":9,"object":{"type":"Identifier","start":0,"end":1,"name":"a"},"property":{"type":"Literal","start":2,"end":8,"value":"test","raw":"\"test\""},"computed":true,"optional":false},"right":{"type":"Identifier","start":12,"end":15,"name":"bar"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"a.call().chain().member = x;"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"ExpressionStatement","start":0,"end":28,"expression":{"type":"AssignmentExpression","start":0,"end":27,"operator":"=","left":{"type":"MemberExpression","start":0,"end":23,"object":{"type":"CallExpression","start":0,"end":16,"callee":{"type":"MemberExpression","start":0,"end":14,"object":{"type":"CallExpression","start":0,"end":8,"callee":{"type":"MemberExpression","start":0,"end":6,"object":{"type":"Identifier","start":0,"end":1,"name":"a"},"property":{"type":"Identifier","start":2,"end":6,"name":"call"},"computed":false,"optional":false},"arguments":[],"optional":false},"property":{"type":"Identifier","start":9,"end":14,"name":"chain"},"computed":false,"optional":false},"arguments":[],"optional":false},"property":{"type":"Identifier","start":17,"end":23,"name":"member"},"computed":false,"optional":false},"right":{"type":"Identifier","start":26,"end":27,"name":"x"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"++count === 3"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"BinaryExpression","start":0,"end":13,"left":{"type":"UpdateExpression","start":0,"end":7,"operator":"++","prefix":true,"argument":{"type":"Identifier","start":2,"end":7,"name":"count"}},"operator":"===","right":{"type":"Literal","start":12,"end":13,"value":3,"raw":"3"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"a['b'] = c[d] = "test""#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"AssignmentExpression","start":0,"end":22,"operator":"=","left":{"type":"MemberExpression","start":0,"end":6,"object":{"type":"Identifier","start":0,"end":1,"name":"a"},"property":{"type":"Literal","start":2,"end":5,"value":"b","raw":"'b'"},"computed":true,"optional":false},"right":{"type":"AssignmentExpression","start":9,"end":22,"operator":"=","left":{"type":"MemberExpression","start":9,"end":13,"object":{"type":"Identifier","start":9,"end":10,"name":"c"},"property":{"type":"Identifier","start":11,"end":12,"name":"d"},"computed":true,"optional":false},"right":{"type":"Literal","start":16,"end":22,"value":"test","raw":"\"test\""}}}}],"sourceType":"script"}"#
    );
}
