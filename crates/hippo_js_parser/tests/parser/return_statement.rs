// use crate::parser::common::assert_parser_eq;

#[test]
fn return_statement() {
    // assert_parser_eq!(
    //     r#"
    //     () => {
    //         return;
    //     }
    //     "#,
    //     r#"{"type":"Program","start":0,"end":19,"body":[{"type":"ExpressionStatement","start":0,"end":18,"expression":{"type":"ArrowFunctionExpression","start":0,"end":18,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":6,"end":18,"body":[{"type":"ReturnStatement","start":9,"end":16,"argument":null}]}}}],"sourceType":"module"}"#
    // );

    // assert_parser_eq!(
    //     r#"
    //     () => {
    //         return 1 + 1;
    //     }
    //     "#,
    //     r#"{"type":"Program","start":0,"end":25,"body":[{"type":"ExpressionStatement","start":0,"end":24,"expression":{"type":"ArrowFunctionExpression","start":0,"end":24,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":6,"end":24,"body":[{"type":"ReturnStatement","start":9,"end":22,"argument":{"type":"BinaryExpression","start":16,"end":21,"left":{"type":"Literal","start":16,"end":17,"value":1,"raw":"1"},"operator":"+","right":{"type":"Literal","start":20,"end":21,"value":1,"raw":"1"}}}]}}}],"sourceType":"module"}"#
    // );
}
