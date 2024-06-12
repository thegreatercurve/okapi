use crate::parser::assert_parser_script_eq;

#[test]
fn unary_delete() {
    assert_parser_script_eq!(
        r#"delete obj.key;"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"UnaryExpression","start":0,"end":14,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":7,"end":14,"object":{"type":"Identifier","start":7,"end":10,"name":"obj"},"property":{"type":"Identifier","start":11,"end":14,"name":"key"},"computed":false,"optional":false}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"delete (obj).key;"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"UnaryExpression","start":0,"end":16,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":7,"end":16,"object":{"type":"Identifier","start":8,"end":11,"name":"obj"},"property":{"type":"Identifier","start":13,"end":16,"name":"key"},"computed":false,"optional":false}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"delete obj[key];"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"ExpressionStatement","start":0,"end":16,"expression":{"type":"UnaryExpression","start":0,"end":15,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":7,"end":15,"object":{"type":"Identifier","start":7,"end":10,"name":"obj"},"property":{"type":"Identifier","start":11,"end":14,"name":"key"},"computed":true,"optional":false}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"delete (obj)[key];"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"UnaryExpression","start":0,"end":17,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":7,"end":17,"object":{"type":"Identifier","start":8,"end":11,"name":"obj"},"property":{"type":"Identifier","start":13,"end":16,"name":"key"},"computed":true,"optional":false}}}],"sourceType":"script"}"#
    );
}
