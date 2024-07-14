use crate::parser::assert_parser_eq;

#[test]
fn decorator_export_top_level() {
    assert_parser_eq!(r#"@decorator"#, r#"undefined"#);
    assert_parser_eq!(r#"export class Foo { }"#, r#"undefined"#);
    assert_parser_eq!(
        r#"@first.field @second @(() => decorator)()"#,
        r#"undefined"#
    );

    assert_parser_eq!(r#"export class Bar {}"#, r#"undefined"#);
    assert_parser_eq!(r#"@before"#, r#"undefined"#);
    assert_parser_eq!(r#"export @after class Foo { }"#, r#"undefined"#);
    assert_parser_eq!(r#"@before"#, r#"undefined"#);
    assert_parser_eq!(r#"export abstract class Foo { }"#, r#"undefined"#);
    assert_parser_eq!(r#"@before"#, r#"undefined"#);
    assert_parser_eq!(r#"export @after abstract class Foo { }"#, r#"undefined"#);
}
