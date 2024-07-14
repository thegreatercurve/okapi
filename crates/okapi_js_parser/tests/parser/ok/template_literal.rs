use crate::parser::assert_parser_script_eq;

#[test]
fn template_literal_simple() {
    assert_parser_script_eq!(
        r#"let a = `foo`;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"VariableDeclaration","start":0,"end":14,"kind":"let","declarations":[{"type":"VariableDeclarator","start":4,"end":13,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"TemplateLiteral","start":8,"end":13,"expressions":[],"quasis":[{"type":"TemplateElement","start":9,"end":12,"value":{"raw":"foo","cooked":"foo"},"tail":true}]}}]}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let b = `foo ${bar}`;"#,
        r#"{"type":"Program","start":0,"end":21,"body":[{"type":"VariableDeclaration","start":0,"end":21,"kind":"let","declarations":[{"type":"VariableDeclarator","start":4,"end":20,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"TemplateLiteral","start":8,"end":20,"expressions":[{"type":"Identifier","start":15,"end":18,"name":"bar"}],"quasis":[{"type":"TemplateElement","start":9,"end":13,"value":{"raw":"foo ","cooked":"foo "},"tail":false},{"type":"TemplateElement","start":19,"end":19,"value":{"raw":"","cooked":""},"tail":true}]}}]}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let c = ``;"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"VariableDeclaration","start":0,"end":11,"kind":"let","declarations":[{"type":"VariableDeclarator","start":4,"end":10,"id":{"type":"Identifier","start":4,"end":5,"name":"c"},"init":{"type":"TemplateLiteral","start":8,"end":10,"expressions":[],"quasis":[{"type":"TemplateElement","start":9,"end":9,"value":{"raw":"","cooked":""},"tail":true}]}}]}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let d = `${foo}`;"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"VariableDeclaration","start":0,"end":17,"declarations":[{"type":"VariableDeclarator","start":4,"end":16,"id":{"type":"Identifier","start":4,"end":5,"name":"d"},"init":{"type":"TemplateLiteral","start":8,"end":16,"expressions":[{"type":"Identifier","start":11,"end":14,"name":"foo"}],"quasis":[{"type":"TemplateElement","start":9,"end":9,"value":{"raw":"","cooked":""},"tail":false},{"type":"TemplateElement","start":15,"end":15,"value":{"raw":"","cooked":""},"tail":true}]}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let e = `${{ a: "string" }}`;"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"VariableDeclaration","start":0,"end":29,"declarations":[{"type":"VariableDeclarator","start":4,"end":28,"id":{"type":"Identifier","start":4,"end":5,"name":"e"},"init":{"type":"TemplateLiteral","start":8,"end":28,"expressions":[{"type":"ObjectExpression","start":11,"end":26,"properties":[{"type":"Property","start":13,"end":24,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":13,"end":14,"name":"a"},"value":{"type":"Literal","start":16,"end":24,"value":"string","raw":"\"string\""},"kind":"init"}]}],"quasis":[{"type":"TemplateElement","start":9,"end":9,"value":{"raw":"","cooked":""},"tail":false},{"type":"TemplateElement","start":27,"end":27,"value":{"raw":"","cooked":""},"tail":true}]}}],"kind":"let"}],"sourceType":"script"}"#
    );
}

#[test]
fn template_literal_nested_template_literals() {
    assert_parser_script_eq!(
        r#"`foo${`bar${baz}`}`;"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"TemplateLiteral","start":0,"end":19,"expressions":[{"type":"TemplateLiteral","start":6,"end":17,"expressions":[{"type":"Identifier","start":12,"end":15,"name":"baz"}],"quasis":[{"type":"TemplateElement","start":7,"end":10,"value":{"raw":"bar","cooked":"bar"},"tail":false},{"type":"TemplateElement","start":16,"end":16,"value":{"raw":"","cooked":""},"tail":true}]}],"quasis":[{"type":"TemplateElement","start":1,"end":4,"value":{"raw":"foo","cooked":"foo"},"tail":false},{"type":"TemplateElement","start":18,"end":18,"value":{"raw":"","cooked":""},"tail":true}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn template_literal_sibling_template_middles() {
    assert_parser_script_eq!(
        r#"`foo ${bar} ${baz}`;"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":0,"end":20,"expression":{"type":"TemplateLiteral","start":0,"end":19,"expressions":[{"type":"Identifier","start":7,"end":10,"name":"bar"},{"type":"Identifier","start":14,"end":17,"name":"baz"}],"quasis":[{"type":"TemplateElement","start":1,"end":5,"value":{"raw":"foo ","cooked":"foo "},"tail":false},{"type":"TemplateElement","start":11,"end":12,"value":{"raw":" ","cooked":" "},"tail":false},{"type":"TemplateElement","start":18,"end":18,"value":{"raw":"","cooked":""},"tail":true}]}}],"sourceType":"script"}"#
    );
}
