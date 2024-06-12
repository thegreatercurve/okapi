use crate::parser::assert_parser_eq;

#[test]
fn function_expression() {
    assert_parser_eq!(
        r#"let a = function() {}"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"VariableDeclaration","start":0,"end":21,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"FunctionExpression","start":8,"end":21,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":19,"end":21,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"let b = function foo() {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"VariableDeclaration","start":0,"end":25,"declarations":[{"type":"VariableDeclarator","start":4,"end":25,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"FunctionExpression","start":8,"end":25,"id":{"type":"Identifier","start":17,"end":20,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );
}

#[test]
fn function_expression_async_generator() {
    assert_parser_eq!(
        r#"let foo = async function* () {
  yield;
};"#,
        r#"{"type":"Program","start":0,"end":42,"body":[{"type":"VariableDeclaration","start":0,"end":42,"declarations":[{"type":"VariableDeclarator","start":4,"end":41,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":{"type":"FunctionExpression","start":10,"end":41,"id":null,"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":29,"end":41,"body":[{"type":"ExpressionStatement","start":33,"end":39,"expression":{"type":"YieldExpression","start":33,"end":38,"delegate":false,"argument":null}}]}}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
