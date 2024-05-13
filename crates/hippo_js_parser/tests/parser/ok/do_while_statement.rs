use crate::parser::assert_parser_eq;

#[test]
fn do_while_statement() {
    assert_parser_eq!(
        r#"do { } while (true)"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"DoWhileStatement","start":0,"end":19,"body":{"type":"BlockStatement","start":3,"end":6,"body":[]},"test":{"type":"Literal","start":14,"end":18,"value":true,"raw":"true"}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"do { throw Error("foo") } while (true)"#,
        r#"{"type":"Program","start":0,"end":38,"body":[{"type":"DoWhileStatement","start":0,"end":38,"body":{"type":"BlockStatement","start":3,"end":25,"body":[{"type":"ThrowStatement","start":5,"end":23,"argument":{"type":"CallExpression","start":11,"end":23,"callee":{"type":"Identifier","start":11,"end":16,"name":"Error"},"arguments":[{"type":"Literal","start":17,"end":22,"value":"foo","raw":"\"foo\""}],"optional":false}}]},"test":{"type":"Literal","start":33,"end":37,"value":true,"raw":"true"}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"do { break; } while (true)"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"DoWhileStatement","start":0,"end":26,"body":{"type":"BlockStatement","start":3,"end":13,"body":[{"type":"BreakStatement","start":5,"end":11,"label":null}]},"test":{"type":"Literal","start":21,"end":25,"value":true,"raw":"true"}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"do console.log("test"); while (true);"#,
        r#"{"type":"Program","start":0,"end":37,"body":[{"type":"DoWhileStatement","start":0,"end":37,"body":{"type":"ExpressionStatement","start":3,"end":23,"expression":{"type":"CallExpression","start":3,"end":22,"callee":{"type":"MemberExpression","start":3,"end":14,"object":{"type":"Identifier","start":3,"end":10,"name":"console"},"property":{"type":"Identifier","start":11,"end":14,"name":"log"},"computed":false,"optional":false},"arguments":[{"type":"Literal","start":15,"end":21,"value":"test","raw":"\"test\""}],"optional":false}},"test":{"type":"Literal","start":31,"end":35,"value":true,"raw":"true"}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"do {
    console.log("test");
} while (true);"#,
        r#"{"type":"Program","start":0,"end":45,"body":[{"type":"DoWhileStatement","start":0,"end":45,"body":{"type":"BlockStatement","start":3,"end":31,"body":[{"type":"ExpressionStatement","start":9,"end":29,"expression":{"type":"CallExpression","start":9,"end":28,"callee":{"type":"MemberExpression","start":9,"end":20,"object":{"type":"Identifier","start":9,"end":16,"name":"console"},"property":{"type":"Identifier","start":17,"end":20,"name":"log"},"computed":false,"optional":false},"arguments":[{"type":"Literal","start":21,"end":27,"value":"test","raw":"\"test\""}],"optional":false}}]},"test":{"type":"Literal","start":39,"end":43,"value":true,"raw":"true"}}],"sourceType":"module"}"#
    );

    assert_parser_eq!(
        r#"let a = 1;
do
    do {
        a = a + 1;
    } while (a < 5);
while (a < 100);"#,
        r#"{"type":"Program","start":0,"end":79,"body":[{"type":"VariableDeclaration","start":0,"end":10,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"Literal","start":8,"end":9,"value":1,"raw":"1"}}],"kind":"let"},{"type":"DoWhileStatement","start":11,"end":79,"body":{"type":"DoWhileStatement","start":18,"end":62,"body":{"type":"BlockStatement","start":21,"end":47,"body":[{"type":"ExpressionStatement","start":31,"end":41,"expression":{"type":"AssignmentExpression","start":31,"end":40,"operator":"=","left":{"type":"Identifier","start":31,"end":32,"name":"a"},"right":{"type":"BinaryExpression","start":35,"end":40,"left":{"type":"Identifier","start":35,"end":36,"name":"a"},"operator":"+","right":{"type":"Literal","start":39,"end":40,"value":1,"raw":"1"}}}}]},"test":{"type":"BinaryExpression","start":55,"end":60,"left":{"type":"Identifier","start":55,"end":56,"name":"a"},"operator":"<","right":{"type":"Literal","start":59,"end":60,"value":5,"raw":"5"}}},"test":{"type":"BinaryExpression","start":70,"end":77,"left":{"type":"Identifier","start":70,"end":71,"name":"a"},"operator":"<","right":{"type":"Literal","start":74,"end":77,"value":100,"raw":"100"}}}],"sourceType":"module"}"#
    );
}
