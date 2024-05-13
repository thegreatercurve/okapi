use crate::parser::assert_parser_eq;

#[test]
fn directives_redundant() {
    assert_parser_eq!(
        r#"function test() { "use strict"; function inner_a() { "use strict"; } function inner_b() { function inner_inner() { "use strict"; } } }"#,
        r#"{"type":"Program","start":0,"end":134,"body":[{"type":"FunctionDeclaration","start":0,"end":134,"id":{"type":"Identifier","start":9,"end":13,"name":"test"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":16,"end":134,"body":[{"type":"ExpressionStatement","start":18,"end":31,"expression":{"type":"Literal","start":18,"end":30,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"},{"type":"FunctionDeclaration","start":32,"end":68,"id":{"type":"Identifier","start":41,"end":48,"name":"inner_a"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":51,"end":68,"body":[{"type":"ExpressionStatement","start":53,"end":66,"expression":{"type":"Literal","start":53,"end":65,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"}]}},{"type":"FunctionDeclaration","start":69,"end":132,"id":{"type":"Identifier","start":78,"end":85,"name":"inner_b"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":88,"end":132,"body":[{"type":"FunctionDeclaration","start":90,"end":130,"id":{"type":"Identifier","start":99,"end":110,"name":"inner_inner"},"expression":false,"generator":false,"async":false,"params":[],"body":{"type":"BlockStatement","start":113,"end":130,"body":[{"type":"ExpressionStatement","start":115,"end":128,"expression":{"type":"Literal","start":115,"end":127,"value":"use strict","raw":"\"use strict\""},"directive":"use strict"}]}}]}}]}}],"sourceType":"script"}"#
    );
}
