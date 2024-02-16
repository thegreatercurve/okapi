use crate::parser::common::assert_parse_module_eq;

#[test]
fn semicolons() {
    assert_parse_module_eq!(
        r#"let foo = bar;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"VariableDeclaration","start":0,"end":14,"declarations":[{"type":"VariableDeclarator","start":4,"end":13,"id":{"type":"Identifier","start":4,"end":7,"name":"foo"},"init":{"type":"Identifier","start":10,"end":13,"name":"bar"}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let foo2 = b;"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"VariableDeclaration","start":0,"end":13,"declarations":[{"type":"VariableDeclarator","start":4,"end":12,"id":{"type":"Identifier","start":4,"end":8,"name":"foo2"},"init":{"type":"Identifier","start":11,"end":12,"name":"b"}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let foo3;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"VariableDeclaration","start":0,"end":9,"declarations":[{"type":"VariableDeclarator","start":4,"end":8,"id":{"type":"Identifier","start":4,"end":8,"name":"foo3"},"init":null}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let foo4"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"VariableDeclaration","start":0,"end":8,"declarations":[{"type":"VariableDeclarator","start":4,"end":8,"id":{"type":"Identifier","start":4,"end":8,"name":"foo4"},"init":null}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let foo5"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"VariableDeclaration","start":0,"end":8,"declarations":[{"type":"VariableDeclarator","start":4,"end":8,"id":{"type":"Identifier","start":4,"end":8,"name":"foo5"},"init":null}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"function foo6() { return true }"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"FunctionDeclaration","start":0,"end":31,"id":{"type":"Identifier","start":9,"end":13,"name":"foo6"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":31,"body":[{"type":"ReturnStatement","start":18,"end":29,"argument":{"type":"Literal","start":25,"end":29,"value":true,"raw":"true"}}]}}],"sourceType":"module"}"#
    );
}
