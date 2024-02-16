use crate::parser::common::assert_parse_module_eq;

#[test]
fn function_expression() {
    assert_parse_module_eq!(
        r#"let a = function() {}"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"VariableDeclaration","start":0,"end":21,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"FunctionExpression","start":8,"end":21,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":19,"end":21,"body":[]}}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let b = function foo() {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"VariableDeclaration","start":0,"end":25,"declarations":[{"type":"VariableDeclarator","start":4,"end":25,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"FunctionExpression","start":8,"end":25,"id":{"type":"Identifier","start":17,"end":20,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
