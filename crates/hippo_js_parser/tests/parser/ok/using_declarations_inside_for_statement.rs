use crate::parser::common::assert_parse_module_eq;

#[test]
fn using_declarations_inside_for_statement() {
    assert_parse_module_eq!(r#"for (using x of y) {};"#, r#"undefined"#);
    assert_parse_module_eq!(r#"for await (using x of y) {};"#, r#"undefined"#);
    assert_parse_module_eq!(r#"for (await using x of y) {};"#, r#"undefined"#);
    assert_parse_module_eq!(r#"for await (await using x of y) {};"#, r#"undefined"#);
}
