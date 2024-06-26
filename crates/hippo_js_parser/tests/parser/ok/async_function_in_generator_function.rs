use crate::parser::assert_parser_script_eq;

#[test]
fn async_function_in_generator_function() {
    assert_parser_script_eq!(
        r#"function* hello() { 
  yield; 
  async function world() { 
    yield; 
  } 
}"#,
        r#"{"type":"Program","start":0,"end":77,"body":[{"type":"FunctionDeclaration","start":0,"end":77,"id":{"type":"Identifier","start":10,"end":15,"name":"hello"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":18,"end":77,"body":[{"type":"ExpressionStatement","start":23,"end":29,"expression":{"type":"YieldExpression","start":23,"end":28,"delegate":false,"argument":null}},{"type":"FunctionDeclaration","start":33,"end":74,"id":{"type":"Identifier","start":48,"end":53,"name":"world"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":56,"end":74,"body":[{"type":"ExpressionStatement","start":63,"end":69,"expression":{"type":"Identifier","start":63,"end":68,"name":"yield"}}]}}]}}],"sourceType":"script"}"#
    );
}
