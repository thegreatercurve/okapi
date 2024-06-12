use crate::parser::assert_parser_eq;

#[test]
fn async_in_class_constructor() {
    assert_parser_eq!(
        r#"class Foo {
    constructor() {
        this.bar = async function () {
            await baz;
        };
    }
}"#,
        r#"{"type":"Program","start":0,"end":112,"body":[{"type":"ClassDeclaration","start":0,"end":112,"id":{"type":"Identifier","start":6,"end":9,"name":"Foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":112,"body":[{"type":"MethodDefinition","start":16,"end":110,"static":false,"computed":false,"key":{"type":"Identifier","start":16,"end":27,"name":"constructor"},"kind":"constructor","value":{"type":"FunctionExpression","start":27,"end":110,"id":null,"params":[],"generator":false,"expression":false,"async":false,"body":{"type":"BlockStatement","start":30,"end":110,"body":[{"type":"ExpressionStatement","start":40,"end":104,"expression":{"type":"AssignmentExpression","start":40,"end":103,"operator":"=","left":{"type":"MemberExpression","start":40,"end":48,"object":{"type":"ThisExpression","start":40,"end":44},"property":{"type":"Identifier","start":45,"end":48,"name":"bar"},"computed":false,"optional":false},"right":{"type":"FunctionExpression","start":51,"end":103,"id":null,"params":[],"generator":false,"expression":false,"async":true,"body":{"type":"BlockStatement","start":69,"end":103,"body":[{"type":"ExpressionStatement","start":83,"end":93,"expression":{"type":"AwaitExpression","start":83,"end":92,"argument":{"type":"Identifier","start":89,"end":92,"name":"baz"}}}]}}}}]}}}]}}],"sourceType":"script"}"#
    );
}
