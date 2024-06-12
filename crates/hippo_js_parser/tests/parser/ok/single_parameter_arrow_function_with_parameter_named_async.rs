use crate::parser::assert_parser_script_eq;

#[test]
fn single_parameter_arrow_function_with_parameter_named_async() {
    assert_parser_script_eq!(
        r#"let id = async => async;"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"VariableDeclaration","start":0,"end":24,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"Identifier","start":4,"end":6,"name":"id"},"init":{"type":"ArrowFunctionExpression","start":9,"end":23,"id":null,"expression":true,"generator":false,"async":false,"params":[{"type":"Identifier","start":9,"end":14,"name":"async"}],"body":{"type":"Identifier","start":18,"end":23,"name":"async"}}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
