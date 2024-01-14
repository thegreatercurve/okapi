use std::vec;

use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

fn is_token_kind_unary_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Increment | TokenKind::Decrement => true,
        _ => false,
    }
}

fn is_token_kind_multiplicative_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Multiplication | TokenKind::Division | TokenKind::Modulus => true,
        _ => false,
    }
}

fn is_token_kind_additive_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Addition | TokenKind::Subtraction => true,
        _ => false,
    }
}

fn march_token_kind_to_update_operator(token_kind: &TokenKind) -> Option<UpdateOperator> {
    match token_kind {
        TokenKind::Increment => Some(UpdateOperator::PlusPlus),
        TokenKind::Decrement => Some(UpdateOperator::MinusMinus),
        _ => None,
    }
}

fn match_token_kind_to_unary_operator(token_kind: &TokenKind) -> Option<UnaryOperator> {
    match token_kind {
        TokenKind::Keyword(KeywordKind::Delete) => Some(UnaryOperator::Delete),
        TokenKind::Keyword(KeywordKind::Void) => Some(UnaryOperator::Void),
        TokenKind::Keyword(KeywordKind::Typeof) => Some(UnaryOperator::Typeof),
        TokenKind::Addition => Some(UnaryOperator::Plus),
        TokenKind::Subtraction => Some(UnaryOperator::Minus),
        TokenKind::BitwiseNot => Some(UnaryOperator::Tilde),
        TokenKind::LogicalNot => Some(UnaryOperator::Bang),
        _ => None,
    }
}

fn match_token_kind_to_multiplicative_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::Multiplication => Some(BinaryOperator::Star),
        TokenKind::Division => Some(BinaryOperator::Slash),
        TokenKind::Modulus => Some(BinaryOperator::Percent),
        _ => None,
    }
}

fn match_token_kind_to_additive_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::Addition => Some(BinaryOperator::Plus),
        TokenKind::Subtraction => Some(BinaryOperator::Minus),
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Expression
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let left = self.parse_assignment_expression()?;

        if self.current_token_kind() == TokenKind::Comma {
            self.expect_and_advance(TokenKind::Comma)?;

            let right = self.parse_expression()?;

            return Ok(Expression::Sequence(SequenceExpression {
                node: self.finish_node(&node),
                expressions: vec![left, right],
            }));
        } else {
            // Ok(Expression::Assignment(AssignmentExpression {
            //     node: self.finish_node(&node),
            //     operator: todo!(),
            //     left,
            //     right: todo!(),
            // }))
            todo!()
        }
    }

    // https://tc39.es/ecma262/#prod-AssignmentExpression
    fn parse_assignment_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let left = self.parse_conditional_expression()?;

        todo!()
    }

    // https://tc39.es/ecma262/#prod-ConditionalExpression
    fn parse_conditional_expression(&mut self) -> Result<ConditionalExpression, ParserError> {
        let node = self.start_node();

        let short_circuit = self.parse_short_circuit_expression()?;

        todo!()
    }

    // https://tc39.es/ecma262/#prod-ShortCircuitExpression
    fn parse_short_circuit_expression(&mut self) -> Result<ConditionalExpression, ParserError> {
        let node = self.start_node();

        // let logical = self.parse_logical_or_expression()?;

        todo!()
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        todo!()
    }

    // 13.3 Left-Hand-Side Expressions
    // https://tc39.es/ecma262/#prod-LeftHandSideExpression
    fn parse_left_hand_side_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        todo!()
    }

    // 13.4 Update Expressions
    // https://tc39.es/ecma262/#prod-UpdateExpression
    fn parse_update_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let mut current_token_kind = self.current_token_kind();

        if is_token_kind_unary_operator(&current_token_kind) {
            self.advance(); // Eat the ++ or -- token.

            let unary_expression = self.parse_unary_expression()?;

            let operator = march_token_kind_to_update_operator(&current_token_kind).unwrap();

            Ok(Expression::Update(UpdateExpression {
                node: self.finish_node(&node),
                operator,
                argument: Box::new(unary_expression),
                prefix: true,
            }))
        } else {
            let left_hand_side_expression = self.parse_left_hand_side_expression()?;

            current_token_kind = self.current_token_kind();

            if !is_token_kind_unary_operator(&current_token_kind) {
                return Ok(left_hand_side_expression);
            }

            let update_operator = march_token_kind_to_update_operator(&current_token_kind).unwrap();

            Ok(Expression::Update(UpdateExpression {
                node: self.finish_node(&node),
                operator: update_operator,
                argument: Box::new(left_hand_side_expression),
                prefix: false,
            }))
        }
    }

    // 13.5 Unary Operators
    // https://tc39.es/ecma262/#prod-UnaryExpression
    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let current_token_kind = self.current_token_kind();

        match current_token_kind {
            TokenKind::Keyword(KeywordKind::Delete)
            | TokenKind::Keyword(KeywordKind::Void)
            | TokenKind::Keyword(KeywordKind::Typeof)
            | TokenKind::Addition
            | TokenKind::Subtraction
            | TokenKind::BitwiseNot
            | TokenKind::LogicalNot => {
                let unary_argument = self.parse_unary_expression()?;

                let operator = match_token_kind_to_unary_operator(&current_token_kind).unwrap();

                self.advance();

                Ok(Expression::Unary(UnaryExpression {
                    node: self.finish_node(&node),
                    operator,
                    prefix: true,
                    argument: Box::new(unary_argument),
                }))
            }
            TokenKind::Keyword(KeywordKind::Await) => {
                // TODO check if is supported by ECMA script version.
                Ok(Expression::Await(AwaitExpression {
                    node: self.finish_node(&node),
                    argument: Box::new(self.parse_unary_expression()?),
                }))
            }
            _ => self.parse_update_expression(),
        }
    }

    // 13.6 Exponentiation Operators
    // https://tc39.es/ecma262/#prod-ExponentiationExpression
    fn parse_exponentiation_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let unary_expression = self.parse_unary_expression()?;

        if self.current_token_kind() == TokenKind::Exponentiation {
            self.advance(); // Eat the ** token.

            Ok(Expression::Binary(BinaryExpression {
                node: self.finish_node(&node),
                operator: BinaryOperator::StarStar,
                left: Box::new(unary_expression),
                right: Box::new(self.parse_exponentiation_expression()?),
            }))
        } else {
            Ok(unary_expression)
        }
    }

    // 13.7 Multiplicative Operators
    // https://tc39.es/ecma262/#prod-MultiplicativeExpression
    fn parse_multiplicative_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let exponentiation_expression = self.parse_exponentiation_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_multiplicative_operator(&current_token_kind) {
            self.advance(); // Eat the * or / or % token.

            let operator =
                match_token_kind_to_multiplicative_operator(&current_token_kind).unwrap();

            Ok(Expression::Binary(BinaryExpression {
                node: self.finish_node(&node),
                operator,
                left: Box::new(exponentiation_expression),
                right: Box::new(self.parse_multiplicative_expression()?),
            }))
        } else {
            Ok(exponentiation_expression)
        }
    }

    // 13.8 Additive Operators
    // https://tc39.es/ecma262/#prod-AdditiveExpression
    fn parse_additive_expression(&mut self) -> Result<Expression, ParserError> {
        let node = self.start_node();

        let multiplicative_expression = self.parse_multiplicative_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_additive_operator(&current_token_kind) {
            self.advance(); // Eat the + or - token.

            let operator = match_token_kind_to_additive_operator(&current_token_kind).unwrap();

            let additive_expression = self.parse_additive_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.finish_node(&node),
                operator,
                left: Box::new(additive_expression),
                right: Box::new(multiplicative_expression),
            }))
        } else {
            Ok(multiplicative_expression)
        }
    }

    // 13.12 Binary Bitwise Operators
    // https://tc39.es/ecma262/#prod-BitwiseANDExpression
    fn parse_bitwise_and_expression(&mut self) -> Result<ConditionalExpression, ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-BitwiseXORExpression
    fn parse_bitwise_xor_expression(&mut self) -> Result<ConditionalExpression, ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-BitwiseORExpression
    fn parse_bitwise_or_expression(&mut self) -> Result<ConditionalExpression, ParserError> {
        todo!()
    }
}
