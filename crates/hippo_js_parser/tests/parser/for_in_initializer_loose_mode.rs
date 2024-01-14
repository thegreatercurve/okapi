use crate::parser::common::assert_parser_eq;

#[test]
fn for_in_initializer_loose_mode() {
    assert_parser_eq!(
        r#"// SCRIPT"#,
        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"for (var i = 0 in []) {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ForInStatement","start":0,"end":24,"left":{"type":"VariableDeclaration","start":5,"end":14,"declarations":[{"type":"VariableDeclarator","start":9,"end":14,"id":{"type":"Identifier","start":9,"end":10,"name":"i"},"init":{"type":"Literal","start":13,"end":14,"value":0,"raw":"0"}}],"kind":"var"},"right":{"type":"ArrayExpression","start":18,"end":20,"elements":[]},"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
    );
}
