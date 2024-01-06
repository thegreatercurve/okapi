use crate::parser::common::assert_parser_eq;

#[test]
fn binding_identifier() {
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
                    node: Node::new(0, 9)
                })
            ))],
            node: Node::new(0, 10),
            source_type: ProgramSourceTypes::Module
        }
    );
}

#[test]
fn binding_list() {
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
                    node: Node::new(0, 16)
                })
            ))],
            node: Node::new(0, 17),
            source_type: ProgramSourceTypes::Module
        }
    );
}

// assert_parser_eq!(
//     "var foo = 'bar'",
//     Program {
//         body: vec![ProgramBody::Statement(Statement::Declaration(
//             DeclarationData::Variable(VariableDeclaration {
//                 kind: VariableKind::Var,
//                 declarations: vec![VariableDeclarator {
//                     id: Identifier {
//                         name: "foo".to_string(),
//                         node: todo!()
//                     },
//                     init: Some(Expression::Literal(Literal {
//                         value: LiteralValue::String("bar".to_string()),
//                         node: todo!()
//                     })),
//                     node: todo!()
//                 }],
//                 node: todo!()
//             })
//         ))],
//         node: todo!(),
//         source_type: todo!()
//     }
// );

// assert_parser_eq!(
//     "1 + 1 + 1",
//     Program {
//         body: vec![ProgramBody::Statement(Statement::Expression(
//             ExpressionStatement {
//                 expression: Expression::Binary(BinaryExpression {
//                     left: Box::new(Expression::Binary(BinaryExpression {
//                         operator: BinaryOperator::Plus,
//                         left: Box::new(Expression::Literal(Literal {
//                             value: LiteralValue::Number(1.0),
//                             node: todo!()
//                         })),
//                         right: Box::new(Expression::Literal(Literal {
//                             value: LiteralValue::Number(1.0),
//                             node: todo!()
//                         })),
//                         node: todo!()
//                     })),
//                     operator: BinaryOperator::Plus,
//                     right: Box::new(Expression::Literal(Literal {
//                         value: LiteralValue::Number(1.0),
//                         node: todo!()
//                     })),
//                     node: todo!()
//                 }),
//                 node: todo!()
//             }
//         ))],
//         node: todo!(),
//         source_type: todo!()
//     }
// );

// assert_parser_eq!(
//     "5 + 5 * 10",
//     Program {
//         body: vec![ProgramBody::Statement(Statement::Expression(
//             ExpressionStatement {
//                 expression: Expression::Binary(BinaryExpression {
//                     left: Box::new(Expression::Literal(Literal {
//                         value: LiteralValue::Number(5.0),
//                         node: todo!()
//                     })),
//                     operator: BinaryOperator::Plus,
//                     right: Box::new(Expression::Binary(BinaryExpression {
//                         left: Box::new(Expression::Literal(Literal {
//                             value: LiteralValue::Number(5.0),
//                             node: todo!()
//                         })),
//                         operator: BinaryOperator::Star,
//                         right: Box::new(Expression::Literal(Literal {
//                             value: LiteralValue::Number(10.0),
//                             node: todo!()
//                         })),
//                         node: todo!()
//                     })),
//                     node: todo!()
//                 }),
//                 node: todo!()
//             }
//         ))],
//         node: todo!(),
//         source_type: todo!()
//     }
// );
