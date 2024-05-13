use crate::parser::assert_parser_eq;

#[test]
fn array_assignment_target() {
    assert_parser_eq!(
        r#"[foo, bar] = baz;"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"AssignmentExpression","start":0,"end":16,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":10,"elements":[{"type":"Identifier","start":1,"end":4,"name":"foo"},{"type":"Identifier","start":6,"end":9,"name":"bar"}]},"right":{"type":"Identifier","start":13,"end":16,"name":"baz"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"[,,,b,,c,] = baz;"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"AssignmentExpression","start":0,"end":16,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":10,"elements":[null,null,null,{"type":"Identifier","start":4,"end":5,"name":"b"},null,{"type":"Identifier","start":7,"end":8,"name":"c"}]},"right":{"type":"Identifier","start":13,"end":16,"name":"baz"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"[a.b] = baz;"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"AssignmentExpression","start":0,"end":11,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":5,"elements":[{"type":"MemberExpression","start":1,"end":4,"object":{"type":"Identifier","start":1,"end":2,"name":"a"},"property":{"type":"Identifier","start":3,"end":4,"name":"b"},"computed":false,"optional":false}]},"right":{"type":"Identifier","start":8,"end":11,"name":"baz"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"[a = "test", a.b, call().b] = baz;"#,
        r#"{"type":"Program","start":0,"end":34,"body":[{"type":"ExpressionStatement","start":0,"end":34,"expression":{"type":"AssignmentExpression","start":0,"end":33,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":27,"elements":[{"type":"AssignmentPattern","start":1,"end":11,"left":{"type":"Identifier","start":1,"end":2,"name":"a"},"right":{"type":"Literal","start":5,"end":11,"value":"test","raw":"\"test\""}},{"type":"MemberExpression","start":13,"end":16,"object":{"type":"Identifier","start":13,"end":14,"name":"a"},"property":{"type":"Identifier","start":15,"end":16,"name":"b"},"computed":false,"optional":false},{"type":"MemberExpression","start":18,"end":26,"object":{"type":"CallExpression","start":18,"end":24,"callee":{"type":"Identifier","start":18,"end":22,"name":"call"},"arguments":[],"optional":false},"property":{"type":"Identifier","start":25,"end":26,"name":"b"},"computed":false,"optional":false}]},"right":{"type":"Identifier","start":30,"end":33,"name":"baz"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"[((a))] = baz;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"AssignmentExpression","start":0,"end":13,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":7,"elements":[{"type":"Identifier","start":3,"end":4,"name":"a"}]},"right":{"type":"Identifier","start":10,"end":13,"name":"baz"}}}],"sourceType":"module"}"#
    );
}
