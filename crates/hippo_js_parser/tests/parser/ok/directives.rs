use crate::parser::assert_parser_script_eq;

#[test]
fn directives() {
    assert_parser_script_eq!(
        r#""use new"
let a = 10;
"use strict"; // not a directive"#,
        r#"{"type":"Program","start":0,"end":54,"body":[{"type":"ExpressionStatement","start":0,"end":9,"expression":{"type":"Literal","start":0,"end":9,"value":"use new","raw":"\"use new\""},"directive":"use new"},{"type":"VariableDeclaration","start":10,"end":21,"declarations":[{"type":"VariableDeclarator","start":14,"end":20,"id":{"type":"Identifier","start":14,"end":15,"name":"a"},"init":{"type":"Literal","start":18,"end":20,"value":10.0,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":22,"end":35,"expression":{"type":"Literal","start":22,"end":34,"value":"use strict","raw":"\"use strict\""}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"function test() {
    'use strict';
    let b = 10;
    'use strict'; // not a directive
}"#,
        r#"{"type":"Program","start":0,"end":90,"body":[{"type":"FunctionDeclaration","start":0,"end":90,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":90,"body":[{"type":"ExpressionStatement","start":22,"end":35,"expression":{"type":"Literal","start":22,"end":34,"value":"use strict","raw":"'use strict'"},"directive":"use strict"},{"type":"VariableDeclaration","start":40,"end":51,"declarations":[{"type":"VariableDeclarator","start":44,"end":50,"id":{"type":"Identifier","start":44,"end":45,"name":"b"},"init":{"type":"Literal","start":48,"end":50,"value":10.0,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":56,"end":69,"expression":{"type":"Literal","start":56,"end":68,"value":"use strict","raw":"'use strict'"}}]}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"(function () {
    "use strict";
    "use strict"
    .length; // not a directive
    let c = 10;
    "use strict"; // not a directive
});"#,
        r#"{"type":"Program","start":0,"end":138,"body":[{"type":"ExpressionStatement","start":0,"end":138,"expression":{"type":"FunctionExpression","start":1,"end":136,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":13,"end":136,"body":[{"type":"ExpressionStatement","start":19,"end":32,"expression":{"type":"Literal","start":19,"end":31,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"ExpressionStatement","start":37,"end":62,"expression":{"type":"MemberExpression","start":37,"end":61,"object":{"type":"Literal","start":37,"end":49,"value":"use strict","raw":"\"use strict\""},"property":{"type":"Identifier","start":55,"end":61,"name":"length"},"computed":false,"optional":false}},{"type":"VariableDeclaration","start":86,"end":97,"declarations":[{"type":"VariableDeclarator","start":90,"end":96,"id":{"type":"Identifier","start":90,"end":91,"name":"c"},"init":{"type":"Literal","start":94,"end":96,"value":10.0,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":102,"end":115,"expression":{"type":"Literal","start":102,"end":114,"value":"use strict","raw":"\"use strict\""}}]}}}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"let b = () => {
    "use strict";
    let e = 10;
    "use strict";  // not a directive
}"#,
        r#"{"type":"Program","start":0,"end":89,"body":[{"type":"VariableDeclaration","start":0,"end":89,"declarations":[{"type":"VariableDeclarator","start":4,"end":89,"id":{"type":"Identifier","start":4,"end":5,"name":"b"},"init":{"type":"ArrowFunctionExpression","start":8,"end":89,"id":null,"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":14,"end":89,"body":[{"type":"ExpressionStatement","start":20,"end":33,"expression":{"type":"Literal","start":20,"end":32,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"VariableDeclaration","start":38,"end":49,"declarations":[{"type":"VariableDeclarator","start":42,"end":48,"id":{"type":"Identifier","start":42,"end":43,"name":"e"},"init":{"type":"Literal","start":46,"end":48,"value":10.0,"raw":"10"}}],"kind":"let"},{"type":"ExpressionStatement","start":54,"end":67,"expression":{"type":"Literal","start":54,"end":66,"value":"use strict","raw":"\"use strict\""}}]}}}],"kind":"let"}],"sourceType":"script"}"#
    );

    assert_parser_script_eq!(
        r#"{
    "use strict"; // not a directive
}"#,
        r#"{"type":"Program","start":0,"end":40,"body":[{"type":"BlockStatement","start":0,"end":40,"body":[{"type":"ExpressionStatement","start":6,"end":19,"expression":{"type":"Literal","start":6,"end":18,"value":"use strict","raw":"\"use strict\""}}]}],"sourceType":"script"}"#
    );
}
