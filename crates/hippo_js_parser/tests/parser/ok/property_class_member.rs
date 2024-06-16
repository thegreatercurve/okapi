use crate::parser::assert_parser_script_eq;

#[test]
fn property_class_member() {
    assert_parser_script_eq!(
        r#"class foo { 
    property; 
    declare; 
    initializedProperty = "a"; 
    a; 
    5; 
    ["a" + "b"]; 
    static staticProperty; 
    static staticInitializedProperty = 1; 
    #private; #privateInitialized = "a"; 
    static #staticPrivate; 
    static #staticPrivateInitializedProperty = 1; 
}"#,
        r#"{"type":"Program","start":0,"end":301,"body":[{"type":"ClassDeclaration","start":0,"end":301,"id":{"type":"Identifier","start":6,"end":9,"name":"foo"},"superClass":null,"body":{"type":"ClassBody","start":10,"end":301,"body":[{"type":"PropertyDefinition","start":17,"end":26,"static":false,"computed":false,"key":{"type":"Identifier","start":17,"end":25,"name":"property"},"value":null},{"type":"PropertyDefinition","start":32,"end":40,"static":false,"computed":false,"key":{"type":"Identifier","start":32,"end":39,"name":"declare"},"value":null},{"type":"PropertyDefinition","start":46,"end":72,"static":false,"computed":false,"key":{"type":"Identifier","start":46,"end":65,"name":"initializedProperty"},"value":{"type":"Literal","start":68,"end":71,"value":"a","raw":"\"a\""}},{"type":"PropertyDefinition","start":78,"end":80,"static":false,"computed":false,"key":{"type":"Identifier","start":78,"end":79,"name":"a"},"value":null},{"type":"PropertyDefinition","start":86,"end":88,"static":false,"computed":false,"key":{"type":"Literal","start":86,"end":87,"value":5.0,"raw":"5"},"value":null},{"type":"PropertyDefinition","start":94,"end":106,"static":false,"computed":true,"key":{"type":"BinaryExpression","start":95,"end":104,"left":{"type":"Literal","start":95,"end":98,"value":"a","raw":"\"a\""},"operator":"+","right":{"type":"Literal","start":101,"end":104,"value":"b","raw":"\"b\""}},"value":null},{"type":"PropertyDefinition","start":112,"end":134,"static":true,"computed":false,"key":{"type":"Identifier","start":119,"end":133,"name":"staticProperty"},"value":null},{"type":"PropertyDefinition","start":140,"end":177,"static":true,"computed":false,"key":{"type":"Identifier","start":147,"end":172,"name":"staticInitializedProperty"},"value":{"type":"Literal","start":175,"end":176,"value":1.0,"raw":"1"}},{"type":"PropertyDefinition","start":183,"end":192,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":183,"end":191,"name":"private"},"value":null},{"type":"PropertyDefinition","start":193,"end":219,"static":false,"computed":false,"key":{"type":"PrivateIdentifier","start":193,"end":212,"name":"privateInitialized"},"value":{"type":"Literal","start":215,"end":218,"value":"a","raw":"\"a\""}},{"type":"PropertyDefinition","start":225,"end":247,"static":true,"computed":false,"key":{"type":"PrivateIdentifier","start":232,"end":246,"name":"staticPrivate"},"value":null},{"type":"PropertyDefinition","start":253,"end":298,"static":true,"computed":false,"key":{"type":"PrivateIdentifier","start":260,"end":293,"name":"staticPrivateInitializedProperty"},"value":{"type":"Literal","start":296,"end":297,"value":1.0,"raw":"1"}}]}}],"sourceType":"script"}"#
    );
}
