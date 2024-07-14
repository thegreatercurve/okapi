use crate::parser::assert_parser_script_eq;

#[test]
fn async_arrow_expression() {
    assert_parser_script_eq!(
        r#"let a = async foo => {}"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"VariableDeclaration","start":0,"end":23,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ArrowFunctionExpression","start":8,"end":23,"id":null,"expression":false,"generator":false,"async":true,"params":[{"type":"Identifier","start":14,"end":17,"name":"foo"}],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let b = async (bar) => {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"VariableDeclaration","start":0,"end":25,"declarations":[{"type":"VariableDeclarator","start":4,"end":25,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ArrowFunctionExpression","start":8,"end":25,"id":null,"expression":false,"generator":false,"async":true,"params":[{"type":"Identifier","start":15,"end":18,"name":"bar"}],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"async (foo, bar, ...baz) => foo"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"ExpressionStatement","start":0,"end":31,"expression":{"type":"ArrowFunctionExpression","start":0,"end":31,"id":null,"expression":true,"generator":false,"async":true,"params":[{"type":"Identifier","start":7,"end":10,"name":"foo"},{"type":"Identifier","start":12,"end":15,"name":"bar"},{"type":"RestElement","start":17,"end":23,"argument":{"type":"Identifier","start":20,"end":23,"name":"baz"}}],"body":{"type":"Identifier","start":28,"end":31,"name":"foo"}}}],"sourceType":"script"}"#
    );
}
