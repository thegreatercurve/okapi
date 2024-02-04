use crate::parser::common::assert_parse_module_eq;

#[test]
fn assignment_expression() {
    // assert_parse_module_eq!(
    //     r#"foo -= bar;"#,
    //     r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"AssignmentExpression","start":0,"end":10,"operator":"-=","left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"right":{"type":"Identifier","start":7,"end":10,"name":"bar"}}}],"sourceType":"module"}"#
    // );
    // assert_parse_module_eq!(
    //     r#"foo += bar = b ??= 3;"#,
    //     r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ExpressionStatement","start":0,"end":21,"expression":{"type":"AssignmentExpression","start":0,"end":20,"operator":"+=","left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"right":{"type":"AssignmentExpression","start":7,"end":20,"operator":"=","left":{"type":"Identifier","start":7,"end":10,"name":"bar"},"right":{"type":"AssignmentExpression","start":13,"end":20,"operator":"??=","left":{"type":"Identifier","start":13,"end":14,"name":"b"},"right":{"type":"Literal","start":19,"end":20,"value":3.0,"raw":"3"}}}}}],"sourceType":"module"}"#
    // );
    // assert_parse_module_eq!(
    //     r#"(foo = bar);"#,
    //     r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"AssignmentExpression","start":1,"end":10,"operator":"=","left":{"type":"Identifier","start":1,"end":4,"name":"foo"},"right":{"type":"Identifier","start":7,"end":10,"name":"bar"}}}],"sourceType":"module"}"#
    // );
    assert_parse_module_eq!(
        r#"[foo, bar] = baz;"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"AssignmentExpression","start":0,"end":16,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":10,"elements":[{"type":"Identifier","start":1,"end":4,"name":"foo"},{"type":"Identifier","start":6,"end":9,"name":"bar"}]},"right":{"type":"Identifier","start":13,"end":16,"name":"baz"}}}],"sourceType":"module"}"#
    );
    // assert_parse_module_eq!(
    //     r#"[foo, bar = "default", ...rest] = baz;"#,
    //     r#"{"type":"Program","start":0,"end":38,"body":[{"type":"ExpressionStatement","start":0,"end":38,"expression":{"type":"AssignmentExpression","start":0,"end":37,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":31,"elements":[{"type":"Identifier","start":1,"end":4,"name":"foo"},{"type":"AssignmentPattern","start":6,"end":21,"left":{"type":"Identifier","start":6,"end":9,"name":"bar"},"right":{"type":"Literal","start":12,"end":21,"value":"default","raw":"\"default\""}},{"type":"RestElement","start":23,"end":30,"argument":{"type":"Identifier","start":26,"end":30,"name":"rest"}}]},"right":{"type":"Identifier","start":34,"end":37,"name":"baz"}}}],"sourceType":"module"}"#
    // );
    // assert_parse_module_eq!(
    //     r#"[,,,foo,bar] = baz;"#,
    //     r#"{"type":"Program","start":0,"end":19,"body":[{"type":"ExpressionStatement","start":0,"end":19,"expression":{"type":"AssignmentExpression","start":0,"end":18,"operator":"=","left":{"type":"ArrayPattern","start":0,"end":12,"elements":[null,null,null,{"type":"Identifier","start":4,"end":7,"name":"foo"},{"type":"Identifier","start":8,"end":11,"name":"bar"}]},"right":{"type":"Identifier","start":15,"end":18,"name":"baz"}}}],"sourceType":"module"}"#
    // );
    // assert_parse_module_eq!(
    //     r#"({ bar, baz } = {});"#,
    //     r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"AssignmentExpression","start":1,"end":18,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":13,"properties":[{"type":"Property","start":3,"end":6,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":3,"end":6,"name":"bar"},"kind":"init","value":{"type":"Identifier","start":3,"end":6,"name":"bar"}},{"type":"Property","start":8,"end":11,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":8,"end":11,"name":"baz"},"kind":"init","value":{"type":"Identifier","start":8,"end":11,"name":"baz"}}]},"right":{"type":"ObjectExpression","start":16,"end":18,"properties":[]}}}],"sourceType":"module"}"#
    // );
    // assert_parse_module_eq!(
    //     r#"({ bar: [baz = "baz"], foo = "foo", ...rest } = {});"#,
    //     r#"{"type":"Program","start":0,"end":52,"body":[{"type":"ExpressionStatement","start":0,"end":52,"expression":{"type":"AssignmentExpression","start":1,"end":50,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":45,"properties":[{"type":"Property","start":3,"end":21,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":3,"end":6,"name":"bar"},"value":{"type":"ArrayPattern","start":8,"end":21,"elements":[{"type":"AssignmentPattern","start":9,"end":20,"left":{"type":"Identifier","start":9,"end":12,"name":"baz"},"right":{"type":"Literal","start":15,"end":20,"value":"baz","raw":"\"baz\""}}]},"kind":"init"},{"type":"Property","start":23,"end":34,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":23,"end":26,"name":"foo"},"kind":"init","value":{"type":"AssignmentPattern","start":23,"end":34,"left":{"type":"Identifier","start":23,"end":26,"name":"foo"},"right":{"type":"Literal","start":29,"end":34,"value":"foo","raw":"\"foo\""}}},{"type":"RestElement","start":36,"end":43,"argument":{"type":"Identifier","start":39,"end":43,"name":"rest"}}]},"right":{"type":"ObjectExpression","start":48,"end":50,"properties":[]}}}],"sourceType":"module"}"#
    // );
}
