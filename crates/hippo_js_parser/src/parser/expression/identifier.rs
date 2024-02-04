use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};
use hippo_estree::*;

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 13.1 Identifiers
    // https://tc39.es/ecma262/#prod-IdentifierReference
    pub(crate) fn parse_identifier_reference(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete. Nede to handle yield and await.

        self.start_node();

        let token_value = self.cursor.current_token_value();

        self.expect_one_of_and_advance(vec![
            TokenKind::Identifier,
            TokenKind::Keyword(KeywordKind::Await),
            TokenKind::Keyword(KeywordKind::Yield),
        ])?;

        match token_value {
            TokenValue::String(name) => Ok(Expression::Identifier(Identifier {
                node: self.end_node()?,
                name,
            })),
            _ => Err(ParserError::UnexpectedTokenValue),
        }
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        todo!("parse_label_identifier")
    }
}
