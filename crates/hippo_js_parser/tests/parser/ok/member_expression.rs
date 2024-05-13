use crate::parser::assert_parser_eq;

#[test]
fn member_expression_array_index() {
    assert_parser_eq!(
        r#"foo[bar]"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":7,"name":"bar"},"computed":true,"optional":false}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo[bar ? "baz" : "bat"]"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ExpressionStatement","start":0,"end":24,"expression":{"type":"MemberExpression","start":0,"end":24,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"ConditionalExpression","start":4,"end":23,"test":{"type":"Identifier","start":4,"end":7,"name":"bar"},"consequent":{"type":"Literal","start":10,"end":15,"value":"baz","raw":"\"baz\""},"alternate":{"type":"Literal","start":18,"end":23,"value":"bat","raw":"\"bat\""}},"computed":true,"optional":false}}],"sourceType":"module"}"#
    );
}

#[test]
fn member_expression_object_property_access() {
    assert_parser_eq!(
        r#"foo.bar"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"MemberExpression","start":0,"end":7,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":4,"end":7,"name":"bar"},"computed":false,"optional":false}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo.#bar"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"PrivateIdentifier","start":4,"end":8,"name":"bar"},"computed":false,"optional":false}}],"sourceType":"module" }"#
    );
}

#[test]
fn member_expression_template_literal() {
    assert_parser_eq!(
        r#"foo`bar`"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"TaggedTemplateExpression","start":0,"end":8,"tag":{"type":"Identifier","start":0,"end":3,"name":"foo"},"quasi":{"type":"TemplateLiteral","start":3,"end":8,"expressions":[],"quasis":[{"type":"TemplateElement","start":4,"end":7,"value":{"raw":"bar","cooked":"bar"},"tail":true}]}}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"foo`bar ${baz}`"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"TaggedTemplateExpression","start":0,"end":15,"tag":{"type":"Identifier","start":0,"end":3,"name":"foo"},"quasi":{"type":"TemplateLiteral","start":3,"end":15,"expressions":[{"type":"Identifier","start":10,"end":13,"name":"baz"}],"quasis":[{"type":"TemplateElement","start":4,"end":8,"value":{"raw":"bar ","cooked":"bar "},"tail":false},{"type":"TemplateElement","start":14,"end":14,"value":{"raw":"","cooked":""},"tail":true}]}}}],"sourceType":"module"}"#
    );
}

#[test]
fn member_expression_meta_property() {
    assert_parser_eq!(
        r#"function hello() { new.target; }"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"FunctionDeclaration","start":0,"end":32,"id":{"type":"Identifier","start":9,"end":14,"name":"hello"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":32,"body":[{"type":"ExpressionStatement","start":19,"end":30,"expression":{"type":"MetaProperty","start":19,"end":29,"meta":{"type":"Identifier","start":19,"end":22,"name":"new"},"property":{"type":"Identifier","start":23,"end":29,"name":"target"}}}]}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"function hello() { import.meta; }"#,
        r#"{"type":"Program","start":0,"end":33,"body":[{"type":"FunctionDeclaration","start":0,"end":33,"id":{"type":"Identifier","start":9,"end":14,"name":"hello"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":33,"body":[{"type":"ExpressionStatement","start":19,"end":31,"expression":{"type":"MetaProperty","start":19,"end":30,"meta":{"type":"Identifier","start":19,"end":25,"name":"import"},"property":{"type":"Identifier","start":26,"end":30,"name":"meta"}}}]}}],"sourceType":"module"}"#
    );
}

#[test]
fn member_expression_super_property() {
    assert_parser_eq!(
        r#"class Foo {
  constructor() {
    super.bar;
  }
}"#,
        r#"{"type":"Program","start":0,"end":50,"body":[{"type":"ClassDeclaration","start":0,"end":50,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":50,"body":[{"type":"MethodDefinition","start":14,"end":48,"static":false,"computed":false,"key":{"type":"Identifier","start":14,"end":25,"name":"constructor"},"kind":"constructor","value":{"type":"FunctionExpression","start":25,"end":48,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":28,"end":48,"body":[{"type":"ExpressionStatement","start":34,"end":44,"expression":{"type":"MemberExpression","start":34,"end":43,"object":{"type":"Super","start":34,"end":39},"property":{"type":"Identifier","start":40,"end":43,"name":"bar"},"computed":false,"optional":false}}]}}}]}}],"sourceType":"module"}"#
    );
}
