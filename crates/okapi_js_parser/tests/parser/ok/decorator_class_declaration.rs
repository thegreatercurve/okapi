use crate::parser::assert_parser_eq;

#[test]
fn decorator_class_declaration() {
    assert_parser_eq!(
        r#"function foo() { @decorator class Foo {} @firt.field @second @((() => decorator)()) class Bar {} }"#,
        r#"undefined"#
    );
}
