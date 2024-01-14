use crate::parser::common::assert_parser_eq;

#[test]
fn using_declarations_inside_for_statement() {
    assert_parser_eq!(r#"for (using x of y) {};"#, r#"undefined"#);
    assert_parser_eq!(r#"for await (using x of y) {};"#, r#"undefined"#);
    assert_parser_eq!(r#"for (await using x of y) {};"#, r#"undefined"#);
    assert_parser_eq!(r#"for await (await using x of y) {};"#, r#"undefined"#);
}
