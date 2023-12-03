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
        "1 + 1 + 1",
        Program {
            body: vec![ProgramBody::Statement(StatementData::Expression(
                ExpressionStatement {
                    expression: ExpressionData::Binary(BinaryExpression {
                        left: Box::new(ExpressionData::Binary(BinaryExpression {
                            operator: BinaryOperator::Plus,
                            left: Box::new(Literal {
                                value: LiteralValue::Number(1)
                            }),
                            right: Box::new(Literal {
                                value: LiteralValue::Number(1)
                            }),
                        })),
                        operator: BinaryOperator::Plus,
                        right: Box::new(ExpressionData::Literal(1)),
                    })
                }
            ))]
        }
    );
    assert_parser_eq!(
        "5 + 5 * 10",
        Program {
            body: vec![StatementData { expression: None }]
        }
    );
}
