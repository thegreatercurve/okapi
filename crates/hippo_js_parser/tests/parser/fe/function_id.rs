
use crate::parser::common::assert_parser_eq;

#[test]
fn function_id() {
    
                    assert_parser_eq!(
                        r#"// SCRIPT"#,
                        r#"{"type":"Program","start":0,"end":9,"body":[],"sourceType":"script"}"#
                    );
                    assert_parser_eq!(
                        r#"function test() {}"#,
                        r#"{"type":"Program","start":0,"end":18,"body":[{"type":"FunctionDeclaration","start":0,"end":18,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":18,"body":[]}}],"sourceType":"script"}"#
                    );
                    assert_parser_eq!(
                        r#"function await(test) {}"#,
                        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"FunctionDeclaration","start":0,"end":23,"id":{"type":"Identifier","start":9,"end":14,"name":"await"},"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":15,"end":19,"name":"test"}],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}}],"sourceType":"script"}"#
                    );
                    assert_parser_eq!(
                        r#"async function await(test) {}"#,
                        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"FunctionDeclaration","start":0,"end":29,"id":{"type":"Identifier","start":15,"end":20,"name":"await"},"expression":false,"generator":false,"async":true,"params":[{"type":"Identifier","start":21,"end":25,"name":"test"}],"body":{"type":"BlockStatement","start":27,"end":29,"body":[]}}],"sourceType":"script"}"#
                    );
                    assert_parser_eq!(
                        r#"function yield(test) {}"#,
                        r#"{"type":"Program","start":0,"end":23,"body":[{"type":"FunctionDeclaration","start":0,"end":23,"id":{"type":"Identifier","start":9,"end":14,"name":"yield"},"expression":false,"generator":false,"async":false,"params":[{"type":"Identifier","start":15,"end":19,"name":"test"}],"body":{"type":"BlockStatement","start":21,"end":23,"body":[]}}],"sourceType":"script"}"#
                    );
                    assert_parser_eq!(
                        r#"function* yield(test) {}"#,
                        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"FunctionDeclaration","start":0,"end":24,"id":{"type":"Identifier","start":10,"end":15,"name":"yield"},"expression":false,"generator":true,"async":false,"params":[{"type":"Identifier","start":16,"end":20,"name":"test"}],"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
                    );
}
