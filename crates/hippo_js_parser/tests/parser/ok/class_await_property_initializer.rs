use crate::parser::assert_parser_eq;

#[test]
fn class_await_property_initializer() {
    assert_parser_eq!(
        r#"async function* test() { class A { prop = await; } }"#,
        r#"{"type":"Program","start":0,"end":52,"body":[{"type":"FunctionDeclaration","start":0,"end":52,"id":{"type":"Identifier","start":16,"end":20,"name":"test"},"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":23,"end":52,"body":[{"type":"ClassDeclaration","start":25,"end":50,"id":{"type":"Identifier","start":31,"end":32,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":33,"end":50,"body":[{"type":"PropertyDefinition","start":35,"end":48,"static":false,"computed":false,"key":{"type":"Identifier","start":35,"end":39,"name":"prop"},"value":{"type":"Identifier","start":42,"end":47,"name":"await"}}]}}]}}],"sourceType":"script"}"#
    );
}
