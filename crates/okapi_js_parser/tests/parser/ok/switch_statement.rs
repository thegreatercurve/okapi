use crate::parser::assert_parser_script_eq;

#[test]
fn switch_statement() {
    assert_parser_script_eq!(
        r#"switch (foo) {
    case bar:
    case bat: {}
    default:
    case baz: 1 << 2
}"#,
        r#"{"type":"Program","start":0,"end":81,"body":[{"type":"SwitchStatement","start":0,"end":81,"discriminant":{"type":"Identifier","start":8,"end":11,"name":"foo"},"cases":[{"type":"SwitchCase","start":19,"end":28,"consequent":[],"test":{"type":"Identifier","start":24,"end":27,"name":"bar"}},{"type":"SwitchCase","start":33,"end":45,"consequent":[{"type":"BlockStatement","start":43,"end":45,"body":[]}],"test":{"type":"Identifier","start":38,"end":41,"name":"bat"}},{"type":"SwitchCase","start":50,"end":58,"consequent":[],"test":null},{"type":"SwitchCase","start":63,"end":79,"consequent":[{"type":"ExpressionStatement","start":73,"end":79,"expression":{"type":"BinaryExpression","start":73,"end":79,"left":{"type":"Literal","start":73,"end":74,"value":1.0,"raw":"1"},"operator":"<<","right":{"type":"Literal","start":78,"end":79,"value":2.0,"raw":"2"}}}],"test":{"type":"Identifier","start":68,"end":71,"name":"baz"}}]}],"sourceType":"script"}"#
    );
}
