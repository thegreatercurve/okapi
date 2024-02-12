use crate::parser::common::assert_parse_module_eq;

#[test]
fn subscripts() {
    // assert_parse_module_eq!(
    //     r#"foo`bar`"#,
    //     r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"TaggedTemplateExpression","start":0,"end":8,"tag":{"type":"Identifier","start":0,"end":3,"name":"foo"},"quasi":{"type":"TemplateLiteral","start":3,"end":8,"expressions":[],"quasis":[{"type":"TemplateElement","start":4,"end":7,"value":{"raw":"bar","cooked":"bar"},"tail":true}]}}}],"sourceType":"module"}"#
    // );
    assert_parse_module_eq!(
        r#"foo(bar)(baz)(baz)[bar]"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"ExpressionStatement","start":0,"end":23,"expression":{"type":"MemberExpression","start":0,"end":23,"object":{"type":"CallExpression","start":0,"end":18,"callee":{"type":"CallExpression","start":0,"end":13,"callee":{"type":"CallExpression","start":0,"end":8,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[{"type":"Identifier","start":4,"end":7,"name":"bar"}],"optional":false},"arguments":[{"type":"Identifier","start":9,"end":12,"name":"baz"}],"optional":false},"arguments":[{"type":"Identifier","start":14,"end":17,"name":"baz"}],"optional":false},"property":{"type":"Identifier","start":19,"end":22,"name":"bar"},"computed":true,"optional":false}}],"sourceType":"module"}"#
    );
}
