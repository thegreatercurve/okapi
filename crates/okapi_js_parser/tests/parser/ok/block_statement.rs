use crate::parser::assert_parser_script_eq;

#[test]
fn block_statement() {
    assert_parser_script_eq!(
        r#"{
}"#,
        r#"{"type":"Program","start":0,"end":3,"body":[{"type":"BlockStatement","start":0,"end":3,"body":[]}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"{
    {
        {
            {
            }
        }
    }
}"#,
        r#"{"type":"Program","start":0,"end":63,"body":[{"type":"BlockStatement","start":0,"end":63,"body":[{"type":"BlockStatement","start":6,"end":61,"body":[{"type":"BlockStatement","start":16,"end":55,"body":[{"type":"BlockStatement","start":30,"end":45,"body":[]}]}]}]}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"{
    foo = bar;
}"#,
        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"BlockStatement","start":0,"end":18,"body":[{"type":"ExpressionStatement","start":6,"end":16,"expression":{"type":"AssignmentExpression","start":6,"end":15,"operator":"=","left":{"type":"Identifier","start":6,"end":9,"name":"foo"},"right":{"type":"Identifier","start":12,"end":15,"name":"bar"}}}]}],"sourceType":"script"}"#
    );
}
