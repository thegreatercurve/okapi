use crate::parser::assert_parser_eq;

#[test]
fn static_initialization_block_member() {
    assert_parser_eq!(
        r#"class A { static a; static { this.a = "test"; } }"#,
        r#"{"type":"Program","start":0,"end":49,"body":[{"type":"ClassDeclaration","start":0,"end":49,"id":{"type":"Identifier","start":6,"end":7,"name":"A"},"superClass":null,"body":{"type":"ClassBody","start":8,"end":49,"body":[{"type":"PropertyDefinition","start":10,"end":19,"static":true,"computed":false,"key":{"type":"Identifier","start":17,"end":18,"name":"a"},"value":null},{"type":"StaticBlock","start":20,"end":47,"body":[{"type":"ExpressionStatement","start":29,"end":45,"expression":{"type":"AssignmentExpression","start":29,"end":44,"operator":"=","left":{"type":"MemberExpression","start":29,"end":35,"object":{"type":"ThisExpression","start":29,"end":33},"property":{"type":"Identifier","start":34,"end":35,"name":"a"},"computed":false,"optional":false},"right":{"type":"Literal","start":38,"end":44,"value":"test","raw":"\"test\""}}}]}]}}],"sourceType":"module"}"#
    );
}
