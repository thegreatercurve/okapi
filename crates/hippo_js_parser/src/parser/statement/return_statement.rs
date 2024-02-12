use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.10 The return Statement
    // https://tc39.es/ecma262/#prod-ReturnStatement
    pub(crate) fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Return))?;

        let argument = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect_optional_semicolon_and_advance();

        Ok(Statement::Return(ReturnStatement {
            node: self.end_node()?,
            argument,
        }))
    }
}
