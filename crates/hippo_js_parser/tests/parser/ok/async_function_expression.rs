use crate::parser::assert_parser_eq;

#[test]
fn async_function_expression() {
    assert_parser_eq!(
        r#"let a = async function() {};"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"VariableDeclaration","start":0,"end":28,"declarations":[{"type":"VariableDeclarator","start":4,"end":27,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"FunctionExpression","start":8,"end":27,"id":null,"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":25,"end":27,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"let b = async function foo() {};"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"VariableDeclaration","start":0,"end":32,"declarations":[{"type":"VariableDeclarator","start":4,"end":31,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"FunctionExpression","start":8,"end":31,"id":{"type":"Identifier","start":23,"end":26,"name":"foo"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
