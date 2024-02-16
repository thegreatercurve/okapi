use crate::parser::common::assert_parse_module_eq;

#[test]
fn super_expression_in_constructor_parameter_list() {
    assert_parse_module_eq!(
        r#"class A extends B { constructor(c = super()) {} }"#,
        r#"{"type":"Program","start":0,"end":49,"body":[{"type":"ClassDeclaration","start":0,"end":49,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":{"type":"Identifier","start":16,"end":17,"name":"B"},"body":{"type":"ClassBody","start":18,"end":49,"body":[{"type":"MethodDefinition","start":20,"end":47,"static":false,"computed":false,"key":{"type":"Identifier","start":20,"end":31,"name":"constructor"},"kind":"constructor","value":{"type":"FunctionExpression","start":31,"end":47,"id":null,"expression":false,"generator":false,"async":false,"params":[{"type":"AssignmentPattern","start":32,"end":43,"left":{"type":"Identifier","start":32,"end":33,"name":"c"},"right":{"type":"CallExpression","start":36,"end":43,"callee":{"type":"Super","start":36,"end":41},"arguments":[],"optional":false}}],"body":{"type":"BlockStatement","start":45,"end":47,"body":[]}}}]}}],"sourceType":"module"}"#
    );
}
