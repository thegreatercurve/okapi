use crate::parser::assert_parser_eq;

#[test]
fn for_classic_statement() {
    assert_parser_eq!(
        r#"for (var i = 0; i < 10; i++) {}"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"ForStatement","start":0,"end":31,"init":{"type":"VariableDeclaration","start":5,"end":14,"declarations":[{"type":"VariableDeclarator","start":9,"end":14,"id":{"type":"Identifier","start":9,"end":10,"name":"i"},"init":{"type":"Literal","start":13,"end":14,"value":0,"raw":"0"}}],"kind":"var"},"test":{"type":"BinaryExpression","start":16,"end":22,"left":{"type":"Identifier","start":16,"end":17,"name":"i"},"operator":"<","right":{"type":"Literal","start":20,"end":22,"value":10,"raw":"10"}},"update":{"type":"UpdateExpression","start":24,"end":27,"operator":"++","prefix":false,"argument":{"type":"Identifier","start":24,"end":25,"name":"i"}},"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"for (let i = 5; i < 10; i++) {}"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"ForStatement","start":0,"end":31,"init":{"type":"VariableDeclaration","start":5,"end":14,"declarations":[{"type":"VariableDeclarator","start":9,"end":14,"id":{"type":"Identifier","start":9,"end":10,"name":"i"},"init":{"type":"Literal","start":13,"end":14,"value":5,"raw":"5"}}],"kind":"let"},"test":{"type":"BinaryExpression","start":16,"end":22,"left":{"type":"Identifier","start":16,"end":17,"name":"i"},"operator":"<","right":{"type":"Literal","start":20,"end":22,"value":10,"raw":"10"}},"update":{"type":"UpdateExpression","start":24,"end":27,"operator":"++","prefix":false,"argument":{"type":"Identifier","start":24,"end":25,"name":"i"}},"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"for (let i = 5, j = 6; i < j; ++j) {}"#,
        r#"{"type":"Program","start":0,"end":37,"body":[{"type":"ForStatement","start":0,"end":37,"init":{"type":"VariableDeclaration","start":5,"end":21,"declarations":[{"type":"VariableDeclarator","start":9,"end":14,"id":{"type":"Identifier","start":9,"end":10,"name":"i"},"init":{"type":"Literal","start":13,"end":14,"value":5,"raw":"5"}},{"type":"VariableDeclarator","start":16,"end":21,"id":{"type":"Identifier","start":16,"end":17,"name":"j"},"init":{"type":"Literal","start":20,"end":21,"value":6,"raw":"6"}}],"kind":"let"},"test":{"type":"BinaryExpression","start":23,"end":28,"left":{"type":"Identifier","start":23,"end":24,"name":"i"},"operator":"<","right":{"type":"Identifier","start":27,"end":28,"name":"j"}},"update":{"type":"UpdateExpression","start":30,"end":33,"operator":"++","prefix":true,"argument":{"type":"Identifier","start":32,"end":33,"name":"j"}},"body":{"type":"BlockStatement","start":35,"end":37,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"let i; for ({}; i < 10; i++) {}"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"VariableDeclaration","start":0,"end":6,"declarations":[{"type":"VariableDeclarator","start":4,"end":5,"id":{"type":"Identifier","start":4,"end":5,"name":"i"},"init":null}],"kind":"let"},{"type":"ForStatement","start":7,"end":31,"init":{"type":"ObjectExpression","start":12,"end":14,"properties":[]},"test":{"type":"BinaryExpression","start":16,"end":22,"left":{"type":"Identifier","start":16,"end":17,"name":"i"},"operator":"<","right":{"type":"Literal","start":20,"end":22,"value":10,"raw":"10"}},"update":{"type":"UpdateExpression","start":24,"end":27,"operator":"++","prefix":false,"argument":{"type":"Identifier","start":24,"end":25,"name":"i"}},"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn for_statement_empty() {
    assert_parser_eq!(
        r#"for (;;) {}"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"ForStatement","start":0,"end":11,"init":null,"test":null,"update":null,"body":{"type":"BlockStatement","start":9,"end":11,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn for_of_statement() {
    assert_parser_eq!(
        r#"for (let { foo, bar } of {}) {}"#,
        r#"{"type":"Program","start":0,"end":31,"body":[{"type":"ForOfStatement","start":0,"end":31,"await":false,"left":{"type":"VariableDeclaration","start":5,"end":21,"declarations":[{"type":"VariableDeclarator","start":9,"end":21,"id":{"type":"ObjectPattern","start":9,"end":21,"properties":[{"type":"Property","start":11,"end":14,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":11,"end":14,"name":"foo"},"kind":"init","value":{"type":"Identifier","start":11,"end":14,"name":"foo"}},{"type":"Property","start":16,"end":19,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":16,"end":19,"name":"bar"},"kind":"init","value":{"type":"Identifier","start":16,"end":19,"name":"bar"}}]},"init":null}],"kind":"let"},"right":{"type":"ObjectExpression","start":25,"end":27,"properties":[]},"body":{"type":"BlockStatement","start":29,"end":31,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"for (let foo of []) {}"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ForOfStatement","start":0,"end":22,"await":false,"left":{"type":"VariableDeclaration","start":5,"end":12,"declarations":[{"type":"VariableDeclarator","start":9,"end":12,"id":{"type":"Identifier","start":9,"end":12,"name":"foo"},"init":null}],"kind":"let"},"right":{"type":"ArrayExpression","start":16,"end":18,"elements":[]},"body":{"type":"BlockStatement","start":20,"end":22,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"for ({} of []) {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ForOfStatement","start":0,"end":17,"await":false,"left":{"type":"ObjectPattern","start":5,"end":7,"properties":[]},"right":{"type":"ArrayExpression","start":11,"end":13,"elements":[]},"body":{"type":"BlockStatement","start":15,"end":17,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn for_in_statement() {
    assert_parser_eq!(
        r#"for (const foo in {}) {}"#,
        r#"{"type":"Program","start":0,"end":24,"body":[{"type":"ForInStatement","start":0,"end":24,"left":{"type":"VariableDeclaration","start":5,"end":14,"declarations":[{"type":"VariableDeclarator","start":11,"end":14,"id":{"type":"Identifier","start":11,"end":14,"name":"foo"},"init":null}],"kind":"const"},"right":{"type":"ObjectExpression","start":18,"end":20,"properties":[]},"body":{"type":"BlockStatement","start":22,"end":24,"body":[]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"for ({} in []) {}"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ForInStatement","start":0,"end":17,"left":{"type":"ObjectPattern","start":5,"end":7,"properties":[]},"right":{"type":"ArrayExpression","start":11,"end":13,"elements":[]},"body":{"type":"BlockStatement","start":15,"end":17,"body":[]}}],"sourceType":"script"}"#
    );
}

#[test]
fn for_of_async_statement() {
    assert_parser_eq!(
        r#"for await (let a of []) {}"#,
        r#"{"type":"Program","start":0,"end":26,"body":[{"type":"ForOfStatement","start":0,"end":26,"await":true,"left":{"type":"VariableDeclaration","start":11,"end":16,"declarations":[{"type":"VariableDeclarator","start":15,"end":16,"id":{"type":"Identifier","start":15,"end":16,"name":"a"},"init":null}],"kind":"let"},"right":{"type":"ArrayExpression","start":20,"end":22,"elements":[]},"body":{"type":"BlockStatement","start":24,"end":26,"body":[]}}],"sourceType":"script"}"#
    );
}
