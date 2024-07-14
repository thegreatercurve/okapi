use crate::parser::assert_parser_eq;

#[test]
fn decorator_expression_class() {
    assert_parser_eq!(r#"let a = ( @decorator class {} );"#, r#"undefined"#);

    assert_parser_eq!(
        r#"let b = ( @first @second class foo { constructor() {} } );"#,
        r#"undefined"#
    );
}
