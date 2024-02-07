use crate::{Parser, ParserError, TokenKind};
use hippo_estree::*;

fn march_token_kind_to_update_operator(token_kind: &TokenKind) -> Option<UpdateOperator> {
    println!("march_token_kind_to_update_operator: {:?}", token_kind);
    match token_kind {
        TokenKind::Increment => Some(UpdateOperator::PlusPlus),
        TokenKind::Decrement => Some(UpdateOperator::MinusMinus),
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 13.4 Update Expressions
    // https://tc39.es/ecma262/#prod-UpdateExpression
    pub(crate) fn parse_update_expression(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        let mut operator_token_kind = self.cursor.current_token_kind();

        if operator_token_kind.is_unary_operator() {
            self.cursor.advance(); // Eat the ++ or -- token.

            let operator = march_token_kind_to_update_operator(&operator_token_kind).unwrap();

            let unary_expression = self.parse_unary_expression()?;

            return Ok(Expression::Update(UpdateExpression {
                node: self.end_node()?,
                operator,
                argument: Box::new(unary_expression),
                prefix: true,
            }));
        }

        let left_hand_side_expression = self.parse_left_hand_side_expression()?;

        if !&self.cursor.current_token_kind().is_unary_operator() {
            self.end_node()?;

            return Ok(left_hand_side_expression);
        }

        operator_token_kind = self.cursor.current_token_kind();

        self.expect_one_of_and_advance(vec![TokenKind::Increment, TokenKind::Decrement])?;

        let update_operator = march_token_kind_to_update_operator(&operator_token_kind).unwrap();

        Ok(Expression::Update(UpdateExpression {
            node: self.end_node()?,
            operator: update_operator,
            argument: Box::new(left_hand_side_expression),
            prefix: false,
        }))
    }
}
