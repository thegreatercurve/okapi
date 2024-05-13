use crate::ast::*;
use crate::{Parser, ParserError, TokenKind};

fn match_token_kind_to_update_operator(token_kind: &TokenKind) -> Option<UpdateOperator> {
    match token_kind {
        TokenKind::Increment => Some(UpdateOperator::PlusPlus),
        TokenKind::Decrement => Some(UpdateOperator::MinusMinus),
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.4 Update Expressions
    // https://tc39.es/ecma262/#prod-UpdateExpression
    pub(crate) fn parse_update_expression(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        let mut operator_token_kind = self.token_kind();

        if operator_token_kind.is_update_operator() {
            self.advance_any(); // Eat '++' or '--' token.

            let operator = match_token_kind_to_update_operator(&operator_token_kind).unwrap();

            let unary_expression = self.parse_unary_expression()?;

            return Ok(Expression::Update(UpdateExpression {
                node: self.end_node(start_index)?,
                operator,
                argument: Box::new(unary_expression),
                prefix: true,
            }));
        }

        let left_hand_side_expression = self.parse_left_hand_side_expression()?;

        if !&self.token_kind().is_update_operator() {
            return Ok(left_hand_side_expression);
        }

        operator_token_kind = self.token_kind();

        self.expect_one_of_and_advance(vec![TokenKind::Increment, TokenKind::Decrement])?;

        let update_operator = match_token_kind_to_update_operator(&operator_token_kind).unwrap();

        Ok(Expression::Update(UpdateExpression {
            node: self.end_node(start_index)?,
            operator: update_operator,
            argument: Box::new(left_hand_side_expression),
            prefix: false,
        }))
    }
}
