use crate::parser::common::assert_parse_module_eq;

#[test]
fn object_expression_spread_prop() {
    assert_parse_module_eq!(
        r#"let a = {...foo}"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"VariableDeclaration","start":0,"end":16,"declarations":[{"type":"VariableDeclarator","start":4,"end":16,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":16,"properties":[{"type":"SpreadElement","start":9,"end":15,"argument":{"type":"Identifier","start":12,"end":15,"name":"foo"}}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
