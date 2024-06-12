use crate::parser::assert_parser_script_eq;

#[test]
fn object_member_name() {
    assert_parser_script_eq!(
        r#"let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}"#,
        r#"{"type":"Program","start":0,"end":52,"body":[{"type":"VariableDeclaration","start":0,"end":52,"declarations":[{"type":"VariableDeclarator","start":4,"end":52,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":52,"properties":[{"type":"Property","start":9,"end":19,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":9,"end":14,"value":"foo","raw":"\"foo\""},"value":{"type":"Identifier","start":16,"end":19,"name":"foo"},"kind":"init"},{"type":"Property","start":21,"end":33,"method":false,"shorthand":false,"computed":true,"key":{"type":"BinaryExpression","start":22,"end":27,"left":{"type":"Literal","start":22,"end":23,"value":6,"raw":"6"},"operator":"+","right":{"type":"Literal","start":26,"end":27,"value":6,"raw":"6"}},"value":{"type":"Identifier","start":30,"end":33,"name":"foo"},"kind":"init"},{"type":"Property","start":35,"end":43,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":35,"end":38,"name":"bar"},"value":{"type":"Identifier","start":40,"end":43,"name":"foo"},"kind":"init"},{"type":"Property","start":45,"end":51,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":45,"end":46,"value":7,"raw":"7"},"value":{"type":"Identifier","start":48,"end":51,"name":"foo"},"kind":"init"}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}
