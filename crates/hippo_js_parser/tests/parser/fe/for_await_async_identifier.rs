use crate::parser::common::assert_parser_eq;

#[test]
fn for_await_async_identifier() {
    assert_parser_eq!(
        r#"let async;"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"VariableDeclaration","start":0,"end":10,"declarations":[{"type":"VariableDeclarator","start":4,"end":9,"id":{"type":"Identifier","start":4,"end":9,"name":"async"},"init":null}],"kind":"let"}],"sourceType":"script"}"#
    );
    assert_parser_eq!(
        r#"async function fn() { for await (async of [7]); }"#,
        r#"{"type":"Program","start":0,"end":49,"body":[{"type":"FunctionDeclaration","start":0,"end":49,"id":{"type":"Identifier","start":15,"end":17,"name":"fn"},"expression":false,"generator":false,"async":true,"params":[],"body":{"type":"BlockStatement","start":20,"end":49,"body":[{"type":"ForOfStatement","start":22,"end":47,"await":true,"left":{"type":"Identifier","start":33,"end":38,"name":"async"},"right":{"type":"ArrayExpression","start":42,"end":45,"elements":[{"type":"Literal","start":43,"end":44,"value":7,"raw":"7"}]},"body":{"type":"EmptyStatement","start":46,"end":47}}]}}],"sourceType":"script"}"#
    );
}
