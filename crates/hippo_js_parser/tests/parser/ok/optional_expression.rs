use crate::parser::assert_parser_script_eq;

#[test]
fn optional_expression() {
    assert_parser_script_eq!(
        r#"foo?.for;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"ChainExpression","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":5,"end":8,"name":"for"},"computed":false,"optional":true}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo?.bar;"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"ChainExpression","start":0,"end":8,"expression":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":5,"end":8,"name":"bar"},"computed":false,"optional":true}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo?.bar?.baz;"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"ChainExpression","start":0,"end":13,"expression":{"type":"MemberExpression","start":0,"end":13,"object":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":5,"end":8,"name":"bar"},"computed":false,"optional":true},"property":{"type":"Identifier","start":10,"end":13,"name":"baz"},"computed":false,"optional":true}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo?.();"#,
        r#"{"type":"Program","start":0,"end":8,"body":[{"type":"ExpressionStatement","start":0,"end":8,"expression":{"type":"ChainExpression","start":0,"end":7,"expression":{"type":"CallExpression","start":0,"end":7,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[],"optional":true}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo?.bar?.baz.bat"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"ChainExpression","start":0,"end":17,"expression":{"type":"MemberExpression","start":0,"end":17,"object":{"type":"MemberExpression","start":0,"end":13,"object":{"type":"MemberExpression","start":0,"end":8,"object":{"type":"Identifier","start":0,"end":3,"name":"foo"},"property":{"type":"Identifier","start":5,"end":8,"name":"bar"},"computed":false,"optional":true},"property":{"type":"Identifier","start":10,"end":13,"name":"baz"},"computed":false,"optional":true},"property":{"type":"Identifier","start":14,"end":17,"name":"bat"},"computed":false,"optional":false}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo?.()?.bar;"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"ChainExpression","start":0,"end":12,"expression":{"type":"MemberExpression","start":0,"end":12,"object":{"type":"CallExpression","start":0,"end":7,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[],"optional":true},"property":{"type":"Identifier","start":9,"end":12,"name":"bar"},"computed":false,"optional":true}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"foo()?.()?.();"#,
        r#"{"type":"Program","start":0,"end":14,"body":[{"type":"ExpressionStatement","start":0,"end":14,"expression":{"type":"ChainExpression","start":0,"end":13,"expression":{"type":"CallExpression","start":0,"end":13,"callee":{"type":"CallExpression","start":0,"end":9,"callee":{"type":"CallExpression","start":0,"end":5,"callee":{"type":"Identifier","start":0,"end":3,"name":"foo"},"arguments":[],"optional":false},"arguments":[],"optional":true},"arguments":[],"optional":true}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class Test {
    #bar;

    test(other) {
        this.#bar;
        this?.#bar;
        other.#bar;
        other?.#bar;
    }
}"#,
        r#"{"type":"Program","start":0,"end":129,"body":[{"type":"ClassDeclaration","start":0,"end":129,"id":{"type":"Identifier","start":6,"end":10,"name":"Test"},"superClass":null,"body":{"type":"ClassBody","start":11,"end":129,"body":[{"type":"PropertyDefinition","start":17,"end":22,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":17,"end":21,"name":"bar"},"value":null},{"type":"MethodDefinition","start":28,"end":127,"static":false,"computed":false,"key":{"type":"Identifier","start":28,"end":32,"name":"test"},"kind":"method","value":{"type":"FunctionExpression","start":32,"end":127,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":33,"end":38,"name":"other"}],"body":{"type":"BlockStatement","start":40,"end":127,"body":[{"type":"ExpressionStatement","start":50,"end":60,"expression":{"type":"MemberExpression","start":50,"end":59,"object":{"type":"ThisExpression","start":50,"end":54},"property":{"type":"PrivateIdentifier","start":55,"end":59,"name":"bar"},"computed":false,"optional":false}},{"type":"ExpressionStatement","start":69,"end":80,"expression":{"type":"ChainExpression","start":69,"end":79,"expression":{"type":"MemberExpression","start":69,"end":79,"object":{"type":"ThisExpression","start":69,"end":73},"property":{"type":"PrivateIdentifier","start":75,"end":79,"name":"bar"},"computed":false,"optional":true}}},{"type":"ExpressionStatement","start":89,"end":100,"expression":{"type":"MemberExpression","start":89,"end":99,"object":{"type":"Identifier","start":89,"end":94,"name":"other"},"property":{"type":"PrivateIdentifier","start":95,"end":99,"name":"bar"},"computed":false,"optional":false}},{"type":"ExpressionStatement","start":109,"end":121,"expression":{"type":"ChainExpression","start":109,"end":120,"expression":{"type":"MemberExpression","start":109,"end":120,"object":{"type":"Identifier","start":109,"end":114,"name":"other"},"property":{"type":"PrivateIdentifier","start":116,"end":120,"name":"bar"},"computed":false,"optional":true}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"new Foo.bar()?.baz"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"ChainExpression","start":0,"end":18,"expression":{"type":"MemberExpression","start":0,"end":18,"object":{"type":"NewExpression","start":0,"end":13,"callee":{"type":"MemberExpression","start":4,"end":11,"object":{"type":"Identifier","start":4,"end":7,"name":"Foo"},"property":{"type":"Identifier","start":8,"end":11,"name":"bar"},"computed":false,"optional":false},"arguments":[]},"property":{"type":"Identifier","start":15,"end":18,"name":"baz"},"computed":false,"optional":true}}}],"sourceType":"script"}"#
    );
}
