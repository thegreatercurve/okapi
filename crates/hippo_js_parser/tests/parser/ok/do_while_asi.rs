use crate::parser::assert_parser_eq;

#[test]
fn do_while() {
    assert_parser_eq!(
        r#"do do do ; while (x) while (x) while (x) x = 39;"#,
        r#"{"type":"Program","start":0,"end":48,"body":[{"type":"DoWhileStatement","start":0,"end":40,"body":{"type":"DoWhileStatement","start":3,"end":30,"body":{"type":"DoWhileStatement","start":6,"end":20,"body":{"type":"EmptyStatement","start":9,"end":10},"test":{"type":"Identifier","start":18,"end":19,"name":"x"}},"test":{"type":"Identifier","start":28,"end":29,"name":"x"}},"test":{"type":"Identifier","start":38,"end":39,"name":"x"}},{"type":"ExpressionStatement","start":41,"end":48,"expression":{"type":"AssignmentExpression","start":41,"end":47,"operator":"=","left":{"type":"Identifier","start":41,"end":42,"name":"x"},"right":{"type":"Literal","start":45,"end":47,"value":39,"raw":"39"}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"do do ; while (x); while (x) x = 39"#,
        r#"{"type":"Program","start":0,"end":35,"body":[{"type":"DoWhileStatement","start":0,"end":28,"body":{"type":"DoWhileStatement","start":3,"end":18,"body":{"type":"EmptyStatement","start":6,"end":7},"test":{"type":"Identifier","start":15,"end":16,"name":"x"}},"test":{"type":"Identifier","start":26,"end":27,"name":"x"}},{"type":"ExpressionStatement","start":29,"end":35,"expression":{"type":"AssignmentExpression","start":29,"end":35,"operator":"=","left":{"type":"Identifier","start":29,"end":30,"name":"x"},"right":{"type":"Literal","start":33,"end":35,"value":39,"raw":"39"}}}],"sourceType":"module"}"#
    );
}
