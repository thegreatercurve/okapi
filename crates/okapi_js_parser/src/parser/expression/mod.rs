mod assignment;
mod binary_or_logical;
mod conditional;
mod identifier;
mod left_hand_side;
mod primary;
mod unary;
mod update;

use crate::ast::*;
use crate::{Parser, ParserError, TokenKind};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // https://tc39.es/ecma262/#prod-Expression
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        let assignment_expression = self.parse_assignment_expression()?;

        let mut expressions = vec![assignment_expression.clone()];

        while self.token_kind() == TokenKind::Comma {
            self.advance_any(); // Eat ',' token.

            let expression = self.parse_assignment_expression()?;

            expressions.push(expression);
        }

        if expressions.len() > 1 {
            return Ok(Expression::Sequence(SequenceExpression {
                node: self.end_node(start_index)?,
                expressions,
            }));
        }

        Ok(assignment_expression)
    }
}
