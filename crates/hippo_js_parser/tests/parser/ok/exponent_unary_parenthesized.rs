use crate::parser::assert_parser_eq;

#[test]
fn exponent_unary_parenthesized() {
    assert_parser_eq!(
        r#"(delete a.b) ** 2;"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"BinaryExpression","start":0,"end":17,"left":{"type":"UnaryExpression","start":1,"end":11,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":8,"end":11,"object":{"type":"Identifier","start":8,"end":9,"name":"a"},"property":{"type":"Identifier","start":10,"end":11,"name":"b"},"computed":false,"optional":false}},"operator":"**","right":{"type":"Literal","start":16,"end":17,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"(void ident) ** 2;"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"BinaryExpression","start":0,"end":17,"left":{"type":"UnaryExpression","start":1,"end":11,"operator":"void","prefix":true,"argument":{"type":"Identifier","start":6,"end":11,"name":"ident"}},"operator":"**","right":{"type":"Literal","start":16,"end":17,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"(typeof ident) ** 2;"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"BinaryExpression","start":0,"end":19,"left":{"type":"UnaryExpression","start":1,"end":13,"operator":"typeof","prefix":true,"argument":{"type":"Identifier","start":8,"end":13,"name":"ident"}},"operator":"**","right":{"type":"Literal","start":18,"end":19,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"(-3) ** 2;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"BinaryExpression","start":0,"end":9,"left":{"type":"UnaryExpression","start":1,"end":3,"operator":"-","prefix":true,"argument":{"type":"Literal","start":2,"end":3,"value":3,"raw":"3"}},"operator":"**","right":{"type":"Literal","start":8,"end":9,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"(+3) ** 2;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"BinaryExpression","start":0,"end":9,"left":{"type":"UnaryExpression","start":1,"end":3,"operator":"+","prefix":true,"argument":{"type":"Literal","start":2,"end":3,"value":3,"raw":"3"}},"operator":"**","right":{"type":"Literal","start":8,"end":9,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"(~3) ** 2;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"BinaryExpression","start":0,"end":9,"left":{"type":"UnaryExpression","start":1,"end":3,"operator":"~","prefix":true,"argument":{"type":"Literal","start":2,"end":3,"value":3,"raw":"3"}},"operator":"**","right":{"type":"Literal","start":8,"end":9,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"(!true) ** 2;"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"BinaryExpression","start":0,"end":12,"left":{"type":"UnaryExpression","start":1,"end":6,"operator":"!","prefix":true,"argument":{"type":"Literal","start":2,"end":6,"value":true,"raw":"true"}},"operator":"**","right":{"type":"Literal","start":11,"end":12,"value":2,"raw":"2"}}}],"sourceType":"module"}"#
    );
}
