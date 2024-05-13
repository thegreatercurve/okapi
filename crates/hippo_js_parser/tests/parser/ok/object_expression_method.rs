use crate::parser::assert_parser_eq;

#[test]
fn object_expression_method() {
    assert_parser_eq!(
        r#"let b = {
    foo() {},
    bar(a, b, c) {},
    ["foo" + "bar"](a) {},
    5(...ret) {},
};"#,
        r#"{"type":"Program","start":0,"end":92,"body":[{"type":"VariableDeclaration","start":0,"end":92,"declarations":[{"type":"VariableDeclarator","start":4,"end":91,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":91,"properties":[{"type":"Property","start":14,"end":22,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":14,"end":17,"name":"foo"},"kind":"init","value":{"type":"FunctionExpression","start":17,"end":22,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":20,"end":22,"body":[]}}},{"type":"Property","start":28,"end":43,"method":true,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":28,"end":31,"name":"bar"},"kind":"init","value":{"type":"FunctionExpression","start":31,"end":43,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":32,"end":33,"name":"a"},{"type":"Identifier","start":35,"end":36,"name":"b"},{"type":"Identifier","start":38,"end":39,"name":"c"}],"body":{"type":"BlockStatement","start":41,"end":43,"body":[]}}},{"type":"Property","start":49,"end":70,"method":true,"shorthand":false,"computed":true,"key":{"type":"BinaryExpression","start":50,"end":63,"left":{"type":"Literal","start":50,"end":55,"value":"foo","raw":"\"foo\""},"operator":"+","right":{"type":"Literal","start":58,"end":63,"value":"bar","raw":"\"bar\""}},"kind":"init","value":{"type":"FunctionExpression","start":64,"end":70,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":65,"end":66,"name":"a"}],"body":{"type":"BlockStatement","start":68,"end":70,"body":[]}}},{"type":"Property","start":76,"end":88,"method":true,"shorthand":false,"computed":false,"key":{"type":"Literal","start":76,"end":77,"value":5,"raw":"5"},"kind":"init","value":{"type":"FunctionExpression","start":77,"end":88,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"RestElement","start":78,"end":84,"argument":{"type":"Identifier","start":81,"end":84,"name":"ret"}}],"body":{"type":"BlockStatement","start":86,"end":88,"body":[]}}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
