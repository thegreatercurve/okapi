use crate::parser::common::assert_parse_module_eq;

#[test]
fn module() {
    assert_parse_module_eq!(
        r#"import a from "b";"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ImportDeclaration","start":0,"end":18,"specifiers":[{"type":"ImportDefaultSpecifier","start":7,"end":8,"local":{"type":"Identifier","start":7,"end":8,"name":"a"}}],"source":{"type":"Literal","start":14,"end":17,"value":"b","raw":"\"b\""}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let a;
        export { a };"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":6,"declarations":[{"type":"VariableDeclarator","start":4,"end":5,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":null}],"kind":"let"},{"type":"ExportNamedDeclaration","start":7,"end":20,"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":16,"end":17,"local":{"type":"Identifier","start":16,"end":17,"name":"a"},"exported":{"type":"Identifier","start":16,"end":17,"name":"a"}}],"source":null}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"c();"#,
        r#"{"type":"Program","start":0,"end":4,"body":[{"type":"ExpressionStatement","start":0,"end":4,"expression":{"type":"CallExpression","start":0,"end":3,"callee":{"type":"Identifier","start":0,"end":1,"name":"c"},"arguments":[],"optional":false}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"import { c } from "c";"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ImportDeclaration","start":0,"end":22,"specifiers":[{"type":"ImportSpecifier","start":9,"end":10,"imported":{"type":"Identifier","start":9,"end":10,"name":"c"},"local":{"type":"Identifier","start":9,"end":10,"name":"c"}}],"source":{"type":"Literal","start":18,"end":21,"value":"c","raw":"\"c\""}}],"sourceType":"module"}"#
    );
}
