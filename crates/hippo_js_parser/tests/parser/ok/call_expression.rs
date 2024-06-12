use crate::parser::assert_parser_eq;

#[test]
fn call_expression() {
    assert_parser_eq!(
        r#"foo(bar)"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"CallExpression","start":0,"end":8,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":7,"name":"bar"}],"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"foo(bar).baz"#,
        r#"{"type":"Program","start":0,"end":12,"body":[{"type":"ExpressionStatement","start":0,"end":12,"expression":{"type":"MemberExpression","start":0,"end":12,"object":{"type":"CallExpression","start":0,"end":8,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":7,"name":"bar"}],"optional":false},"property":{"type":"Identifier","start":9,"end":12,"name":"baz"},"computed":false,"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"foo(bar).baz()"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"CallExpression","start":0,"end":14,"callee":{"type":"MemberExpression","start":0,"end":12,"object":{"type":"CallExpression","start":0,"end":8,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":7,"name":"bar"}],"optional":false},"property":{"type":"Identifier","start":9,"end":12,"name":"baz"},"computed":false,"optional":false},"arguments":[],"optional":false}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"foo(bar)(baz)(baz)[bar]"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"ExpressionStatement","start":0,"end":23,"expression":{"type":"MemberExpression","start":0,"end":23,"object":{"type":"CallExpression","start":0,"end":18,"callee":{"type":"CallExpression","start":0,"end":13,"callee":{"type":"CallExpression","start":0,"end":8,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":7,"name":"bar"}],"optional":false},"arguments":[{"type":"Identifier","start":9,"end":12,"name":"baz"}],"optional":false},"arguments":[{"type":"Identifier","start":14,"end":17,"name":"baz"}],"optional":false},"property":{"type":"Identifier","start":19,"end":22,"name":"bar"},"computed":true,"optional":false}}],"sourceType":"script"}"#
    );
}

#[test]
fn call_expression_super_call() {
    assert_parser_eq!(
        r#"class Foo extends Bar {
  constructor() {
    super(baz, bat)
  }
}"#,
        r#"{"type":"Program","start":0,"end":67,"body":[{"type":"ClassDeclaration","start":0,"end":67,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":{"type":"Identifier","start":18,"end":21,"name":"Bar"},"body":{"type":"ClassBody","start":22,"end":67,"body":[{"type":"MethodDefinition","start":26,"end":65,"static":false,"computed":false,"key":{"type":"Identifier","start":26,"end":37,"name":"constructor"},"kind":"constructor","value":{"type":"FunctionExpression","start":37,"end":65,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":40,"end":65,"body":[{"type":"ExpressionStatement","start":46,"end":61,"expression":{"type":"CallExpression","start":46,"end":61,"callee":{"type":"Super","start":46,"end":51},"arguments":[{"type":"Identifier","start":52,"end":55,"name":"baz"},{"type":"Identifier","start":57,"end":60,"name":"bat"}],"optional":false}}]}}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn call_expression_import_call() {
    assert_parser_eq!(
        r#"function foo() { import ("bar") }"#,
        r#"{"type":"Program","start":0,"end":33,"body":[{"type":"FunctionDeclaration","start":0,"end":33,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":33,"body":[{"type":"ExpressionStatement","start":17,"end":31,"expression":{"type":"ImportExpression","start":17,"end":31,"source":{"type":"Literal","start":25,"end":30,"value":"bar","raw":"\"bar\""}}}]}}],"sourceType":"script"}"#
    );
}
