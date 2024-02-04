use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

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
impl<'a> Parser<'a> {
    // 13.5 Unary Operators
    // https://tc39.es/ecma262/#prod-UnaryExpression
    pub(crate) fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        let current_token_kind = self.cursor.current_token_kind();

        match current_token_kind {
            TokenKind::Keyword(KeywordKind::Delete)
            | TokenKind::Keyword(KeywordKind::Void)
            | TokenKind::Keyword(KeywordKind::Typeof)
            | TokenKind::Addition
            | TokenKind::Subtraction
            | TokenKind::BitwiseNot
            | TokenKind::LogicalNot => {
                self.start_node();

                self.cursor.advance(); // Eat the delete or void or typeof or + or - or ~ or ! token.

                let unary_argument = self.parse_unary_expression()?;

                let operator =
                    match_token_kind_to_unary_operator(&self.cursor.current_token_kind()).unwrap();

                self.cursor.advance();

                Ok(Expression::Unary(UnaryExpression {
                    node: self.end_node()?,
                    operator,
                    prefix: true,
                    argument: Box::new(unary_argument),
                }))
            }
            TokenKind::Keyword(KeywordKind::Await) => {
                self.start_node();

                let unary_expression = self.parse_unary_expression()?;

                // TODO check if is supported by ECMA script version.
                Ok(Expression::Await(AwaitExpression {
                    node: self.end_node()?,
                    argument: Box::new(unary_expression),
                }))
            }
            _ => self.parse_update_expression(),
        }
    }
}
