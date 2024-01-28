use crate::parser::common::assert_parse_module_eq;

#[test]
fn object_expression_ident_literal_prop() {
    assert_parse_module_eq!(
        r#"let b = { a: true }"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"VariableDeclaration","start":0,"end":19,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":19,"properties":[{"type":"Property","start":10,"end":17,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":10,"end":11,"name":"a"},"kind":"init","value":{"type":"Literal","start":13,"end":17,"value":true,"raw":"true"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
