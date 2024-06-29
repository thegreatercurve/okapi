use crate::parser::assert_parser_script_eq;

#[test]
fn function_declaration() {
    assert_parser_script_eq!(
        r#"function foo() {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"FunctionDeclaration","start":0,"end":17,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":17,"body":[]}}],"sourceType":"script"}"#
    );
}
#[test]
fn function_declaration_async() {
    assert_parser_script_eq!(
        r#"async function foo4() {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"FunctionDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":15,"end":19,"name":"foo4"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn function_declaration_with_formal_parameters_identifiers() {
    assert_parser_script_eq!(
        r#"function foo(bar, baz) {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"FunctionDeclaration","start":0,"end":25,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":13,"end":16,"name":"bar"},{"type":"Identifier","start":18,"end":21,"name":"baz"}],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn function_declaration_with_array_or_object_literal_parameters() {
    assert_parser_script_eq!(
        r#"function a([b] = [1]) {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"FunctionDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":9,"end":10,"name":"a"},"expression":false,"generator":false,"async":false,"params":[{"type":"AssignmentPattern","start":11,"end":20,"left":{"type":"ArrayPattern","start":11,"end":14,"elements":[{"type":"Identifier","start":12,"end":13,"name":"b"}]},"right":{"type":"ArrayExpression","start":17,"end":20,"elements":[{"type":"Literal","start":18,"end":19,"value":1.0,"raw":"1"}]}}],"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function a({b} = {b: 1}) {}"#,
        r#"{"type":"Program","start":0,"end":27,"body":[{"type":"FunctionDeclaration","start":0,"end":27,"id":{"type":"Identifier","start":9,"end":10,"name":"a"},"expression":false,"generator":false,"async":false,"params":[{"type":"AssignmentPattern","start":11,"end":23,"left":{"type":"ObjectPattern","start":11,"end":14,"properties":[{"type":"Property","start":12,"end":13,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":12,"end":13,"name":"b"},"kind":"init","value":{"type":"Identifier","start":12,"end":13,"name":"b"}}]},"right":{"type":"ObjectExpression","start":17,"end":23,"properties":[{"type":"Property","start":18,"end":22,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":18,"end":19,"name":"b"},"value":{"type":"Literal","start":21,"end":22,"value":1.0,"raw":"1"},"kind":"init"}]}}],"body":{"type":"BlockStatement","start":25,"end":27,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn function_declaration_with_formal_parameters_rest_parameter() {
    assert_parser_script_eq!(
        r#"function foo(...bar) {}"#,
        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"FunctionDeclaration","start":0,"end":23,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[{"type":"RestElement","start":13,"end":19,"argument":{"type":"Identifier","start":16,"end":19,"name":"bar"}}],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function foo(bar, ...baz) {}"#,
        r#"{"type":"Program","start":0,"end":28,"body":[{"type":"FunctionDeclaration","start":0,"end":28,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":13,"end":16,"name":"bar"},{"type":"RestElement","start":18,"end":24,"argument":{"type":"Identifier","start":21,"end":24,"name":"baz"}}],"body":{"type":"BlockStatement","start":26,"end":28,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn function_declaration_with_block_statement() {
    assert_parser_script_eq!(
        r#"function foo() { return 1 + 1; }"#,
        r#"{"type":"Program","start":0,"end":32,"body":[{"type":"FunctionDeclaration","start":0,"end":32,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":15,"end":32,"body":[{"type":"ReturnStatement","start":17,"end":30,"argument":{"type":"BinaryExpression","start":24,"end":29,"left":{"type":"Literal","start":24,"end":25,"value":1.0,"raw":"1"},"operator":"+","right":{"type":"Literal","start":28,"end":29,"value":1.0,"raw":"1"}}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn function_declaration_generators() {
    assert_parser_script_eq!(
        r#"function* foo2() {}"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"FunctionDeclaration","start":0,"end":19,"id":{"type":"Identifier","start":10,"end":14,"name":"foo2"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":19,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function* foo5() { yield foo; }"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"FunctionDeclaration","start":0,"end":31,"id":{"type":"Identifier","start":10,"end":14,"name":"foo5"},"expression":false,"generator":true,"async":false,"params":[],"body":{"type":"BlockStatement","start":17,"end":31,"body":[{"type":"ExpressionStatement","start":19,"end":29,"expression":{"type":"YieldExpression","start":19,"end":28,"delegate":false,"argument":{"type":"Identifier","start":25,"end":28,"name":"foo"}}}]}}],"sourceType":"script"}"#
    );
}

#[test]
fn function_declaration_async_generators() {
    assert_parser_script_eq!(
        r#"async function* foo3() {}"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"FunctionDeclaration","start":0,"end":25,"id":{"type":"Identifier","start":16,"end":20,"name":"foo3"},"expression":false,"generator":true,"async":true,"params":[],"body":{"type":"BlockStatement","start":23,"end":25,"body":[]}}],"sourceType":"script"}"#
    );
}
