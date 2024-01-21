use crate::parser::common::assert_parse_module_eq;

#[test]
fn function_in_if_or_labelled_stmt_loose_mode() {
    assert_parse_module_eq!(
        r#"// SCRIPT"#,
        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"label1: function a() {}"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"LabeledStatement","start":0,"end":23,"body":{"type":"FunctionDeclaration","start":8,"end":23,"id":{"type":"Identifier","start":17,"end":18,"name":"a"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}},"label":{"type":"Identifier","start":0,"end":6,"name":"label1"}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"if (true) function b() {} else function c() {}"#,
        r#"{"type":"Program","start":0,"end":46,"body":[{"type":"IfStatement","start":0,"end":46,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"FunctionDeclaration","start":10,"end":25,"id":{"type":"Identifier","start":19,"end":20,"name":"b"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}},"alternate":{"type":"FunctionDeclaration","start":31,"end":46,"id":{"type":"Identifier","start":40,"end":41,"name":"c"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":44,"end":46,"body":[]}}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"if (true) function d() {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"IfStatement","start":0,"end":25,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"FunctionDeclaration","start":10,"end":25,"id":{"type":"Identifier","start":19,"end":20,"name":"d"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}},"alternate":null}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"if (true) "test"; else function e() {}"#,
        r#"{"type":"Program","start":0,"end":38,"body":[{"type":"IfStatement","start":0,"end":38,"test":{"type":"Literal","start":4,"end":8,"value":true,"raw":"true"},"consequent":{"type":"ExpressionStatement","start":10,"end":17,"expression":{"type":"Literal","start":10,"end":16,"value":"test","raw":"\"test\""}},"alternate":{"type":"FunctionDeclaration","start":23,"end":38,"id":{"type":"Identifier","start":32,"end":33,"name":"e"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":36,"end":38,"body":[]}}}],"sourceType":"script"}"#
    );
}
