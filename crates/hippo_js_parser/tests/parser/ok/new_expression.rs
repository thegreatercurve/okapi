use crate::parser::common::assert_parse_module_eq;

#[test]
fn new_expression() {
    assert_parse_module_eq!(
        r#"new Foo()"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"NewExpression","start":0,"end":9,"callee":{"type":"Identifier","start":4,"end":7,"name":"Foo"},"arguments":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"new foo;"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"NewExpression","start":0,"end":7,"callee":{"type":"Identifier","start":4,"end":7,"name":"foo"},"arguments":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(r#"new.target"#, r#"undefined"#);
    assert_parse_module_eq!(
        r#"new new new new Foo();"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"NewExpression","start":0,"end":21,"callee":{"type":"NewExpression","start":4,"end":21,"callee":{"type":"NewExpression","start":8,"end":21,"callee":{"type":"NewExpression","start":12,"end":21,"callee":{"type":"Identifier","start":16,"end":19,"name":"Foo"},"arguments":[]},"arguments":[]},"arguments":[]},"arguments":[]}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"new Foo(bar, baz, 6 + 6, foo[bar] + ((foo) => {}) * foo?.bar)"#,
        r#"{"type":"Program","start":0,"end":61,"body":[{"type":"ExpressionStatement","start":0,"end":61,"expression":{"type":"NewExpression","start":0,"end":61,"callee":{"type":"Identifier","start":4,"end":7,"name":"Foo"},"arguments":[{"type":"Identifier","start":8,"end":11,"name":"bar"},{"type":"Identifier","start":13,"end":16,"name":"baz"},{"type":"BinaryExpression","start":18,"end":23,"left":{"type":"Literal","start":18,"end":19,"value":6,"raw":"6"},"operator":"+","right":{"type":"Literal","start":22,"end":23,"value":6,"raw":"6"}},{"type":"BinaryExpression","start":25,"end":60,"left":{"type":"MemberExpression","start":25,"end":33,"object":{"type":"Identifier","start":25,"end":28,"name":"foo"},"property":{"type":"Identifier","start":29,"end":32,"name":"bar"},"computed":true,"optional":false},"operator":"+","right":{"type":"BinaryExpression","start":36,"end":60,"left":{"type":"ArrowFunctionExpression","start":37,"end":48,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":38,"end":41,"name":"foo"}],"body":{"type":"BlockStatement","start":46,"end":48,"body":[]}},"operator":"*","right":{"type":"ChainExpression","start":52,"end":60,"expression":{"type":"MemberExpression","start":52,"end":60,"object":{"type":"Identifier","start":52,"end":55,"name":"foo"},"property":{"type":"Identifier","start":57,"end":60,"name":"bar"},"computed":false,"optional":true}}}}]}}],"sourceType":"module"}"#
    );
}
