use crate::parser::common::assert_parse_module_eq;

#[test]
fn try_statement() {
    assert_parse_module_eq!(
        r#"try {} catch {}"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"TryStatement","start":0,"end":15,"block":{"type":"BlockStatement","start":4,"end":6,"body":[]},"handler":{"type":"CatchClause","start":7,"end":15,"param":null,"body":{"type":"BlockStatement","start":13,"end":15,"body":[]}},"finalizer":null}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"try {} catch (e) {}"#,
        r#"{"type":"Program","start":0,"end":19,"body":[{"type":"TryStatement","start":0,"end":19,"block":{"type":"BlockStatement","start":4,"end":6,"body":[]},"handler":{"type":"CatchClause","start":7,"end":19,"param":{"type":"Identifier","start":14,"end":15,"name":"e"},"body":{"type":"BlockStatement","start":17,"end":19,"body":[]}},"finalizer":null}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"try {} catch {} finally {}"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"TryStatement","start":0,"end":26,"block":{"type":"BlockStatement","start":4,"end":6,"body":[]},"handler":{"type":"CatchClause","start":7,"end":15,"param":null,"body":{"type":"BlockStatement","start":13,"end":15,"body":[]}},"finalizer":{"type":"BlockStatement","start":24,"end":26,"body":[]}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"try {} catch (e) {} finally {}"#,
        r#"{"type":"Program","start":0,"end":30,"body":[{"type":"TryStatement","start":0,"end":30,"block":{"type":"BlockStatement","start":4,"end":6,"body":[]},"handler":{"type":"CatchClause","start":7,"end":19,"param":{"type":"Identifier","start":14,"end":15,"name":"e"},"body":{"type":"BlockStatement","start":17,"end":19,"body":[]}},"finalizer":{"type":"BlockStatement","start":28,"end":30,"body":[]}}],"sourceType":"script"}"#
    );
    assert_parse_module_eq!(
        r#"try {} finally {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"TryStatement","start":0,"end":17,"block":{"type":"BlockStatement","start":4,"end":6,"body":[]},"handler":null,"finalizer":{"type":"BlockStatement","start":15,"end":17,"body":[]}}],"sourceType":"script"}"#
    );
}
