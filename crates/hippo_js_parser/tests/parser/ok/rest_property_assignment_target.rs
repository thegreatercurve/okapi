use crate::parser::assert_parser_eq;

#[test]
fn rest_property_assignment_target() {
    assert_parser_eq!(
        r#"({ ...abcd } = a);"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"AssignmentExpression","start":1,"end":16,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":12,"properties":[{"type":"RestElement","start":3,"end":10,"argument":{"type":"Identifier","start":6,"end":10,"name":"abcd"}}]},"right":{"type":"Identifier","start":15,"end":16,"name":"a"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"({ ...abcd } = a);"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"AssignmentExpression","start":1,"end":16,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":12,"properties":[{"type":"RestElement","start":3,"end":10,"argument":{"type":"Identifier","start":6,"end":10,"name":"abcd"}}]},"right":{"type":"Identifier","start":15,"end":16,"name":"a"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"({ ...m.test } = c);"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"AssignmentExpression","start":1,"end":18,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":14,"properties":[{"type":"RestElement","start":3,"end":12,"argument":{"type":"MemberExpression","start":6,"end":12,"object":{"type":"Identifier","start":6,"end":7,"name":"m"},"property":{"type":"Identifier","start":8,"end":12,"name":"test"},"computed":false,"optional":false}}]},"right":{"type":"Identifier","start":17,"end":18,"name":"c"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"({ ...m[call()] } = c);"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"ExpressionStatement","start":0,"end":23,"expression":{"type":"AssignmentExpression","start":1,"end":21,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":17,"properties":[{"type":"RestElement","start":3,"end":15,"argument":{"type":"MemberExpression","start":6,"end":15,"object":{"type":"Identifier","start":6,"end":7,"name":"m"},"property":{"type":"CallExpression","start":8,"end":14,"callee":{"type":"Identifier","start":8,"end":12,"name":"call"},"arguments":[],"optional":false},"computed":true,"optional":false}}]},"right":{"type":"Identifier","start":20,"end":21,"name":"c"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"({ ...any.expression().b } = c);"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"ExpressionStatement","start":0,"end":32,"expression":{"type":"AssignmentExpression","start":1,"end":30,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":26,"properties":[{"type":"RestElement","start":3,"end":24,"argument":{"type":"MemberExpression","start":6,"end":24,"object":{"type":"CallExpression","start":6,"end":22,"callee":{"type":"MemberExpression","start":6,"end":20,"object":{"type":"Identifier","start":6,"end":9,"name":"any"},"property":{"type":"Identifier","start":10,"end":20,"name":"expression"},"computed":false,"optional":false},"arguments":[],"optional":false},"property":{"type":"Identifier","start":23,"end":24,"name":"b"},"computed":false,"optional":false}}]},"right":{"type":"Identifier","start":29,"end":30,"name":"c"}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"({ b: { ...a } } = c);"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"AssignmentExpression","start":1,"end":20,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":16,"properties":[{"type":"Property","start":3,"end":14,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":3,"end":4,"name":"b"},"value":{"type":"ObjectPattern","start":6,"end":14,"properties":[{"type":"RestElement","start":8,"end":12,"argument":{"type":"Identifier","start":11,"end":12,"name":"a"}}]},"kind":"init"}]},"right":{"type":"Identifier","start":19,"end":20,"name":"c"}}}],"sourceType":"script"}"#
    );
}
