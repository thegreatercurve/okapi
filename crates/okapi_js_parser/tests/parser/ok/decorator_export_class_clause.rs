use crate::parser::assert_parser_eq;

#[test]
fn decorator_export_class_clause() {
    assert_parser_eq!(r#"export @decorator class Bar {}"#, r#"undefined"#);

    assert_parser_eq!(
        r#"export @firt @second class Foo { contructor() {} }"#,
        r#"undefined"#
    );
}
