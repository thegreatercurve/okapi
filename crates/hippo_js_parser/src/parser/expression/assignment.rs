use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

fn match_token_kind_to_assignment_operator(token_kind: &TokenKind) -> Option<AssignmentOperator> {
    match token_kind {
        TokenKind::MultiplyAssignment => Some(AssignmentOperator::MultiplyAssignment),
        TokenKind::DivisionAssignment => Some(AssignmentOperator::DivisionAssignment),
        TokenKind::ModulusAssignment => Some(AssignmentOperator::ModulusAssignment),
        TokenKind::AdditionAssignment => Some(AssignmentOperator::AdditionAssignment),
        TokenKind::MinusAssignment => Some(AssignmentOperator::MinusAssignment),
        TokenKind::LeftShiftAssignment => Some(AssignmentOperator::LeftShiftAssignment),
        TokenKind::RightShiftAssignment => Some(AssignmentOperator::RightShiftAssignment),
        TokenKind::UnsignedRightShiftAssignment => {
            Some(AssignmentOperator::UnsignedRightShiftAssignment)
        }
        TokenKind::BitwiseAndAssignment => Some(AssignmentOperator::BitwiseAndAssignment),
        TokenKind::BitwiseOrAssignment => Some(AssignmentOperator::BitwiseOrAssignment),
        TokenKind::BitwiseXorAssignment => Some(AssignmentOperator::BitwiseXorAssignment),
        TokenKind::ExponentiationAssignment => Some(AssignmentOperator::ExponentiationAssignment),
        TokenKind::LogicalOrAssignment => Some(AssignmentOperator::LogicalOrAssignment),
        TokenKind::LogicalAndAssignment => Some(AssignmentOperator::LogicalAndAssignment),
        TokenKind::NullishCoalescingAssignment => {
            Some(AssignmentOperator::NullishCoalescingAssignment)
        }
        TokenKind::Assignment => Some(AssignmentOperator::Assignment), // TODO Remove this.
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 13.15 Assignment Operators
    // https://tc39.es/ecma262/#prod-AssignmentExpression
    pub(crate) fn parse_assignment_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.
        if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::Yield) {
            return self.parse_yield_expression();
        }

        if self.cursor.current_token_kind() == TokenKind::Identifier {
            // TODO Handle CoverParenthesizedExpressionAndArrowParameterList.
            if self.cursor.peek_token_kind() == TokenKind::ArrowFunction {
                return self.parse_arrow_function();
            }
        }

        // TODO Handle async arrow functions.

        self.start_node();

        let left_expression = self.parse_conditional_expression()?;

        match self.cursor.current_token_kind() {
            // TokenKind::Assignment => {
            //     self.cursor.advance(); // Eat the assignment operator token.

            //     let right = self.parse_assignment_expression()?;

            //     // TODO Handle ArrayAssignmentPattern and ObjectAssignmentPattern similar to ArrayLiteral and ObjectLiteral binding patterns:

            //     // https://tc39.es/ecma262/#sec-assignment-operators-static-semantics-early-errors
            //     let left_array_or_object_pattern = match &left_expression {
            //         // Expression::Array(array_expression) => array_expression.to_pattern(),
            //         // Expression::Object(object_expression) => object_expression.to_pattern(),
            //         _ => None,
            //     }
            //     .unwrap();

            //     return Ok(Expression::Assignment(AssignmentExpression {
            //         node: self.end_node()?,
            //         operator: AssignmentOperator::Assignment,
            //         left: Box::new(left_array_or_object_pattern),
            //         right: Box::new(right),
            //     }));
            // }
            current_token_kind if current_token_kind.is_assignment_operator() => {
                let operator =
                    match_token_kind_to_assignment_operator(&self.cursor.current_token_kind())
                        .unwrap();

                self.cursor.advance(); // Eat the assignment operator token.

                let right = self.parse_assignment_expression()?;

                return Ok(Expression::Assignment(AssignmentExpression {
                    node: self.end_node()?,
                    operator,
                    left: Box::new(AssignmentExpressionLeft::Expression(left_expression)),
                    right: Box::new(right),
                }));
            }
            _ => {}
        }

        Ok(left_expression)
    }
}
