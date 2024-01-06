use crate::parser::common::assert_parser_eq;

#[test]
fn single_binding_identifier() {
    assert_parser_eq!(
        "let hello;",
        Program {
            body: vec![ProgramBody::Statement(Statement::Declaration(
                Declaration::Variable(VariableDeclaration {
                    kind: VariableKind::Let,
                    declarations: vec![VariableDeclarator {
                        id: Identifier {
                            name: "hello".to_string(),
                            node: Node::new(4, 9)
                        },
                        init: None,
                        node: Node::new(4, 9)
                    }],
                    node: Node::new(0, 10)
                })
            ))],
            node: Node::new(0, 10),
            source_type: ProgramSourceTypes::Module
        }
    );

    assert_parser_eq!(
        "var hello;",
        Program {
            body: vec![ProgramBody::Statement(Statement::Declaration(
                Declaration::Variable(VariableDeclaration {
                    kind: VariableKind::Var,
                    declarations: vec![VariableDeclarator {
                        id: Identifier {
                            name: "hello".to_string(),
                            node: Node::new(4, 9)
                        },
                        init: None,
                        node: Node::new(4, 9)
                    }],
                    node: Node::new(0, 10)
                })
            ))],
            node: Node::new(0, 10),
            source_type: ProgramSourceTypes::Module
        }
    );
}

#[test]
fn multiple_binding_identifiers() {
    assert_parser_eq!(
        "let hello, world;",
        Program {
            body: vec![ProgramBody::Statement(Statement::Declaration(
                Declaration::Variable(VariableDeclaration {
                    kind: VariableKind::Let,
                    declarations: vec![
                        VariableDeclarator {
                            id: Identifier {
                                name: "hello".to_string(),
                                node: Node::new(4, 9)
                            },
                            init: None,
                            node: Node::new(4, 9)
                        },
                        VariableDeclarator {
                            id: Identifier {
                                name: "world".to_string(),
                                node: Node::new(11, 16)
                            },
                            init: None,
                            node: Node::new(11, 16)
                        }
                    ],
                    node: Node::new(0, 17)
                })
            ))],
            node: Node::new(0, 17),
            source_type: ProgramSourceTypes::Module
        }
    );

    assert_parser_eq!(
        "var hello, world;",
        Program {
            body: vec![ProgramBody::Statement(Statement::Declaration(
                Declaration::Variable(VariableDeclaration {
                    kind: VariableKind::Var,
                    declarations: vec![
                        VariableDeclarator {
                            id: Identifier {
                                name: "hello".to_string(),
                                node: Node::new(4, 9)
                            },
                            init: None,
                            node: Node::new(4, 9)
                        },
                        VariableDeclarator {
                            id: Identifier {
                                name: "world".to_string(),
                                node: Node::new(11, 16)
                            },
                            init: None,
                            node: Node::new(11, 16)
                        }
                    ],
                    node: Node::new(0, 17)
                })
            ))],
            node: Node::new(0, 17),
            source_type: ProgramSourceTypes::Module
        }
    );
}

#[test]
fn simple_object_binding_pattern() {
    assert_parser_eq!(
        r"const {} = null",
        Program {
            body: vec![ProgramBody::Statement(Statement::Declaration(
                Declaration::Variable(VariableDeclaration {
                    kind: VariableKind::Let,
                    declarations: vec![
                        VariableDeclarator {
                            id: Identifier {
                                name: "hello".to_string(),
                                node: Node::new(4, 9)
                            },
                            init: None,
                            node: Node::new(4, 9)
                        },
                        VariableDeclarator {
                            id: Identifier {
                                name: "world".to_string(),
                                node: Node::new(11, 16)
                            },
                            init: None,
                            node: Node::new(11, 16)
                        }
                    ],
                    node: Node::new(0, 17)
                })
            ))],
            node: Node::new(0, 17),
            source_type: ProgramSourceTypes::Module
        }
    );
}
