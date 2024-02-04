mod assignment;
mod binary_or_logical;
mod conditional;
mod identifier;
mod left_hand_side;
mod primary;
mod unary;
mod update;

use crate::{Parser, ParserError, TokenKind};
use hippo_estree::*;

fn march_token_kind_to_update_operator(token_kind: &TokenKind) -> Option<UpdateOperator> {
    match token_kind {
        TokenKind::Increment => Some(UpdateOperator::PlusPlus),
        TokenKind::Decrement => Some(UpdateOperator::MinusMinus),
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Expression
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        let assignment_expression = self.parse_assignment_expression()?;

        if self.cursor.current_token_kind() == TokenKind::Comma {
            self.expect_and_advance(TokenKind::Comma)?;

            let right = self.parse_expression()?;

            return Ok(Expression::Sequence(SequenceExpression {
                node: self.end_node()?,
                expressions: vec![assignment_expression, right],
            }));
        } else {
            Ok(assignment_expression)
        }
    }
}
