use crate::parser::common::assert_parser_eq;

#[test]
fn using_declaration_statement() {
    assert_parser_eq!(r#"using a = b;"#, r#"undefined"#);
    assert_parser_eq!(r#"using c = d, e = _;"#, r#"undefined"#);
    assert_parser_eq!(
        r#"using [g] = h;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"AssignmentExpression","start":0,"end":13,"operator":"=","left":{"type":"MemberExpression","start":0,"end":9,"object":{"type":"Identifier","start":0,"end":5,"name":"using"},"property":{"type":"Identifier","start":7,"end":8,"name":"g"},"computed":true,"optional":false},"right":{"type":"Identifier","start":12,"end":13,"name":"h"}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"using [j]"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"MemberExpression","start":0,"end":9,"object":{"type":"Identifier","start":0,"end":5,"name":"using"},"property":{"type":"Identifier","start":7,"end":8,"name":"j"},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(r#"= k;"#, r#"undefined"#);
    assert_parser_eq!(r#"await using l = m;"#, r#"undefined"#);
    assert_parser_eq!(
        r#"await"#,
        r#"{"type":"Program","start":0,"end":5,"body":[{"type":"ExpressionStatement","start":0,"end":5,"expression":{"type":"Identifier","start":0,"end":5,"name":"await"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(r#"using p = q;"#, r#"undefined"#);
    assert_parser_eq!(r#"await using[r];"#, r#"undefined"#);
    assert_parser_eq!(r#"await using ([s] = t);"#, r#"undefined"#);
    assert_parser_eq!(
        r#"await (using [u] = v);"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"CallExpression","start":0,"end":21,"callee":{"type":"Identifier","start":0,"end":5,"name":"await"},"arguments":[{"type":"AssignmentExpression","start":7,"end":20,"operator":"=","left":{"type":"MemberExpression","start":7,"end":16,"object":{"type":"Identifier","start":7,"end":12,"name":"using"},"property":{"type":"Identifier","start":14,"end":15,"name":"u"},"computed":true,"optional":false},"right":{"type":"Identifier","start":19,"end":20,"name":"v"}}],"optional":false}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(r#"using w = {};"#, r#"undefined"#);
    assert_parser_eq!(r#"using x = null;"#, r#"undefined"#);
    assert_parser_eq!(r#"using y = undefined;"#, r#"undefined"#);
    assert_parser_eq!(r#"using z = (foo, bar);"#, r#"undefined"#);
}
