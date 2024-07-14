use crate::parser::assert_parser_module_eq;

#[test]
fn module_import_specifier() {
    assert_parser_module_eq!(
        r#"import "foo";"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ImportDeclaration","start":0,"end":13,"specifiers":[],"source":{"type":"Literal","start":7,"end":12,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_imported_default_binding() {
    assert_parser_module_eq!(
        r#"import foo from "foo";"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ImportDeclaration","start":0,"end":22,"specifiers":[{"type":"ImportDefaultSpecifier","start":7,"end":10,"local":{"type":"Identifier","start":7,"end":10,"name":"foo"}}],"source":{"type":"Literal","start":16,"end":21,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_name_space_import() {
    assert_parser_module_eq!(
        r#"import * as foo from "foo";"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"ImportDeclaration","start":0,"end":27,"specifiers":[{"type":"ImportNamespaceSpecifier","start":7,"end":15,"local":{"type":"Identifier","start":12,"end":15,"name":"foo"}}],"source":{"type":"Literal","start":21,"end":26,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_named_imports() {
    assert_parser_module_eq!(
        r#"import {} from "foo";"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"ImportDeclaration","start":0,"end":21,"specifiers":[],"source":{"type":"Literal","start":15,"end":20,"value":"foo","raw":"\"foo\""}}],"sourceType":"module"}"#
    );

    assert_parser_module_eq!(
        r#"import {foo, bar as baz, bat, "qux" as quux} from "bar";"#,
        r#"{"type":"Program","start":0,"end":56,"body":[{"type":"ImportDeclaration","start":0,"end":56,"specifiers":[{"type":"ImportSpecifier","start":8,"end":11,"imported":{"type":"Identifier","start":8,"end":11,"name":"foo"},"local":{"type":"Identifier","start":8,"end":11,"name":"foo"}},{"type":"ImportSpecifier","start":13,"end":23,"imported":{"type":"Identifier","start":13,"end":16,"name":"bar"},"local":{"type":"Identifier","start":20,"end":23,"name":"baz"}},{"type":"ImportSpecifier","start":25,"end":28,"imported":{"type":"Identifier","start":25,"end":28,"name":"bat"},"local":{"type":"Identifier","start":25,"end":28,"name":"bat"}},{"type":"ImportSpecifier","start":30,"end":43,"imported":{"type":"Literal","start":30,"end":35,"value":"qux","raw":"\"qux\""},"local":{"type":"Identifier","start":39,"end":43,"name":"quux"}}],"source":{"type":"Literal","start":50,"end":55,"value":"bar","raw":"\"bar\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_imported_default_binding_and_name_space_import() {
    assert_parser_module_eq!(
        r#"import foo, * as bar from "baz";"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"ImportDeclaration","start":0,"end":32,"specifiers":[{"type":"ImportDefaultSpecifier","start":7,"end":10,"local":{"type":"Identifier","start":7,"end":10,"name":"foo"}},{"type":"ImportNamespaceSpecifier","start":12,"end":20,"local":{"type":"Identifier","start":17,"end":20,"name":"bar"}}],"source":{"type":"Literal","start":26,"end":31,"value":"baz","raw":"\"baz\""}}],"sourceType":"module"}"#
    );
}

#[test]
fn module_imported_default_binding_and_named_import() {
    assert_parser_module_eq!(
        r#"import foo, { bar, baz } from "bar";"#,
        r#"{"type":"Program","start":0,"end":36,"body":[{"type":"ImportDeclaration","start":0,"end":36,"specifiers":[{"type":"ImportDefaultSpecifier","start":7,"end":10,"local":{"type":"Identifier","start":7,"end":10,"name":"foo"}},{"type":"ImportSpecifier","start":14,"end":17,"imported":{"type":"Identifier","start":14,"end":17,"name":"bar"},"local":{"type":"Identifier","start":14,"end":17,"name":"bar"}},{"type":"ImportSpecifier","start":19,"end":22,"imported":{"type":"Identifier","start":19,"end":22,"name":"baz"},"local":{"type":"Identifier","start":19,"end":22,"name":"baz"}}],"source":{"type":"Literal","start":30,"end":35,"value":"bar","raw":"\"bar\""}}],"sourceType":"module"}"#
    );
}
