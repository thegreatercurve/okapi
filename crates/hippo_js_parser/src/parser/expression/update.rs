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
    // 13.4 Update Expressions
    // https://tc39.es/ecma262/#prod-UpdateExpression
    pub(crate) fn parse_update_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let mut current_token_kind = self.cursor.current_token_kind();

        if current_token_kind.is_unary_operator() {
            self.cursor.advance(); // Eat the ++ or -- token.

            let unary_expression = self.parse_unary_expression()?;

            let operator = march_token_kind_to_update_operator(&current_token_kind).unwrap();

            Ok(Expression::Update(UpdateExpression {
                node: self.create_node(&start_token, &self.cursor.current_token),
                operator,
                argument: Box::new(unary_expression),
                prefix: true,
            }))
        } else {
            let left_hand_side_expression = self.parse_left_hand_side_expression()?;

            current_token_kind = self.cursor.current_token_kind();

            if !&current_token_kind.is_unary_operator() {
                return Ok(left_hand_side_expression);
            }

            let update_operator = march_token_kind_to_update_operator(&current_token_kind).unwrap();

            Ok(Expression::Update(UpdateExpression {
                node: self.create_node(&start_token, &self.cursor.current_token),
                operator: update_operator,
                argument: Box::new(left_hand_side_expression),
                prefix: false,
            }))
        }
    }
}
