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

        let identifier_name = match token_value {
            TokenValue::String(name) => name,
            _ => return Err(ParserError::UnexpectedTokenValue),
        };

        self.expect_one_of_and_advance(vec![
            TokenKind::Identifier,
            TokenKind::Keyword(KeywordKind::Await),
            TokenKind::Keyword(KeywordKind::Yield),
        ])?;

        Ok(Expression::Identifier(Identifier {
            node: self.end_node()?,
            name: identifier_name,
        }))
    }

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    pub(crate) fn parse_binding_identifier(&mut self) -> Result<Identifier, ParserError> {
        self.start_node();

        let token_value = match self.cursor.current_token_kind() {
            TokenKind::Identifier => self.cursor.current_token_value(),
            TokenKind::Keyword(KeywordKind::Await) => todo!("parse_binding_identifier await"),
            TokenKind::Keyword(KeywordKind::Yield) => todo!("parse_binding_identifier yield"),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let identifier_name = match token_value {
            TokenValue::String(name) => name,
            _ => return Err(ParserError::UnexpectedTokenValue),
        };

        self.expect_one_of_and_advance(vec![
            TokenKind::Identifier,
            TokenKind::Keyword(KeywordKind::Await),
            TokenKind::Keyword(KeywordKind::Yield),
        ])?; // Eat identifier token.

        Ok(Identifier {
            node: self.end_node()?,
            name: identifier_name,
        })
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        todo!("parse_label_identifier")
    }
}
