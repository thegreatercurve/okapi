use crate::Parser;
use hippo_estree::*;

macro_rules! assert_parser_eq {
    ($input_str: expr, $expected_ast: expr) => {{
        let mut parser = Parser::new($input_str);

        assert_eq!(parser.parse(), $expected_ast);
    }};
}

#[test]
fn parser() {
    assert_parser_eq!(
        "var foo = 'bar'",
        Program {
            body: vec![ProgramBody::Statement(StatementData::Declaration(
                DeclarationData::Variable(VariableDeclaration {
                    kind: VariableKind::Var,
                    declarations: vec![VariableDeclarator {
                        id: Pattern::Identifier(Identifier {
                            name: "foo".to_string()
                        }),
                        init: Some(ExpressionData::Literal(Literal {
                            value: LiteralValue::String("bar".to_string())
                        }))
                    }]
                })
            ))]
        }
    );

    assert_parser_eq!(
        "1 + 1 + 1",
        Program {
            body: vec![ProgramBody::Statement(StatementData::Expression(
                ExpressionStatement {
                    expression: ExpressionData::Binary(BinaryExpression {
                        left: Box::new(ExpressionData::Binary(BinaryExpression {
                            operator: BinaryOperator::Plus,
                            left: Box::new(ExpressionData::Literal(Literal {
                                value: LiteralValue::Number(1.0)
                            })),
                            right: Box::new(ExpressionData::Literal(Literal {
                                value: LiteralValue::Number(1.0)
                            })),
                        })),
                        operator: BinaryOperator::Plus,
                        right: Box::new(ExpressionData::Literal(Literal {
                            value: LiteralValue::Number(1.0)
                        })),
                    })
                }
            ))]
        }
    );

    assert_parser_eq!(
        "5 + 5 * 10",
        Program {
            body: vec![ProgramBody::Statement(StatementData::Expression(
                ExpressionStatement {
                    expression: ExpressionData::Binary(BinaryExpression {
                        left: Box::new(ExpressionData::Literal(Literal {
                            value: LiteralValue::Number(5.0)
                        })),
                        operator: BinaryOperator::Plus,
                        right: Box::new(ExpressionData::Binary(BinaryExpression {
                            left: Box::new(ExpressionData::Literal(Literal {
                                value: LiteralValue::Number(5.0)
                            })),
                            operator: BinaryOperator::Star,
                            right: Box::new(ExpressionData::Literal(Literal {
                                value: LiteralValue::Number(10.0)
                            })),
                        })),
                    })
                }
            ))]
        }
    );
}
