use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.11 The throw Statement
    // https://tc39.es/ecma262/#prod-ThrowStatement
    pub(crate) fn parse_throw_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Throw))?;

        let argument = self.parse_expression()?;

        self.expect_optional_semicolon_and_advance();

        Ok(Statement::Throw(ThrowStatement {
            node: self.end_node()?,
            argument,
        }))
    }
}
