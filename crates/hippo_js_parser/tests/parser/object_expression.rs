use crate::parser::common::assert_parse_module_eq;

#[test]
fn object_expression() {
    assert_parse_module_eq!(
        r#"let a = {};"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"VariableDeclaration","start":0,"end":11,"declarations":[{"type":"VariableDeclarator","start":4,"end":10,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":10,"properties":[]}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let b = {foo,};"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"VariableDeclaration","start":0,"end":15,"declarations":[{"type":"VariableDeclarator","start":4,"end":14,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":14,"properties":[{"type":"Property","start":9,"end":12,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":9,"end":12,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":9,"end":12,"name":"foo"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn object_expression_spread_property() {
    assert_parse_module_eq!(
        r#"let a = {...foo}"#,
        r#"{"type":"Program","start":0,"end":16,"body":[{"type":"VariableDeclaration","start":0,"end":16,"declarations":[{"type":"VariableDeclarator","start":4,"end":16,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"ObjectExpression","start":8,"end":16,"properties":[{"type":"SpreadElement","start":9,"end":15,"argument":{"type":"Identifier","start":12,"end":15,"name":"foo"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn object_expression_identifier_property() {
    assert_parse_module_eq!(
        r#"({foo})"#,
        r#"{"type":"Program","start":0,"end":7,"body":[{"type":"ExpressionStatement","start":0,"end":7,"expression":{"type":"ObjectExpression","start":1,"end":6,"properties":[{"type":"Property","start":2,"end":5,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":2,"end":5,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":2,"end":5,"name":"foo"}}]}}],"sourceType":"module"}"#
    );
}

#[test]
fn object_expression_identifier_property_with_assignment_expression() {
    assert_parse_module_eq!(
        r#"let b = { a: true };"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":20,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":19,"properties":[{"type":"Property","start":10,"end":17,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":10,"end":11,"name":"a"},"kind":"init","value":{"type":"Literal","start":13,"end":17,"value":true,"raw":"true"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn object_expression_identifier_literal_property_key() {
    assert_parse_module_eq!(
        r#"let b = { "foo": true };"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"VariableDeclaration","start":0,"end":24,"declarations":[{"type":"VariableDeclarator","start":4,"end":23,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":23,"properties":[{"type":"Property","start":10,"end":21,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":10,"end":15,"value":"foo","raw":"\"foo\""},"kind":"init","value":{"type":"Literal","start":17,"end":21,"value":true,"raw":"true"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"let b = { 1: true };"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"VariableDeclaration","start":0,"end":20,"declarations":[{"type":"VariableDeclarator","start":4,"end":19,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":19,"properties":[{"type":"Property","start":10,"end":17,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":10,"end":11,"value":1.0,"raw":"1"},"kind":"init","value":{"type":"Literal","start":13,"end":17,"value":true,"raw":"true"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}

#[test]
fn object_expression_identifier_computed_property() {
    assert_parse_module_eq!(
        r#"let b = { [true === true]: true };"#,
        r#"{"type":"Program","start":0,"end":34,"body":[{"type":"VariableDeclaration","start":0,"end":34,"declarations":[{"type":"VariableDeclarator","start":4,"end":33,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ObjectExpression","start":8,"end":33,"properties":[{"type":"Property","start":10,"end":31,"method":false,"shorthand":false,"computed":true,"key":{"type":"BinaryExpression","start":11,"end":24,"left":{"type":"Literal","start":11,"end":15,"value":true,"raw":"true"},"operator":"===","right":{"type":"Literal","start":20,"end":24,"value":true,"raw":"true"}},"kind":"init","value":{"type":"Literal","start":27,"end":31,"value":true,"raw":"true"}}]}}],"kind":"let"}],"sourceType":"module"}"#
    );
}
