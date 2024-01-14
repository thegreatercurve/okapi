use crate::parser::common::assert_parser_eq;

#[test]
fn object_expr_generator_method() {
    assert_parser_eq!(
        r#"let b = { *foo() {} }"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"VariableDeclaration","start":0,"end":21,"declarations":[{"type":"VariableDeclarator","start":4,"end":21,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":21,"properties":[{"type":"Property","start":10,"end":19,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":11,"end":14,"name":"foo"},"kind":"init","value":{"type":"FunctionExpression","start":14,"end":19,"id":null,"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":19,"body":[]}}}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
