use crate::parser::assert_parser_script_eq;

#[test]
fn parameter_list() {
    assert_parser_script_eq!(
        r#"function evalInComputedPropertyKey({ [computed]: ignored }) {}"#,
        r#"{"type":"Program","start":0,"end":62,"body":[{"type":"FunctionDeclaration","start":0,"end":62,"id":{"type":"Identifier","start":9,"end":34,"name":"evalInComputedPropertyKey"},"expression":false,"generator":false,"async":false,"params":[{"type":"ObjectPattern","start":35,"end":58,"properties":[{"type":"Property","start":37,"end":56,"method":false,"shorthand":false,"computed":true,"key":{"type":"Identifier","start":38,"end":46,"name":"computed"},"value":{"type":"Identifier","start":49,"end":56,"name":"ignored"},"kind":"init"}]}],"body":{"type":"BlockStatement","start":60,"end":62,"body":[]}}],"sourceType":"script"}"#
    );
}
