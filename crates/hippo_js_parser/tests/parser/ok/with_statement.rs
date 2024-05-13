use crate::parser::test_helper_macros::assert_parse_script_eq;

#[test]
fn with_statement() {
    assert_parse_script_eq!(
        r"let a;
const r = 10;

with (Math) {
  a = PI * r * r;
}",
        r#"{"type":"Program","start":0,"end":55,"body":[{"type":"VariableDeclaration","start":0,"end":6,"declarations":[{"type":"VariableDeclarator","start":4,"end":5,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":null}],"kind":"let"},{"type":"VariableDeclaration","start":7,"end":20,"declarations":[{"type":"VariableDeclarator","start":13,"end":19,"id":{"type":"Identifier","start":13,"end":14,"name":"r"},"init":{"type":"Literal","start":17,"end":19,"value":10,"raw":"10"}}],"kind":"const"},{"type":"WithStatement","start":22,"end":55,"object":{"type":"Identifier","start":28,"end":32,"name":"Math"},"body":{"type":"BlockStatement","start":34,"end":55,"body":[{"type":"ExpressionStatement","start":38,"end":53,"expression":{"type":"AssignmentExpression","start":38,"end":52,"operator":"=","left":{"type":"Identifier","start":38,"end":39,"name":"a"},"right":{"type":"BinaryExpression","start":42,"end":52,"left":{"type":"BinaryExpression","start":42,"end":48,"left":{"type":"Identifier","start":42,"end":44,"name":"PI"},"operator":"*","right":{"type":"Identifier","start":47,"end":48,"name":"r"}},"operator":"*","right":{"type":"Identifier","start":51,"end":52,"name":"r"}}}}]}}],"sourceType":"script"}"#
    );
}
