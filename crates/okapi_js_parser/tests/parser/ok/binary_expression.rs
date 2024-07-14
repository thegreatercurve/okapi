use crate::parser::assert_parser_script_eq;

#[test]
fn binary_expression() {
    assert_parser_script_eq!(
        r#""a" + "b";"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"BinaryExpression","start":0,"end":9,"left":{"type":"Literal","start":0,"end":3,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":6,"end":9,"value":"b","raw":"\"b\""}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a + b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"+","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a - b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"-","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a * b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"*","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a / b;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"/","right":{"type":"Identifier","start":4,"end":5,"name":"b"}}}],"sourceType":"script"}"#
    );
}

#[test]
fn binary_expression_bitwise_operators() {
    assert_parser_script_eq!(
        r#"1 ^ 2;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"^","right":{"type":"Literal","start":4,"end":5,"value":2.0,"raw":"2"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"1 | 2;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"|","right":{"type":"Literal","start":4,"end":5,"value":2.0,"raw":"2"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"1 & 2;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"&","right":{"type":"Literal","start":4,"end":5,"value":2.0,"raw":"2"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a >> b"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":">>","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a << b"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"BinaryExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"<<","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"74 in foo;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"BinaryExpression","start":0,"end":9,"left":{"type":"Literal","start":0,"end":2,"value":74.0,"raw":"74"},"operator":"in","right":{"type":"Identifier","start":6,"end":9,"name":"foo"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo instanceof Array;"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ExpressionStatement","start":0,"end":21,"expression":{"type":"BinaryExpression","start":0,"end":20,"left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"operator":"instanceof","right":{"type":"Identifier","start":15,"end":20,"name":"Array"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo ?? bar;"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"LogicalExpression","start":0,"end":10,"left":{"type":"Identifier","start":0,"end":3,"name":"foo"},"operator":"??","right":{"type":"Identifier","start":7,"end":10,"name":"bar"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a >> b;"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"BinaryExpression","start":0,"end":6,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":">>","right":{"type":"Identifier","start":5,"end":6,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"a >>> b;"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"BinaryExpression","start":0,"end":7,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":">>>","right":{"type":"Identifier","start":6,"end":7,"name":"b"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"1 + 1 + 1 + 1;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"BinaryExpression","start":0,"end":13,"left":{"type":"BinaryExpression","start":0,"end":9,"left":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":1.0,"raw":"1"},"operator":"+","right":{"type":"Literal","start":4,"end":5,"value":1.0,"raw":"1"}},"operator":"+","right":{"type":"Literal","start":8,"end":9,"value":1.0,"raw":"1"}},"operator":"+","right":{"type":"Literal","start":12,"end":13,"value":1.0,"raw":"1"}}}],"sourceType":"script"}"#
    );
}

#[test]
fn binary_expression_exponentiation_with_right_associativity() {
    assert_parser_script_eq!(
        r#"4 ** 4 ** 4;"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"BinaryExpression","start":0,"end":11,"left":{"type":"Literal","start":0,"end":1,"value":4.0,"raw":"4"},"operator":"**","right":{"type":"BinaryExpression","start":5,"end":11,"left":{"type":"Literal","start":5,"end":6,"value":4.0,"raw":"4"},"operator":"**","right":{"type":"Literal","start":10,"end":11,"value":4.0,"raw":"4"}}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"(4 ** 4) ** 4;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"BinaryExpression","start":0,"end":13,"left":{"type":"BinaryExpression","start":1,"end":7,"left":{"type":"Literal","start":1,"end":2,"value":4.0,"raw":"4"},"operator":"**","right":{"type":"Literal","start":6,"end":7,"value":4.0,"raw":"4"}},"operator":"**","right":{"type":"Literal","start":12,"end":13,"value":4.0,"raw":"4"}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"6 ** (6 ** 7);"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"BinaryExpression","start":0,"end":13,"left":{"type":"Literal","start":0,"end":1,"value":6.0,"raw":"6"},"operator":"**","right":{"type":"BinaryExpression","start":6,"end":12,"left":{"type":"Literal","start":6,"end":7,"value":6.0,"raw":"6"},"operator":"**","right":{"type":"Literal","start":11,"end":12,"value":7.0,"raw":"7"}}}}],"sourceType":"script"}"#
    );
}

#[test]
fn binary_expression_complex_precedence_grouping() {
    assert_parser_script_eq!(
        r#"a + b * (b - 3) ** 1"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"BinaryExpression","start":0,"end":20,"left":{"type":"Identifier","start":0,"end":1,"name":"a"},"operator":"+","right":{"type":"BinaryExpression","start":4,"end":20,"left":{"type":"Identifier","start":4,"end":5,"name":"b"},"operator":"*","right":{"type":"BinaryExpression","start":8,"end":20,"left":{"type":"BinaryExpression","start":9,"end":14,"left":{"type":"Identifier","start":9,"end":10,"name":"b"},"operator":"-","right":{"type":"Literal","start":13,"end":14,"value":3.0,"raw":"3"}},"operator":"**","right":{"type":"Literal","start":19,"end":20,"value":1.0,"raw":"1"}}}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"5 + 6 - (1 * 2) / 1 ** 6;"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"ExpressionStatement","start":0,"end":25,"expression":{"type":"BinaryExpression","start":0,"end":24,"left":{"type":"BinaryExpression","start":0,"end":5,"left":{"type":"Literal","start":0,"end":1,"value":5.0,"raw":"5"},"operator":"+","right":{"type":"Literal","start":4,"end":5,"value":6.0,"raw":"6"}},"operator":"-","right":{"type":"BinaryExpression","start":8,"end":24,"left":{"type":"BinaryExpression","start":9,"end":14,"left":{"type":"Literal","start":9,"end":10,"value":1.0,"raw":"1"},"operator":"*","right":{"type":"Literal","start":13,"end":14,"value":2.0,"raw":"2"}},"operator":"/","right":{"type":"BinaryExpression","start":18,"end":24,"left":{"type":"Literal","start":18,"end":19,"value":1.0,"raw":"1"},"operator":"**","right":{"type":"Literal","start":23,"end":24,"value":6.0,"raw":"6"}}}}}],"sourceType":"script"}"#
    );
}
