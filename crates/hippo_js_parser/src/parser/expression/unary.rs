use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 13.5 Unary Operators
// https://tc39.es/ecma262/#prod-UnaryExpression
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

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.5 Unary Operators
    // https://tc39.es/ecma262/#prod-UnaryExpression
    pub(crate) fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        match self.token_kind() {
            TokenKind::Keyword(KeywordKind::Delete)
            | TokenKind::Keyword(KeywordKind::Void)
            | TokenKind::Keyword(KeywordKind::Typeof)
            | TokenKind::Addition
            | TokenKind::Subtraction
            | TokenKind::BitwiseNot
            | TokenKind::LogicalNot => {
                let start_index = self.start_node();

                let Some(operator) = match_token_kind_to_unary_operator(&self.token_kind()) else {
                    return Err(self.unexpected_current_token_kind());
                };

                self.advance_any(); // Eat delete or void or typeof or + or - or ~ or ! token.

                let unary_argument = self.parse_unary_expression()?;

                Ok(Expression::Unary(UnaryExpression {
                    node: self.end_node(start_index)?,
                    operator,
                    prefix: true,
                    argument: Box::new(unary_argument),
                }))
            }
            TokenKind::Keyword(KeywordKind::Await) => {
                let start_index = self.start_node();

                self.advance_any(); // Eat `await` token.

                let unary_expression = self.parse_unary_expression()?;

                // TODO check if is supported by ECMA script version.
                Ok(Expression::Await(AwaitExpression {
                    node: self.end_node(start_index)?,
                    argument: Box::new(unary_expression),
                }))
            }
            _ => self.parse_update_expression(),
        }
    }
}
