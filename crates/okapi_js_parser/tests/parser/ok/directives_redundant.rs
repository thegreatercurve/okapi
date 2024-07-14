use crate::parser::assert_parser_script_eq;

#[test]
fn directives_redundant() {
    assert_parser_script_eq!(
        r#"function test() {
  "use strict";
  function inner_a() {
    "use strict";
  }
  function inner_b() {
    function inner_inner() {
      "use strict";
    }
  }
}"#,
        r#"{"type":"Program","start":0,"end":162,"body":[{"type":"FunctionDeclaration","start":0,"end":162,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":162,"body":[{"type":"ExpressionStatement","start":20,"end":33,"expression":{"type":"Literal","start":20,"end":32,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"FunctionDeclaration","start":36,"end":78,"id":{"type":"Identifier","start":45,"end":52,"name":"inner_a"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":55,"end":78,"body":[{"type":"ExpressionStatement","start":61,"end":74,"expression":{"type":"Literal","start":61,"end":73,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"}]}},{"type":"FunctionDeclaration","start":81,"end":160,"id":{"type":"Identifier","start":90,"end":97,"name":"inner_b"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":100,"end":160,"body":[{"type":"FunctionDeclaration","start":106,"end":156,"id":{"type":"Identifier","start":115,"end":126,"name":"inner_inner"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":129,"end":156,"body":[{"type":"ExpressionStatement","start":137,"end":150,"expression":{"type":"Literal","start":137,"end":149,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"}]}}]}}]}}],"sourceType":"script"}"#
    );
}
