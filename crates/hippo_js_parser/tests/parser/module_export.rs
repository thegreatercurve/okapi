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

// #[test]
// fn module_named_export() {
//     assert_parse_module_eq!(
//         r#"export {};"#,
//         r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExportNamedDeclaration","start":0,"end":10,"declaration":null,"specifiers":[],"source":null}],"sourceType":"module"}"#
//     );

//     assert_parse_module_eq!(
//         r#"export {} from "foo";"#,
//         r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ExportNamedDeclaration","start":0,"end":21,"declaration":null,"specifiers":[],"source":{"type":"Literal","start":15,"end":20,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
//     );

//     assert_parse_module_eq!(
//         r#"let foo, bar, bat; export {foo, bar as baz, bat};"#,
//         r#"{"type":"Program","start":0,"end":49,"body":[{"type":"VariableDeclaration","start":0,"end":18,"declarations":[{"type":"VariableDeclarator","start":4,"end":7,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":null},{"type":"VariableDeclarator","start":9,"end":12,"id":{"type":"Identifier","start":9,"end":12,"name":"bar"},"init":null},{"type":"VariableDeclarator","start":14,"end":17,"id":{"type":"Identifier","start":14,"end":17,"name":"bat"},"init":null}],"kind":"let"},{"type":"ExportNamedDeclaration","start":19,"end":49,"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":27,"end":30,"local":{"type":"Identifier","start":27,"end":30,"name":"foo"},"exported":{"type":"Identifier","start":27,"end":30,"name":"foo"}},{"type":"ExportSpecifier","start":32,"end":42,"local":{"type":"Identifier","start":32,"end":35,"name":"bar"},"exported":{"type":"Identifier","start":39,"end":42,"name":"baz"}},{"type":"ExportSpecifier","start":44,"end":47,"local":{"type":"Identifier","start":44,"end":47,"name":"bat"},"exported":{"type":"Identifier","start":44,"end":47,"name":"bat"}}],"source":null}],"sourceType":"module"}"#
//     );

//     assert_parse_module_eq!(
//         r#"let foo, bar, bat;export {foo, bar as baz, bat, "qux" as "quux"} from "bar";"#,
//         r#"{"type":"Program","start":0,"end":76,"body":[{"type":"VariableDeclaration","start":0,"end":18,"declarations":[{"type":"VariableDeclarator","start":4,"end":7,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":null},{"type":"VariableDeclarator","start":9,"end":12,"id":{"type":"Identifier","start":9,"end":12,"name":"bar"},"init":null},{"type":"VariableDeclarator","start":14,"end":17,"id":{"type":"Identifier","start":14,"end":17,"name":"bat"},"init":null}],"kind":"let"},{"type":"ExportNamedDeclaration","start":18,"end":76,"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":26,"end":29,"local":{"type":"Identifier","start":26,"end":29,"name":"foo"},"exported":{"type":"Identifier","start":26,"end":29,"name":"foo"}},{"type":"ExportSpecifier","start":31,"end":41,"local":{"type":"Identifier","start":31,"end":34,"name":"bar"},"exported":{"type":"Identifier","start":38,"end":41,"name":"baz"}},{"type":"ExportSpecifier","start":43,"end":46,"local":{"type":"Identifier","start":43,"end":46,"name":"bat"},"exported":{"type":"Identifier","start":43,"end":46,"name":"bat"}},{"type":"ExportSpecifier","start":48,"end":63,"local":{"type":"Literal","start":48,"end":53,"value":"qux","raw":"\"qux\""},"exported":{"type":"Literal","start":57,"end":63,"value":"quux","raw":"\"quux\""}}],"source":{"type":"Literal","start":70,"end":75,"value":"bar","raw":"\"bar\""}}],"sourceType":"module"}"#
//     );
// }
