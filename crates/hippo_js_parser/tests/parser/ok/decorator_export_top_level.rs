use crate::parser::common::assert_parse_module_eq;

#[test]
fn decorator_export_top_level() {
    assert_parse_module_eq!(r#"@decorator"#, r#"undefined"#);
    assert_parse_module_eq!(r#"export class Foo { }"#, r#"undefined"#);
    assert_parse_module_eq!(
        r#"@first.field @second @(() => decorator)()"#,
        r#"undefined"#
    );
    assert_parse_module_eq!(r#"export class Bar {}"#, r#"undefined"#);
    assert_parse_module_eq!(r#"@before"#, r#"undefined"#);
    assert_parse_module_eq!(r#"export @after class Foo { }"#, r#"undefined"#);
    assert_parse_module_eq!(r#"@before"#, r#"undefined"#);
    assert_parse_module_eq!(r#"export abstract class Foo { }"#, r#"undefined"#);
    assert_parse_module_eq!(r#"@before"#, r#"undefined"#);
    assert_parse_module_eq!(r#"export @after abstract class Foo { }"#, r#"undefined"#);
}
