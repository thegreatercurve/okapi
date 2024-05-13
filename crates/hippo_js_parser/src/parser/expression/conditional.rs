use crate::ast::*;
use crate::{Parser, ParserError, TokenKind};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.14 Conditional Operator ( ? : )
    // https://tc39.es/ecma262/#prod-ConditionalExpression
    pub(crate) fn parse_conditional_expression(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        let short_circuit_expression = self.parse_binary_expression(0)?;

        if self.token_kind() == TokenKind::QuestionMark {
            self.advance_any(); // Eat '?' token.

            let consequent = self.parse_assignment_expression()?;

            self.expect_and_advance(TokenKind::Colon)?;

            let alternate = self.parse_assignment_expression()?;

            return Ok(Expression::Conditional(ConditionalExpression {
                node: self.end_node(start_index)?,
                test: Box::new(short_circuit_expression),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
            }));
        }

        self.end_node(start_index)?;

        Ok(short_circuit_expression)
    }
}
