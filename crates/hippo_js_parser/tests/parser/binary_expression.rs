use crate::parser::common::assert_parse_module_eq;

#[test]
fn binary_expression() {
    assert_parse_module_eq!(
        r#"a + b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"+","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"a - b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"-","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"a * b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"*","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"a / b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"/","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"module"}"#
    );
}

#[test]
fn binary_expression_bitwise_operators() {
    assert_parse_module_eq!(
        r#"1 ^ 2;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"^","right":{"type":"Literal","start":4,"end":5,"value":2.0,"raw":"2"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"1 | 2;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"|","right":{"type":"Literal","start":4,"end":5,"value":2.0,"raw":"2"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"1 & 2;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"&","right":{"type":"Literal","start":4,"end":5,"value":2.0,"raw":"2"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"a >> b"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":">>","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"a << b"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"<<","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"module"}"#
    );
}

#[test]
fn binary_expression_exponentiation_with_right_associativity() {
    assert_parse_module_eq!(
        r#"4 ** 4 ** 4;"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"BinaryExpression","start":0,"end":11,"left":{"type":"Literal","start":0,"end":1,"value":4.0,"raw":"4"},"operator":"**","right":{"type":"BinaryExpression","start":5,"end":11,"left":{"type":"Literal","start":5,"end":6,"value":4.0,"raw":"4"},"operator":"**","right":{"type":"Literal","start":10,"end":11,"value":4.0,"raw":"4"}}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"(4 ** 4) ** 4;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"BinaryExpression","start":0,"end":13,"left":{"type":"BinaryExpression","start":1,"end":7,"left":{"type":"Literal","start":1,"end":2,"value":4.0,"raw":"4"},"operator":"**","right":{"type":"Literal","start":6,"end":7,"value":4.0,"raw":"4"}},"operator":"**","right":{"type":"Literal","start":12,"end":13,"value":4.0,"raw":"4"}}}],"sourceType":"module"}"#
    );
}

#[test]
fn binary_expression_complex_precedence_grouping() {
    assert_parse_module_eq!(
        r#"a + b * (b - 3) ** 1"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"BinaryExpression","start":0,"end":20,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"+","right":{"type":"BinaryExpression","start":4,"end":20,"left":{"type":"Identifier","start":4,"end":5,"name":"b"},"operator":"*","right":{"type":"BinaryExpression","start":8,"end":20,"left":{"type":"BinaryExpression","start":9,"end":14,"left":{"type":"Identifier","start":9,"end":10,"name":"b"},"operator":"-","right":{"type":"Literal","start":13,"end":14,"value":3.0,"raw":"3"}},"operator":"**","right":{"type":"Literal","start":19,"end":20,"value":1.0,"raw":"1"}}}}}],"sourceType":"module"}"#
    );
}
