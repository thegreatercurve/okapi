use crate::parser::common::assert_parse_module_eq;

#[test]
fn module_export_from_clause() {
    assert_parse_module_eq!(
        r#"export * from "foo";"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExportAllDeclaration","start":0,"end":20,"exported":null,"source":{"type":"Literal","start":14,"end":19,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_export_from_clause_with_module_name_export() {
    assert_parse_module_eq!(
        r#"export * as foo from "bar";"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ExportAllDeclaration","start":0,"end":27,"exported":{"type":"Identifier","start":12,"end":15,"name":"foo"},"source":{"type":"Literal","start":21,"end":26,"value":"bar","raw":"\"bar\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_named_exports() {
    assert_parse_module_eq!(
        r#"export {};"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExportNamedDeclaration","start":0,"end":10,"declaration":null,"specifiers":[],"source":null}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"export {} from "foo";"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ExportNamedDeclaration","start":0,"end":21,"declaration":null,"specifiers":[],"source":{"type":"Literal","start":15,"end":20,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_named_exports_with_exports_list() {
    assert_parse_module_eq!(
        r#"let foo, bar, bat; export {foo, bar as baz, bat};"#,
        r#"{"type":"Program","start":0,"end":49,"body":[{"type":"VariableDeclaration","start":0,"end":18,"declarations":[{"type":"VariableDeclarator","start":4,"end":7,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":null},{"type":"VariableDeclarator","start":9,"end":12,"id":{"type":"Identifier","start":9,"end":12,"name":"bar"},"init":null},{"type":"VariableDeclarator","start":14,"end":17,"id":{"type":"Identifier","start":14,"end":17,"name":"bat"},"init":null}],"kind":"let"},{"type":"ExportNamedDeclaration","start":19,"end":49,"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":27,"end":30,"local":{"type":"Identifier","start":27,"end":30,"name":"foo"},"exported":{"type":"Identifier","start":27,"end":30,"name":"foo"}},{"type":"ExportSpecifier","start":32,"end":42,"local":{"type":"Identifier","start":32,"end":35,"name":"bar"},"exported":{"type":"Identifier","start":39,"end":42,"name":"baz"}},{"type":"ExportSpecifier","start":44,"end":47,"local":{"type":"Identifier","start":44,"end":47,"name":"bat"},"exported":{"type":"Identifier","start":44,"end":47,"name":"bat"}}],"source":null}],"sourceType":"module"}"#
    );

    assert_parse_module_eq!(
        r#"let foo, bar, bat; export {foo, bar as baz, bat, "qux" as "quux"} from "bar";"#,
        r#"{"type":"Program","start":0,"end":77,"body":[{"type":"VariableDeclaration","start":0,"end":18,"declarations":[{"type":"VariableDeclarator","start":4,"end":7,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":null},{"type":"VariableDeclarator","start":9,"end":12,"id":{"type":"Identifier","start":9,"end":12,"name":"bar"},"init":null},{"type":"VariableDeclarator","start":14,"end":17,"id":{"type":"Identifier","start":14,"end":17,"name":"bat"},"init":null}],"kind":"let"},{"type":"ExportNamedDeclaration","start":19,"end":77,"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":27,"end":30,"local":{"type":"Identifier","start":27,"end":30,"name":"foo"},"exported":{"type":"Identifier","start":27,"end":30,"name":"foo"}},{"type":"ExportSpecifier","start":32,"end":42,"local":{"type":"Identifier","start":32,"end":35,"name":"bar"},"exported":{"type":"Identifier","start":39,"end":42,"name":"baz"}},{"type":"ExportSpecifier","start":44,"end":47,"local":{"type":"Identifier","start":44,"end":47,"name":"bat"},"exported":{"type":"Identifier","start":44,"end":47,"name":"bat"}},{"type":"ExportSpecifier","start":49,"end":64,"local":{"type":"Literal","start":49,"end":54,"value":"qux","raw":"\"qux\""},"exported":{"type":"Literal","start":58,"end":64,"value":"quux","raw":"\"quux\""}}],"source":{"type":"Literal","start":71,"end":76,"value":"bar","raw":"\"bar\""}}],"sourceType":"module"}"#
    );
}
