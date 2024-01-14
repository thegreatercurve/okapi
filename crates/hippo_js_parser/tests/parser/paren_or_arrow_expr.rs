use crate::parser::common::assert_parser_eq;

#[test]
fn paren_or_arrow_expr() {
    assert_parser_eq!(
        r#"(foo);"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":1,"end":4,"name":"foo"}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"(foo) => {};"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"ArrowFunctionExpression","start":0,"end":11,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":1,"end":4,"name":"foo"}],"body":{"type":"BlockStatement","start":9,"end":11,"body":[]}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"(5 + 5);"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"BinaryExpression","start":1,"end":6,"left":{"type":"Literal","start":1,"end":2,"value":5,"raw":"5"},"operator":"+","right":{"type":"Literal","start":5,"end":6,"value":5,"raw":"5"}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"({foo, bar, b: [f, ...baz]}) => {};"#,
        r#"{"type":"Program","start":0,"end":35,"body":[{"type":"ExpressionStatement","start":0,"end":35,"expression":{"type":"ArrowFunctionExpression","start":0,"end":34,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"ObjectPattern","start":1,"end":27,"properties":[{"type":"Property","start":2,"end":5,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":2,"end":5,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":2,"end":5,"name":"foo"}},{"type":"Property","start":7,"end":10,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":7,"end":10,"name":"bar"},"kind":"init","value":{"type":"Identifier","start":7,"end":10,"name":"bar"}},{"type":"Property","start":12,"end":26,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":12,"end":13,"name":"b"},"value":{"type":"ArrayPattern","start":15,"end":26,"elements":[{"type":"Identifier","start":16,"end":17,"name":"f"},{"type":"RestElement","start":19,"end":25,"argument":{"type":"Identifier","start":22,"end":25,"name":"baz"}}]},"kind":"init"}]}],"body":{"type":"BlockStatement","start":32,"end":34,"body":[]}}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"(foo, ...bar) => {}"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"ExpressionStatement","start":0,"end":19,"expression":{"type":"ArrowFunctionExpression","start":0,"end":19,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":1,"end":4,"name":"foo"},{"type":"RestElement","start":6,"end":12,"argument":{"type":"Identifier","start":9,"end":12,"name":"bar"}}],"body":{"type":"BlockStatement","start":17,"end":19,"body":[]}}}],"sourceType":"script"}"#
    );
}