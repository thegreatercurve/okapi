use crate::parser::common::assert_parse_module_eq;

#[test]
fn js_unary_expression() {
    assert_parse_module_eq!(
        r#"delete a['test'];"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"UnaryExpression","start":0,"end":16,"operator":"delete","prefix":true,"argument":{"type":"MemberExpression","start":7,"end":16,"object":{"type":"Identifier","start":7,"end":8,"name":"a"},"property":{"type":"Literal","start":9,"end":15,"value":"test","raw":"'test'"},"computed":true,"optional":false}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"void a;"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"UnaryExpression","start":0,"end":6,"operator":"void","prefix":true,"argument":{"type":"Identifier","start":5,"end":6,"name":"a"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"typeof a;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"UnaryExpression","start":0,"end":8,"operator":"typeof","prefix":true,"argument":{"type":"Identifier","start":7,"end":8,"name":"a"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"+1;"#,
        r#"{"type":"Program","start":0,"end":3,"body":[{"type":"ExpressionStatement","start":0,"end":3,"expression":{"type":"UnaryExpression","start":0,"end":2,"operator":"+","prefix":true,"argument":{"type":"Literal","start":1,"end":2,"value":1,"raw":"1"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"-1;"#,
        r#"{"type":"Program","start":0,"end":3,"body":[{"type":"ExpressionStatement","start":0,"end":3,"expression":{"type":"UnaryExpression","start":0,"end":2,"operator":"-","prefix":true,"argument":{"type":"Literal","start":1,"end":2,"value":1,"raw":"1"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"~1;"#,
        r#"{"type":"Program","start":0,"end":3,"body":[{"type":"ExpressionStatement","start":0,"end":3,"expression":{"type":"UnaryExpression","start":0,"end":2,"operator":"~","prefix":true,"argument":{"type":"Literal","start":1,"end":2,"value":1,"raw":"1"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"!true;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"UnaryExpression","start":0,"end":5,"operator":"!","prefix":true,"argument":{"type":"Literal","start":1,"end":5,"value":true,"raw":"true"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"-a + -b + +a;"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"BinaryExpression","start":0,"end":12,"left":{"type":"BinaryExpression","start":0,"end":7,"left":{"type":"UnaryExpression","start":0,"end":2,"operator":"-","prefix":true,"argument":{"type":"Identifier","start":1,"end":2,"name":"a"}},"operator":"+","right":{"type":"UnaryExpression","start":5,"end":7,"operator":"-","prefix":true,"argument":{"type":"Identifier","start":6,"end":7,"name":"b"}}},"operator":"+","right":{"type":"UnaryExpression","start":10,"end":12,"operator":"+","prefix":true,"argument":{"type":"Identifier","start":11,"end":12,"name":"a"}}}}],"sourceType":"module"}"#
    );
}
