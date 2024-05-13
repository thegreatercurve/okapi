use crate::parser::assert_parser_eq;

#[test]
fn directives() {
    assert_parser_eq!(
        r#""ue new";"#,
        r#"{"type":"Program","start":0,"end":9,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"Literal","start":0,"end":8,"value":"ue new","raw":"\"ue new\""},"directive":"ue new"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"let a = 10;"#,
        r#"{"type":"Program","start":0,"end":11,"body":[{"type":"VariableDeclaration","start":0,"end":11,"declarations":[{"type":"VariableDeclarator","start":4,"end":10,"id":{"type":"Identifier","start":4,"end":5,"name":"a"},"init":{"type":"Literal","start":8,"end":10,"value":10,"raw":"10"}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"("use strict");"#,
        r#"{"type":"Program","start":0,"end":15,"body":[{"type":"ExpressionStatement","start":0,"end":15,"expression":{"type":"Literal","start":1,"end":13,"value":"use strict","raw":"\"use strict\""}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"function test() { "use strict"; let b = 10; ("use strict"); }"#,
        r#"{"type":"Program","start":0,"end":61,"body":[{"type":"FunctionDeclaration","start":0,"end":61,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":61,"body":[{"type":"ExpressionStatement","start":18,"end":31,"expression":{"type":"Literal","start":18,"end":30,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"VariableDeclaration","start":32,"end":43,"declarations":[{"type":"VariableDeclarator","start":36,"end":42,"id":{"type":"Identifier","start":36,"end":37,"name":"b"},"init":{"type":"Literal","start":40,"end":42,"value":10,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":44,"end":59,"expression":{"type":"Literal","start":45,"end":57,"value":"use strict","raw":"\"use strict\""}}]}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"(function () { "use strict"; "use strict".length;  let c = 10; ("use strict");  });"#,
        r#"{"type":"Program","start":0,"end":83,"body":[{"type":"ExpressionStatement","start":0,"end":83,"expression":{"type":"FunctionExpression","start":1,"end":81,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":13,"end":81,"body":[{"type":"ExpressionStatement","start":15,"end":28,"expression":{"type":"Literal","start":15,"end":27,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"ExpressionStatement","start":29,"end":49,"expression":{"type":"MemberExpression","start":29,"end":48,"object":{"type":"Literal","start":29,"end":41,"value":"use strict","raw":"\"use strict\""},"property":{"type":"Identifier","start":42,"end":48,"name":"length"},"computed":false,"optional":false}},{"type":"VariableDeclaration","start":51,"end":62,"declarations":[{"type":"VariableDeclarator","start":55,"end":61,"id":{"type":"Identifier","start":55,"end":56,"name":"c"},"init":{"type":"Literal","start":59,"end":61,"value":10,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":63,"end":78,"expression":{"type":"Literal","start":64,"end":76,"value":"use strict","raw":"\"use strict\""}}]}}}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"let b = () => { "use strict"; let e = 10; ("use strict");  };"#,
        r#"{"type":"Program","start":0,"end":61,"body":[{"type":"VariableDeclaration","start":0,"end":61,"declarations":[{"type":"VariableDeclarator","start":4,"end":60,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ArrowFunctionExpression","start":8,"end":60,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":14,"end":60,"body":[{"type":"ExpressionStatement","start":16,"end":29,"expression":{"type":"Literal","start":16,"end":28,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"VariableDeclaration","start":30,"end":41,"declarations":[{"type":"VariableDeclarator","start":34,"end":40,"id":{"type":"Identifier","start":34,"end":35,"name":"e"},"init":{"type":"Literal","start":38,"end":40,"value":10,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":42,"end":57,"expression":{"type":"Literal","start":43,"end":55,"value":"use strict","raw":"\"use strict\""}}]}}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_eq!(
        r#"{ ("use strict");  }"#,
        r#"{"type":"Program","start":0,"end":20,"body":[{"type":"BlockStatement","start":0,"end":20,"body":[{"type":"ExpressionStatement","start":2,"end":17,"expression":{"type":"Literal","start":3,"end":15,"value":"use strict","raw":"\"use strict\""}}]}],"sourceType":"script"}"#
    );
}
