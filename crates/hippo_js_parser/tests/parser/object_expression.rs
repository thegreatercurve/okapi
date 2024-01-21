use crate::parser::common::assert_parse_module_eq;

#[test]
fn object_expression() {
    assert_parse_module_eq!(
        r#"let a = {};"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"VariableDeclaration","start":0,"end":11,"declarations":[{"type":"VariableDeclarator","start":4,"end":10,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":10,"properties":[]}}],"kind":"let"}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"let b = {foo,}"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"VariableDeclaration","start":0,"end":14,"declarations":[{"type":"VariableDeclarator","start":4,"end":14,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":14,"properties":[{"type":"Property","start":9,"end":12,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":9,"end":12,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":9,"end":12,"name":"foo"}}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
