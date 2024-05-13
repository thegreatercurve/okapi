use crate::parser::assert_parser_eq;

#[test]
fn call_arguments() {
    assert_parser_eq!(
        r#"function foo(...args) {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"FunctionDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[{"type":"RestElement","start":13,"end":20,"argument":{"type":"Identifier","start":16,"end":20,"name":"args"}}],"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"let a, b, c, d;"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"VariableDeclaration","start":0,"end":15,"declarations":[{"type":"VariableDeclarator","start":4,"end":5,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":null},{"type":"VariableDeclarator","start":7,"end":8,"id":{"type":"Identifier","start":7,"end":8,"name":"b"},"init":null},{"type":"VariableDeclarator","start":10,"end":11,"id":{"type":"Identifier","start":10,"end":11,"name":"c"},"init":null},{"type":"VariableDeclarator","start":13,"end":14,"id":{"type":"Identifier","start":13,"end":14,"name":"d"},"init":null}],"kind":"let"}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo(a);"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"CallExpression","start":0,"end":6,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":5,"name":"a"}],"optional":false}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo(a, b,);"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ExpressionStatement","start":0,"end":11,"expression":{"type":"CallExpression","start":0,"end":10,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":5,"name":"a"},{"type":"Identifier","start":7,"end":8,"name":"b"}],"optional":false}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo(a, b, ...c);"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"ExpressionStatement","start":0,"end":16,"expression":{"type":"CallExpression","start":0,"end":15,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":5,"name":"a"},{"type":"Identifier","start":7,"end":8,"name":"b"},{"type":"SpreadElement","start":10,"end":14,"argument":{"type":"Identifier","start":13,"end":14,"name":"c"}}],"optional":false}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo(...a, ...b, c, ...d,);"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"ExpressionStatement","start":0,"end":26,"expression":{"type":"CallExpression","start":0,"end":25,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"SpreadElement","start":4,"end":8,"argument":{"type":"Identifier","start":7,"end":8,"name":"a"}},{"type":"SpreadElement","start":10,"end":14,"argument":{"type":"Identifier","start":13,"end":14,"name":"b"}},{"type":"Identifier","start":16,"end":17,"name":"c"},{"type":"SpreadElement","start":19,"end":23,"argument":{"type":"Identifier","start":22,"end":23,"name":"d"}}],"optional":false}}],"sourceType":"module"}"#
    );
}
