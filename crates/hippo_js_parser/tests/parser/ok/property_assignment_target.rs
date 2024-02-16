use crate::parser::common::assert_parse_module_eq;

#[test]
fn property_assignment_target() {
    assert_parse_module_eq!(
        r#"({x}= {});"#,
        r#"{"type":"Program","start":0,"end":10,"body":[{"type":"ExpressionStatement","start":0,"end":10,"expression":{"type":"AssignmentExpression","start":1,"end":8,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":4,"properties":[{"type":"Property","start":2,"end":3,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"kind":"init","value":{"type":"Identifier","start":2,"end":3,"name":"x"}}]},"right":{"type":"ObjectExpression","start":6,"end":8,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({x: y}= {});"#,
        r#"{"type":"Program","start":0,"end":13,"body":[{"type":"ExpressionStatement","start":0,"end":13,"expression":{"type":"AssignmentExpression","start":1,"end":11,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":7,"properties":[{"type":"Property","start":2,"end":6,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"value":{"type":"Identifier","start":5,"end":6,"name":"y"},"kind":"init"}]},"right":{"type":"ObjectExpression","start":9,"end":11,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({x: y.test().z}= {});"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"AssignmentExpression","start":1,"end":20,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":16,"properties":[{"type":"Property","start":2,"end":15,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"value":{"type":"MemberExpression","start":5,"end":15,"object":{"type":"CallExpression","start":5,"end":13,"callee":{"type":"MemberExpression","start":5,"end":11,"object":{"type":"Identifier","start":5,"end":6,"name":"y"},"property":{"type":"Identifier","start":7,"end":11,"name":"test"},"computed":false,"optional":false},"arguments":[],"optional":false},"property":{"type":"Identifier","start":14,"end":15,"name":"z"},"computed":false,"optional":false},"kind":"init"}]},"right":{"type":"ObjectExpression","start":18,"end":20,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({x: ((z))}= {});"#,
        r#"{"type":"Program","start":0,"end":17,"body":[{"type":"ExpressionStatement","start":0,"end":17,"expression":{"type":"AssignmentExpression","start":1,"end":15,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":11,"properties":[{"type":"Property","start":2,"end":10,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"value":{"type":"Identifier","start":7,"end":8,"name":"z"},"kind":"init"}]},"right":{"type":"ObjectExpression","start":13,"end":15,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({x: z["computed"]}= {});"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"ExpressionStatement","start":0,"end":25,"expression":{"type":"AssignmentExpression","start":1,"end":23,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":19,"properties":[{"type":"Property","start":2,"end":18,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"value":{"type":"MemberExpression","start":5,"end":18,"object":{"type":"Identifier","start":5,"end":6,"name":"z"},"property":{"type":"Literal","start":7,"end":17,"value":"computed","raw":"\"computed\""},"computed":true,"optional":false},"kind":"init"}]},"right":{"type":"ObjectExpression","start":21,"end":23,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({x = "default"}= {});"#,
        r#"{"type":"Program","start":0,"end":22,"body":[{"type":"ExpressionStatement","start":0,"end":22,"expression":{"type":"AssignmentExpression","start":1,"end":20,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":16,"properties":[{"type":"Property","start":2,"end":15,"method":false,"shorthand":true,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"kind":"init","value":{"type":"AssignmentPattern","start":2,"end":15,"left":{"type":"Identifier","start":2,"end":3,"name":"x"},"right":{"type":"Literal","start":6,"end":15,"value":"default","raw":"\"default\""}}}]},"right":{"type":"ObjectExpression","start":18,"end":20,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({x: y = "default"}= {});"#,
        r#"{"type":"Program","start":0,"end":25,"body":[{"type":"ExpressionStatement","start":0,"end":25,"expression":{"type":"AssignmentExpression","start":1,"end":23,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":19,"properties":[{"type":"Property","start":2,"end":18,"method":false,"shorthand":false,"computed":false,"key":{"type":"Identifier","start":2,"end":3,"name":"x"},"value":{"type":"AssignmentPattern","start":5,"end":18,"left":{"type":"Identifier","start":5,"end":6,"name":"y"},"right":{"type":"Literal","start":9,"end":18,"value":"default","raw":"\"default\""}},"kind":"init"}]},"right":{"type":"ObjectExpression","start":21,"end":23,"properties":[]}}}],"sourceType":"module"}"#
    );
    assert_parse_module_eq!(
        r#"({0: y, [computed]: z} = {});"#,
        r#"{"type":"Program","start":0,"end":29,"body":[{"type":"ExpressionStatement","start":0,"end":29,"expression":{"type":"AssignmentExpression","start":1,"end":27,"operator":"=","left":{"type":"ObjectPattern","start":1,"end":22,"properties":[{"type":"Property","start":2,"end":6,"method":false,"shorthand":false,"computed":false,"key":{"type":"Literal","start":2,"end":3,"value":0,"raw":"0"},"value":{"type":"Identifier","start":5,"end":6,"name":"y"},"kind":"init"},{"type":"Property","start":8,"end":21,"method":false,"shorthand":false,"computed":true,"key":{"type":"Identifier","start":9,"end":17,"name":"computed"},"value":{"type":"Identifier","start":20,"end":21,"name":"z"},"kind":"init"}]},"right":{"type":"ObjectExpression","start":25,"end":27,"properties":[]}}}],"sourceType":"module"}"#
    );
}
