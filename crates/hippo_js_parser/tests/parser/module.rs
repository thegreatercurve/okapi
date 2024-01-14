use crate::parser::common::assert_parser_eq;

#[test]
fn module() {
    assert_parser_eq!(r#"import a from "b";"#, r#"undefined"#);
    assert_parser_eq!(r#"export { a };"#, r#"undefined"#);
    assert_parser_eq!(
        r#"c();"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"CallExpression","start":0,"end":3,"callee":{"type":"Identifier","start":0,"end":1,"name":"c"},"arguments":[],"optional":false}}],"sourceType":"script"}"#
    );
    assert_parser_eq!(r#"import { c } from "c";"#, r#"undefined"#);
}
