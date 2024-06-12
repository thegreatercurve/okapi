use crate::parser::assert_parser_script_eq;

#[test]
fn unary_delete_nested() {
    assert_parser_script_eq!(
        r#"class TestClass { #member = true; method() { delete func(this.#member) } }"#,
        r#"{"type":"Program","start":0,"end":74,"body":[{"type":"ClassDeclaration","start":0,"end":74,"id":{"type":"Identifier","start":6,"end":15,"name":"TestClass"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":74,"body":[{"type":"PropertyDefinition","start":18,"end":33,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":18,"end":25,"name":"member"},"value":{"type":"Literal","start":28,"end":32,"value":true,"raw":"true"}},{"type":"MethodDefinition","start":34,"end":72,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":40,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":72,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":72,"body":[{"type":"ExpressionStatement","start":45,"end":70,"expression":{"type":"UnaryExpression","start":45,"end":70,"operator":"delete","prefix":true,"argument":{"type":"CallExpression","start":52,"end":70,"callee":{"type":"Identifier","start":52,"end":56,"name":"func"},"arguments":[{"type":"MemberExpression","start":57,"end":69,"object":{"type":"ThisExpression","start":57,"end":61},"property":{"type":"PrivateIdentifier","start":62,"end":69,"name":"member"},"computed":false,"optional":false}],"optional":false}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class TestClass { #member = true; method() { delete [this.#member] } }"#,
        r#"{"type":"Program","start":0,"end":70,"body":[{"type":"ClassDeclaration","start":0,"end":70,"id":{"type":"Identifier","start":6,"end":15,"name":"TestClass"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":70,"body":[{"type":"PropertyDefinition","start":18,"end":33,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":18,"end":25,"name":"member"},"value":{"type":"Literal","start":28,"end":32,"value":true,"raw":"true"}},{"type":"MethodDefinition","start":34,"end":68,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":40,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":68,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":68,"body":[{"type":"ExpressionStatement","start":45,"end":66,"expression":{"type":"UnaryExpression","start":45,"end":66,"operator":"delete","prefix":true,"argument":{"type":"ArrayExpression","start":52,"end":66,"elements":[{"type":"MemberExpression","start":53,"end":65,"object":{"type":"ThisExpression","start":53,"end":57},"property":{"type":"PrivateIdentifier","start":58,"end":65,"name":"member"},"computed":false,"optional":false}]}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class TestClass { #member = true; method() { delete { key: this.#member } } }"#,
        r#"{"type":"Program","start":0,"end":77,"body":[{"type":"ClassDeclaration","start":0,"end":77,"id":{"type":"Identifier","start":6,"end":15,"name":"TestClass"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":77,"body":[{"type":"PropertyDefinition","start":18,"end":33,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":18,"end":25,"name":"member"},"value":{"type":"Literal","start":28,"end":32,"value":true,"raw":"true"}},{"type":"MethodDefinition","start":34,"end":75,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":40,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":75,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":75,"body":[{"type":"ExpressionStatement","start":45,"end":73,"expression":{"type":"UnaryExpression","start":45,"end":73,"operator":"delete","prefix":true,"argument":{"type":"ObjectExpression","start":52,"end":73,"properties":[{"type":"Property","start":54,"end":71,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":54,"end":57,"name":"key"},"value":{"type":"MemberExpression","start":59,"end":71,"object":{"type":"ThisExpression","start":59,"end":63},"property":{"type":"PrivateIdentifier","start":64,"end":71,"name":"member"},"computed":false,"optional":false},"kind":"init"}]}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class TestClass { #member = true; method() { delete (() => { this.#member; }) } }"#,
        r#"{"type":"Program","start":0,"end":81,"body":[{"type":"ClassDeclaration","start":0,"end":81,"id":{"type":"Identifier","start":6,"end":15,"name":"TestClass"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":81,"body":[{"type":"PropertyDefinition","start":18,"end":33,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":18,"end":25,"name":"member"},"value":{"type":"Literal","start":28,"end":32,"value":true,"raw":"true"}},{"type":"MethodDefinition","start":34,"end":79,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":40,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":79,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":79,"body":[{"type":"ExpressionStatement","start":45,"end":77,"expression":{"type":"UnaryExpression","start":45,"end":77,"operator":"delete","prefix":true,"argument":{"type":"ArrowFunctionExpression","start":53,"end":76,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":59,"end":76,"body":[{"type":"ExpressionStatement","start":61,"end":74,"expression":{"type":"MemberExpression","start":61,"end":73,"object":{"type":"ThisExpression","start":61,"end":65},"property":{"type":"PrivateIdentifier","start":66,"end":73,"name":"member"},"computed":false,"optional":false}}]}}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class TestClass { #member = true; method() { delete (param => { this.#member; }) } }"#,
        r#"{"type":"Program","start":0,"end":84,"body":[{"type":"ClassDeclaration","start":0,"end":84,"id":{"type":"Identifier","start":6,"end":15,"name":"TestClass"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":84,"body":[{"type":"PropertyDefinition","start":18,"end":33,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":18,"end":25,"name":"member"},"value":{"type":"Literal","start":28,"end":32,"value":true,"raw":"true"}},{"type":"MethodDefinition","start":34,"end":82,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":40,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":82,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":82,"body":[{"type":"ExpressionStatement","start":45,"end":80,"expression":{"type":"UnaryExpression","start":45,"end":80,"operator":"delete","prefix":true,"argument":{"type":"ArrowFunctionExpression","start":53,"end":79,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":53,"end":58,"name":"param"}],"body":{"type":"BlockStatement","start":62,"end":79,"body":[{"type":"ExpressionStatement","start":64,"end":77,"expression":{"type":"MemberExpression","start":64,"end":76,"object":{"type":"ThisExpression","start":64,"end":68},"property":{"type":"PrivateIdentifier","start":69,"end":76,"name":"member"},"computed":false,"optional":false}}]}}}}]}}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"class TestClass { #member = true; method() { delete (async () => { this.#member; }) } }"#,
        r#"{"type":"Program","start":0,"end":87,"body":[{"type":"ClassDeclaration","start":0,"end":87,"id":{"type":"Identifier","start":6,"end":15,"name":"TestClass"},"superClass":null,"body":{"type":"ClassBody","start":16,"end":87,"body":[{"type":"PropertyDefinition","start":18,"end":33,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":18,"end":25,"name":"member"},"value":{"type":"Literal","start":28,"end":32,"value":true,"raw":"true"}},{"type":"MethodDefinition","start":34,"end":85,"static":false,"computed":false,"key":{"type":"Identifier","start":34,"end":40,"name":"method"},"kind":"method","value":{"type":"FunctionExpression","start":40,"end":85,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":43,"end":85,"body":[{"type":"ExpressionStatement","start":45,"end":83,"expression":{"type":"UnaryExpression","start":45,"end":83,"operator":"delete","prefix":true,"argument":{"type":"ArrowFunctionExpression","start":53,"end":82,"id":null,"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":65,"end":82,"body":[{"type":"ExpressionStatement","start":67,"end":80,"expression":{"type":"MemberExpression","start":67,"end":79,"object":{"type":"ThisExpression","start":67,"end":71},"property":{"type":"PrivateIdentifier","start":72,"end":79,"name":"member"},"computed":false,"optional":false}}]}}}}]}}}]}}],"sourceType":"script"}"#
    );
}
