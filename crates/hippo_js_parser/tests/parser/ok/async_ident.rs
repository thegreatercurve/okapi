use crate::parser::assert_parser_eq;

#[test]
fn async_ident() {
    assert_parser_eq!(
        r#"async;"#,
        r#"{"type":"Program","start":0,"end":6,"body":[{"type":"ExpressionStatement","start":0,"end":6,"expression":{"type":"Identifier","start":0,"end":5,"name":"async"}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"let a = async;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"VariableDeclaration","start":0,"end":14,"declarations":[{"type":"VariableDeclarator","start":4,"end":13,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"Identifier","start":8,"end":13,"name":"async"}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
