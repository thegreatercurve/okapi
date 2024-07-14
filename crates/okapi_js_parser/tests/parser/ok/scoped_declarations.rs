


use crate::parser::assert_parser_script_eq;

#[test]
fn scoped_declarations() {
    assert_parser_script_eq!(
        r#"let a = { test() { let a = "inner"; }, };"#,
        r#"{"type":"Program","start":0,"end":41,"body":[{"type":"VariableDeclaration","start":0,"end":41,"declarations":[{"type":"VariableDeclarator","start":4,"end":40,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":40,"properties":[{"type":"Property","start":10,"end":37,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":10,"end":14,"name":"test"},"kind":"init","value":{"type":"FunctionExpression","start":14,"end":37,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":37,"body":[{"type":"VariableDeclaration","start":19,"end":35,"declarations":[{"type":"VariableDeclarator","start":23,"end":34,"id":{"type":"Identifier","start":23,"end":24,"name":"a"},"init":{"type":"Literal","start":27,"end":34,"value":"inner","raw":"\"inner\""}}],"kind":"let"}]}}}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
