use crate::parser::assert_parser_eq;

#[test]
fn class_named_abstract_is_valid_in_js() {
    assert_parser_eq!(
        r#"class abstract {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ClassDeclaration","start":0,"end":17,"id":{"type":"Identifier","start":6,"end":14,"name":"abstract"},"superClass":null,"body":{"type":"ClassBody","start":15,"end":17,"body":[]}}],"sourceType":"script"}"#
    );
}
